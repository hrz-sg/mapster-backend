use std::sync::Arc;

use axum::{http::StatusCode, response::{IntoResponse, Response}};
use lib_auth::token;
use lib_core::model;
use serde::Serialize;
use tracing::debug;
use derive_more::From;
use crate::middleware;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPwd { user_id: i64 },
    LoginFailPwdNotMatching { user_id: i64 },
    
    // -- CtxExtError
    CtxExt(middleware::mw_auth::CtxExtError),

    // -- Extractors
	ReqStampNotInReqExt,

    // -- Config
    ConfigMissingEnv(&'static str),
    ConfigWrongFormat(&'static str),

    #[from]
	Token(token::Error),
    
    // - Modules
	Model(model::Error),
}

// region: ---- Froms
impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Self::Model(val)
    }
}
// endregion: ---- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		debug!("{:<12} - model::Error {self:?}", "INTO_RES");

		// Create a placeholder Axum reponse.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(Arc::new(self));

		response
	}
}

// region: ---- Error Boilerplate
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use Error::*;

        #[allow(unreachable_patterns)]
        match self {
            // -- Login
            Self::LoginFailUsernameNotFound
            | Self::LoginFailUserHasNoPwd { user_id: _ }
            | Self::LoginFailPwdNotMatching { user_id: _ } => {
                (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
            }

            // -- Auth
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Validation
            Self::Model(model::Error::ValidationFail(_)) => {
                (StatusCode::BAD_REQUEST, ClientError::RPC_PARAMS_INVALID("Validation failed".to_string()))
            }

            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR, 
                ClientError::SERVICE_ERROR
            ),
        }
    }
}
// endregion: ---- Error Boilerplate

// region: ---- ClientError
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
    // SERVICE_ERROR,
	ENTITY_NOT_FOUND { entity: &'static str, id: i64 },

	RPC_REQUEST_INVALID(String),
	RPC_REQUEST_METHOD_UNKNOWN(String),
	RPC_PARAMS_INVALID(String),

	SERVICE_ERROR,
}
// endregion: ---- ClientError