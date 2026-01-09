use crate::{model::Error as ModelError};
use derive_more::From;
use crate::model::store::dbx;
use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    /// --- Errors for user
    EntityNotFound(String),
    EntityAlreadyExists(String),
    Validation(String),
    PermissionDenied(String),

    /// --- Dbx (transactions)
    #[from]
    Dbx(dbx::Error),

    /// --- Technical errors
    Internal,
}

impl Error {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Error::EntityNotFound(msg.into())
    }
    
    pub fn already_exists(msg: impl Into<String>) -> Self {
        Error::EntityAlreadyExists(msg.into())
    }
    
    pub fn validation(msg: impl Into<String>) -> Self {
        Error::Validation(msg.into())
    }
    
    pub fn permission_denied(msg: impl Into<String>) -> Self {
        Error::PermissionDenied(msg.into())
    }

    pub fn internal() -> Self { Error::Internal }
}

// region:    --- Froms

impl From<ModelError> for Error {
    fn from(err: ModelError) -> Self {
        match err {
            ModelError::EntityNotFound { entity, .. } => 
                Error::EntityNotFound(format!("{} not found", entity)),
            
            ModelError::UniqueViolation { table, .. } => 
                Error::EntityAlreadyExists(format!("{} already exists", table)),
                
            ModelError::Dbx(_) => {
                Error::Internal
            }
            
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