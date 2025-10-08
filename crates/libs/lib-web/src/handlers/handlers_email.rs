use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use lib_core::ctx::Ctx;
use lib_core::model::user::{UserBmc, UserForCreate, User};
use lib_core::model::ModelManager;
use serde::Deserialize;
use serde_json::json;
use tracing::debug;
use serde_valid::Validate;

use crate::error::{Error, Result};

// region: --- Registration
pub async fn api_registration_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<RegistrationPayload>,
) -> Result<impl IntoResponse> {
    debug!("{:<12} - api_registration_handler", "HANDLER");

    // Validate payload
    payload.validate().map_err(|errs| {
        let msg = errs.to_string();
        Error::Model(lib_core::model::Error::ValidationFail(msg))
    })?;

    // Verify password match
    if payload.pwd != payload.pwd_confirm {
        return Err(Error::Model(lib_core::model::Error::ValidationFail(
            "Passwords do not match".into(),
        )));
    }

    let root_ctx = Ctx::root_ctx();

    // Check if username exists
    if UserBmc::first_by_username::<User>(&root_ctx, &mm, &payload.username)
        .await?
        .is_some()
    {
        return Err(Error::Model(lib_core::model::Error::ValidationFail(
            "Username already exists".into(),
        )));
    }

    // Create user
    let user_c = UserForCreate {
        username: payload.username.clone(),
        email: payload.email.clone(),
        pwd_clear: payload.pwd.clone(),
        pwd_confirm: payload.pwd_confirm.clone(),
    };

    let user_id = UserBmc::create(&root_ctx, &mm, user_c).await?;

    // Return success JSON
    Ok(Json(json!({
        "result": {
            "success": true,
            "user_id": user_id,
            "message": "Registration successful. Please check your email for verification."
        }
    })))
}

#[derive(Debug, Deserialize, serde_valid::Validate)]
pub struct RegistrationPayload {
    #[validate(min_length = 1, message = "Username is required")]
    pub username: String,

    #[validate(min_length = 1, message = "Email is required")]
    #[validate(pattern = r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$", message = "Email is invalid")]
    pub email: String,

    #[validate(min_length = 6, message = "Password must be at least 6 characters")]
    pub pwd: String,

    #[validate(min_length = 1, message = "Confirm Password is required")]
    pub pwd_confirm: String,
}
// endregion: --- Registration

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
