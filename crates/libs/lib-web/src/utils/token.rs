use axum::{body::Body, extract::Request, http::HeaderMap};
use tower_cookies::{Cookies, Cookie};

use lib_auth::token::generate_web_tokens;
use uuid::Uuid;
pub use crate::error::{Error, Result};

pub(crate) const AUTH_TOKEN: &str = "auth-token";

pub(crate) fn set_token_cookie(
    cookies: &Cookies, 
    user: &str, 
    salt: Uuid
) -> Result<String> {
    let (access_token, _refresh_token) = generate_web_tokens(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, access_token.clone());
    cookie.set_http_only(true);
    cookie.set_secure(!cfg!(debug_assertions)); // true only in release
    cookie.set_path("/"); // Default path is the URI path of the request (which is '/api/login' for login request)

    cookies.add(cookie);

    Ok(access_token)
}

pub(crate) fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}

pub(crate) fn extract_token(req: &Request<Body>, cookies: &Cookies) -> Option<String> {
    
    if let Some(cookie) = cookies.get(AUTH_TOKEN) {
        return Some(cookie.value().to_string());
    }

    if let Some(header_value) = req.headers().get("Authorization") {
        if let Ok(header_str) = header_value.to_str() {
            if let Some(token) = extract_bearer_from_header_str(header_str) {
                return Some(token);
            }
        }
    }

    None
}

pub(crate) fn extract_bearer_token(headers: &HeaderMap) -> Result<String> {
    let header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(Error::Token(lib_auth::token::Error::Unauthorized))?;

    extract_bearer_from_header_str(header)
        .ok_or(Error::Token(lib_auth::token::Error::InvalidToken))
}

fn extract_bearer_from_header_str(header: &str) -> Option<String> {
    if header.starts_with("Bearer ") {
        Some(header.trim_start_matches("Bearer ").to_string())
    } else {
        None
    }
}
