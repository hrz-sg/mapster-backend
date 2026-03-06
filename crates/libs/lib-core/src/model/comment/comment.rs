use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::{ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValString, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- Comment Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "comment_entity_type")]
pub enum CommentEntityType {
    Post,
}

// Covert custom CommentEntityType into sea_query::Value
impl From<CommentEntityType> for sea_query::Value {
    fn from(val: CommentEntityType) -> Self {
        val.to_string().into()
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Comment {
    pub id: String,
    pub owner_id: String,
    pub entity_type: CommentEntityType,
    pub entity_id: String, // post_id
    pub parent_id: Option<String>,
    pub text: String,
}

#[derive(Fields, Deserialize, Debug)]
pub struct CommentForCreate {
    #[field(cast_as = "comment_entity_type")]
    pub entity_type: CommentEntityType,
    pub entity_id: String,
    pub parent_id: Option<String>,
    pub text: String,
}

#[derive(Fields, Deserialize)]
pub struct CommentForUpdate {
    pub text: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct CommentFilter {
    pub entity_type: Option<Vec<CommentEntityType>>,
    pub entity_id: Option<OpValsString>,
    pub parent_id: Option<OpValsString>,
    pub owner_id: Option<OpValsString>,
}

// endregion: ---- Comment Types

// region: ---- CommentBmc
pub struct CommentBmc;

impl DbBmc for CommentBmc {
    const TABLE: &'static str = "comment";

    fn has_owner_id() -> bool {
        true
    }
}

impl CommentBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, comment_c: CommentForCreate) -> Result<String> {
        base::create::<Self, _>(ctx, mm, comment_c).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, comment_id: &str, comment_u: CommentForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, comment_id, comment_u).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, comment_id: &str) -> Result<Comment> {
        base::get::<Self, _>(ctx, mm, comment_id).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, comment_id: &str) -> Result<()> {
        base::delete::<Self>(ctx, mm, comment_id).await
    }

    pub async fn list_by_entity(
        ctx: &Ctx,
        mm: &ModelManager,
        entity_type: CommentEntityType,
        entity_id: &str,
        // list_options: Option<ListOptions>,
    ) -> Result<Vec<Comment>> {
        let filter = CommentFilter {
            entity_type: Some(vec![entity_type]),
            entity_id: Some(entity_id.into()),
            parent_id: Some(OpValsString(vec![OpValString::Null(true)])),
            ..Default::default()
        };

        base::list::<Self, _, _>(ctx, mm, Some(filter), None).await
    }

    pub async fn list_replies(
        ctx: &Ctx,
        mm: &ModelManager,
        comment_id: &str,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Comment>> {
        let filter = CommentFilter {
            parent_id: Some(comment_id.into()),
            ..Default::default()
        };

        base::list::<Self, _, _>(ctx, mm, Some(filter), list_options).await
    }

    pub async fn count_by_entity(
        ctx: &Ctx,
        mm: &ModelManager,
        entity_type: CommentEntityType,
        entity_id: &str,
    ) -> Result<i64> {
        let filter = CommentFilter {
            entity_type: Some(vec![entity_type]),
            entity_id: Some(entity_id.into()),
            ..Default::default()
        };

        base::count::<Self, _>(ctx, mm, Some(filter)).await
    }
}
// endregion: ---- CommentBmc
