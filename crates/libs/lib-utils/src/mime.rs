use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;
use cans::mime::{set_mime_types};

type MimeMap = HashMap<String, String>;

// --- Lazy init global map
static MIME_MAP: OnceLock<MimeMap> = OnceLock::new();

fn get_mime_map() -> &'static MimeMap {
    MIME_MAP.get_or_init(|| set_mime_types()) // only default types
}

/// --- Return MIME with extension
/// --- If not found — fallback: `application/octet-stream`
pub fn get_mime_from_extension(filename: &str) -> String {
    if let Some(ext) = Path::new(filename).extension() {
        if let Some(ext_str) = ext.to_str() {
            if let Some(mime) = get_mime_map().get(ext_str) {
                return mime.clone();
            }
        }
    }
    "application/octet-stream".to_string()
}

/// --- Return MIME with bytes (magical signatures + fallback)
pub fn get_mime_from_bytes(data: &[u8], filename: &str) -> String {
    if data.len() < 12 {
        return get_mime_from_extension(filename);
    }

    match &data[0..12] {
        [0xFF, 0xD8, ..] => "image/jpeg".to_string(),         // JPEG
        [0x89, b'P', b'N', b'G', ..] => "image/png".to_string(), // PNG
        [b'G', b'I', b'F', ..] => "image/gif".to_string(),     // GIF
        [b'f', b't', b'y', b'p', ..] => "video/mp4".to_string(), // MP4
        _ => get_mime_from_extension(filename),
    }
}
