use lib_core::model::ModelManager;
use serde_json::Value;

type Result<T> = core::result::Result<T, Error>;
type Error = Box<dyn std::error::Error>;

const BASE_URL: &str = "http://localhost:8080";

pub fn create_client() -> reqwest::Client {
    reqwest::Client::new()
}

async fn print_response(response: reqwest::Response) -> Result<()> {
    println!("Status: {}", response.status());
    let text = response.text().await?;
    println!("Body: {}", text);

    if let Ok(json) = serde_json::from_str::<Value>(&text) {
        println!("Formatted JSON: {}", serde_json::to_string_pretty(&json)?);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let _mm = ModelManager::new().await?;

    let client = create_client();

    println!("=== Testing Posts Like API ===");

    let post_id = "pst_japan_inspiring_00000";

    println!("\n--- Test 1: Toggle Like (First time - should like) ---");
    let like_resp = client.post(&format!("{}/api/posts/{}/like", BASE_URL, post_id)).send().await?;
    print_response(like_resp).await?;

    println!("\n--- Test 2: Toggle Like (Second time - should unlike) ---");
    let unlike_resp = client.post(&format!("{}/api/posts/{}/like", BASE_URL, post_id)).send().await?;
    print_response(unlike_resp).await?;

    println!("\n--- Test 3: Toggle Like (Third time - should like again) ---");
    let second_like_resp = client.post(&format!("{}/api/posts/{}/like", BASE_URL, post_id)).send().await?;
    print_response(second_like_resp).await?;

    println!("\n--- Test 4: Get post to see like count ---");
    let get_post_resp = client.get(&format!("{}/api/posts/{}/likes", BASE_URL, post_id)).send().await?;
    print_response(get_post_resp).await?;

    println!("\n--- Test 5: Get users who liked the post ---");
    let get_likers_resp = client.get(&format!("{}/api/posts/{}/likers", BASE_URL, post_id)).send().await?;
    print_response(get_likers_resp).await?;

    println!("\nPost like_unlike test completed successfully!");

    Ok(())
}
