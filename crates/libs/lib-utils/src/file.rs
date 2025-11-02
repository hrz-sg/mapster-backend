use std::path::Path;
use crate::mime::get_mime_from_bytes;
use serde::Serialize;

const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const MAX_VIDEO_SIZE: usize = 100 * 1024 * 1024; // 100MB

pub fn validate_file(filename: &str, data: &[u8]) -> Result<String> {
    let mime = get_mime_from_bytes(data, filename);

    if mime.starts_with("image/") {
        if data.len() > MAX_IMAGE_SIZE {
            return Err(Error::FileTooLarge);
        }
    } else if mime.starts_with("video/") {
        if data.len() > MAX_VIDEO_SIZE {
            return Err(Error::FileTooLarge);
        }
    } else {
        return Err(Error::UnsupportedMime);
    }

    // -- Check file extension
    let ext = Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let valid_ext = if mime.starts_with("image/") {
        ["jpg", "jpeg", "png", "webp"].contains(&ext.as_str())
    } else {
        ["mp4", "mov"].contains(&ext.as_str())
    };

    if !valid_ext {
        return Err(Error::UnsupportedMime);
    }

    Ok(mime)
}

// region:    --- Error
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
	FileTooLarge,
    UnsupportedMime,
    ValidationFail(String),
}
// endregion: --- Error

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