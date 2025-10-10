use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub const BASE_URL: &str = "http://localhost:8080";

pub fn create_client() -> reqwest::Client {
    reqwest::Client::new()
}

pub async fn print_response(response: reqwest::Response) -> Result<()> {
    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());
    println!("Body: {}", response.text().await?);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = create_client();
    
    println!("=== Testing Auth Endpoints ===\n");

    // // Test registration
    println!("\nTesting Registration ...");
    let response = client
        .post(&format!("{}/api/register", BASE_URL))
        .json(&json!({
            "username": "testuser",
            "email": "testuser@example.com",
            "pwd": "password123",
            "pwd_confirm": "password123"
        }))
        .send()
        .await?;
    
    print_response(response).await?;

    // Test login
    println!("\nTesting Login ...");
    let response = client
        .post(&format!("{}/api/login", BASE_URL))
        .json(&json!({
            "username": "demo1",
            "pwd": "welcome"
        }))
        .send()
        .await?;
    
    print_response(response).await?;

    // Test logoff
    println!("\nTesting logoff ...");
    let response = client
        .post(&format!("{}/api/logoff", BASE_URL))
        .json(&json!({
            "logoff": true
        }))
        .send()
        .await?;
    
    print_response(response).await?;

    Ok(())
}
