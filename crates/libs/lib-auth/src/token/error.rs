use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
	HmacFailNewFromSlice,

	InvalidToken,
	CannotDecodeIdent,
	CannotDecodeExp,
	TokenSignatureMismatch,
	ExpNotIso,
	ExpiredToken,
	TokenDecodeFailed,
	TokenCreationFailed,
	Unauthorized,
	InvalidSubject
}

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

/// Convert from jsonwebtoken::Error
impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind::*;
        match err.kind() {
            ExpiredSignature => Self::ExpiredToken,
            InvalidSignature => Self::TokenSignatureMismatch,
            InvalidToken => Self::InvalidToken,
            _ => Self::TokenDecodeFailed,
        }
    }
}