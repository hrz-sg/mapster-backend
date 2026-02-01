use image::imageops::FilterType::CatmullRom;
use image::{ImageFormat, ImageReader};
use std::fs;
use std::io::Cursor;
use std::process::{Command, Stdio};

pub async fn generate_thumbnail(mime: &str, data: &[u8], thumbnail_override: Option<&[u8]>) -> Result<Vec<u8>> {
    if let Some(thumb) = thumbnail_override {
        return Ok(thumb.to_vec());
    }

    if mime.starts_with("image/") {
        generate_image_thumbnail(data)
    } else if mime.starts_with("video/") {
        generate_video_thumbnail(data).await
    } else {
        Ok(Vec::new())
    }
}

/// --- Generate image thumbnail
pub fn generate_image_thumbnail(data: &[u8]) -> Result<Vec<u8>> {
    let img = ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .map_err(|_| Error::DecodeError)?
        .decode()
        .map_err(|_| Error::DecodeError)?;

    let thumbnail = img.resize(400, 400, CatmullRom);

    let mut buffer = Vec::new();
    thumbnail
        .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Jpeg)
        .map_err(|_| Error::EncodeError)?;

    Ok(buffer)
}

/// --- Generate video thumbnail
pub async fn generate_video_thumbnail(data: &[u8]) -> Result<Vec<u8>> {
    use std::time::SystemTime;

    let timestamp = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| Error::Io)?
        .as_nanos();

    let video_path = format!("temp_video_{}.mp4", timestamp);
    let thumb_path = format!("temp_thumb_{}.jpg", timestamp);

    // Save input video to a temporary file
    fs::write(&video_path, data).map_err(|_| Error::Io)?;

    // Generate thumbnail with ffmpeg
    let output = Command::new("ffmpeg")
        .args(&[
            "-i",
            &video_path,
            "-ss",
            "00:00:01",
            "-vframes",
            "1",
            "-q:v",
            "2",
            "-y",
            &thumb_path,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .map_err(|_| Error::FfmpegError)?;

    // Cleanup video temp file
    let _ = fs::remove_file(&video_path);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("FFmpeg failed: {}", stderr);
        let _ = fs::remove_file(&thumb_path);
        return Err(Error::FfmpegError);
    }

    // Read thumbnail bytes
    let thumb_bytes = fs::read(&thumb_path).map_err(|_| Error::Io)?;

    // Cleanup thumb temp file
    let _ = fs::remove_file(&thumb_path);

    Ok(thumb_bytes)
}

// region:    --- Error
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io,
    DecodeError,
    EncodeError,
    FfmpegError,
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
