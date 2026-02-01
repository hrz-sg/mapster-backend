use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, TimestampType};
use crate::model::{ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, OpValsBool, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- PostCollection Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct PostCollection {
    pub id: String,
    pub owner_id: String,
    pub title: String,
    pub is_default: bool,
}

#[derive(Fields, Deserialize)]
pub struct PostCollectionForCreate {
    pub title: String,
    pub is_default: bool,
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct PostCollectionFilter {
    owner_id: Option<OpValsString>,
    is_default: Option<OpValsBool>,
}
// endregion: ---- PostCollection Types

// region: ---- PostCollectionBmc
pub struct PostCollectionBmc;

impl DbBmc for PostCollectionBmc {
    const TABLE: &'static str = "post_collection";

    fn has_owner_id() -> bool {
        true
    }

    fn timestamp_fields() -> TimestampType {
        TimestampType::CtimeOnly
    }
}

impl PostCollectionBmc {
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<PostCollection> {
        base::get::<Self, PostCollection>(ctx, mm, id).await
    }

    pub async fn create(ctx: &Ctx, mm: &ModelManager, data: PostCollectionForCreate) -> Result<String> {
        base::create::<Self, _>(ctx, mm, data).await
    }

    pub async fn find_default(ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<Option<PostCollection>> {
        let filter = vec![PostCollectionFilter {
            owner_id: Some(user_id.into()),
            is_default: Some(true.into()),
        }];

        base::first::<Self, PostCollection, _>(ctx, mm, Some(filter), None).await
    }

    pub async fn get_or_create_default(ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<PostCollection> {
        if let Some(collection) = Self::find_default(ctx, mm, user_id).await? {
            return Ok(collection);
        }

        let collection_c = PostCollectionForCreate {
            title: "Posts".to_string(),
            is_default: true,
        };

        let collection_id = base::create::<Self, _>(ctx, mm, collection_c).await?;
        base::get::<Self, PostCollection>(ctx, mm, &collection_id).await
    }
}
// endregion: ---- PostCollectionBmc
