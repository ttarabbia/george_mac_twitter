use clap::{Arg, Command};
// use oauth2::RefreshToken;
use time::OffsetDateTime;   
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{Duration, SystemTime};
use std::{borrow::Cow, collections::HashMap, env, fs};
use std::io::{self, Write};
use twitter_v2::authorization::{Oauth2Client, Oauth2Token, Scope};
use twitter_v2::oauth2::{AuthorizationCode, PkceCodeChallenge, PkceCodeVerifier};
use twitter_v2::TwitterApi;

#[derive(Debug, Deserialize, Serialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Candidate {
    content: Content,
}

#[derive(Debug, Deserialize, Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Part {
    text: String,
}

pub fn extract_text_from_response(
    response_body: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let response: GeminiResponse = serde_json::from_str(response_body)?;

    let texts: Vec<String> = response
        .candidates
        .into_iter()
        .flat_map(|candidate| candidate.content.parts)
        .map(|part| part.text)
        .collect();

    Ok(texts)
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("read File failed")
}

pub fn split_books(split_str: String, books: &String) -> Vec<&str> {
    // first line is book title - into hashmap?
    books.split(&split_str).collect()
}

pub async fn call_gemini(
    prompt: &String,
    book: &String,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let api_key = get_env_var_or_fallback("GOOGLE_API_KEY", "API_KEY")?;
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
        api_key
    );

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&json!({
            "contents": [
                {
                    "role": "user",
                    "parts": [
                        {
                            "text": format!("{},\n\n{}\n\n,{}", prompt, book, prompt)
                        }
                    ]
                }
            ]
        }))
        .send()
        .await?;

    let body = response.text().await?;

    let texts = match extract_text_from_response(&body) {
        Ok(texts) => {
            for text in &texts {
                println!("Second: \n {} \n\n", text);
            }
            texts
        }
        Err(e) => {
            println!("Error extractin: {}", e);
            Vec::new()
        }
    };

    Ok(texts)
}

pub fn get_env_var_or_fallback(var1: &str, var2: &str) -> Result<String, std::env::VarError> {
    match std::env::var(var1) {
        Ok(val) => Ok(val),
        Err(_) => match std::env::var(var2) {
            Ok(val) => Ok(val),
            Err(e) => Err(e),
        },
    }
}

pub fn extract_args() -> clap::ArgMatches {
    let matches = Command::new("gmac_tweet")
        .version("0.3")
        .author("Thomas ttarabbia@gmail.com")
        .about("Generate Tweets from an author")
        .arg(
            Arg::new("keywords")
                .short('k')
                .long("keywords")
                .value_parser(clap::value_parser!(String))
                .num_args(1..)
                .help("Pass keywords to focus the quotes from the book"),
        )
        .arg(
            Arg::new("character")
                .short('c')
                .long("character")
                .value_parser(clap::value_parser!(String))
                .help("Pass a character from the book to focus the quotes"),
        )
        .get_matches();

    matches
}


pub async fn auth_and_tweet(
    tweet: &String,
    book_title: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let oauth2_token = auth().await?;

    // Post a tweet
    let tweet_text = format!("{}\n        -{}", tweet, book_title).to_string();

    println!("Length: {}", tweet_text.chars().count());

    if !tweet_text.is_empty() {
        post_tweet(&oauth2_token, &tweet_text).await?;
    }

    Ok(())
}

async fn post_tweet(
    oauth2_token: &Oauth2Token,
    tweet_text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("OAuth2 token: {:?}", oauth2_token);
    let api = TwitterApi::new(oauth2_token.clone());
    println!("Posting tweet: {}", tweet_text);

    if let Err(e) = api.post_tweet().text(tweet_text.to_string()).send().await {
        eprintln!("Failed to post tweet: {}", e);
    } else {
        println!("Tweet posted successfully!");
    }

    Ok(())
}

async fn auth() -> Result<Oauth2Token, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // Get client_id and client_secret from env variables
    let client_id = env::var("TWITTER_OAUTH_CLIENT_ID").expect("TWITTER_OAUTH_CLIENT_ID not set");
    let client_secret =
        env::var("TWITTER_OAUTH_CLIENT_SECRET").expect("TWITTER_OAUTH_CLIENT_SECRET not set");

    // Use a dummy callback URL
    let callback_url = Url::parse("http://localhost:8080/redirect").unwrap();

    // Create Twitter OAuth2 client
    let twitter_oauth2_client = Oauth2Client::new(client_id, client_secret, callback_url);

    // Set scopes
    let scopes = [Scope::TweetRead, Scope::TweetWrite, Scope::UsersRead, Scope::OfflineAccess];

    // Generate auth URL
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, _state) = twitter_oauth2_client.auth_url(challenge, scopes);

    println!("Please open this URL in your browser:");
    println!("{}", auth_url);
    println!("After authorizing, you will be redirected to a blank page. Copy the 'code' parameter from the URL.");

    print!("Enter the code: ");
    io::stdout().flush()?;

    let mut code = String::new();
    io::stdin().read_line(&mut code)?;
    let code = code.trim();

    let authorization_code = AuthorizationCode::new(code.to_string());
    let code_verifier = PkceCodeVerifier::new(verifier.secret().to_string());

    // Exchange the code for a token
    let mut token_result = twitter_oauth2_client
        .request_token(authorization_code, code_verifier)
        .await?;


    let mut oauth2_token = token_result.access_token().clone();
    // twitter_oauth2_client.revoke_token(oauth2_token.clone().into()).await?;
    println!("oauth2 token {:?}", oauth2_token);
    println!("token_result {:?}", token_result);
    println!("is_expired {:?}", &token_result.is_expired());
    // twitter_oauth2_client.refresh_token_if_expired(&mut token_result).await?;
    // let refresh_token = token_result.refresh_token().expect("should have a refresh token");
    // let expires_at = SystemTime::now().checked_add(token_result.expires());
    // let expires_at = OffsetDateTime::now_utc() + token_result.expires();

    // println!("Access token: {:?}", oauth2_token);
    // println!("Refresh token: {:?}", refresh_token);
    // println!("Expires at: {:?}", expires_at);

    // refresh_token_if_expired(
    //     &twitter_oauth2_client,
    //     &mut oauth2_token.secret().to_string(),
    //     refresh_token,
    //     &mut expires_at,
    // )
    // .await?;

    // let token_result = twitter_oauth2_client.refresh_token(&refresh_token).await?;
    println!("Successfully obtained OAuth2 token!");
     

    Ok(token_result)
}

