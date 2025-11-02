use std::vec::Vec;

use crate::ctx::Ctx;
use crate::model::{
    Result, ModelManager,
    post::{PostBmc, PostForCreate},
    post_media::{PostMediaBmc, PostMediaForCreate},
};
use lib_storage::oss::OssClient;
use lib_utils::file::validate_file;

pub struct PostService;

#[derive(Debug)]
pub struct CreatePostPayload {
    pub title: String,
    pub description: String,
    pub files: Vec<(String, Vec<u8>)>,
}

impl PostService {
    pub async fn create_with_media(
        ctx: &Ctx,
        mm: &ModelManager,
        payload: CreatePostPayload,
    ) -> Result<i64> {
        let CreatePostPayload {
            title,
            description,
            files,
        } = payload;

        // -- Create tx manager
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let oss = OssClient::new();
        let mut uploaded_urls = Vec::new();
        let mut media_infos = Vec::new();
        let mut has_video = false;

        for (filename, data) in &files {
            let mime = validate_file(filename, data)?;
            let (url, _) = oss.upload(filename, data).await?;
            uploaded_urls.push(url.clone());
            if mime.starts_with("video/") {
                has_video = true;
            }
            media_infos.push((url, mime, data.len()));
        }

        let post_id = PostBmc::create(
            ctx,
            &mm_txn,
            PostForCreate {
                user_id: ctx.user_id(),
                title,
                description,
                is_published: Some(true),
                cover_media_url: Some(media_infos[0].0.clone()),
                thumbnail_url: None,
                media_count: Some(media_infos.len() as i32),
                has_video: Some(has_video),
            },
        )
        .await?;

        for (i, (url, mime, size)) in media_infos.into_iter().enumerate() {
            PostMediaBmc::create(
                ctx,
                &mm_txn, 
                PostMediaForCreate {
                    post_id,
                    media_url: url,
                    media_type: if mime.starts_with("video/") {
                        "video".into()
                    } else {
                        "image".into()
                    },
                    mime_type: mime,
                    width: None,
                    height: None,
                    file_size: Some(size as i64),
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
}
