use crate::ctx::Ctx;
use crate::model::comment::{Comment, CommentBmc, CommentEntityType, CommentForCreate, CommentForUpdate};
use crate::model::post::{
    MediaType, PostCollection, 
    PostCollectionBmc, PostCollectionForCreate, 
    PostCollectionItemBmc, PostDetail, PostFeedItem, 
    PostFilter, PostForUpdate, PostForwardBmc, 
    PostForwardForCreate, PostLikeBmc, 
    PostMediaForDisplay, PostProfileItem, PostStats, PostStatus
};
use crate::model::user::{User, UserBmc, UserForPreview};
use crate::model::{
    ModelManager,
    post::{PostBmc, PostForCreate},
    post_media::{PostMediaBmc, PostMediaForCreate},
};
use crate::service::error::{Error, Result};
use crate::service::post_media::PostMediaService;
use modql::filter::{ListOptions, OpValsString};
use serde::{Deserialize};
use tracing::{debug, info};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub enum SaveToCollectionOption {
    Default,
    Existing { collection_id: String },
    New { title: String },
}

pub struct PostService;

impl PostService {
    pub async fn create_with_media_meta(
        ctx: &Ctx,
        mm: &ModelManager,
        payload: CreatePostPayload,
    ) -> Result<String> {

        let CreatePostPayload {
            title,
            description,
            medias,
            user_cover_key, 
        } = payload;

        if medias.is_empty() {
            return Err(Error::validation_failed("Post must have at least one media"));
        }

        info!("Creating post: {}", title);

        let first_media = &medias[0];
        let mut metas = Vec::with_capacity(medias.len());

        // -- Validate files exist in OSS
        for media in &medias {
            let meta = mm.bucket().head_object(&media.object_key).await?;

            // validate mime_type vs media_type
            match media.media_type {
                MediaType::Image if !media.mime_type.starts_with("image/") => {
                    return Err(Error::validation_failed("Invalid mime type for image"));
                }
                MediaType::Video if !media.mime_type.starts_with("video/") => {
                    return Err(Error::validation_failed("Invalid mime type for video"));
                }
                _ => {}
            }

            metas.push(meta);
        }

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Determine cover media
        let cover_media_key = user_cover_key
            .unwrap_or_else(|| first_media.object_key.clone());

        let post_c = PostForCreate { 
            title, 
            description, 
            status: PostStatus::Published, 
            cover_media_key,
        };

        let post_id = PostBmc::create(ctx, &mm_txn, post_c).await?;

        // -- Create PostMedia
        let post_medias_c: Vec<PostMediaForCreate> = medias
            .iter()
            .zip(metas.iter())
            .enumerate()
            .map(|(idx, (media, meta))| PostMediaForCreate {
                post_id: post_id.clone(),
                object_key: media.object_key.clone(),
                media_type: media.media_type.clone(),
                mime_type: media.mime_type.clone(),
                etag: meta.etag.clone(),
                file_size: meta.content_length as i64,
                width: media.width,
                height: media.height,
                duration: media.duration,
                sort_order: idx as i32,
            })
            .collect();

        // --- Batch insert
        let _ids = PostMediaBmc::create_many(ctx, &mm_txn, post_medias_c).await?;

        dbx.commit_txn().await?;

        info!("Post created successfully: {}", post_id);

        Ok(post_id)
    }

