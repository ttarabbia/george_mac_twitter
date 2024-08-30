#![allow(warnings)]
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs;
use reqwest::Client;
use serde_json::json;

use utils::*;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut matches = extract_args();

    let book = read_file("george_mac.txt");
    let split_string = "------------";
    let contents = split_books(split_string.to_owned(), &book);
    let book = contents[1];

    let book_title = book.lines().nth(1).unwrap();
    println!("Title: {}", &book_title);
    println!("Length: {}", book.chars().count());

    let mut tweet = Vec::<String>::new();

    if let Some(keywords) = matches.get_many::<String>("keywords") {
        let keywords_str = keywords
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(" and ");
        tweet = generate_tweet_from_word(&book, &keywords_str).await?;
        // tweet = generate_fake_story(&book).await?;

    } else if let Some(character) = matches.get_one::<String>("character") {
        tweet = generate_tweet_from_character(&book, &character).await?;
    } else {
        tweet = generate_random_tweet(&book).await?;
    }

    auth_and_tweet(&tweet[0], &book_title).await?;

    Ok(())
}


async fn generate_fake_story(book: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let prompt = 
    "You are a funny and witty writer. 
        Make a short entertaining story out of passages from only this book. 
        Make sure the story you craft is not related to the book's story at all. 
        Emphasize a satirical theme that criticizes hard work.  
        Make sure there is a clear narrative thread and a conflict that's resolved by the end of the story. 
        Do not add new material, only use sentences from this book, separated by | that string together into a witty story, 400-600 words";

    let response = call_gemini(&prompt.to_string(), &book.to_string()).await?;


    Ok(response)
}


async fn generate_random_tweet(book: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let prompt = "You are a poet and avid reader of George MacDonald.
        Identify excerpts or sentences that evoke strong emotions or imagery and that could stand alone without commentary.
        Look for earnestness.
        Select excerpts that encapsulate the essence of the book's themes in a concise and impactful way.
        Make sure to respond only in excerpts or portions from the provided book. 
        Take a deep breath and carefully choose 20 poetic or surprising portions from this book.";

    let response = call_gemini(&prompt.to_string(), &book.to_string()).await?;

    let prompt = "You are a poetic and literary Tweeter.
        Choose 4 portions you find most interesting out of the following that you would send in a up to 280 character tweet.
        Find snippets that could stand alone as poetic reflections.
        Sort them in order of level of interestingness.
        Respond only with the text from the book and without commentary";

    let response = call_gemini(&prompt.to_string(), &response[0].to_string()).await?;

    Ok(response)
}

async fn generate_tweet_from_word(
    book: &str,
    word: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let prompt = format!("You are a poet and avid reader of George MacDonald.
        Look for earnestness.
        Find excerpt that have to do with {}
        Select excerpts that encapsulate the essence of the book's themes in a concise and impactful way.
        Each section should be unique. 
        Identify portions and sentences that evoke strong emotions or imagery and that could stand alone without commentary.
        Make sure to respond only in excerpts or portions from the provided book. 
        Take a deep breath and carefully choose 20 poetic excerpts that have to do with {} from this book.", word, word);

    let response = call_gemini(&prompt.to_string(), &book.to_string()).await?;

    let prompt = format!("You are a poetic and literary Tweeter.
        Choose 1 excerpt you find most interesting out of the following.
        Find portions that could stand alone as poetic reflections.
        Sort them in order of level of closeness to the theme {}.
        Respond only with the text from the book and without commentary", word);

    let response = call_gemini(&prompt.to_string(), &response[0].to_string()).await?;

    Ok(response)
}

async fn generate_tweet_from_character(
    book: &str,
    character: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let prompt = format!("You are a poet and avid reader of George MacDonald.
        Make sure to respond only in excerpts from the provided book. 
        Identify portions and sentences that evoke strong emotions or imagery and that could stand alone without commentary.
        Look for earnestness.
        Find excerpt that have to do with the following character {}
        Select excerpts that encapsulate the essence of the book's themes in a concise and impactful way.
        Take a deep breath and carefully choose 20 poetic excerpts that have to do with {} from this book.", character, character);

    let response = call_gemini(&prompt.to_string(), &book.to_string()).await?;

    let prompt = format!("You are a poetic and literary Tweeter.
        Choose the 4 excerpts you find most interesting out of the following that you would send in a up to 280 character tweet.
        Find portions that could stand alone as poetic reflections.
        Sort them in order of level of closeness to the theme {}.
        Respond only with the text from the book and without commentary", character);

    let response = call_gemini(&prompt.to_string(), &response[0].to_string()).await?;

    Ok(response)
}

