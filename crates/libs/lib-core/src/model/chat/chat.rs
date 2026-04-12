// region: --- Imports

use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, TimestampType};
use crate::model::{Error, ModelManager, Result};
use modql::field::Fields;
use modql::filter::FilterNodes;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// endregion: --- Imports

// region: ---- Chat Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "chat_type")]
pub enum ChatType {
    Direct,
    Group,
}

// Covert custom ChatType into sea_query::Value
impl From<ChatType> for sea_query::Value {
    fn from(val: ChatType) -> Self {
        val.to_string().into()
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Chat {
    pub id: String,
    #[field(cast_as = "chat_type")]
    pub chat_type: ChatType,
    pub title: Option<String>,
    pub owner_id: Option<String>,
    pub event_id: Option<String>,
}

#[derive(Fields, Deserialize)]
pub struct ChatForCreate {
    #[field(cast_as = "chat_type")]
    pub chat_type: ChatType,
    pub title: Option<String>,
    pub direct_key: Option<String>
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ChatFilter {
    pub id: Option<String>,
    pub chat_type: Option<ChatType>,
    pub title: Option<String>,
    pub owner_id: Option<String>,
    pub event_id: Option<String>,
    pub direct_key: Option<String>,
}

#[derive(Iden, Clone)]
pub enum ChatIden {
    #[iden = "direct_key"]
    DirectKey,
}

// endregion: ---- Chat Types

// region: ---- ChatBmc
pub struct ChatBmc;

impl DbBmc for ChatBmc {
    const TABLE: &'static str = "chat";

    fn has_owner_id() -> bool {
        true
    }

    fn timestamp_fields() -> TimestampType {
        TimestampType::CtimeMtime
    }
}

impl ChatBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, chat_c: ChatForCreate) -> Result<String> {
        base::create::<Self, _>(ctx, mm, chat_c).await
    }

    pub async fn create_on_conflict(ctx: &Ctx, mm: &ModelManager, chat_c: ChatForCreate) -> Result<bool> {
        let conflict_columns = &[ChatIden::DirectKey];
        
        base::create_on_conflict::<Self, _, _>(
            ctx,
            mm,
            chat_c,
            conflict_columns,
        ).await
    }
    
    pub async fn get(ctx: &Ctx, mm: &ModelManager, chat_id: &str) -> Result<Chat> {
        base::get::<Self, _>(ctx, mm, chat_id).await
    }

    pub async fn find_by_direct_key(ctx: &Ctx, mm: &ModelManager, key: &str) -> Result<Chat> {
        let filter = vec![ChatFilter {
            direct_key: Some(key.to_string()),
            ..Default::default()
        }];

        base::first_by_composite_key::<Self, Chat, _>(ctx, mm, Some(filter))
            .await?
            .ok_or_else(|| Error::EntityNotFound {
                    entity: Self::TABLE,
                    id: format!("{key}"),
                })
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, chat_id: &str) -> Result<()> {
        base::delete::<Self>(ctx, mm, chat_id).await
    }
}
// endregion: ---- ChatBmc
