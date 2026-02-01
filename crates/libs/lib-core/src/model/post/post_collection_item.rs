use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, TimestampType};
use crate::model::{Error, ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, OpValsString};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct PostCollectionItem {
    pub id: String,
    pub collection_id: String,
    pub post_id: String,
    pub sort_order: i32,
}

#[derive(Fields, Deserialize)]
pub struct PostCollectionItemForCreate {
    pub collection_id: String,
    pub post_id: String,
    pub sort_order: Option<i32>,
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct PostCollectionItemFilter {
    pub collection_id: Option<OpValsString>,
    pub post_id: Option<OpValsString>,
}
// endregion: ---- Types

// region: ---- Iden
#[derive(Iden, Clone)]
pub enum PostCollectionItemIden {
    CollectionId,
    PostId,
}
// endregion: ---- Iden

// region: ---- PostCollectionItemBmc
pub struct PostCollectionItemBmc;

impl DbBmc for PostCollectionItemBmc {
    const TABLE: &'static str = "post_collection_item";

    fn has_owner_id() -> bool {
        false // the item does not have a direct owner_id, it inherits through collection
    }

    fn timestamp_fields() -> TimestampType {
        TimestampType::CtimeMtime
    }
}

impl PostCollectionItemBmc {
    pub async fn add_to_collection(ctx: &Ctx, mm: &ModelManager, collection_id: &str, post_id: &str) -> Result<bool> {
        let sort_order = Self::get_next_sort_order(ctx, mm, collection_id).await?;

        let item_c = PostCollectionItemForCreate {
            collection_id: collection_id.to_string(),
            post_id: post_id.to_string(),
            sort_order: Some(sort_order),
        };

        base::create_on_conflict::<Self, _, _>(
            ctx,
            mm,
            item_c,
            &[PostCollectionItemIden::CollectionId, PostCollectionItemIden::PostId],
        )
        .await
    }

    pub async fn remove_from_collection(
        ctx: &Ctx,
        mm: &ModelManager,
        collection_id: &str,
        post_id: &str,
    ) -> Result<()> {
        let filter = vec![PostCollectionItemFilter {
            collection_id: Some(collection_id.into()),
            post_id: Some(post_id.into()),
        }];

        if let Some(item) = base::first::<Self, PostCollectionItem, _>(ctx, mm, Some(filter), None).await? {
            base::delete::<Self>(ctx, mm, &item.id).await
        } else {
            Err(Error::EntityNotFound {
                entity: Self::TABLE,
                id: format!("{collection_id}:{post_id}"),
            })
        }
    }

    pub async fn count(ctx: &Ctx, mm: &ModelManager, filter: Option<Vec<PostCollectionItemFilter>>) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filter).await
    }

    // TODO: need to join with user preview and journey
    pub async fn list_in_collection(
        ctx: &Ctx,
        mm: &ModelManager,
        collection_id: &str,
    ) -> Result<Vec<PostCollectionItem>> {
        let filter = vec![PostCollectionItemFilter {
            collection_id: Some(collection_id.into()),
            ..Default::default()
        }];

        base::list::<Self, PostCollectionItem, _>(ctx, mm, Some(filter), None).await
    }

    pub async fn get_next_sort_order(ctx: &Ctx, mm: &ModelManager, collection_id: &str) -> Result<i32> {
        let items = Self::list_in_collection(ctx, mm, collection_id).await?;

        if items.is_empty() {
            Ok(0)
        } else {
            let max_sort = items.iter().map(|item| item.sort_order).max().unwrap_or(0);
            Ok(max_sort + 1)
        }
    }
}
// endregion: ---- PostCollectionItemBmc
