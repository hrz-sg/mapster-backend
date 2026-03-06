#![allow(unused)]

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use lib_core::service::upload_media::PartPresignedUrl;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // -- Login
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo0",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    // Files to upload
    let files_to_upload = vec![
        // ("file_1", "crates/services/web-server/examples/assets/IMG_4832.MOV", "video/quicktime"),
        // ("file_2", "crates/services/web-server/examples/assets/123.mp4", "video/mp4"),
        ("file_3", "crates/services/web-server/examples/assets/image-1.jpg", "image/jpg"),
        ("file_4", "crates/services/web-server/examples/assets/image-2.jpg", "image/jpg"),
    ];

    // Prepare payload
    let files_payload: Vec<_> = files_to_upload.iter().map(|(id, path, ct)| {
        let size = std::fs::metadata(path).unwrap().len();
        json!({
            "file_id": id,
            "filename": std::path::Path::new(path).file_name().unwrap().to_string_lossy(),
            "size": size,
            "content_type": ct
        })
    }).collect();

    let req_init_upload = hc.do_post(
        "/api/rpc",
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "init_upload_media_session",
            "params": {
                "data": {
                    "entity": "post",
                    "entity_id": "pst_test123",
                    "files": files_payload
                }
            }
        }),
    );
    let result = req_init_upload.await?;
    result.print().await?;

    // -- Get uploads
    let binding = result.json_value::<Value>("/result/uploads")?;
    let uploads = binding.as_array().unwrap();

    let client = Client::new();

	let mut uploaded_bytes: u64 = 0;

    // Iterate each file
    for upload_info in uploads {
        let object_key = upload_info["object_key"].as_str().unwrap().to_string();
        let multipart = upload_info["multipart"].as_bool().unwrap_or(false);
        let file_id = upload_info["file_id"].as_str().unwrap();

        let (file_path, ct) = files_to_upload
            .iter()
            .find(|(id, _, _)| id == &file_id)
            .map(|(_, path, ct)| (path, *ct))
            .unwrap();

        let file_metadata = std::fs::metadata(file_path)?;
        let file_size = file_metadata.len();

        let mut file = File::open(file_path).await?;
        let mut file_buffer = Vec::new();
        file.read_to_end(&mut file_buffer).await?;

        if multipart {
            // Multipart upload
            let upload_id = upload_info["upload_id"].as_str().unwrap().to_string();
            let total_parts = upload_info["total_parts"].as_u64().unwrap() as u32;
            let part_size = upload_info["part_size"].as_u64().unwrap();
            let mut completed_parts = Vec::new();

            println!("Multipart upload: {} parts, {} bytes each", total_parts, part_size);

            for part_number in 1..=total_parts {
                println!("--- Uploading part {}/{} ---", part_number, total_parts);

                // -- Generate Presigned URL
                let req_presigned_part = hc.do_post(
                    "/api/rpc",
                    json!({
                        "jsonrpc": "2.0",
                        "id": 2,
                        "method": "generate_presigned_url_for_part",
                        "params": {
                            "data": {
                                "object_key": object_key.clone(),
                                "upload_id": upload_id.clone(),
                                "part_number": part_number,
                                "content_type": ct
                            }
                        }
                    }),
                ).await?;

                let part: PartPresignedUrl = req_presigned_part.json_value("/result")?;

                let start = ((part_number - 1) as u64) * part_size;
                let end = std::cmp::min(start + part_size, file_size);
                let part_data = &file_buffer[start as usize..end as usize];

                println!("Uploading bytes {}-{}", start, end - 1);

                let mut req = client.put(&part.url);
                for (k, v) in part.headers.iter() {
                    if k.to_lowercase() != "authorization" {
                        req = req.header(k, v);
                    }
                }
                req = req.header("content-length", part_data.len().to_string())
                         .header("content-type", ct);

                let resp = req.body(part_data.to_vec()).send().await?;
                if !resp.status().is_success() {
                    let error_text = resp.text().await?;
                    panic!("Failed to upload part {}: {}", part_number, error_text);
                }

                let etag = resp.headers()
                    .get("etag")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .trim_matches('"')
                    .to_string();

                println!("Part {} uploaded with ETag: {}", part_number, etag);

                completed_parts.push(json!({
                    "part_number": part_number,
                    "etag": etag
                }));
            }

            // --- Complete Upload ---
            println!("Completing multipart upload for {}", file_path);
            let req_complete_upload = hc.do_post(
                "/api/rpc",
                json!({
                    "jsonrpc": "2.0",
                    "id": 3,
                    "method": "complete_upload_session",
                    "params": {
                        "data": {
                            "upload_id": upload_id,
                            "object_key": object_key,
                            "parts": completed_parts
                        }
                    }
                }),
            ).await?;
            req_complete_upload.print().await?;
        } else {
            // Single file upload
            println!("Signle file upload for {}", file_path);
            let url = upload_info["urls"][0].as_str().unwrap();
            client.put(url)
                .header("content-length", file_buffer.len())
                .header("content-type", ct)
                .body(file_buffer)
                .send()
                .await?;
            println!("Upload completed for {}", file_path);
        }

    }

    // -- Logoff
    let req_logoff = hc.do_post(
        "/api/logout",
        json!({ "logout": true }),
    );
    req_logoff.await?.print().await?;

    Ok(())
}
