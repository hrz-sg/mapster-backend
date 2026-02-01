// region: --- Modules
use crate::error::Result;
use axum::{
    Json,
    extract::{Path, State},
};
use lib_core::{
    ctx::Ctx,
    model::ModelManager,
    service::user_profile::{UserProfile, UserProfileService},
};
use serde::Serialize;
use tracing::debug;
// endregion: --- Modules

// region: --- User Profile Handlers
pub async fn api_get_user_profile_by_id(
    State(mm): State<ModelManager>,
    Path(user_id): Path<String>,
    // ctx: &Ctx,
) -> Result<Json<GetUserProfileResponse>> {
    debug!("{:<12} - api_get_user_profile_handler", "HANDLER");

    let ctx = Ctx::root_ctx();
    let profile = UserProfileService::get_user_profile(&ctx, &mm, &user_id).await?;

    Ok(Json(GetUserProfileResponse { success: true, profile }))
}

pub async fn api_get_my_profile(
    State(mm): State<ModelManager>,
    // ctx: &Ctx,
) -> Result<Json<GetUserProfileResponse>> {
    debug!("{:<12} - api_get_my_profile_handler", "HANDLER");

    let ctx = Ctx::root_ctx();
    let profile = UserProfileService::get_my_profile(&ctx, &mm).await?;

    Ok(Json(GetUserProfileResponse { success: true, profile }))
}
// endregion: --- User Profile Handlers

// region: --- Handlers Response structs
#[derive(Debug, Serialize)]
pub struct GetUserProfileResponse {
    pub success: bool,
    pub profile: UserProfile,
}
// endregion: --- Handlers Response structs
