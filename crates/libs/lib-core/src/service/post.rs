use std::vec::Vec;
use crate::ctx::Ctx;
use crate::model::post::Post;
use crate::model::{
    Result, ModelManager,
    post::{PostBmc, PostForCreate},
    post_media::{PostMediaBmc, PostMediaForCreate},
};
use crate::service::media_storage::{MediaStorageService, Storage};
use crate::service::post_media::PostMediaService;
use crate::service::thumbnail::ThumbnailService;
use lib_utils::file::validate_file;

pub struct PostService;

impl PostService {
    /// --- Create post with media files
    pub async fn create_with_media(
        ctx: &Ctx,
        mm: &ModelManager,
        payload: CreatePostPayload,
    ) -> Result<i64> {
        let CreatePostPayload {
            title,
            description,
            files,
            thumbnail,
        } = payload;

        // --- services
        let storage = MediaStorageService::new();

        // -- Create tx manager
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let mut media_infos = Vec::new();
        let mut has_video = false;

        // -- Upload all media
        for (filename, data) in &files {
            let (mime, media_type) = validate_file(filename, data)?;
            let media_url = storage.upload(filename, data, &mime).await?;
            if media_type == "video" {
                has_video = true;
            }
            media_infos.push((media_url, mime, media_type.to_string(), data.clone()));
        }

        // -- Create cover & thumbnail
        let (cover_media_url, thumbnail_url) = if let Some(first) = media_infos.first() {
            let cover_url = first.0.clone();

            // if clients sends thumbnail - use it
            if let Some(ref thumb_bytes) = thumbnail {
                let thumb_url = ThumbnailService::generate_and_upload(
                    &first.1,
                    &first.3,
                    Some(thumb_bytes),
                )
                .await?;
                (Some(cover_url), Some(thumb_url))
            }
            // if video — autogenerate thumbnail
            else if first.2 == "video" {
                let thumb_url = ThumbnailService::generate_and_upload(
                    &first.1,
                    &first.3,
                    None,
                )
                .await?;
                (Some(cover_url), Some(thumb_url))
            }
            // if photo — thumbnail = cover
            else {
                (Some(cover_url.clone()), Some(cover_url))
            }
        } else {
            (None, None)
        };

        // -- Create post
        let post_id = PostBmc::create(
            ctx,
            &mm_txn,
            PostForCreate {
                user_id: ctx.user_id(),
                title,
                description,
                is_published: Some(true),
                cover_media_url,
                thumbnail_url,
                media_count: Some(media_infos.len() as i32),
                has_video: Some(has_video),
            },
        )
        .await?;

        // -- Save post media data
        for (i, (url, mime, media_type, data)) in media_infos.into_iter().enumerate() {
            PostMediaBmc::create(
                ctx,
                &mm_txn, 
                PostMediaForCreate {
                    post_id,
                    media_url: url,
                    media_type,
                    mime_type: mime,
                    width: None,
                    height: None,
                    file_size: Some(data.len() as i64),
                    duration: None,
                    sort_order: i as i32,
                    alt_text: None,
                },
            )
            .await?;
        }

        dbx.commit_txn().await?;
        Ok(post_id)
    }

    // --- Get post for preview in feed
    pub async fn get_post(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: i64,
    ) -> Result<Post> {
        PostBmc::get(ctx, mm, post_id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Post>> {
        PostBmc::list(ctx, mm, None, None).await
    }

    // --- Update post
    pub async fn update_with_media(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: i64,
        payload: UpdatePostPayload,
    ) -> Result<()> {
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let media_svc = PostMediaService::default();

        if let Some(ids) = &payload.remove_ids {
            media_svc.delete_many(ctx, &mm_txn, ids).await?;
        }

        if let Some(update_files) = &payload.update_files {
            for (id, name, data) in update_files {
                media_svc.replace_media(ctx, &mm_txn, *id, post_id, name, data).await?;
            }
        }

        if let Some(add_files) = &payload.add_files {
            let next_sort = PostMediaService::<MediaStorageService>::next_sort(ctx, &mm_txn, post_id).await?;
            for (i, (name, data)) in add_files.iter().enumerate() {
                media_svc.upload_and_create(ctx, &mm_txn, post_id, name, data, next_sort + i as i32).await?;
            }
        }

        dbx.commit_txn().await?;
        Ok(())
    }

    // --- Delete post
    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let media_svc = PostMediaService::default();

        let medias = media_svc.list_by_post(ctx, &mm_txn, id).await?;

        for media in medias {
            media_svc.delete_media(ctx, &mm_txn, media.id).await?;
        }

        PostBmc::delete(ctx, &mm_txn, id).await?;
        dbx.commit_txn().await?;
        Ok(())
    }

}

#[derive(Debug)]
pub struct UpdatePostPayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_published: Option<bool>,
    pub add_files: Option<Vec<(String, Vec<u8>)>>,  
    pub update_files: Option<Vec<(i64, String, Vec<u8>)>>, 
    pub remove_ids: Option<Vec<i64>>,                
    pub new_cover_id: Option<i64>,                   
}

#[derive(Debug)]
pub struct CreatePostPayload {
    pub title: String,
    pub description: String,
    pub files: Vec<(String, Vec<u8>)>,
    pub thumbnail: Option<Vec<u8>>,
}