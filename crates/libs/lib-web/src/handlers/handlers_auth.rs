// region: -- Modules
use crate::error::{Error, Result};
use crate::utils::token;
use axum::Json;
use axum::extract::State;
use lib_core::model::ModelManager;
use lib_core::service::user::UserService;
use lib_core::{ctx::Ctx, model::user::UserForCreate};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use tower_cookies::Cookies;
use tracing::debug;
// endregion: -- Modules

// region:    --- Auth handlers
pub async fn api_registration_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<RegistrationPayload>,
) -> Result<Json<RegistrationResponse>> {
    debug!("{:<12} - api_registration_handler", "HANDLER");

    // Validate payload
    payload.validate()?;

    // Verify password match
    if payload.pwd != payload.pwd_confirm {
        return Err(Error::ValidationFailed("Passwords do not match".into()));
    }

    let root_ctx = Ctx::root_ctx();

    // Prepare UserForCreate
    let user_c = UserForCreate {
        username: payload.username.clone(),
        email: payload.email.clone(),
        pwd_clear: payload.pwd.clone(),
    };

    // -- Register the user
    UserService::register(&root_ctx, &mm, user_c).await?;

    // Return success JSON
    Ok(Json(RegistrationResponse {
        success: true,
        message: format!(
            "User '{}' created successfully! Please check your email to verify your account.",
            payload.username
        ),
    }))
}

pub async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<LoginResponse>> {
    debug!("{:<12} - api_login_handler", "HANDLER");

    payload.validate()?;

    let root_ctx = Ctx::root_ctx();

    let user = UserService::login(&root_ctx, &mm, &payload.username, &payload.pwd).await?;

    // -- Set web token.
    let access_token = token::set_token_cookie(&cookies, &user.username, user.token_salt)?;

    // -- Create success body.
    Ok(Json(LoginResponse {
        success: true,
        message: format!("Welcome back, {}!", user.username),
        user: UserInfo {
            id: user.user_id,
            username: user.username,
        },
        token: Some(access_token),
    }))
}

pub async fn api_logout_handler(cookies: Cookies, Json(payload): Json<LogoutPayload>) -> Result<Json<LogoutResponse>> {
    debug!("{:<12} - api_logout_handler", "HANDLER");

    if payload.logout {
        token::remove_token_cookie(&cookies)?;
    }

    // -- Create and return the success body.
    Ok(Json(LogoutResponse {
        success: true,
        message: "Logged out successfully!".to_string(),
    }))
}
// endregion: --- Auth handlers

// region:    --- Payload & Response structs
#[derive(Debug, Deserialize, serde_valid::Validate)]
pub struct RegistrationPayload {
    #[validate(min_length = 1, message = "Username is required")]
    pub username: String,

    #[validate(min_length = 1, message = "Email is required")]
    #[validate(
        pattern = r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$",
        message = "Email is invalid"
    )]
    pub email: String,

    #[validate(min_length = 6, message = "Password must be at least 6 characters")]
    pub pwd: String,

    #[validate(min_length = 1, message = "Confirm Password is required")]
    pub pwd_confirm: String,
}

#[derive(Serialize)]
pub struct RegistrationResponse {
    success: bool,
    message: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(min_length = 1, message = "Username can not be empty")]
    username: String,
    #[validate(min_length = 1, message = "Password can not be empty")]
    pwd: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    success: bool,
    message: String,
    user: UserInfo,
    token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutPayload {
    logout: bool,
}

#[derive(Serialize)]
pub struct LogoutResponse {
    success: bool,
    message: String,
}
// endregion: --- Payload & response structs
