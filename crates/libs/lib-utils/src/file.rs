use crate::mime::get_mime_from_bytes;
use serde::Serialize;
use std::path::Path;

const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const MAX_VIDEO_SIZE: usize = 100 * 1024 * 1024; // 100MB

pub fn validate_file(filename: &str, data: &[u8]) -> Result<(String, String)> {
    let mime = get_mime_from_bytes(data, filename);
    let media_type = extract_media_type(&mime);

    match media_type {
        "image" if data.len() > MAX_IMAGE_SIZE => return Err(Error::FileTooLarge),
        "video" if data.len() > MAX_VIDEO_SIZE => return Err(Error::FileTooLarge),
        "unknown" => return Err(Error::UnsupportedMime),
        _ => {}
    }

    // -- Check file extension
    let ext = Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let valid_ext = match media_type {
        "image" => ["jpg", "jpeg", "png", "webp"].contains(&ext.as_str()),
        "video" => ["mp4", "mov"].contains(&ext.as_str()),
        _ => false,
    };

    if !valid_ext {
        return Err(Error::UnsupportedMime);
    }

    Ok((mime, media_type.to_string()))
}

pub fn extract_media_type(mime: &str) -> &'static str {
    if mime.starts_with("image/") {
        "image"
    } else if mime.starts_with("video/") {
        "video"
    } else {
        "unknown"
    }
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
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
