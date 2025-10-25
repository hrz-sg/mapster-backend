use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::config::auth_config;
mod error;
pub use self::error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub salt: String,
    pub typ: String, // "access" or "refresh"
}

// region:    --- Web Token Gen and Validation

pub fn generate_web_tokens(user: &str, salt: Uuid) -> Result<(String, String)> {
	let access_token = generate_access_token(user, salt,)?;
	let refresh_token = generate_refresh_token(user, salt)?;
	Ok((access_token, refresh_token))
}

fn generate_access_token(user: &str, salt: Uuid) -> Result<String> {
    let config = &auth_config();
    create_jwt_token(user, &config.TOKEN_KEY, config.ACCESS_TOKEN_TTL, salt, "access")
}

fn generate_refresh_token(user: &str, salt: Uuid) -> Result<String> {
    let config = &auth_config();
    create_jwt_token(user, &config.TOKEN_KEY, config.REFRESH_TOKEN_TTL, salt, "refresh")
}

pub fn validate_web_token(token: &String) -> Result<TokenClaims> {
	let config = &auth_config();
	decode_jwt_token(token, &config.TOKEN_KEY)
}

// endregion: --- Web Token Gen and Validation

fn create_jwt_token(
    user_id: &str,
    secret: &[u8],
    expires_in_seconds: i64,
    salt: Uuid,
    typ: &str,
) -> Result<String> {
    if user_id.is_empty() {
        return Err(Error::InvalidSubject);
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::seconds(expires_in_seconds)).timestamp() as usize;
    let claims = TokenClaims {
        sub: user_id.to_string(),
        iat,
        exp,
        salt: salt.to_string(),
        typ: typ.to_string(),
    };

    encode(
        &Header::default(),
        &claims, 
        &EncodingKey::from_secret(secret)
    ).map_err(Error::from)
}

fn decode_jwt_token<T: Into <String>> (
    token: T,
    secret: &[u8]
) -> Result<TokenClaims> {
	let token = token.into();

    let token_decoded = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    ).map_err(Error::from)?;

	Ok(token_decoded.claims)
}
