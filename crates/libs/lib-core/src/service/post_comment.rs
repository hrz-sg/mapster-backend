use modql::filter::ListOptions;
use crate::ctx::Ctx;
use crate::model::Error;
use crate::model::post::PostBmc;
use crate::model::post_comment::{CommentType, PostCommentForUpdate};
use crate::model::{
    Result, ModelManager,
    post_comment::{PostComment, PostCommentBmc, PostCommentForCreate}
};

pub struct PostCommentService;

impl PostCommentService {
    /// --- Create comment
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
        text: String,
        parent_id: Option<String>, // parent comment id
    ) -> Result<PostComment> {

        // -- Check if post exists
        PostBmc::get(ctx, mm, &post_id).await?;

        // -- Check parent comment if this is reply
        if let Some(parent_comment_id) = &parent_id {
            let parent_comment = PostCommentBmc::get(ctx, mm, parent_comment_id).await?;

            if parent_comment.entity_id != post_id {
                return Err(Error::ValidationFail("Parent comment belongs to different post".to_string()))
            }
        }

        // -- Create comment DTO
        let comment_c = PostCommentForCreate {
            user_id: ctx.user_id().to_owned(),
            entity_id: post_id.to_owned(),
            entity_type:  CommentType::Post, // TODO: improve
            parent_id,
            text,
        };

        // Create comment
        let comment_id = PostCommentBmc::create(ctx, &mm, comment_c).await?;

        // Get the comment by id
        let comment = PostCommentBmc::get(ctx, &mm, &comment_id).await?;

        Ok(comment)
    }

    /// --- Get signle comment
    pub async fn get_comment(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: &str,
    ) -> Result<PostComment> {

        // -- Get comment
        let comment = PostCommentBmc::get(ctx, &mm, comment_id).await?;

        Ok(comment)
    }

    /// --- Get post comments
    pub async fn get_post_comments(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
        list_options: Option<ListOptions>
    ) -> Result<Vec<PostComment>> {
        let comments = PostCommentBmc::list_for_post(ctx, mm, post_id, list_options).await?;

        Ok(comments)
    }

    /// --- Get comment replies
    pub async fn get_comment_replies(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: &str,
        list_options: Option<ListOptions>
    ) -> Result<Vec<PostComment>> {
        let replies = PostCommentBmc::list_replies(ctx, mm, comment_id, list_options).await?;

        Ok(replies)
    }

    /// --- Count post comments
    pub async fn get_count_post_comments(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
    ) -> Result<i64> {
        let comments_count = PostCommentBmc::count_for_post(ctx, mm, post_id).await?;

        Ok(comments_count)
    }

    /// --- Update comment
    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: &str,
        text: String,
    ) -> Result<PostComment> {
        // Check if the comment exists
        let comment = PostCommentBmc::get(ctx, mm, comment_id).await?;

        // Check permissions
        if comment.user_id != ctx.user_id() {
            return Err(Error::ValidationFail("Permission denied: you are not the owner of the comment".to_string()));
        }

        // -- Create comment DTO
        let comment_u = PostCommentForUpdate {
            text: text,
        };

        PostCommentBmc::update(ctx, mm, comment_id, comment_u).await?;

        let updated_comment = PostCommentBmc::get(ctx, mm, comment_id).await?;

        Ok(updated_comment)
    }
    
    /// --- Delete comment
    pub async fn delete(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: &str,
    ) -> Result<()> {

        // -- Get comment
        let comment = PostCommentBmc::get(ctx, &mm, comment_id).await?;

        if comment.user_id != ctx.user_id() {
            return Err(Error::NotEntityOwner( "Permission denied: your are not the owner of the comment".into()))
        }

        PostCommentBmc::delete(ctx, mm, &comment.id).await?;

        Ok(())
    }
}
