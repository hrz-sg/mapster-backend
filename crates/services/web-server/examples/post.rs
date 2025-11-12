use std::{fs, path::Path};

use lib_core::model::ModelManager;
use reqwest::multipart;
use serde_json::Value;

type Result<T> = core::result::Result<T, Error>;
type Error = Box<dyn std::error::Error>;

const BASE_URL: &str = "http://localhost:8080";
const ASSETS_DIR: &str = "crates/services/web-server/examples/assets";

pub fn create_client() -> reqwest::Client {
    reqwest::Client::new()
}

async fn print_response(response: reqwest::Response) -> Result<()> {
    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());
    println!("Body: {}", response.text().await?);
    Ok(())
}

async fn parse_json(response: reqwest::Response) -> Result<Value> {
    let text = response.text().await?;
    let json: Value = serde_json::from_str(&text)
        .map_err(|e| format!("Invalid JSON: {}\nBody: {}", e, text))?;
    println!("Body: {}", serde_json::to_string_pretty(&json)?);
    Ok(json)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let _mm = ModelManager::new().await?;

    let client = create_client();

    println!("=== Testing Posts API ===");

    // -- Create multipart form
    println!("\n--- Upload post ---");
    let form = multipart::Form::new()
        .text("title", "Post title")
        .text("description", "Post description")
        .file("media", format!("{}/image-1.jpg", ASSETS_DIR))
        .await?
        .file("media", format!("{}/image-2.jpg", ASSETS_DIR))
        .await?;

    // -- Send POST-request
    let response = client
        .post(&format!("{}/api/posts", BASE_URL))
        .multipart(form)
        .send()
        .await?;

    print_response(response).await?;

    let response = client
        .post(&format!("{}/api/posts", BASE_URL))
        .multipart(
            multipart::Form::new()
                .text("title", "Post title")
                .text("description", "Post description")
                .file("media", format!("{}/image-1.jpg", ASSETS_DIR))
                .await?
        )
        .send()
        .await?;

    let create_post_json = parse_json(response).await?;

     // Get post_id for futher tests
    let post_id = create_post_json["id"]
        .as_i64()
        .expect("Missing 'id' in create response");
    println!("Created post ID: {}\n", post_id);
    println!("\nPost upload test completed successfully!");

    println!("\n--- Listing posts ---");
    let response = client
        .get(&format!("{}/api/posts", BASE_URL))
        .send()
        .await?;
    print_response(response).await?;
    println!("\nPost listing test completed successfully!");


    println!("\n--- Getting post by ID ---");
    let get_resp = client
        .get(&format!("{}/api/posts/{}", BASE_URL, post_id))
        .send()
        .await?;
    print_response(get_resp).await?;
    println!("\nPost get by ID test completed successfully!");

    println!("\n--- Updating post ---");
    let image3 = format!("{}/image-3.jpg", ASSETS_DIR);
    if !Path::new(&image3).exists() {
        fs::copy(format!("{}/image-1.jpg", ASSETS_DIR), &image3)?;
    }

    let update_form = multipart::Form::new()
        .text("title", "Updated Title from Test")
        .file("add_media", image3)
        .await?;

    let update_resp = client
        .patch(&format!("{}/api/posts/{}", BASE_URL, post_id))
        .multipart(update_form)
        .send()
        .await?;
    print_response(update_resp).await?;
    println!("\nPost update test completed successfully!");


    println!("\n--- Deleting post ---");
    let delete_resp = client
        .delete(&format!("{}/api/posts/{}", BASE_URL, post_id))
        .send()
        .await?;
    print_response(delete_resp).await?;
    println!("\nPost delete test completed successfully!");

    println!("\nAPI test completed successfully!");
    
    Ok(())
}
