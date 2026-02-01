use crate::error::{Error, Result};
use crate::utils::token::extract_bearer_token;
use axum::{Json, extract::State, http::HeaderMap};
use lib_auth::auth_config;
use lib_auth::token::{generate_web_tokens, validate_web_token};
use lib_core::model::ModelManager;
use lib_core::service::user::UserService;
use serde_json::json;

pub async fn api_refresh_token_handler(
    State(mm): State<ModelManager>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(Error::Token(lib_auth::token::Error::Unauthorized))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(Error::Token(lib_auth::token::Error::InvalidToken));
    }

    let token = extract_bearer_token(&headers)?;
    let claims = validate_web_token(&token.to_string())?;

    // -- Check token type
    if claims.typ != "refresh" {
        return Err(Error::Token(lib_auth::token::Error::InvalidToken));
    }

    // -- Find user
    let user = UserService::validate_refresh_token(&mm, &claims.sub, &claims.salt.to_string()).await?;

    // -- Validate salt after changing password
    if claims.salt != user.token_salt.to_string() {
        return Err(Error::Token(lib_auth::token::Error::InvalidToken));
    }

    // -- Ggenerate new tokens
    let (access_token, refresh_token) = generate_web_tokens(&user.username, user.token_salt)?;

    Ok(Json(json!({
        "access_token": access_token,
        "refresh_token": refresh_token,
        "expires_in": auth_config().ACCESS_TOKEN_TTL
    })))
}
