use crate::middleware;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use derive_more::From;
use lib_auth::token;
use lib_core::service;
use lib_storage::oss;
use lib_utils::file;
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPwd {
        user_id: String,
    },
    LoginFailPwdNotMatching {
        user_id: String,
    },

    // -- Entity
    EntityNotFound,

    // -- CtxExtError
    CtxExt(middleware::mw_auth::CtxExtError),

    // -- Extractors
    ReqStampNotInReqExt,

    // -- Config
    ConfigMissingEnv(&'static str),
    ConfigWrongFormat(&'static str),

    #[from]
    Token(token::Error),

    // -- Payload Validation
    ValidationFailed(String),

    // -- Service
    Service(service::Error),

    // -- OSS
    #[from]
    Oss(oss::Error),

    // -- File
    #[from]
    File(file::Error),
}

// region: ---- Froms
impl From<service::Error> for Error {
    fn from(val: service::Error) -> Self {
        Self::Service(val)
    }
}

impl From<serde_valid::validation::Errors> for Error {
    fn from(err: serde_valid::validation::Errors) -> Self {
        Self::ValidationFailed(err.to_string())
    }
}
// endregion: ---- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - lib_web::Error {self:?}", "INTO_RES");

        let (status, client_error) = self.client_status_and_error();

        let body = axum::Json(serde_json::json!({
            "success": false,
            "error": client_error
        }));

        (status, body).into_response()
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
            | Self::LoginFailPwdNotMatching { user_id: _ } => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Auth
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Validation
            Self::Service(service::Error::ValidationFailed(_)) => (
                StatusCode::BAD_REQUEST,
                ClientError::RPC_PARAMS_INVALID("Validation failed".to_string()),
            ),

            Self::Service(service::Error::LoginUsernameNotFound)
            | Self::Service(service::Error::LoginUserHasNoPwd)
            | Self::Service(service::Error::LoginPwdNotMatching) => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Entity not found
            Self::Service(service::Error::EntityNotFound { entity, id }) => (
                StatusCode::NOT_FOUND,
                ClientError::ENTITY_NOT_FOUND { entity, id: id.clone() },
            ),

            // -- Fallback
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),
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
    ENTITY_NOT_FOUND { entity: &'static str, id: String },

    RPC_REQUEST_INVALID(String),
    RPC_REQUEST_METHOD_UNKNOWN(String),
    RPC_PARAMS_INVALID(String),

    SERVICE_ERROR,
}
// endregion: ---- ClientError
