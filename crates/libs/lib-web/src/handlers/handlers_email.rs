// region: --- Modules

use crate::error::Result;
use axum::Json;
use axum::extract::State;
use lib_core::model::ModelManager;
use lib_core::{ctx::Ctx, service::user::UserService};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use tracing::debug;

// endregion: --- Modules

// region: --- Email Verification
pub async fn api_verify_email_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<VerifyEmailPayload>,
) -> Result<Json<VerifyEmailResponse>> {
    debug!("{:<12} - api_verify_email_handler", "HANDLER");

    payload.validate()?;

    let root_ctx = Ctx::root_ctx();

    UserService::verify_email(&root_ctx, &mm, &payload.token).await?;

    Ok(Json(VerifyEmailResponse {
        success: true,
        message: "Email verified successfully".to_string(),
    }))
}
// endregion: --- Email Verification

// region: --- Verify Email payload & response structs
#[derive(Debug, Deserialize, Validate)]
pub struct VerifyEmailPayload {
    #[validate(min_length = 1, message = "Verification token is required")]
    pub token: String,
}

#[derive(Serialize)]
pub struct VerifyEmailResponse {
    success: bool,
    message: String,
}
// endregion: --- Verify Email payload & response structs
