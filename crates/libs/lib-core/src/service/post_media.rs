use crate::ctx::Ctx;
use crate::model::{ModelManager, Result};
use crate::model::post_media::{PostMedia, PostMediaBmc, PostMediaForCreate, PostMediaForUpdate};
use crate::service::media_storage::{Storage, MediaStorageService};
use lib_utils::file::validate_file;

pub struct PostMediaService<S: Storage> {
    storage: S,
}

impl Default for PostMediaService<MediaStorageService> {
    fn default() -> Self {
        Self::new(MediaStorageService::new())
    }
}

impl<S: Storage> PostMediaService<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub async fn upload_and_create(
        &self,
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
        filename: &str,
        data: &[u8],
        sort_order: i32,
    ) -> Result<String> {
        let (mime, media_type) = validate_file(filename, data)?;

        let media_url = self.storage.upload(filename, data, &mime).await?;

        PostMediaBmc::create(
            ctx,
            mm,
            PostMediaForCreate {
                post_id: post_id.to_string(),
                media_url,
                media_type,
                mime_type: mime,
                width: None,
                height: None,
                file_size: Some(data.len() as i64),
                duration: None,
                sort_order,
            },
        )
        .await
    }

    pub async fn replace_media(
        &self,
        ctx: &Ctx,
        mm: &ModelManager,
        media_id: &str,
        post_id: &str,
        filename: &str,
        data: &[u8],
    ) -> Result<()> {
        let old = PostMediaBmc::get(ctx, mm, media_id).await?;
        if old.post_id != post_id {
            tracing::warn!("skip updating media id={} (belongs to another post)", media_id);
            return Ok(());
        }

        // -- Delete old file
        self.storage.delete_by_url(&old.media_url).await?;

        // -- Update new file
        let (mime, media_type) = validate_file(filename, data)?;
        let media_url = self.storage.upload(filename, data, &mime).await?;

        PostMediaBmc::update(
            ctx,
            mm,
            media_id,
            PostMediaForUpdate {
                media_url: Some(media_url),
                mime_type: Some(mime),
                media_type: Some(media_type),
                file_size: Some(data.len() as i64),
                ..Default::default()
            },
        )
        .await
    }

    pub async fn delete_many(&self, ctx: &Ctx, mm: &ModelManager, ids: &[String]) -> Result<()> {
        for id in ids {
            self.delete_media(ctx, mm, &id).await?;
        }
        Ok(())
    }

    pub async fn delete_media(&self, ctx: &Ctx, mm: &ModelManager, media_id: &str) -> Result<()> {
        let media = PostMediaBmc::get(ctx, mm, media_id).await?;
        self.storage.delete_by_url(&media.media_url).await?;
        PostMediaBmc::delete(ctx, mm, media_id).await
    }

    pub async fn list_by_post(&self, ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<Vec<PostMedia>> {
        PostMediaBmc::list_by_post(ctx, mm, post_id).await
    }

    pub async fn next_sort(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<i32> {
        let medias = PostMediaBmc::list_by_post(ctx, mm, post_id).await?;
        Ok(medias.iter().map(|m| m.sort_order).max().unwrap_or(-1) + 1)
    }
}