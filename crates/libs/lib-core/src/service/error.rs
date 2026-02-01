use crate::model;
use crate::model::store::dbx;
use derive_more::From;
use lib_auth::pwd;
use lib_utils::file;
use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    /// --- Login errors
    LoginUsernameNotFound,
    LoginUserHasNoPwd,
    LoginPwdNotMatching,

    /// --- Token errors

    /// --- Errors for user
    EntityNotFound {
        entity: &'static str,
        id: String,
    },
    EntityAlreadyExists(String),
    ValidationFailed(String),
    PermissionDenied(String),

    /// --- Dbx (transactions)
    #[from]
    Dbx(dbx::Error),

    /// --- Password / Auth errors
    #[from]
    Pwd(pwd::Error),

    /// --- Technical errors
    Internal,

    /// --- File errors
    #[from]
    File(file::Error),
}

impl Error {
    pub fn entity_not_found(entity: &'static str, id: impl Into<String>) -> Self {
        Error::EntityNotFound { entity, id: id.into() }
    }

    pub fn already_exists(msg: impl Into<String>) -> Self {
        Error::EntityAlreadyExists(msg.into())
    }

    pub fn validation_failed(msg: impl Into<String>) -> Self {
        Error::ValidationFailed(msg.into())
    }

    pub fn permission_denied(msg: impl Into<String>) -> Self {
        Error::PermissionDenied(msg.into())
    }

    pub fn internal() -> Self {
        Error::Internal
    }
}

// region:    --- Froms

impl From<model::Error> for Error {
    fn from(err: model::Error) -> Self {
        match err {
            model::Error::EntityNotFound { entity, id } => Error::EntityNotFound { entity: entity, id: id },

            model::Error::UniqueViolation { table, .. } => {
                Error::EntityAlreadyExists(format!("{} already exists", table))
            }

            model::Error::Dbx(_) => Error::Internal,

            _ => Error::Internal,
        }
    }
}

// endregion: --- Froms

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
