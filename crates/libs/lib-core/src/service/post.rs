use std::collections::HashMap;
use std::vec::Vec;
use crate::ctx::Ctx;
use crate::model::post::{PostFeedItem, PostFilter, PostDetail};
use crate::model::user::{User, UserBmc, UserForPreview};
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
    ) -> Result<String> {
        let CreatePostPayload {
            title,
            description,
            media,
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
        for (filename, data) in &media {
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
                    post_id: post_id.clone(),
                    media_url: url,
                    media_type,
                    mime_type: mime,
                    width: None,
                    height: None,
                    file_size: Some(data.len() as i64),
                    duration: None,
                    sort_order: i as i32,
                },
            )
            .await?;
        }

        dbx.commit_txn().await?;
        Ok(post_id)
    }

    /// --- Get post details
    pub async fn get_post_detail_by_id(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
    ) -> Result<PostDetail> {
        // -- Get the post itself
        let post = PostBmc::get(ctx, mm, post_id).await?;

        // -- Get Post author
        let user: User = UserBmc::get(ctx, mm, &post.owner_id).await?;

        // -- Get all Post Medias
        let medias = PostMediaBmc::list_by_post(ctx, mm, post_id).await?;

        // -- Create and send PostDetail
        Ok(PostDetail {
            id: post.id,
            title: post.title,
            description: post.description,
            author: UserForPreview {
                id: user.id,
                username: user.username,
                avatar_url: None,
            },
            thumbnail_url: post.thumbnail_url,
            medias,
            like_count: post.like_count,
            comment_count: post.comment_count,
            saved_count: post.saved_count,
        })
    }

    /// --- Get posts list for feed
    pub async fn list_feed_posts(
        ctx: &Ctx, 
        mm: &ModelManager
    ) -> Result<Vec<PostFeedItem>> {
        let posts = PostBmc::list(ctx, mm, None, None).await?;
        let user_ids: Vec<String> = posts.iter().map(|p| p.owner_id.clone()).collect();

        let users = UserBmc::list_by_ids(ctx, mm, &user_ids).await?;
        let user_map: HashMap<String, UserForPreview> = users 
            .into_iter()
            .map(|u| (u.id.clone(), u))
            .collect();

        let feed = posts
            .into_iter()
            .map(|post| {
                let user = user_map.get(&post.owner_id);
                let author = user.map_or(
                    UserForPreview {
                        id: post.owner_id,
                        username: "Unknown".into(),
                        avatar_url: None,
                    },
                    |u| UserForPreview {
                        id: u.id.clone(),
                        username: u.username.clone(),
                        avatar_url: u.avatar_url.clone(),
                    },
                );

                PostFeedItem {
                    id: post.id,
                    title: post.title,
                    author,
                    thumbnail_url: post.thumbnail_url,
                    media_count: post.media_count,
                    has_video: post.has_video,
                    like_count: post.like_count,
                }
            })
            .collect();

        Ok(feed)
    }

    /// --- Get user posts
    pub async fn list_user_posts(
    ctx: &Ctx,
    mm: &ModelManager,
    user_id: &str,
) -> Result<Vec<PostFeedItem>> {

        // -- Create filters
        let filters = vec![PostFilter::by_user(user_id)];

        // -- Get User posts
        let posts = PostBmc::list(ctx, mm, Some(filters), None).await?;

        // -- Get User data
        let users = UserBmc::list_by_ids(ctx, mm, &[user_id.to_string()]).await?;
        let user_map: HashMap<String, UserForPreview> = users
            .into_iter()
            .map(|u| (u.id.clone(), u))  
            .collect();

        // -- Create DTO
        let feed = posts.into_iter().map(|post| {
            let user = user_map.get(&post.owner_id);
            let author = user.map_or(
                UserForPreview {
                    id: post.owner_id,
                    username: "Unknown".into(),
                    avatar_url: None,
                },
                |u| UserForPreview {
                    id: u.id.clone(),
                    username: u.username.clone(),
                    avatar_url: u.avatar_url.clone(),
                },
            );

            PostFeedItem {
                id: post.id,
                title: post.title,
                author,
                thumbnail_url: post.thumbnail_url,
                media_count: post.media_count,
                has_video: post.has_video,
                like_count: post.like_count,
            }
        }).collect();

        Ok(feed)
    }

    // --- Update post
    pub async fn update_with_media(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
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
                media_svc.replace_media(ctx, &mm_txn, &id, post_id, name, data).await?;
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
    pub async fn delete(
        ctx: &Ctx, 
        mm: &ModelManager, 
        id: &str
    ) -> Result<()> {
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let media_svc = PostMediaService::default();

        let medias = media_svc.list_by_post(ctx, &mm_txn, id).await?;

        for media in medias {
            media_svc.delete_media(ctx, &mm_txn, &media.id).await?;
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
    pub update_files: Option<Vec<(String, String, Vec<u8>)>>, 
    pub remove_ids: Option<Vec<String>>,                
    pub new_cover_id: Option<String>,                   
}

#[derive(Debug)]
pub struct CreatePostPayload {
    pub title: String,
    pub description: String,
    pub media: Vec<(String, Vec<u8>)>,
    pub thumbnail: Option<Vec<u8>>,
}
