use crate::error::{Error, Result};
use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use lib_core::{
    ctx::Ctx,
    model::{ModelManager, post::Post},
    service::post::{CreatePostPayload, PostService, UpdatePostPayload},
};
use serde::Serialize;
use tracing::debug;

pub async fn api_create_post_handler(
    State(mm): State<ModelManager>,
    mut multipart: Multipart,
    // ctx: &Ctx,
) -> Result<Json<CreatePostResponse>> {
    debug!("{:<12} - api_create_post_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    let mut title = String::new();
    let mut description = String::new();
    let mut files = Vec::new();
    let mut thumbnail: Option<Vec<u8>> = None;

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

            "thumbnail" => {
                let data = field.bytes().await.unwrap().to_vec();
                thumbnail = Some(data);
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
        thumbnail,
    };

    let post_id = PostService::create_with_media(&ctx, &mm, payload).await?;

    Ok(Json(CreatePostResponse {
        success: true,
        id: Some(post_id),
        message: "Post created successfully!".into(),
    }))
}

pub async fn api_list_posts_handler(
    State(mm): State<ModelManager>,
    // ctx: &Ctx,
) -> Result<Json<ListPostsResponse>> {
    debug!("{:<12} - api_list_post_handler", "HANDLER");

    let ctx = Ctx::root_ctx();
    let posts = PostService::list(&ctx, &mm).await?;

    Ok(Json(ListPostsResponse {
        success: true,
        posts,
    }))
}

pub async fn api_get_post_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<i64>,
    // ctx: &Ctx,
) -> Result<Json<GetPostResponse>> {
    debug!("{:<12} - api_get_post_handler", "HANDLER");

    let ctx = Ctx::root_ctx();
    let post = PostService::get_post(&ctx, &mm, post_id).await?;

    Ok(Json(GetPostResponse {
        success: true,
        post,
    }))
}

pub async fn api_update_post_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<i64>,
    mut multipart: Multipart,
    // ctx: &Ctx,
) -> Result<Json<UpdatePostResponse>> {
    debug!("{:<12} - api_update_post_handler", "HANDLER");

    let ctx = Ctx::root_ctx();

    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut add_files = Vec::new();
    let mut update_files = Vec::new();
    let mut remove_ids = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "title" => title = Some(field.text().await.unwrap_or_default()),
            "description" => description = Some(field.text().await.unwrap_or_default()),
            "add_files[]" | "add_file" => {
                let filename = field.file_name().unwrap_or("upload.bin").to_string();
                let data = field.bytes().await.unwrap().to_vec();
                add_files.push((filename, data));
            }
            "update_files[]" => {
                // Example: field can be "update_files_3" where 3 — id is media
                let filename = field.file_name().unwrap_or("upload.bin").to_string();

                // try to get id from filename (ex. "update_3.png" → id=3)
                let id: Option<i64> = filename
                    .split('_')
                    .filter_map(|s| s.parse::<i64>().ok())
                    .next();

                let data = field.bytes().await.unwrap().to_vec();

                if let Some(id) = id {
                    update_files.push((id, filename, data));
                }
            }
            "remove_ids[]" => {
                if let Ok(id_str) = field.text().await {
                    if let Ok(id) = id_str.parse::<i64>() {
                        remove_ids.push(id);
                    }
                }
            }
            _ => (),
        }
    }

    let payload = UpdatePostPayload {
        title,
        description,
        is_published: None,
        add_files: if add_files.is_empty() { None } else { Some(add_files) },
        update_files: if update_files.is_empty() { None } else { Some(update_files) },
        remove_ids: if remove_ids.is_empty() { None } else { Some(remove_ids) },
        new_cover_id: None,
    };

    PostService::update_with_media(&ctx, &mm, post_id, payload).await?;

    Ok(Json(UpdatePostResponse {
        success: true,
        message: "Post updated successfully!".into(),
    }))
}

pub async fn api_delete_post_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<i64>,
) -> Result<Json<DeletePostResponse>> {
    let ctx = Ctx::root_ctx();

    PostService::delete(&ctx, &mm, post_id).await?;

    Ok(Json(DeletePostResponse {
        success: true,
        message: "Post successfully deleted!".into(),
    }))
}

// region: ---- Json Reponse Structs
#[derive(Debug, Serialize)]
pub struct CreatePostResponse {
    success: bool,
    id: Option<i64>,
    message: String,
}

#[derive(Debug, Serialize)]
pub struct DeletePostResponse {
    success: bool,
    message: String,
}

#[derive(Debug, Serialize)]
pub struct ListPostsResponse {
    pub success: bool,
    pub posts: Vec<Post>,
}

#[derive(Debug, Serialize)]
pub struct GetPostResponse {
    pub success: bool,
    pub post: Post,
}

#[derive(Debug, Serialize)]
pub struct UpdatePostResponse {
    pub success: bool,
    pub message: String,
}
// endregion: ---- Json Response Structs