    pub async fn get_post_detail(
        ctx: &Ctx, 
        mm: &ModelManager, 
        post_id: &str,
    ) -> Result<PostDetail> {

        debug!("Fetching post detail: {}", post_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Get post
        let post = PostBmc::get(ctx, &mm_txn, post_id).await?;

        // -- Get author
        let user: User = UserBmc::get(ctx, &mm_txn, &post.owner_id).await?;

        // -- Get all Post Medias
        let medias = PostMediaBmc::list_by_post(ctx, &mm_txn, post_id).await?;

        let media_endpoint = &mm_txn.bucket().public_base;

        // -- Generate media URLs
        let medias: Vec<PostMediaForDisplay> = medias
            .into_iter()
            .map(|m| PostMediaForDisplay {
                id: m.id,
                post_id: post.id.clone(),
                url: format!("{media_endpoint}{}", m.object_key), // generate URLs as we store only object keys in DB
                media_type: m.media_type,
                mime_type: m.mime_type,
                sort_order: m.sort_order,
            })
            .collect();

        let user_liked = PostLikeBmc::user_liked_post(ctx, &mm_txn, post_id).await?;

        dbx.commit_txn().await?;

        // -- Create and send PostDetail
        Ok(PostDetail {
            id: post.id,
            title: post.title,
            description: post.description,
            author: UserForPreview {
                id: user.id,
                username: user.username,
                avatar_object_key: user.avatar_object_key
                    .map(|key| format!("{media_endpoint}{key}")),
            },
            cover_url: format!("{media_endpoint}{}", post.cover_media_key),
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

    pub async fn list_feed_posts(
        ctx: &Ctx, 
        mm: &ModelManager, 
        filters: Option<Vec<PostFilter>>,
        options: Option<ListOptions>,
    ) -> Result<Vec<PostFeedItem>> {

        debug!("Fetching feed posts");

        let posts = PostBmc::list(ctx, mm, filters, options).await?;

        let user_ids: Vec<String> = 
            posts.iter().map(|p| p.owner_id.clone()).collect();

        let users = UserBmc::list_by_ids(ctx, mm, &user_ids).await?;

        let user_map: HashMap<String, UserForPreview> =
            users.into_iter().map(|u| (u.id.clone(), u)).collect();

        let media_endpoint = &mm.bucket().public_base;

        let feed = posts
            .into_iter()
            .map(|post| {
                let author = user_map
                    .get(&post.owner_id)
                    .cloned()
                    .unwrap_or(UserForPreview {
                        id: post.owner_id,
                        username: "".into(),
                        avatar_object_key: None,
                    });

                PostFeedItem {
                    id: post.id,
                    title: post.title,
                    author,
                    cover_url: format!("{media_endpoint}{}", post.cover_media_key),
                    like_count: post.like_count,
                }
            })
            .collect();

        Ok(feed)
    }

    pub async fn list_user_posts(
        ctx: &Ctx, 
        mm: &ModelManager, 
        user_id: String,
    ) -> Result<Vec<PostProfileItem>> {

        // -- Determine whose profile to view
        let viewer_id = ctx.user_id();
        let is_my_profile = viewer_id == user_id;

        debug!("Fetching profile posts for user: {}", user_id);

        // -- Determine status filter
        let status_filter = if is_my_profile {
            Some(vec![PostStatus::Published, PostStatus::Draft])
        } else {
            Some(vec![PostStatus::Published])
        };

        let filters = vec![PostFilter {
            owner_id: Some(OpValsString::from(user_id.clone())),
            status: status_filter,
            id: None,
            title: None,
        }];

        let list_options = ListOptions {
            limit: Some(20),
            offset: Some(0),
            order_bys: Some("!ctime".into()),
        };

        dbg!(&user_id);
        // -- Get User posts
        let posts = PostBmc::list(ctx, mm, Some(filters), Some(list_options)).await?;
        dbg!(&posts);

        // -- Get User data
        let users = UserBmc::list_by_ids(ctx, mm, &[user_id.clone()]).await?;
        let user_map: HashMap<String, UserForPreview> =
            users.into_iter().map(|u| (u.id.clone(), u)).collect();

        let media_endpoint = &mm.bucket().public_base;

        // -- Create DTO
        let feed = posts
            .into_iter()
            .map(|post| {
                let user = user_map.get(&post.owner_id);
                let author = user.map_or(
                    UserForPreview {
                        id: post.owner_id,
                        username: "".into(),
                        avatar_object_key: None,
                    },
                    |u| UserForPreview {
                        id: u.id.clone(),
                        username: u.username.clone(),
                        avatar_object_key: u.avatar_object_key.clone()
                            .map(|key| format!("{media_endpoint}{key}")),
                    },
                );

                PostProfileItem {
                    id: post.id,
                    title: post.title,
                    author,
                    cover_url: format!("{media_endpoint}{}", post.cover_media_key),
                    like_count: post.like_count,
                    status: post.status,
                }
            })
            .collect();

        Ok(feed)
    }

    pub async fn create_comment(
        ctx: &Ctx,
        mm: &ModelManager,
        payload: CreatePostCommentPayload,
    ) -> Result<Comment> {

        let CreatePostCommentPayload {
            post_id,
            text,
            parent_id,
        } = payload;

        info!("Creating comment to the post: {}", post_id);

        // -- Check if post exists
        let post = PostBmc::get(ctx, mm, &post_id).await?;

        // -- Validate the owner and the status
        let is_published = post.status == PostStatus::Published;

        if !is_published {
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
        // TODO: add medias to comment
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
        PostBmc::increment_comment_count(ctx, &mm_txn, &post_id).await?;

        let comment = CommentBmc::get(ctx, &mm_txn, &comment_id).await?;

        dbx.commit_txn().await?;

        info!("Comment created successfully: {}", comment_id);

        Ok(comment)
    }

    pub async fn update_comment(
        ctx: &Ctx, 
        mm: &ModelManager,
        comment_id: String,
        comment_u: CommentForUpdate,
    ) -> Result<Comment> {

        info!("Updating comment: {}", comment_id);

        let comment = CommentBmc::get(ctx, mm, &comment_id).await?;

        // -- Check permission
        if comment.owner_id != ctx.user_id() {
            return Err(Error::permission_denied("You are not the owner of this comment"));
        }

        let post = PostBmc::get(ctx, mm, &comment.entity_id).await?;

        // -- Validate the owner and the status
        let is_owner = post.owner_id == ctx.user_id();
        let is_published = post.status == PostStatus::Published;

        if !is_published && !is_owner {
            return Err(Error::validation_failed("Cannot edit comment on unpublished post"));
        }

        let _ = CommentBmc::update(ctx, mm, &comment_id, comment_u).await?;
        let updated_comment = CommentBmc::get(ctx, mm, &comment_id).await?;

        info!("Comment updated successfully");

        Ok(updated_comment)
    }

    pub async fn delete_comment(
        ctx: &Ctx, 
        mm: &ModelManager, 
        comment_id: String
    ) -> Result<()> {

        debug!("Deleting comment: {}", comment_id);

        let comment = CommentBmc::get(ctx, mm, &comment_id).await?;
        let post = PostBmc::get(ctx, mm, &comment.entity_id).await?;

        if comment.owner_id != ctx.user_id() && post.owner_id != ctx.user_id() {
            return Err(Error::permission_denied(
                "You don't have permission to delete this comment",
            ));
        }

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Delete comment
        CommentBmc::delete(ctx, &mm_txn, &comment_id).await?;

        // -- Decrement comment count
        PostBmc::decrement_comment_count(ctx, &mm_txn, &comment.entity_id).await?;

        dbx.commit_txn().await?;

        Ok(())
    }

    pub async fn list_comments(
        ctx: &Ctx,
        mm: &ModelManager, 
        post_id: String,
    ) -> Result<Vec<Comment>> {

        debug!("Fetching comments for post: {}", post_id);

        // -- Check if post exists
        let _post = PostBmc::exists(ctx, mm, &post_id).await?;

        let comments =
            CommentBmc::list_by_entity(ctx, mm, CommentEntityType::Post, &post_id).await?;

        Ok(comments)
    }

    pub async fn list_comment_replies(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: String,
    ) -> Result<Vec<Comment>> {

        debug!("Fetching comment replies for comment: {}", comment_id);

        let _comment = CommentBmc::get(ctx, mm, &comment_id).await?;

        // TODO: add list options
        let replies = CommentBmc::list_replies(ctx, mm, &comment_id, None).await?;

        Ok(replies)
    }

    pub async fn toggle_like(
        ctx: &Ctx, 
        mm: &ModelManager, 
        post_id: String
    ) -> Result<(bool, i64)> {

        debug!("Toggle like for post: {}", post_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check if post exist
        PostBmc::exists(ctx, &mm_txn, &post_id).await?;

        // -- Check if liked by user
        let liked = PostLikeBmc::user_liked_post(ctx, &mm_txn, &post_id).await?;

        if liked {
            // -- Decrement like
            PostLikeBmc::delete_simple(ctx, &mm_txn, &post_id).await?;
            PostBmc::decrement_like_count(ctx, &mm_txn, &post_id).await?;
        } else {
            // -- Increment like
            PostLikeBmc::create(ctx, &mm_txn, &post_id).await?;
            PostBmc::increment_like_count(ctx, &mm_txn, &post_id).await?;
        }

        // -- Get updated
        let post = PostBmc::get(ctx, &mm_txn, &post_id).await?;

        dbx.commit_txn().await?;

        Ok((!liked, post.like_count))
    }

    pub async fn get_likers(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: String,
    ) -> Result<Vec<UserForPreview>> {

        debug!("Fetching likers for post: {}", post_id);

        // -- Check post exists
        PostBmc::exists(ctx, mm, &post_id).await?;

        // TODO: add limit
        let users = PostLikeBmc::get_likers_with_user_preview(ctx, mm, &post_id, None).await?;
        Ok(users)
    }

    pub async fn get_like_count(
        ctx: &Ctx, 
        mm: &ModelManager, 
        post_id: String
    ) -> Result<i64> {

        debug!("Fetching likes for post: {}", post_id);

        let post = PostBmc::get(ctx, mm, &post_id).await?;

        Ok(post.like_count)
    }

    pub async fn save_to_collection(
        ctx: &Ctx,
        mm: &ModelManager,
        payload: SavePostToCollectionPayload,
    ) -> Result<String> {

        let SavePostToCollectionPayload {
            post_id,
            option
        } = payload;

        info!("User {} saving post to collection: {}", ctx.user_id(), post_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Get the post
        let post = PostBmc::get(ctx, &mm_txn, &post_id).await?;

        // -- Save only published posts or own posts
        let is_owner = post.owner_id == ctx.user_id();
        let is_published = post.status == PostStatus::Published;

        if !is_published && !is_owner {
            dbx.rollback_txn().await?;
            return Err(Error::permission_denied("Cannot save unpublished post"));
        }

        // -- Resolve collection
        let collection = Self::resolve_collection(ctx, mm, option).await?;

        // -- Add post to collection (ON CONFLICT DO NOTHING)
        let is_added = PostCollectionItemBmc::add_to_collection(ctx, &mm_txn, &collection.id, &post_id).await?;

        // -- Update counter only if actually added (not a duplicate)
        if is_added {
            PostBmc::increment_save_count(ctx, &mm_txn, &post_id).await?;
        }

        dbx.commit_txn().await?;

        info!("User successfully saved the post to collection: {}", post_id);

        Ok(collection.id)
    }

    pub async fn unsave_from_collection(
        ctx: &Ctx,
        mm: &ModelManager,
        payload: UnsavePostFromCollectionPayload,
    ) -> Result<()> {

        let UnsavePostFromCollectionPayload {
            post_id,
            collection_id
        } = payload;
        
        info!("User {} unsaving post: {}", ctx.user_id(), post_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check post exists
        PostBmc::exists(ctx, &mm_txn, &post_id).await?;

        PostCollectionItemBmc::remove_from_collection(ctx, &mm_txn, &collection_id, &post_id).await?;

        PostBmc::decrement_save_count(ctx, &mm_txn, &post_id).await?;

        dbx.commit_txn().await?;

        info!("User successfully unsaved the post: {}", post_id);

        Ok(())
    }

    // TODO: ADD TEST WITH CHAT
    pub async fn forward_post(
        ctx: &Ctx, 
        mm: &ModelManager,
        post_forawrd_c: PostForwardForCreate, 
    ) -> Result<()> {

        info!("Forwarding post: {}", &post_forawrd_c.post_id);

        let post_id = post_forawrd_c.post_id.clone();

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let post = PostBmc::get(ctx, &mm_txn, &post_id).await?;

        let is_published = post.status == PostStatus::Published;

        if !is_published {
            dbx.rollback_txn().await?;
            return Err(Error::permission_denied("Cannot save unpublished post"));
        }

        let is_added = PostForwardBmc::create_on_conflict(ctx, &mm_txn, post_forawrd_c).await?;

        // -- Increment forward count only if forwarded
        if is_added {
            PostBmc::increment_forward_count(ctx, &mm_txn, &post_id).await?;
        }

        dbx.commit_txn().await?;

        Ok(())
    }

    pub async fn update_post_with_media_meta(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: String,
        payload: UpdatePostPayload,
    ) -> Result<()> {

        info!("Updating post: {}", post_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Delete old medias
        if let Some(remove_ids) = &payload.remove_ids {
            PostMediaService::delete_many(ctx, &mm_txn, remove_ids).await?;
        }

        if let Some(add_medias) = &payload.add_medias {

            let mut metas = Vec::with_capacity(add_medias.len());

            // --- Validate files exist in OSS
            for media in add_medias {

                let meta = mm.bucket()
                    .head_object(&media.object_key)
                    .await?;

                match media.media_type {
                    MediaType::Image if !media.mime_type.starts_with("image/") => {
                        return Err(Error::validation_failed("Invalid mime type for image"));
                    }
                    MediaType::Video if !media.mime_type.starts_with("video/") => {
                        return Err(Error::validation_failed("Invalid mime type for video"));
                    }
                    _ => {}
                }

                metas.push(meta);
            }

            let next_sort =
                PostMediaService::next_sort(ctx, &mm_txn, &post_id).await?;

            let post_medias_c: Vec<PostMediaForCreate> =
                add_medias
                    .iter()
                    .zip(metas.iter())
                    .enumerate()
                    .map(|(idx, (media, meta))| PostMediaForCreate {
                        post_id: post_id.to_string(),
                        object_key: media.object_key.clone(),
                        media_type: media.media_type.clone(),
                        mime_type: media.mime_type.clone(),
                        etag: meta.etag.clone(),
                        file_size: meta.content_length as i64,
                        width: media.width,
                        height: media.height,
                        duration: media.duration,
                        sort_order: next_sort + idx as i32,
                    })
                    .collect();

            PostMediaBmc::create_many(ctx, &mm_txn, post_medias_c).await?;
        }

        let media_count = PostMediaBmc::count(ctx, &mm_txn, &post_id).await?;

        PostBmc::update(
            ctx,
            &mm_txn,
            &post_id,
            PostForUpdate {
                title: payload.title,
                description: payload.description,
                status: payload.status,
                cover_media_key: payload.new_cover_object_key,
                media_count: media_count as i32,
            },
        )
        .await?;

        dbx.commit_txn().await?;

        info!("Post successfully updated!");

        Ok(())
    }

    pub async fn delete_post(
        ctx: &Ctx, 
        mm: &ModelManager, 
        post_id: String
    ) -> Result<()> {

        info!("Deleting post: {}", post_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let medias = PostMediaBmc::list_by_post(ctx, &mm_txn, &post_id).await?;

        if !medias.is_empty() {
            let object_keys: Vec<&str> = medias.iter().map(|m| m.object_key.as_str()).collect();
            mm.bucket().delete_many(&object_keys).await?;
            let ids: Vec<&str> = medias.iter().map(|m| m.id.as_str()).collect();
            PostMediaBmc::delete_many(ctx, mm, ids).await?;
        }

        // -- Delete post
        PostBmc::delete(ctx, &mm_txn, &post_id).await?;

        dbx.commit_txn().await?;

        info!("Successfully deleted the post!");

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

#[derive(Debug, Deserialize)]
pub struct UpdatePostPayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: PostStatus,
    
    pub add_medias: Option<Vec<CreatePostMedia>>,
    pub remove_ids: Option<Vec<String>>,
    pub new_cover_object_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostPayload {
    pub title: String,
    pub description: String,
    pub medias: Vec<CreatePostMedia>,
    pub user_cover_key: Option<String>, // object_key to the media for cover
}

#[derive(Debug, Deserialize)]
pub struct CreatePostMedia {
    pub object_key: String,
    pub file_name: String,
    pub media_type: MediaType,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostCommentPayload {
    pub post_id: String,
    pub text: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SavePostToCollectionPayload {
    pub post_id: String,
    pub option: SaveToCollectionOption,
}

#[derive(Debug, Deserialize)]
pub struct UnsavePostFromCollectionPayload {
    pub post_id: String,
    pub collection_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ForwardPostToChatPayload {
    pub post_id: String,
    pub chat_id: String,
}
