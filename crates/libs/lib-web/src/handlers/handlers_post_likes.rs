use crate::error::Result;
use axum::{
    Json,
    extract::{Path, State},
};
use lib_core::{
    ctx::Ctx, model::{ModelManager, user::UserForPreview}, 
    service::post_like::PostLikeService
};
use serde::Serialize;
use tracing::debug;

#[derive(Serialize)]
pub struct ToggleLikeResponse {
    pub success: bool,
    pub liked: bool,
    pub like_count: i64,
}

#[derive(Serialize)]
pub struct PostLikesCountResponse {
    pub success: bool,
    pub post_id: String,
    pub like_count: i64,
}

#[derive(Serialize)]
pub struct PostLikersResponse {
    pub success: bool,
    pub post_id: String,
    pub users: Vec<UserForPreview>,
}

pub async fn api_toggle_like_post_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
    // ctx: &Ctx,
) -> Result<Json<ToggleLikeResponse>> {
    debug!("{:<12} - api_create_post_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    let (liked, like_count) = PostLikeService::toggle_like(&ctx, &mm, &post_id).await?;

    Ok(Json(ToggleLikeResponse {
        success: true,
        liked,
        like_count,
    }))
}

pub async fn api_post_likes_count_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
    // ctx: &Ctx,
) -> Result<Json<PostLikesCountResponse>> {
    debug!("{:<12} - api_post_likes_count_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    let like_count = PostLikeService::get_post_likes_count(&ctx, &mm, &post_id).await?;

    Ok(Json(PostLikesCountResponse {
        success: true,
        post_id,
        like_count,
    }))
}

pub async fn api_get_post_likers_handler(
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
    // ctx: &Ctx,
) -> Result<Json<PostLikersResponse>> {
    debug!("{:<12} - api_get_post_likers_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    let users = PostLikeService::get_post_likers(&ctx, &mm, &post_id, None).await?;

    Ok(Json(PostLikersResponse {
        success: true,
        post_id,
        users,
    }))
}
