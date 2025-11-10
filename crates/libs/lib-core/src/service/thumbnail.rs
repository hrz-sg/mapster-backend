use lib_utils::media::generate_thumbnail;
use crate::service::media_storage::{MediaStorageService, Storage};
use crate::model::Result;

pub struct ThumbnailService;

impl ThumbnailService {
    pub async fn generate_and_upload(
        mime: &str,
        data: &[u8],
        thumbnail_override: Option<&[u8]>,
    ) -> Result<String> {
        let bytes = generate_thumbnail(mime, data, thumbnail_override).await?;
        let storage = MediaStorageService::new();
        storage.upload("thumb_post_cover.jpg", &bytes, "image/jpeg").await
    }
}
