use lib_storage::oss::OssClient;
use serde_json::json;
use std::str;
use tokio;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

async fn print_json(label: &str, data: serde_json::Value) {
    println!("\n=== {} ===", label);
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init(); 

    println!("\n=== Testing OSS Client ===");

    let oss = OssClient::new();
    let filename = "tests/hello_test.txt";
    let content = b"Hello from Rust!";

    // --- Upload ---
    println!("\n--- Uploading File ---");
    let url = oss.upload(filename, content).await?;
    print_json(
        "Upload Result",
        json!({
            "filename": filename,
            "public_url": url,
            "size_bytes": content.len(),
        }),
    )
    .await;

    // --- Check Exists ---
    println!("\n--- Checking Existence ---");
    let exists = oss.exists(filename).await?;
    print_json("Exists", json!({ "exists": exists })).await;

    // --- Download ---
    println!("\n--- Downloading File ---");
    let downloaded = oss.download(filename).await?;
    let text = str::from_utf8(&downloaded)?;
    print_json(
        "Download Result",
        json!({
            "bytes_downloaded": downloaded.len(),
            "content_preview": text,
        }),
    )
    .await;

    // --- Delete ---
    println!("\n--- Deleting File ---");
    oss.delete(filename).await?;
    print_json("Delete Result", json!({ "deleted": true })).await;

    // --- Verify Delete ---
    println!("\n--- Verify Deletion ---");
    let exists_after = oss.exists(filename).await?;
    print_json("Exists After Delete", json!({ "exists": exists_after })).await;

    println!("\n✅ OSS Client test completed successfully!\n");
    Ok(())
}
