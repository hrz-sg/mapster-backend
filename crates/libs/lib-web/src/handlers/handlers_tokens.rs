use axum::{extract::State, http::HeaderMap, Json};
use serde_json::json;
use lib_core::model::user::{UserBmc, UserForAuth};
use lib_core::ctx::Ctx;
use lib_auth::token::{validate_web_token, generate_web_tokens, access_token_ttl};
use lib_core::model::ModelManager;
use crate::error::{Error, Result};
use crate::utils::token::extract_bearer_token;

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
    let user: UserForAuth = UserBmc::first_by_username(&Ctx::root_ctx(), &mm, &claims.sub)
        .await?
		.ok_or(Error::LoginFailUsernameNotFound)?;

    // -- Validate salt after changing password
    if claims.salt != user.token_salt.to_string() {
        return Err(Error::Token(lib_auth::token::Error::InvalidToken));
    }

    // -- Ggenerate new tokens
    let (access_token, refresh_token) = generate_web_tokens(&user.username, user.token_salt)?;

    Ok(Json(json!({
        "access_token": access_token,
        "refresh_token": refresh_token,
        "expires_in": access_token_ttl()
    })))

}
