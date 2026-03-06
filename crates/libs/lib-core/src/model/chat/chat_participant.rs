use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::{ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- ChatParticipant Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct ChatParticipant {
    pub chat_id: String,
    pub user_id: String,
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub left_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Fields, Deserialize)]
pub struct ChatParticipantForCreate {
    pub chat_id: String,
    pub user_id: String,
}

#[derive(Fields, Deserialize)]
pub struct ChatParticipantForUpdate {
    pub left_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ChatParticipantFilter {
    pub chat_id: Option<String>,
    pub user_id: Option<String>,
}
// endregion: ---- ChatParticipant Types

// region: ---- ChatParticipantBmc
pub struct ChatParticipantBmc;

impl DbBmc for ChatParticipantBmc {
    const TABLE: &'static str = "chat_participant";
}

impl ChatParticipantBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, chat_participant_c: ChatParticipantForCreate) -> Result<String> {
        base::create::<Self, _>(ctx, mm, chat_participant_c).await
    }

    pub async fn update_by_filter(
        ctx: &Ctx,
        mm: &ModelManager,
        filter: ChatParticipantFilter,
        data: ChatParticipantForUpdate,
    ) -> Result<u64> {
        base::update_by_filter::<Self, _, _>(ctx, mm, filter, data).await
    }

    pub async fn exists(ctx: &Ctx, mm: &ModelManager, filter: ChatParticipantFilter) -> Result<bool> {
        base::exists::<Self, _>(ctx, mm, filter).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filter: Option<Vec<ChatParticipantFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ChatParticipant>> {
        base::list::<Self, _, _>(ctx, mm, filter, list_options).await
    }
}
// endregion: ---- ChatParticipantBmc
