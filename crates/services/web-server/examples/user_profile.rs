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
    
    // --- Get my profile
    println!("\n--- Getting MY profile ---");
    let my_resp = client
        .get(&format!("{}/api/profile/me", BASE_URL))
        .send()
        .await?;

    parse_json(my_resp).await?;
    println!("Get MY profile successfully!");

    // --- Get other user profile
    println!("\n--- Getting OTHER user profile ---");
    let my_resp = client
        .get(&format!("{}/api/profile/{}", BASE_URL, "usr_demo10000000000000000"))
        .send()
        .await?;

    parse_json(my_resp).await?;
    println!("Get OTHER USER profile successfully!");

    Ok(())
}
