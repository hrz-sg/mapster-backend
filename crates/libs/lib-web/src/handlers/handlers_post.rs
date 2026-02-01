// region: ---- Modules
use crate::error::{Error, Result};
use axum::{
    Json,
    extract::{Multipart, Path, Query, State},
};
use lib_core::{
    ctx::Ctx,
    model::{
        ModelManager,
        comment::Comment,
        post::{PostDetail, PostFeedItem},
        user::UserForPreview,
    },
    service::post::{CreatePostPayload, PostService, UpdatePostPayload},
};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use tracing::debug;
// endregion: ---- Modules

// region: --- Post Handlers
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
    Query(query): Query<FeedQuery>,
    // ctx: &Ctx,
) -> Result<Json<ListPostsResponse>> {
    debug!("{:<12} - api_list_post_handler", "HANDLER");

    let ctx = Ctx::root_ctx();

    // -- Assign limit if not
    let limit = query.limit.unwrap_or(20);

    let posts = PostService::list_feed_posts(&ctx, &mm, limit).await?;

    Ok(Json(ListPostsResponse { success: true, posts }))
}

pub async fn api_get_post_detail_by_id(
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
    // ctx: &Ctx,
) -> Result<Json<GetPostResponse>> {
    debug!("{:<12} - api_get_post_handler", "HANDLER");

    let ctx = Ctx::root_ctx();
    let post = PostService::get_post_detail_by_id(&ctx, &mm, &post_id).await?;

    Ok(Json(GetPostResponse { success: true, post }))
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
        update_files: if update_files.is_empty() {
            None
        } else {
            Some(update_files)
        },
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

pub async fn api_toggle_like_post_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
) -> Result<Json<ToggleLikeResponse>> {
    debug!("{:<12} - api_toggle_like_post_handler", "HANDLER");

    let ctx = Ctx::root_ctx();

    let (liked, like_count) = PostService::toggle_like(&ctx, &mm, &post_id).await?;

    Ok(Json(ToggleLikeResponse {
        success: true,
        liked,
        like_count,
    }))
}

pub async fn api_post_likes_count_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
) -> Result<Json<PostLikesCountResponse>> {
    debug!("{:<12} - api_post_likes_count_handler", "HANDLER");

    let ctx = Ctx::root_ctx();
    let like_count = PostService::get_like_count(&ctx, &mm, &post_id).await?;

    Ok(Json(PostLikesCountResponse {
        success: true,
        post_id,
        like_count,
    }))
}

pub async fn api_get_post_likers_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
    Query(query): Query<GetCommentsQuery>,
) -> Result<Json<PostLikersResponse>> {
    debug!("{:<12} - api_get_post_likers_handler", "HANDLER");

    let ctx = Ctx::root_ctx();

    let limit = query.limit.map(|v| v as u32).unwrap_or(20);
    let users = PostService::get_likers(&ctx, &mm, &post_id, Some(limit)).await?;

    Ok(Json(PostLikersResponse {
        success: true,
        post_id,
        users,
    }))
}

pub async fn api_create_comment_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
    Json(payload): Json<CreatePostCommentPayload>,
) -> Result<Json<CreatePostCommentResponse>> {
    debug!("{:<12} - api_create_comment_handler", "HANDLER");

    let ctx = Ctx::root_ctx();

    payload.validate()?;

    let comment =
        PostService::create_comment(&ctx, &mm, &post_id, payload.text.clone(), payload.parent_id.clone()).await?;

    Ok(Json(CreatePostCommentResponse { success: true, comment }))
}

pub async fn api_get_post_comments_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
    Query(query): Query<GetCommentsQuery>,
) -> Result<Json<GetPostCommentsResponse>> {
    debug!("{:<12} - api_get_post_comments_handler", "HANDLER");

    let ctx = Ctx::root_ctx();

    let limit: u32 = query.limit.unwrap_or(20).max(1).min(100);

    let comments = PostService::list_comments(&ctx, &mm, &post_id, limit).await?;
    let total = comments.len() as i64;

    Ok(Json(GetPostCommentsResponse {
        success: true,
        comments,
        total,
    }))
}

