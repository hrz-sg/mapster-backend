use tower_cookies::{Cookies, Cookie};

use lib_auth::token::generate_web_token;
use uuid::Uuid;
pub use crate::error::{Error, Result};

pub(crate) const AUTH_TOKEN: &str = "auth-token";

pub(crate) fn set_token_cookie(cookies: &Cookies, user: &str, salt: Uuid) -> Result<()> {
    let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/"); // Default path is the URI path of the request (which is '/api/login' for login request)

    cookies.add(cookie);

    Ok(())
}

pub(crate) fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}