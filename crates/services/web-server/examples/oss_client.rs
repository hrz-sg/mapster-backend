use lib_storage::oss::OssClient;
use serde_json::json;
use std::str;

type Result<T> = core::result::Result<T, Error>;
type Error = Box<dyn std::error::Error>;

async fn print_json(label: &str, data: serde_json::Value) -> Result<()> {
    println!("\n=== {} ===", label);
    println!("{}", serde_json::to_string_pretty(&data)?);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    println!("=== Testing OSS Client ===");

    let oss = OssClient::new();
    let filename = "tests/hello_test.txt";
    let content = b"Hello from Rust!";

    // --- Upload
    println!("\n--- Uploading File ---");
    let (url, _) = oss.upload(filename, content).await?;
    print_json(
        "Upload Result",
        json!({
            "filename": filename,
            "public_url": url,
            "size_bytes": content.len(),
        }),
    )
    .await?;

    // --- Check Exists
    println!("\n--- Checking Existence ---");
    let exists = oss.exists(filename).await?;
    print_json("Exists", json!({ "exists": exists })).await?;

    // --- Download
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
    .await?;

    // --- Delete
    println!("\n--- Deleting File ---");
    oss.delete(filename).await?;
    print_json("Delete Result", json!({ "deleted": true })).await?;

    // --- Verify Delete
    println!("\n--- Verifying Deletion ---");
    let exists_after = oss.exists(filename).await?;
    print_json("Exists After Delete", json!({ "exists": exists_after })).await?;

    Ok(())
}
