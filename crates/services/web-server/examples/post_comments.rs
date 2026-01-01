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

    // Post to be commented
    let post_id = "pst_shanghai_trip_0000000".to_string();

    println!("=== Testing Posts Comments API ===");

    println!("\n--- Create comment for post ---");
    // Create Comment
    let comment = serde_json::json!({
        "text": "Such a beautiful place!",
        "parent_id": null
    });

    // -- Send POST-request
    let response = client
        .post(&format!("{}/api/posts/{post_id}/comments", BASE_URL))
        .json(&comment)
        .send()
        .await?;
    let comment_response = parse_json(response).await?;
    // Get comment ID from response for future tests
    let comment_id = comment_response["comment"]["id"]
        .as_str()
        .ok_or("No comment found in response")?
        .to_string();
    println!("\nCreating comment for post test completed successfully!");

    println!("\n--- Get the comment by comment id ---");
    let response = client
        .get(&format!("{}/api/comments/{comment_id}", BASE_URL))
        .send()
        .await?;
    print_response(response).await?;
    println!("\nGetting comment by comment id test completed successfully!");


    println!("\n--- Get all the comments for the post ---");
    let response = client
        .get(&format!("{}/api/posts/{post_id}/comments", BASE_URL))
        .send()
        .await?;
    print_response(response).await?;
    println!("\nPost listing test completed successfully!");


    println!("\n--- Get replies for the comment ---");
    let get_resp = client
        .get(&format!("{}/api/comments/{comment_id}/replies", BASE_URL))
        .send()
        .await?;
    print_response(get_resp).await?;
    println!("\nGet replies for the comment test completed successfully!");


    println!("\n--- Updating comment ---");
    let update_comment = serde_json::json!({
        "text": "Cool place!",
    });

    let update_resp = client
        .patch(&format!("{}/api/comments/{comment_id}", BASE_URL))
        .json(&update_comment)
        .send()
        .await?;
    print_response(update_resp).await?;
    println!("\nComment update test completed successfully!");

    println!("\n--- Deleting comment ---");
    let delete_resp = client
        .delete(&format!("{}/api/comments/{comment_id}", BASE_URL))
        .send()
        .await?;
    print_response(delete_resp).await?;
    println!("\nComment delete test completed successfully!");

    println!("\nAPI test completed successfully!");
    
    Ok(())
}