pub async fn api_get_comment_replies_handler(
    State(mm): State<ModelManager>,
    Path(comment_id): Path<String>,
    Query(query): Query<GetCommentsQuery>,
) -> Result<Json<GetPostCommentReplies>> {
    debug!("{:<12} - api_get_comment_replies_handler", "HANDLER");

    let ctx = Ctx::root_ctx();
    let limit: u32 = query.limit.unwrap_or(20).max(1).min(100);

    let replies = PostService::list_comment_replies(&ctx, &mm, &comment_id, limit).await?;

    Ok(Json(GetPostCommentReplies { success: true, replies }))
}

pub async fn api_update_comment_handler(
    State(mm): State<ModelManager>,
    Path(comment_id): Path<String>,
    Json(payload): Json<UpdatePostCommentPayload>,
) -> Result<Json<UpdatePostCommentResponse>> {
    debug!("{:<12} - api_update_comment_handler", "HANDLER");

    payload.validate()?;

    let ctx = Ctx::root_ctx();
    let comment = PostService::update_comment(&ctx, &mm, &comment_id, payload.text.clone()).await?;

    Ok(Json(UpdatePostCommentResponse { success: true, comment }))
}

pub async fn api_delete_comment_handler(
    State(mm): State<ModelManager>,
    Path(comment_id): Path<String>,
) -> Result<Json<DeletePostCommentResponse>> {
    debug!("{:<12} - api_delete_comment_handler", "HANDLER");

    let ctx = Ctx::root_ctx();
    PostService::delete_comment(&ctx, &mm, &comment_id).await?;

    Ok(Json(DeletePostCommentResponse {
        success: true,
        comment_id,
    }))
}
// endregion: --- Post Handlers

// region: ---- Json Reponse Structs
#[derive(Deserialize, Default)]
pub struct GetCommentsQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    #[serde(rename = "order_bys")]
    pub order_bys: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdatePostCommentPayload {
    #[validate(min_length = 1, message = "Text can not be empty")]
    pub text: String,
}

#[derive(Deserialize, Validate)]
pub struct CreatePostCommentPayload {
    #[validate(min_length = 1, message = "Text can not be empty")]
    pub text: String,
    pub parent_id: Option<String>,
}

#[derive(Deserialize)]
pub struct FeedQuery {
    limit: Option<u32>,
}

#[derive(Serialize)]
pub struct CreatePostCommentResponse {
    pub success: bool,
    pub comment: Comment,
}

#[derive(Serialize)]
pub struct PostLikersResponse {
    pub success: bool,
    pub post_id: String,
    pub users: Vec<UserForPreview>,
}

#[derive(Serialize)]
pub struct PostLikesCountResponse {
    pub success: bool,
    pub post_id: String,
    pub like_count: i64,
}

#[derive(Serialize)]
pub struct ToggleLikeResponse {
    pub success: bool,
    pub liked: bool,
    pub like_count: i64,
}

#[derive(Serialize)]
pub struct DeletePostCommentResponse {
    pub success: bool,
    pub comment_id: String,
}

#[derive(Serialize)]
pub struct UpdatePostCommentResponse {
    pub success: bool,
    pub comment: Comment,
}

#[derive(Serialize)]
pub struct GetPostCommentReplies {
    pub success: bool,
    pub replies: Vec<Comment>,
}

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

#[derive(Serialize)]
pub struct GetPostCommentsResponse {
    pub success: bool,
    pub comments: Vec<Comment>,
    pub total: i64,
}

#[derive(Serialize)]
pub struct GetPostCommentResponse {
    pub success: bool,
    pub comment: Comment,
}
// endregion: ---- Json Response Structs
