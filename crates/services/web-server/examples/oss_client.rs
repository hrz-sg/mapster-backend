use lib_storage::oss::OssClient;
use lib_utils::file::validate_file;
use serde_json::json;
use std::{fs, path::Path};
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

    println!("=== Testing OSS Client with Images ===");

    let oss = OssClient::new();

    // --- Paths to test images
    let test_images = vec![
        "crates/services/web-server/examples/assets/image-1.jpg",
        "crates/services/web-server/examples/assets/image-2.jpg",
    ];

    for image_path in test_images {
        println!("\n===============================");
        println!("Processing file: {}", image_path);

        // --- Verify file exists locally
        let path = Path::new(image_path);
        if !path.exists() {
            println!("File not found: {}", image_path);
            continue;
        }

        // --- Read file bytes
        let content = fs::read(path)?;
        let filename = path.file_name().unwrap().to_string_lossy().to_string();

        // --- Validate & define mime
        let (mime, _) = validate_file(&filename, &content)?;

        // --- Upload file
        println!("\n--- Uploading File ---");
        let url = oss.upload(&filename, &content, &mime).await?;
        print_json(
            "Upload Result",
            json!({
                "filename": filename,
                "public_url": url,
                "mime_type": mime,
                "size_bytes": content.len(),
            }),
        )
        .await?;

        // --- Check if file exists
        println!("\n--- Checking Existence ---");
        let exists = oss.exists(&filename).await?;
        print_json("Exists", json!({ "exists": exists })).await?;

        // --- Download and verify
        println!("\n--- Downloading File ---");
        let downloaded = oss.download(&filename).await?;
        print_json(
            "Download Result",
            json!({
                "bytes_downloaded": downloaded.len(),
                "matches_original_size": downloaded.len() == content.len(),
            }),
        )
        .await?;

        // --- Delete file
        println!("\n--- Deleting File ---");
        oss.delete(&filename).await?;
        print_json("Delete Result", json!({ "deleted": true })).await?;

        // --- Verify deletion
        println!("\n--- Verifying Deletion ---");
        let exists_after = oss.exists(&filename).await?;
        print_json("Exists After Delete", json!({ "exists": exists_after })).await?;
    }

    println!("\nAll image tests completed successfully!");
    Ok(())
}
