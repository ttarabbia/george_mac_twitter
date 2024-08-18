use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::Client;

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

pub async fn call_gemini(prompt: &String, book: &String) -> Result<Vec::<String>, Box<dyn std::error::Error>> {
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
