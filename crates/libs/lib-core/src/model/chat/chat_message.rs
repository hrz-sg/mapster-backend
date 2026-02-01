use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, TimestampType};
use crate::model::{ModelManager, Result};
use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- ChatMessage Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "message_type")]
pub enum MessageType {
    Text,
    Post,
    Journey,
}

// Covert custom MessageType into sea_query::Value
impl From<MessageType> for sea_query::Value {
    fn from(val: MessageType) -> Self {
        val.to_string().into()
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct ChatMessage {
    pub id: String,
    pub chat_id: String,
    pub user_id: String,

    pub message_type: MessageType,

    pub text: Option<String>,
    pub post_id: Option<String>,
    pub journey_id: Option<String>,

    pub reply_to_id: Option<String>,

    pub ctime: chrono::DateTime<chrono::Utc>,
    pub mtime: Option<chrono::DateTime<chrono::Utc>>,
    pub dtime: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Fields, Deserialize)]
pub struct ChatMessageForCreate {
    pub chat_id: String,
    pub message_type: MessageType,
    pub text: Option<String>,
    pub post_id: Option<String>,
    pub journey_id: Option<String>,
    pub reply_to_id: Option<String>,
}

// endregion: ---- ChatMessage Types

// region: ---- ChatMessageBmc
pub struct ChatMessageBmc;

impl DbBmc for ChatMessageBmc {
    const TABLE: &'static str = "chat_message";

    fn has_owner_id() -> bool {
        true // owner = user_id (sender)
    }

    fn timestamp_fields() -> TimestampType {
        TimestampType::CtimeMtime
    }
}

impl ChatMessageBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, chat_message_c: ChatMessageForCreate) -> Result<String> {
        base::create::<Self, _>(ctx, mm, chat_message_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<ChatMessage> {
        base::get::<Self, _>(ctx, mm, id).await
    }
}
// endregion: ---- ChatMessageBmc
