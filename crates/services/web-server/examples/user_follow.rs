use std::error::Error;
use serde_json::Value;
use reqwest::Client;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const BASE_URL: &str = "http://localhost:8080";

fn create_client() -> Client {
    Client::new()
}

async fn parse_json(response: reqwest::Response) -> Result<Value> {
    let text = response.text().await?;
    let json: Value = serde_json::from_str(&text)
        .map_err(|e| format!("Invalid JSON: {}\nBody: {}", e, text))?;
    println!("Parsed JSON: {}", serde_json::to_string_pretty(&json)?);
    Ok(json)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let client = create_client();
    
    println!("=== Testing Followers/Followings API ===\n");
    
    // --- Get MY followers (my profile)
    println!("1. --- Getting MY followers ---");
    let my_followers_resp = client
        .get(&format!("{}/api/me/followers", BASE_URL))
        .send()
        .await?;

    parse_json(my_followers_resp).await?;
    println!("Get MY followers successfully!");
    
    // --- Get OTHER user's followers (user_id = 1111)
    println!("\n--- Getting OTHER user's followers (user_id=1111) ---");
    let other_followers_resp = client
        .get(&format!("{}/api/users/1111/followers", BASE_URL))
        .send()
        .await?;

    parse_json(other_followers_resp).await?;
    println!("Get OTHER USER followers successfully!");
    
    // --- Get MY followings (my profile)
    println!("\n--- Getting MY followings ---");
    let my_followings_resp = client
        .get(&format!("{}/api/me/followings", BASE_URL))
        .send()
        .await?;

    parse_json(my_followings_resp).await?;
    println!("Get MY followings successfully!");
    
    // --- Get OTHER user's followings (user_id = 1111)
    println!("\n--- Getting OTHER user's followings (user_id=1111) ---");
    let other_followings_resp = client
        .get(&format!("{}/api/users/1111/followings", BASE_URL))
        .send()
        .await?;

    parse_json(other_followings_resp).await?;
    println!("Get OTHER USER followings successfully!");

    Ok(())
}