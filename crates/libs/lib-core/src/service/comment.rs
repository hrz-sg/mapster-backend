// region: ---- Imports

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::comment::{Comment, CommentBmc, CommentEntityType, CommentForCreate, CommentForUpdate};
use crate::service::error::{Error, Result};
use tracing::{info, warn};

// endregion: ---- Imports

pub struct CommentService;

impl CommentService {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_c: CommentForCreate
    ) -> Result<Comment> {

        let CommentForCreate {
            entity_type,
            entity_id,
            parent_id,
            text,
        } = comment_c;

        // -- Check if parent, then check if parent exists
        if let Some(ref parent_id) = parent_id {
            CommentBmc::get(ctx, mm, parent_id).await?;
        }

        // -- Create comment
        let comment_id = CommentBmc::create(
            ctx,
            mm,
            CommentForCreate {
                entity_type,
                entity_id,
                parent_id,
                text,
            },
        ).await?;

        let comment = CommentBmc::get(ctx, mm, &comment_id).await?;
        
        Ok(comment)
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: String,
        comment_u: CommentForUpdate
    ) -> Result<Comment> {

        let CommentForUpdate {
            text
        } = comment_u;

        // -- Check if exists
        let comment = CommentBmc::get(ctx, mm, &comment_id).await?;

        // -- Check owner
        if comment.owner_id != ctx.user_id() {
            return Err(Error::PermissionDenied("Cannot edit other user comment".to_string()));
        }

        // -- Update comment
        CommentBmc::update(
            ctx, 
            mm, 
            &comment_id, 
            CommentForUpdate { text }
        ).await?;

        // -- Return updated comment
        let comment = CommentBmc::get(ctx, mm, &comment_id).await?;

        Ok(comment)
    }

    pub async fn delete(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: String,
    ) -> Result<()> {
        
        let result = CommentBmc::delete(ctx, mm, &comment_id).await?;
        
        Ok(result)
    }

    pub async fn list_comments(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: String,
    ) -> Result<Vec<Comment>> {

        let comments = CommentBmc::list_by_entity(ctx, mm, CommentEntityType::Post, &comment_id).await?;

        Ok(comments)
    }

    pub async fn list_replies(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: String,
    ) -> Result<Vec<Comment>> {

        let replies = CommentBmc::list_replies(ctx, mm, &comment_id, None).await?;

        Ok(replies)
    }
}