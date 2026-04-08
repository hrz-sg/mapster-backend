use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

// need to refactor other error enums

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}