use crate::error::{Error, Result};
use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use lib_core::{
    ctx::Ctx, model::{ModelManager, post::{PostFeedItem, PostDetail}}, 
    service::post::{CreatePostPayload, PostService, UpdatePostPayload}
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
    let mut media = Vec::new();
    let mut thumbnail: Option<Vec<u8>> = None;

    // -- Extract fields from multipart form data
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "title" => title = field.text().await.unwrap_or_default(),
            "description" => description = field.text().await.unwrap_or_default(),

            "media" => {
                let filename = field.file_name().unwrap_or("upload.bin").to_string();
                let data = field.bytes().await.unwrap().to_vec();
                media.push((filename, data));
            }

            "thumbnail" => {
                let data = field.bytes().await.unwrap().to_vec();
                thumbnail = Some(data);
            }

            _ => (),
        }
    }

    // -- Check if files attached
    if media.is_empty() {
        return Err(Error::File(lib_utils::file::Error::ValidationFail(
            "At least one media file required".into(),
        )));
    }

    // -- Create payload
    let payload = CreatePostPayload {
        title,
        description,
        media,
        thumbnail,
    };

    let post_id = PostService::create_with_media(&ctx, &mm, payload).await?;

    Ok(Json(CreatePostResponse {
        success: true,
        id: Some(post_id),
        message: "Post created successfully!".into(),
    }))
}

pub async fn api_list_feed_posts_handler(
    State(mm): State<ModelManager>,
    // ctx: &Ctx,
) -> Result<Json<ListPostsResponse>> {
    debug!("{:<12} - api_list_post_handler", "HANDLER");

    let ctx = Ctx::root_ctx();
    let posts = PostService::list_feed_posts(&ctx, &mm).await?;

    Ok(Json(ListPostsResponse {
        success: true,
        posts,
    }))
}

pub async fn api_get_post_detail_by_id(
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
    // ctx: &Ctx,
) -> Result<Json<GetPostResponse>> {
    debug!("{:<12} - api_get_post_handler", "HANDLER");

    let ctx = Ctx::root_ctx();
    let post = PostService::get_post_detail_by_id(&ctx, &mm, &post_id).await?;

    Ok(Json(GetPostResponse {
        success: true,
        post,
    }))
}

pub async fn api_update_post_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
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

    let mut update_media_ids = Vec::new();
    let mut update_file_data = Vec::new();

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
            "update_media_id[]" => {
                if let Ok(media_id) = field.text().await {
                    let media_id = media_id.trim().to_string();
                    if !media_id.is_empty() {
                        update_media_ids.push(media_id);
                    }
                }
            }
            "update_file[]" => {
                let filename = field.file_name().unwrap_or("upload.bin").to_string();
                let data = field.bytes().await.unwrap().to_vec();
                update_file_data.push((filename, data));
            }
            "remove_ids[]" => {
                if let Ok(media_id) = field.text().await {
                    let media_id = media_id.trim().to_string();
                    if !media_id.is_empty() {
                        remove_ids.push(media_id);
                    }
                }
            }
            _ => (),
        }
    }

    if !update_media_ids.is_empty() && !update_file_data.is_empty() {
        // Check that the number of IDs matches the number of files
        if update_media_ids.len() == update_file_data.len() {
            for i in 0..update_media_ids.len() {
                let media_id = update_media_ids[i].clone();
                let (filename, data) = update_file_data[i].clone();
                update_files.push((media_id, filename, data));
            }
        } else {
            // If the order does not match, there is an error.
            tracing::error!(
                "Mismatched update_media_ids ({}) and update_file_data ({}) count",
                update_media_ids.len(),
                update_file_data.len()
            );
            return Err(Error::File(lib_utils::file::Error::ValidationFail(
                "Number of media IDs does not match number of files".into(),
            )));
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

    PostService::update_with_media(&ctx, &mm, &post_id, payload).await?;

    Ok(Json(UpdatePostResponse {
        success: true,
        message: "Post updated successfully!".into(),
    }))
}

pub async fn api_delete_post_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
) -> Result<Json<DeletePostResponse>> {
    let ctx = Ctx::root_ctx();

    PostService::delete(&ctx, &mm, &post_id).await?;

    Ok(Json(DeletePostResponse {
        success: true,
        message: "Post successfully deleted!".into(),
    }))
}

// region: ---- Json Reponse Structs
#[derive(Debug, Serialize)]
pub struct CreatePostResponse {
    success: bool,
    id: Option<String>,
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
    pub posts: Vec<PostFeedItem>,
}

#[derive(Debug, Serialize)]
pub struct GetPostResponse {
    pub success: bool,
    pub post: PostDetail,
}

#[derive(Debug, Serialize)]
pub struct UpdatePostResponse {
    pub success: bool,
    pub message: String,
}
// endregion: ---- Json Response Structs
