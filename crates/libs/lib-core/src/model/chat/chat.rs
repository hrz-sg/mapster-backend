use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, TimestampType};
use crate::model::{ModelManager, Result};
use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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
    pub async fn create(ctx: &Ctx, mm: &ModelManager, comment_c: ChatForCreate) -> Result<String> {
        base::create::<Self, _>(ctx, mm, comment_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, comment_id: &str) -> Result<Chat> {
        base::get::<Self, _>(ctx, mm, comment_id).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, comment_id: &str) -> Result<()> {
        base::delete::<Self>(ctx, mm, comment_id).await
    }
}
// endregion: ---- ChatBmc
