pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // -- Test Email Sending
    println!("=== Testing Email Endpoints ===");
    
    // Test email verification
    let req_test_verification = hc.do_post(
        "/api/test-email-verification",
        json!({
            "email": "test@example.com",
            "username": "testuser",
            "token": "test-token-123"
        })
    );
    println!("Sending verification email test...");
    req_test_verification.await?.print().await?;

    // Test welcome email
    let req_test_welcome = hc.do_post(
        "/api/test-welcome-email", 
        json!({
            "email": "test@example.com",
            "username": "testuser"
        })
    );
    println!("Sending welcome email test...");
    req_test_welcome.await?.print().await?;

    // Test email config
    let req_email_config = hc.do_get("/api/email-config");
    println!("Checking email configuration...");
    req_email_config.await?.print().await?;

    // -- Registration Test (без реальной отправки)
    println!("=== Testing Registration ===");
    let req_register = hc.do_post(
        "/api/register",
        json!({
            "username": "testuser2",
            "email": "test2@example.com", 
            "pwd": "password123",
            "pwd_confirm": "password123"
        })
    );
    req_register.await?.print().await?;

    // -- Login
    println!("=== Testing Login ===");
    let req_login = hc.do_post(
        "/api/login", 
        json!({
            "username": "demo1",
            "pwd": "welcome"
        })
    );
    req_login.await?.print().await?;

    // -- Logoff
    println!("=== Testing Logoff ===");
    let req_logoff = hc.do_post(
        "/api/logoff",
        json!({
            "logoff": true
        }),
    );
    req_logoff.await?.print().await?;

    Ok(())
}