use axum::extract::{FromRequestParts, State};
use axum::http::Request;            
use axum::body::Body;             
use axum::middleware::Next;
use axum::http::request::Parts;
use axum::response::Response;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;
use serde::Serialize;
use lib_auth::token::validate_web_token;
use lib_core::ctx::Ctx;
use lib_core::model::user::{UserBmc, UserForAuth};
use lib_core::model::ModelManager;
use crate::utils::token::{extract_token, set_token_cookie, AUTH_TOKEN};
use crate::error::{Error, Result};

pub async fn mw_ctx_require(
    ctx: Result<CtxW>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {

    debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    debug!(" {:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let token = extract_token(&req, &cookies);

    let ctx_ext_result = match token {
        Some(token) => ctx_resolve(mm, &cookies, token).await,
        None => Err(CtxExtError::TokenMissing)
    };

    // if token not valid - delete cookie
    if ctx_ext_result.is_err() 
        && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie))
    {
        cookies.remove(Cookie::build(AUTH_TOKEN).into())
    }

    // Store the ctx_ext_result in the request extension
    // (for Ctx extractor)
    req.extensions_mut().insert(ctx_ext_result);

    next.run(req).await
}

async fn ctx_resolve(
    mm: ModelManager, 
    cookies: &Cookies,
    token: String,
) -> CtxExtResult {

    // -- Check token
    let claims = validate_web_token(&token)
        .map_err(|_| CtxExtError::FailValidate)?;

    // -- Get UserForAuth
    let user: UserForAuth = 
        UserBmc::first_by_username(&Ctx::root_ctx(), &mm, &claims.sub)
            .await
            .map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
            .ok_or(CtxExtError::UserNotFound)?;
    
    // -- Validate Salt
    if claims.salt != user.token_salt.to_string() {
        return Err(CtxExtError::FailValidate);
    }

    // -- Update Token if we get get it from Cookie
    if cookies.get(AUTH_TOKEN).is_some() {
        set_token_cookie(cookies, &user.username, user.token_salt)
            .map_err(|_| CtxExtError::CannotSetTokenCookie)?;
    }

    // -- Create CtxExtResult
    Ctx::new(user.id)
        .map(CtxW)
        .map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

// region: ---- Ctx Extractor
#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

impl<S: Send + Sync> FromRequestParts<S> for CtxW {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		debug!("{:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<CtxExtResult>()
			.ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
			.clone()
			.map_err(Error::CtxExt)
	}
}

// endregion: ---- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = core::result::Result<CtxW, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,
    TokenMissing,
    TokenWrongFormat,
    UserNotFound,
    ModelAccessError(String),
    FailValidate,
    CannotSetTokenCookie,

    CtxNotInRequestExt,
    CtxCreateFail(String),
}
// endregion: ---- Ctx Extractor Result/Error