use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use lib_core::ctx::Ctx;
use lib_core::model::user::UserBmc;
use lib_core::model::ModelManager;
use serde::Deserialize;
use serde_json::json;
use tracing::debug;
use serde_valid::Validate;

use crate::error::{Error, Result};

// region: --- Email Verification
pub async fn api_verify_email_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<VerifyEmailPayload>,
) -> Result<impl IntoResponse> {
    debug!("{:<12} - api_verify_email_handler", "HANDLER");

    payload.validate().map_err(|errs| {
        Error::Model(lib_core::model::Error::ValidationFail(errs.to_string()))
    })?;

    let root_ctx = Ctx::root_ctx();

    UserBmc::verify_email(&root_ctx, &mm, &payload.token).await?;

    Ok(Json(json!({
        "result": {
            "success": true,
            "message": "Email verified successfully"
        }
    })))
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyEmailPayload {
    #[validate(min_length = 1, message = "Verification token is required")]
    pub token: String,
}
// endregion: --- Email Verification

