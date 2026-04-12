// region: --- Imports

use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, TimestampType};
use crate::model::{ModelManager, Result};
use crate::model::modql_utils::time_to_sea_value;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// endregion: --- Imports

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
    #[field(cast_as = "message_type")]
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
    #[field(cast_as = "message_type")]
    pub message_type: MessageType,
    pub text: Option<String>,
    pub post_id: Option<String>,
    pub journey_id: Option<String>,
    pub reply_to_id: Option<String>,
}

#[derive(Fields, Deserialize, Default)]
pub struct ChatMessageForUpdate {
    pub text: Option<String>,
    pub mtime: Option<chrono::DateTime<chrono::Utc>>,
    pub dtime: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ChatMessageFilter {
    pub chat_id: Option<OpValsString>,
    pub user_id: Option<OpValsString>,
    pub seq: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub dtime: Option<OpValsValue>
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

    pub async fn update(ctx: &Ctx, mm: &ModelManager, comment_id: &str, message_u: ChatMessageForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, comment_id, message_u).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filter: Option<Vec<ChatMessageFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ChatMessage>> {
        base::list::<Self, _, _>(ctx, mm, filter, list_options).await
    }
}
// endregion: ---- ChatMessageBmc
