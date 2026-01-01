use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::{ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValString, OpValsString};
use sea_query::Func;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

// region: ---- PostComment Types
#[derive(Clone, Debug, Copy, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "comment_entity_typ")]
pub enum CommentType {
    #[sqlx(rename = "Post")]
    Post,
}

// Covert custom CommentType into sea_query::Value
impl From<CommentType> for sea_query::SimpleExpr {
    fn from(val: CommentType) -> Self {
        sea_query::SimpleExpr::FunctionCall(Func::cast_as(val.to_string(), "comment_entity_typ"))
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct PostComment {
    pub id: String,
    pub user_id: String,
    pub entity_type: CommentType,
    pub entity_id: String, // post_id
    pub parent_id: Option<String>,
    pub text: String,
}

#[derive(Fields, Deserialize)]
pub struct PostCommentForCreate {
    pub user_id: String,
    pub entity_id: String, // post_id
    pub entity_type: CommentType,
    pub parent_id: Option<String>, // for comment replies
    pub text: String,
}

#[derive(Fields, Deserialize)]
pub struct PostCommentForUpdate {
    pub text: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct PostCommentFilter {
    pub entity_id: Option<OpValsString>,
    pub parent_id: Option<OpValsString>,
    pub user_id: Option<OpValsString>,
}

// endregion: ---- PostComment Types

// region: ---- PostCommentBmc
pub struct PostCommentBmc;

impl DbBmc for PostCommentBmc {
    const TABLE: &'static str = "comment";
}

impl PostCommentBmc {
    /// Create comment
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_c: PostCommentForCreate,
    ) -> Result<String> {
        base::create::<Self,_>(ctx, mm, comment_c).await
    }

    /// Update comment
    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: &str,
        comment_u: PostCommentForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, comment_id, comment_u).await
    }

    /// Get signle comment by id
    pub async fn get(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: &str,
    ) -> Result<PostComment> {
        base::get::<Self, _>(ctx, mm, comment_id).await
    }
    
    /// Delete comment by id
    pub async fn delete(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: &str,
    ) -> Result<()> {
        base::delete::<Self>(ctx, mm, comment_id).await
    }
   
    /// List of likes by post
    pub async fn list_for_post(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
        list_options: Option<ListOptions>
    ) -> Result<Vec<PostComment>> {

        let filter = PostCommentFilter {
            entity_id: Some(post_id.into()),
            parent_id: Some(OpValsString(vec![OpValString::Null(true)])), // only root comment
            ..Default::default()
        };
        
        base::list::<Self, _, _>(ctx, mm, Some(filter), list_options).await
    }

    /// List replies for comment
    pub async fn list_replies(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: &str,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<PostComment>> {
        let filter = PostCommentFilter {
            parent_id: Some(comment_id.into()),
            ..Default::default()
        };

        base::list::<Self, _, _>(ctx, mm, Some(filter), list_options).await
    }

    /// Count comments for post
    pub async fn count_for_post(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
    ) -> Result<i64> {
        let filter = PostCommentFilter {
            entity_id: Some(post_id.into()),
            ..Default::default()
        };

        base::count::<Self, _>(ctx, mm, Some(filter)).await
    }
}

// endregion: ---- PostLikeBmc