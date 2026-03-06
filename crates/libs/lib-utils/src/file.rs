use serde::Serialize;
use std::path::Path;

pub const MAX_IMAGE_SIZE: u64 = 100 * 1024 * 1024;  // 100MB
pub const MAX_VIDEO_SIZE: u64 = 1024 * 1024 * 1024;  // 1GB
pub const MULTIPART_THRESHOLD: u64 = 10 * 1024 * 1024; // 10MB
pub const MIN_PART_SIZE: u64 = 5 * 1024 * 1024; // 5MB
pub const MAX_PARTS: u64 = 10_000;

pub fn calc_part_size(file_size: u64) -> u64 {
    let mut part_size = 
        if file_size < 50 * 1024 * 1024 {
            5 * 1024 * 1024
        } else if file_size < 500 * 1024 * 1024 {
            10 * 1024 * 1024
        } else {
            20 * 1024 * 1024
        };

    // Guarantee that parts <= 10k
    let parts = file_size / part_size;
    if parts > MAX_PARTS {
        part_size = file_size / MAX_PARTS + 1;
    }

    // Guarantee >= MIN_PART_SIZE
    part_size.max(MIN_PART_SIZE)
}

pub fn validate_file_meta(
    filename: &str,
    content_type: &str,
    size: u64,
) -> Result<String> {
    let media_type = extract_media_type(content_type);

    if media_type == "unknown" {
        return Err(Error::UnsupportedMime);
    }

    validate_meta_size(media_type, size)?;
    validate_ext(filename, media_type)?;

    Ok(media_type.to_string())
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

pub fn get_ext(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase()
}

fn validate_ext(filename: &str, media_type: &str) -> Result<()> {
    let ext = get_ext(filename);

    let valid = match media_type {
        "image" => ["jpg", "jpeg", "png", "webp"].contains(&ext.as_str()),
        "video" => ["mp4", "mov"].contains(&ext.as_str()),
        _ => false,
    };

    if valid {
        Ok(())
    } else {
        Err(Error::UnsupportedMime)
    }
}

fn validate_meta_size(media_type: &str, size: u64) -> Result<()> {
    match media_type {
        "image" if size > MAX_IMAGE_SIZE => Err(Error::FileTooLarge),
        "video" if size > MAX_VIDEO_SIZE => Err(Error::FileTooLarge),
        "unknown" => Err(Error::UnsupportedMime),
        _ => Ok(()),
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
