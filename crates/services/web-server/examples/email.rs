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
    dotenvy::dotenv().ok();

    let client = create_client();
    
    println!("=== Testing Email Endpoints ===");
    
    // Test email verification
    println!("\nSending verification email test...");
    let response = client
        .post(&format!("{}/api/test-email-verification", BASE_URL))
        .json(&json!({
            "email": "test@example.com",
            "username": "testuser",
            "token": "test-token-123"
        }))
        .send()
        .await?;
    print_response(response).await?;

    // Test welcome email
    println!("\nSending welcome email test...");
    let response = client
        .post(&format!("{}/api/test-welcome-email", BASE_URL))
        .json(&json!({
            "email": "test@example.com",
            "username": "testuser"
        }))
        .send()
        .await?;
    print_response(response).await?;

    Ok(())
}