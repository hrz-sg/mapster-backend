use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, TimestampType};
use crate::model::{ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, OpValsBool, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- JourneyCollection Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct JourneyCollection {
    pub id: String,
    pub owner_id: String,
    pub title: String,
    pub is_default: bool,
}

#[derive(Fields, Deserialize)]
pub struct JourneyCollectionForCreate {
    pub title: String,
    pub is_default: bool,
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct JourneyCollectionFilter {
    owner_id: Option<OpValsString>,
    is_default: Option<OpValsBool>,
}

// endregion: ---- JourneyCollection Types

// region: ---- JourneyCollectionBmc
pub struct JourneyCollectionBmc;

impl DbBmc for JourneyCollectionBmc {
    const TABLE: &'static str = "journey_collection";

    fn has_owner_id() -> bool {
        true
    }

    fn timestamp_fields() -> TimestampType {
        TimestampType::CtimeOnly
    }
}

impl JourneyCollectionBmc {
    pub async fn find_default(ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<Option<JourneyCollection>> {
        let filter = vec![JourneyCollectionFilter {
            owner_id: Some(user_id.into()),
            is_default: Some(true.into()),
        }];

        base::first::<Self, JourneyCollection, _>(ctx, mm, Some(filter), None).await
    }

    pub async fn get_or_create_default(ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<JourneyCollection> {
        if let Some(collection) = Self::find_default(ctx, mm, user_id).await? {
            return Ok(collection);
        }

        let collection_c = JourneyCollectionForCreate {
            title: "Journeys".to_string(),
            is_default: true,
        };

        let collection_id = base::create::<Self, _>(ctx, mm, collection_c).await?;
        base::get::<Self, JourneyCollection>(ctx, mm, &collection_id).await
    }

    //// NO DELETE FUNCTION
    //// AS ONLY DEFAULT COLLECTION EXISTS
}
// endregion: ---- JourneyCollectionBmc
