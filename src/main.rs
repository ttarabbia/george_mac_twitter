#![allow(warnings)]
use dotenv::dotenv;
use rand;
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let book = read_file("george_mac.txt");
    let split_string = "------------";
    let contents = split_books(split_string.to_owned(), &book);

    let prompt = "You are a poet and avid reader of George MacDonald.
        Make sure to respond only in quotes from the provided book. 
        Identify quotes that evoke strong emotions or imagery and that could stand alone without commentary.
        Look for earnestness.
        Select excerpts that encapsulate the essence of the book's themes in a concise and impactful way.
        Take a deep breath and carefully choose 20 poetic or surprising quotes from this book.";

    let book = contents[29];
    book.lines().take(2).for_each(|line| println!("{}", line));
    println!("Length: {}", book.chars().count());

    let response = call_gemini(&prompt.to_string(), &book.to_string()).await?;

    let texts = match extract_text_from_response(&response) {
        Ok(texts) => {
            for text in &texts {
                println!("First: \n {} \n\n", text);
            }
            texts
        }
        Err(e) => {
            println!("Error extractin: {}", e);
            Vec::new()
        }
    };

    let prompt = "You are a poetic and literary Tweeter.
        Choose the 4 quotes you find most interesting out of the following that you would send in a up to 280 character tweet.
        Find quotes that could stand alone as poetic reflections.
        Sort them in order of level of interestingness.
        Respond only with the quotes and without commentary";

    let response = call_gemini(&prompt.to_string(), &texts[0].to_string()).await?;

    // println!("Response 2: \n{}", response);
    let _texts = match extract_text_from_response(&response) {
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

    Ok(())
}

async fn call_gemini(prompt: &String, book: &String) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");

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
    Ok(body)
}

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

fn extract_text_from_response(
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

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("read File failed")
}

fn generate_random_tweet(sentences: Vec<&str>) {
    //TODO
}

fn generate_tweet_from_word(sentences: Vec<&str>, word: &str) {
    //TODO
}

fn split_books(split_str: String, books: &String) -> Vec<&str> {
    // first line is book title - into hashmap?
    books.split(&split_str).collect()
}
