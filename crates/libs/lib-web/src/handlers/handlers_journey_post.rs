use crate::error::{Error, Result};
use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use lib_core::{
    ctx::Ctx, model::{ModelManager, journey_post::JourneyPostBmc, post::{PostDetail, PostFeedItem}}, 
    service::post::{CreatePostPayload, PostService, UpdatePostPayload}
};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Deserialize)]
pub struct AddPostToJourneyRequest {
    pub post_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ReorderJourneyRequest {
    pub post_ids: Vec<String>, // new order for posts in journey
}

#[derive(Debug, Serialize)]
pub struct AddPostToJourneyResponse {
    success: bool,
}

#[derive(Debug, Serialize)]
pub struct ReorderJourneyPostsResponse {
    success: bool,
}


pub async fn add_post_to_journey_handler(
    State(mm): State<ModelManager>,
    // ctx: Ctx,
    Path(journey_id): Path<String>,
    Json(req): Json<AddPostToJourneyRequest>,
) -> Result<Json<AddPostToJourneyResponse>> {
    debug!("{:<12} - add_post_to_journey_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    // TODO: add validation for post_id from request
    JourneyService::add_post_to_journey_end(&ctx, &mm, &journey_id, &req.post_id).await?;
    
    Ok(Json(AddPostToJourneyResponse{
        success: true,
    }))
}

pub async fn reorder_journey_posts_handler(
    State(mm): State<ModelManager>,
    // ctx: Ctx,
    Path(journey_id): Path<String>,
    Json(req): Json<AddPostToJourneyRequest>,
) -> Result<Json<ReorderJourneyPostsResponse>> {
    debug!("{:<12} - reorder_journey_posts_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    // TODO: add validation for post_ids from request
    JourneyPostService::reorder_posts_in_journey(&ctx, &mm, &journey_id, req.post_ids).await?;

    Ok(Json(ReorderJourneyPostsResponse{
        success: true,
    }))
}