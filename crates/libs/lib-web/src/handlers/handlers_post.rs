use axum::{
    Json, extract::{Multipart, State}
};
use lib_core::{
    ctx::Ctx,
    model::{ModelManager},
    service::post::{PostService, CreatePostPayload},
};
use serde::Serialize;
use tracing::debug;
use crate::error::{Error, Result};

pub async fn api_create_post_handler(
    State(mm): State<ModelManager>,
    mut multipart: Multipart,
    // ctx: &Ctx,
) -> Result<Json<CreatePostResponse>>{
    debug!("{:<12} - api_create_post_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    let mut title = String::new();
    let mut description = String::new();
    let mut files = Vec::new();

    // -- Extract fields from multipart form data
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "title" => title = field.text().await.unwrap_or_default(),
            "description" => description = field.text().await.unwrap_or_default(),
            "files[]" | "file" => {
                let filename = field.file_name().unwrap_or("upload.bin").to_string();
                let data = field.bytes().await.unwrap().to_vec();
                files.push((filename, data));
            }
            _ => (),
        }
    }

    // -- Check if files attached
    if files.is_empty() {
        return Err(Error::File(lib_utils::file::Error::ValidationFail(
            "At least one media file required".into(),
        )));
    }

    // -- Create payload
    let payload = CreatePostPayload {
        title,
        description,
        files,
    };

    let post_id = PostService::create_with_media(&ctx, &mm, payload).await?;

    Ok(Json(CreatePostResponse {
        success: true,
        id: post_id,
        message: "Post created successfully!".into(),
    }))
}

#[derive(Debug, Serialize)]
pub struct CreatePostResponse {
    success: bool,
    id: i64,
    message: String,
}
