use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use lib_core::ctx::Ctx;
use lib_core::model::user::{UserBmc, UserForCreate};
use lib_core::model::ModelManager;
use serde::{Deserialize, Serialize};
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

    let username = payload.username.clone();
    // Create user
    let user_c = UserForCreate {
        username: username,
        email: payload.email.clone(),
        pwd_clear: payload.pwd.clone(),
    };

    UserBmc::create(&root_ctx, &mm, user_c).await?;

    // Return success JSON
    Ok(Json(RegistrationResponse {
        success: true,
        message: format!(
            "User '{}' created successfully! Please check your email to verify your account.",
            payload.username
        ),
    }))
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

#[derive(Serialize)]
struct RegistrationResponse {
    success: bool,
    message: String,
}
// endregion: --- Registration
