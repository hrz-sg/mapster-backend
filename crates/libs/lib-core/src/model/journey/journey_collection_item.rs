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
pub struct JourneyCollectionItem {
    pub id: String,
    pub collection_id: String,
    pub journey_id: String,
}

#[derive(Fields, Deserialize)]
pub struct JourneyCollectionItemForCreate {
    pub collection_id: String,
    pub journey_id: String,
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct JourneyCollectionItemFilter {
    pub collection_id: Option<OpValsString>,
    pub journey_id: Option<OpValsString>,
}
// endregion: ---- Types

// region: ---- Iden
#[derive(Iden, Clone)]
pub enum JourneyCollectionItemIden {
    CollectionId,
    JourneyId,
}
// endregion: ---- Iden

// region: ---- JourneyCollectionItemBmc
pub struct JourneyCollectionItemBmc;

impl DbBmc for JourneyCollectionItemBmc {
    const TABLE: &'static str = "journey_collection_item";

    fn has_owner_id() -> bool {
        false // the item does not have a direct owner_id, it inherits through collection
    }

    fn timestamp_fields() -> TimestampType {
        TimestampType::CtimeOnly // saved_at
    }
}

impl JourneyCollectionItemBmc {
    pub async fn exists_in_collection(
        ctx: &Ctx,
        mm: &ModelManager,
        collection_id: &str,
        journey_id: &str,
    ) -> Result<bool> {
        let filter = vec![JourneyCollectionItemFilter {
            collection_id: Some(collection_id.into()),
            journey_id: Some(journey_id.into()),
        }];

        base::exists::<Self, _>(ctx, mm, filter).await
    }

    pub async fn add_to_collection(
        ctx: &Ctx,
        mm: &ModelManager,
        collection_id: &str,
        journey_id: &str,
    ) -> Result<bool> {
        let item_c = JourneyCollectionItemForCreate {
            collection_id: collection_id.to_string(),
            journey_id: journey_id.to_string(),
        };

        base::create_on_conflict::<Self, _, _>(
            ctx,
            mm,
            item_c,
            &[JourneyCollectionItemIden::CollectionId, JourneyCollectionItemIden::JourneyId],
        )
        .await
    }

    pub async fn remove_from_collection(
        ctx: &Ctx,
        mm: &ModelManager,
        collection_id: &str,
        journey_id: &str,
    ) -> Result<()> {
        let filter = vec![JourneyCollectionItemFilter {
            collection_id: Some(collection_id.into()),
            journey_id: Some(journey_id.into()),
        }];

        if let Some(item) = base::first::<Self, JourneyCollectionItem, _>(ctx, mm, Some(filter), None).await? {
            base::delete::<Self>(ctx, mm, &item.id).await
        } else {
            Err(Error::EntityNotFound {
                entity: Self::TABLE,
                id: format!("{collection_id}:{journey_id}"),
            })
        }
    }

    pub async fn count(ctx: &Ctx, mm: &ModelManager, filter: Option<Vec<JourneyCollectionItemFilter>>) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filter).await
    }

    // TODO: need to join with user preview and journey
    pub async fn list_in_collection(
        ctx: &Ctx,
        mm: &ModelManager,
        collection_id: &str,
    ) -> Result<Vec<JourneyCollectionItem>> {
        let filter = vec![JourneyCollectionItemFilter {
            collection_id: Some(collection_id.into()),
            ..Default::default()
        }];

        base::list::<Self, JourneyCollectionItem, _>(ctx, mm, Some(filter), None).await
    }
}
// endregion: ---- JourneyCollectionItemBmc
