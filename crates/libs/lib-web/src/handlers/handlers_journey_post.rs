// region: --- Modules
use crate::error::Result;
use axum::{
    Json,
    extract::{Path, State},
};
use lib_core::{ctx::Ctx, model::ModelManager, service::journey_post::JourneyPostService};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use tracing::debug;
// endregion: --- Modules

// region: --- Journey post handlers
pub async fn add_post_to_journey_handler(
    State(mm): State<ModelManager>,
    // ctx: Ctx,
    Path(journey_id): Path<String>,
    Json(req): Json<AddPostToJourneyRequest>,
) -> Result<Json<AddPostToJourneyResponse>> {
    debug!("{:<12} - add_post_to_journey_handler", "HANDLER");

    req.validate()?;

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    JourneyPostService::add_post_to_journey_end(&ctx, &mm, &journey_id, &req.post_id).await?;

    Ok(Json(AddPostToJourneyResponse { success: true }))
}

pub async fn reorder_journey_posts_handler(
    State(mm): State<ModelManager>,
    // ctx: Ctx,
    Path(journey_id): Path<String>,
    Json(req): Json<ReorderJourneyRequest>,
) -> Result<Json<ReorderJourneyPostsResponse>> {
    debug!("{:<12} - reorder_journey_posts_handler", "HANDLER");

    req.validate()?;

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    JourneyPostService::reorder_posts_in_journey(&ctx, &mm, &journey_id, req.post_ids).await?;

    Ok(Json(ReorderJourneyPostsResponse { success: true }))
}
// endregion: --- Journey post handlers

// region: --- Request & Response Structs
#[derive(Debug, Deserialize, Validate)]
pub struct AddPostToJourneyRequest {
    #[validate(min_length = 1, message = "Post id cannot be empty")]
    pub post_id: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReorderJourneyRequest {
    #[validate(min_items = 1, message = "Post ids cannot be empty")]
    pub post_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AddPostToJourneyResponse {
    success: bool,
}

#[derive(Debug, Serialize)]
pub struct ReorderJourneyPostsResponse {
    success: bool,
}
// endregion: --- Request & Response Structs
