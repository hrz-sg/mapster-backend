use crate::error::{Error, Result};
use crate::utils::token;
use axum::extract::State;
use axum::Json;
use lib_auth::pwd::{self, ContentToHash, SchemeStatus};
use lib_core::ctx::Ctx;
use lib_core::model::user::{UserBmc, UserDTO, UserForLogin};
use lib_core::model::ModelManager;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;
use tracing::debug;

// region:    --- Login
pub async fn api_login_handler(
	State(mm): State<ModelManager>,
	cookies: Cookies,
	Json(payload): Json<LoginPayload>,
) -> Result<Json<LoginResponse>> {
	debug!("{:<12} - api_login_handler", "HANDLER");

	let LoginPayload {
		username,
		pwd: pwd_clear,
	} = payload;

	let root_ctx = Ctx::root_ctx();

	// -- Get the user.
	let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
		.await?
		.ok_or(Error::LoginFailUsernameNotFound)?;
	let user_id = user.id;

	// -- Validate the password.
	let Some(pwd) = user.pwd else {
		return Err(Error::LoginFailUserHasNoPwd { user_id });
	};

	let scheme_status = pwd::validate_pwd(
		ContentToHash {
			salt: user.pwd_salt,
			content: pwd_clear.clone(),
		},
		pwd,
	)
	.await
	.map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

	// -- Update password scheme if needed
	if let SchemeStatus::Outdated = scheme_status {
		debug!("pwd encrypt scheme outdated, upgrading.");
		UserBmc::update_pwd(&root_ctx, &mm, user.id, &pwd_clear).await?;
	}

	// -- Set web token.
	let access_token = token::set_token_cookie(&cookies, &user.username, user.token_salt)?;

	// Create the success body.
	 Ok(Json(LoginResponse {
        success: true,
        message: format!("Welcome back, {}!", username),
        user: UserDTO { id: user_id, username },
        token: Some(access_token),
    }))
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
	username: String,
	pwd: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
	success: bool,
	message: String,
	user: UserDTO,
	token: Option<String>,
}
// endregion: --- Login

// region:    --- Logout
pub async fn api_logout_handler(
	cookies: Cookies,
	Json(payload): Json<LogoutPayload>,
) -> Result<Json<LogoutResponse>> {
	debug!("{:<12} - api_logout_handler", "HANDLER");

	let should_logout = payload.logout;

	if should_logout {
		token::remove_token_cookie(&cookies)?;
	}

	// Create and return the success body.
	Ok(Json(LogoutResponse {
		success: true,
		message: "Logged out successfully!".to_string()
    }))
}

#[derive(Debug, Deserialize)]
pub struct LogoutPayload {
	logout: bool,
}

#[derive(Serialize)]
pub struct LogoutResponse {
	success: bool,
	message: String
}
// endregion: --- Logout
