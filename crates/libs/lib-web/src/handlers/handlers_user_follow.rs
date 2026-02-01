// region: --- Modules
use crate::error::Result;
use axum::{
    Json,
    extract::{Path, State},
};
use lib_core::{
    ctx::Ctx,
    model::ModelManager,
    service::user_follow::{FollowListItem, UserFollowService},
};
use serde::Serialize;
use tracing::debug;
// endregion: --- Modules

#[derive(Debug, Serialize)]
pub struct FollowListResponse {
    pub success: bool,
    pub total: i64,
    pub users: Vec<FollowListItem>,
}

pub async fn api_get_my_followers(State(mm): State<ModelManager>) -> Result<Json<FollowListResponse>> {
    debug!("{:<12} - api_get_my_followers", "HANDLER");

    let ctx = Ctx::root_ctx();

    let result = UserFollowService::list_followers(&ctx, &mm, None).await?;

    Ok(Json(FollowListResponse {
        success: true,
        total: result.total,
        users: result.users,
    }))
}

pub async fn api_get_my_followings(State(mm): State<ModelManager>) -> Result<Json<FollowListResponse>> {
    debug!("{:<12} - api_get_my_followings", "HANDLER");

    let ctx = Ctx::root_ctx();

    let result = UserFollowService::list_followings(&ctx, &mm, None).await?;

    Ok(Json(FollowListResponse {
        success: true,
        total: result.total,
        users: result.users,
    }))
}

pub async fn api_get_user_followers(
    State(mm): State<ModelManager>,
    Path(user_id): Path<String>,
) -> Result<Json<FollowListResponse>> {
    debug!("{:<12} - api_get_user_followers", "HANDLER");

    let ctx = Ctx::root_ctx();

    let result = UserFollowService::list_followers(&ctx, &mm, Some(&user_id)).await?;

    Ok(Json(FollowListResponse {
        success: true,
        total: result.total,
        users: result.users,
    }))
}

pub async fn api_get_user_followings(
    State(mm): State<ModelManager>,
    Path(user_id): Path<String>,
) -> Result<Json<FollowListResponse>> {
    debug!("{:<12} - api_get_user_followings", "HANDLER");

    let ctx = Ctx::root_ctx();

    let result = UserFollowService::list_followings(&ctx, &mm, Some(&user_id)).await?;

    Ok(Json(FollowListResponse {
        success: true,
        total: result.total,
        users: result.users,
    }))
}
