use crate::ctx::Ctx;
use crate::model::comment::{Comment, CommentBmc, CommentEntityType, CommentForCreate, CommentForUpdate};
use crate::model::post::{
    PostCollection, PostCollectionBmc, PostCollectionForCreate, PostCollectionItemBmc, PostDetail, PostFeedItem,
    PostFilter, PostForwardBmc, PostForwardForCreate, PostLikeBmc, PostStats,
};
use crate::model::user::{User, UserBmc, UserForPreview};
use crate::model::{
    ModelManager,
    post::{PostBmc, PostForCreate},
    post_media::{PostMediaBmc, PostMediaForCreate},
};
use crate::service::error::{Error, Result};
use crate::service::media_storage::{MediaStorageService, Storage};
use crate::service::post_media::PostMediaService;
use crate::service::thumbnail::ThumbnailService;
use lib_utils::file::validate_file;
use modql::filter::ListOptions;
use std::collections::HashMap;
use tracing::info;

pub enum SaveToCollectionOption {
    Default,
    Existing { collection_id: String },
    New { title: String },
}

pub struct PostService;

impl PostService {
    /// --- Create post with media files
    pub async fn create_with_media(ctx: &Ctx, mm: &ModelManager, payload: CreatePostPayload) -> Result<String> {
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
                let thumb_url = ThumbnailService::generate_and_upload(&first.1, &first.3, Some(thumb_bytes)).await?;
                (Some(cover_url), Some(thumb_url))
            }
            // if video — autogenerate thumbnail
            else if first.2 == "video" {
                let thumb_url = ThumbnailService::generate_and_upload(&first.1, &first.3, None).await?;
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
    pub async fn get_post_detail_by_id(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<PostDetail> {
        // -- Get the post itself
        let post = PostBmc::get(ctx, mm, post_id).await?;

        // -- Get Post author
        let user: User = UserBmc::get(ctx, mm, &post.owner_id).await?;

        // -- Get all Post Medias
        let medias = PostMediaBmc::list_by_post(ctx, mm, post_id).await?;

        let user_liked = PostLikeBmc::user_liked_post(ctx, mm, post_id).await?;

        // -- Create and send PostDetail
        Ok(PostDetail {
            id: post.id,
            title: post.title,
            description: post.description,
            author: UserForPreview {
                id: user.id,
                username: user.username,
                avatar_url: user.avatar_url,
            },
            thumbnail_url: post.thumbnail_url,
            medias,
            stats: PostStats {
                like_count: post.like_count,
                comment_count: post.comment_count,
                save_count: post.save_count,
                forward_count: post.forward_count,
                user_liked,
                user_saved: false,     // TODO
                user_forwarded: false, // TODO
            },
        })
    }

    /// --- Get posts list for feed
    pub async fn list_feed_posts(ctx: &Ctx, mm: &ModelManager, limit: u32) -> Result<Vec<PostFeedItem>> {
        let list_options = ListOptions {
            limit: Some(limit.into()),
            offset: None,
            order_bys: Some("RANDOM()".into()),
        };

        let posts = PostBmc::list(ctx, mm, None, Some(list_options)).await?;

        let user_ids: Vec<String> = posts.iter().map(|p| p.owner_id.clone()).collect();

        let users = UserBmc::list_by_ids(ctx, mm, &user_ids).await?;
        let user_map: HashMap<String, UserForPreview> = users.into_iter().map(|u| (u.id.clone(), u)).collect();

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
    pub async fn list_user_posts(ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<Vec<PostFeedItem>> {
        // -- Create filters
        let filters = vec![PostFilter::by_user(user_id)];

        // -- Get User posts
        let posts = PostBmc::list(ctx, mm, Some(filters), None).await?;

        // TODO: NEED JOIN FOR POSTS & USERS TO AVOID N+1

        // -- Get User data
        let users = UserBmc::list_by_ids(ctx, mm, &[user_id.to_string()]).await?;
        let user_map: HashMap<String, UserForPreview> = users.into_iter().map(|u| (u.id.clone(), u)).collect();

        // -- Create DTO
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

    // Toggle likes
    pub async fn create_comment(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
        text: String,
        parent_id: Option<String>,
    ) -> Result<Comment> {
        // -- Check if post exists
        let post = PostBmc::get(ctx, mm, post_id).await?;

        if !post.is_published && post.owner_id != ctx.user_id() {
            return Err(Error::validation_failed("Cannot comment on unpublished post"));
        }

        // -- If it's reply, check the parent
        if let Some(parent_id) = &parent_id {
            let parent = CommentBmc::get(ctx, mm, parent_id).await?;
            if parent.entity_id != post_id {
                return Err(Error::validation_failed("Parent comment belongs to different post"));
            }
        }

        // -- Create comment
        let comment_c = CommentForCreate {
            entity_type: CommentEntityType::Post,
            entity_id: post_id.to_string(),
            parent_id,
            text,
        };

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let comment_id = CommentBmc::create(ctx, &mm_txn, comment_c).await?;

        // -- Increment comment count
        PostBmc::increment_comment_count(ctx, &mm_txn, post_id).await?;

        let comment = CommentBmc::get(ctx, &mm_txn, &comment_id).await?;

        dbx.commit_txn().await?;

        Ok(comment)
    }

    pub async fn delete_comment(ctx: &Ctx, mm: &ModelManager, comment_id: &str) -> Result<()> {
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let comment = CommentBmc::get(ctx, &mm_txn, comment_id).await?;

        // -- Validate the owner
        let post = PostBmc::get(ctx, &mm_txn, &comment.entity_id).await?;
        if comment.owner_id != ctx.user_id() && post.owner_id != ctx.user_id() {
            return Err(Error::permission_denied(
                "You don't have permission to delete this comment",
            ));
        }

        CommentBmc::delete(ctx, &mm_txn, comment_id).await?;

        // -- Decrement comment count
        PostBmc::decrement_comment_count(ctx, &mm_txn, &comment.entity_id).await?;

        dbx.commit_txn().await?;
        Ok(())
    }

    /// --- Update comment
    pub async fn update_comment(ctx: &Ctx, mm: &ModelManager, comment_id: &str, text: String) -> Result<Comment> {
        let comment = CommentBmc::get(ctx, mm, comment_id).await?;

        // -- Check permission
        if comment.owner_id != ctx.user_id() {
            return Err(Error::permission_denied("You are not the owner of this comment"));
        }

        let post = PostBmc::get(ctx, mm, &comment.entity_id).await?;
        if !post.is_published && post.owner_id != ctx.user_id() {
            return Err(Error::permission_denied("Cannot edit comment on unpublished post"));
        }

        let comment_u = CommentForUpdate { text };
        CommentBmc::update(ctx, mm, comment_id, comment_u).await?;

        let updated_comment = CommentBmc::get(ctx, mm, comment_id).await?;
        Ok(updated_comment)
    }

    /// -- Get list of comments for a post
    pub async fn list_comments(ctx: &Ctx, mm: &ModelManager, post_id: &str, limit: u32) -> Result<Vec<Comment>> {
        let list_options = ListOptions {
            limit: Some(limit.into()),
            offset: None,
            order_bys: None,
        };

        // -- Check if post exists
        let post = PostBmc::get(ctx, mm, post_id).await?;

        // -- Check access
        if !post.is_published && post.owner_id != ctx.user_id() {
            return Err(Error::validation_failed("Access denied"));
        }

        let comments =
            CommentBmc::list_by_entity(ctx, mm, CommentEntityType::Post, post_id, Some(list_options)).await?;

        Ok(comments)
    }

    /// --- List comment replies
    pub async fn list_comment_replies(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: &str,
        limit: u32,
    ) -> Result<Vec<Comment>> {
        let list_options = ListOptions {
            limit: Some(limit.into()),
            offset: None,
            order_bys: None,
        };

        let comment = CommentBmc::get(ctx, mm, comment_id).await?;

        // -- Check access
        let post = PostBmc::get(ctx, mm, &comment.entity_id).await?;
        if !post.is_published && post.owner_id != ctx.user_id() {
            return Err(Error::validation_failed("Access denied"));
        }

        let replies = CommentBmc::list_replies(ctx, mm, comment_id, Some(list_options)).await?;
        Ok(replies)
    }

    /// --- Toggle like (increment / decrement)
    pub async fn toggle_like(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<(bool, i64)> {
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check if post exist
        PostBmc::get(ctx, &mm_txn, post_id).await?;

        // -- Check if liked by user
        let liked = PostLikeBmc::user_liked_post(ctx, &mm_txn, post_id).await?;

        if liked {
            // -- Decrement like
            PostLikeBmc::delete_simple(ctx, &mm_txn, post_id).await?;
            PostBmc::decrement_like_count(ctx, &mm_txn, post_id).await?;
        } else {
            // -- Increment like
            PostLikeBmc::create(ctx, &mm_txn, post_id).await?;
            PostBmc::increment_like_count(ctx, &mm_txn, post_id).await?;
        }

        // -- Get updated
        let post = PostBmc::get(ctx, &mm_txn, post_id).await?;

        dbx.commit_txn().await?;

        Ok((!liked, post.like_count))
    }

    pub async fn get_likers(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
        limit: Option<u32>,
    ) -> Result<Vec<UserForPreview>> {
        // -- Check post exists
        PostBmc::exists(ctx, mm, post_id).await?;

        let users = PostLikeBmc::get_likers_with_user_preview(ctx, mm, post_id, limit).await?;
        Ok(users)
    }

    pub async fn get_like_count(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<i64> {
        let post = PostBmc::get(ctx, mm, post_id).await?;
        Ok(post.like_count)
    }

    pub async fn save_to_collection(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
        option: SaveToCollectionOption,
    ) -> Result<()> {
        info!("User {} saving post: {}", ctx.user_id(), post_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Get the post
        let post = PostBmc::get(ctx, &mm_txn, post_id).await?;

        // -- Save only published posts or own posts
        if !post.is_published && post.owner_id != ctx.user_id() {
            dbx.rollback_txn().await?;
            return Err(Error::permission_denied("Cannot save unpublished post"));
        }

        // -- Resolve collection
        let collection = Self::resolve_collection(ctx, mm, option).await?;

        // -- Add post to collection (ON CONFLICT DO NOTHING)
        let is_added = PostCollectionItemBmc::add_to_collection(ctx, &mm_txn, &collection.id, post_id).await?;

        // -- Update counter only if actually added (not a duplicate)
        if is_added {
            PostBmc::increment_save_count(ctx, &mm_txn, post_id).await?;
        }

        dbx.commit_txn().await?;
        Ok(())
    }

    pub async fn unsave_from_collection(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
        collection_id: &str,
    ) -> Result<()> {
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check post exists
        PostBmc::exists(ctx, &mm_txn, post_id).await?;

        PostCollectionItemBmc::remove_from_collection(ctx, &mm_txn, collection_id, post_id).await?;

        PostBmc::decrement_save_count(ctx, &mm_txn, post_id).await?;

        dbx.commit_txn().await?;
        Ok(())
    }

    pub async fn forward_post(ctx: &Ctx, mm: &ModelManager, post_id: &str, chat_id: &str) -> Result<()> {
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let post = PostBmc::get(ctx, &mm_txn, post_id).await?;

        if !post.is_published {
            dbx.rollback_txn().await?;
            return Err(Error::validation_failed("Cannot forward unpublished post"));
        }

        let forward_c = PostForwardForCreate {
            post_id: post_id.to_string(),
            chat_id: chat_id.to_string(),
        };

        let is_added = PostForwardBmc::create_on_conflict(ctx, &mm_txn, forward_c).await?;

        // -- Increment forward count only if forwarded
        if is_added {
            PostBmc::increment_forward_count(ctx, &mm_txn, post_id).await?;
        }

        dbx.commit_txn().await?;
        Ok(())
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
                media_svc
                    .upload_and_create(ctx, &mm_txn, post_id, name, data, next_sort + i as i32)
                    .await?;
            }
        }

        dbx.commit_txn().await?;
        Ok(())
    }

    // --- Delete post
    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<()> {
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

    // helper function to resolve collection
    async fn resolve_collection(
        ctx: &Ctx,
        mm: &ModelManager,
        option: SaveToCollectionOption,
    ) -> Result<PostCollection> {
        match option {
            SaveToCollectionOption::Default => {
                Ok(PostCollectionBmc::get_or_create_default(ctx, mm, ctx.user_id()).await?)
            }

            SaveToCollectionOption::Existing { collection_id } => {
                let collection = PostCollectionBmc::get(ctx, mm, &collection_id).await?;

                if collection.owner_id != ctx.user_id() {
                    return Err(Error::permission_denied("Collection does not belong to you"));
                }

                Ok(collection)
            }

            SaveToCollectionOption::New { title } => Ok({
                let collection_id = PostCollectionBmc::create(
                    ctx,
                    mm,
                    PostCollectionForCreate {
                        title,
                        is_default: false,
                    },
                )
                .await?;

                PostCollectionBmc::get(ctx, mm, &collection_id).await?
            }),
        }
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
