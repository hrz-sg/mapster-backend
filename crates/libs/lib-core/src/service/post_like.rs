use crate::ctx::Ctx;
use crate::model::post_like::PostLikeBmc;
use crate::model::user::UserForPreview;
use crate::model::{
    Result, ModelManager,
    post::PostBmc,
};

pub struct PostLikeService;

impl PostLikeService {
    /// --- Create post with media files
    pub async fn toggle_like(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
    ) -> Result<(bool, i64)> {
        let mm = mm.new_with_txn()?;
        let dbx = mm.dbx();
        dbx.begin_txn().await?;

        // Check if post exists
        let _post = PostBmc::get(ctx, &mm, post_id).await?;

        // Check if user already liked the post
        let liked = PostLikeBmc::user_liked_post(ctx, &mm, post_id).await?;

        if liked {
            // Try to delete if like exists
            PostLikeBmc::delete(ctx, &mm, post_id).await?;
        } else {
            // If like doesn't exist, then like it
            PostLikeBmc::create(ctx, &mm, post_id).await?;
        }

        // Get updated like count
        let like_count = PostLikeBmc::count_likes_by_post_id(ctx, &mm, post_id).await?;

        dbx.commit_txn().await?;

        Ok((!liked, like_count))
    }

    /// --- Get users liked the post
    pub async fn get_post_likers(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
        limit: Option<u32>
    ) -> Result<Vec<UserForPreview>> {

        let mm = mm.new_with_txn()?;
        let dbx = mm.dbx();
        dbx.begin_txn().await?;

        // Check if post exists
        let _post = PostBmc::get(ctx, &mm, post_id).await?;

        let users = PostLikeBmc::get_likers_with_user_preview(ctx, &mm, post_id, limit).await?;

        Ok(users)
    }

    /// --- Get post likes by post_id
    pub async fn get_post_likes_count(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
    ) -> Result<i64> {
        let mm = mm.new_with_txn()?;
        let dbx = mm.dbx();
        dbx.begin_txn().await?;

        // Check if post exists
        let _post = PostBmc::get(ctx, &mm, post_id).await?;

        let likes_count = PostLikeBmc::count_likes_by_post_id(ctx, &mm, post_id).await?;

        Ok(likes_count)
    }
}
