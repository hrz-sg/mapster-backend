// region: --- Imports

use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::{ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use sea_query::{Expr, Func, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// endregion: --- Imports

// region: ---- ChatMember Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct ChatMember {
    pub chat_id: String,
    pub user_id: String,
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub left_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Fields, Deserialize)]
pub struct ChatMemberForCreate {
    pub chat_id: String,
    pub user_id: String,
}

#[derive(Fields, Deserialize, Default)]
pub struct ChatMemberForUpdate {
    pub left_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_read_seq: Option<i64>
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ChatMemberFilter {
    pub chat_id: Option<OpValsString>,
    pub user_id: Option<OpValsString>,
    pub last_read_seq: Option<OpValsInt64>,
}

#[derive(Iden, Clone)]
pub enum ChatMemberIden {
    #[iden = "chat_id"]
    ChatId,
    #[iden = "user_id"]
    UserId,
    #[iden = "joined_at"]
    JoinedAt,
    #[iden = "left_at"]
    LeftAt,
    #[iden = "last_read_seq"]
    LastReadSeq
}
// endregion: ---- ChatMember Types

// region: ---- ChatMemberBmc
pub struct ChatMemberBmc;

impl DbBmc for ChatMemberBmc {
    const TABLE: &'static str = "chat_member";

    fn has_id() -> bool {
        false
    }

    fn timestamp_fields() -> base::TimestampType {
        base::TimestampType::None
    }
}

impl ChatMemberBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, chat_member_c: ChatMemberForCreate) -> Result<bool> {
         let conflict_columns = &[ChatMemberIden::ChatId, ChatMemberIden::UserId];
        
        base::create_on_conflict::<Self, _, _>(
            ctx,
            mm,
            chat_member_c,
            conflict_columns,
        ).await
    }

    pub async fn update_by_filter(
        ctx: &Ctx,
        mm: &ModelManager,
        filter: ChatMemberFilter,
        data: ChatMemberForUpdate,
    ) -> Result<u64> {
        base::update_by_filter::<Self, _, _>(ctx, mm, filter, data).await
    }

    pub async fn update_seq(
        ctx: &Ctx,
        mm: &ModelManager,
        chat_id: &str,
        seq: i64,
    ) -> Result<()> {
        let mut query = Query::update();

        query
            .table(Self::table_ref())
            .value(
                ChatMemberIden::LastReadSeq,
                Func::cust("GREATEST")
                    .arg(Expr::col(ChatMemberIden::LastReadSeq))
                    .arg(Expr::val(seq)),
            )
            .and_where(Expr::col(ChatMemberIden::ChatId).eq(chat_id))
            .and_where(Expr::col(ChatMemberIden::UserId).eq(ctx.user_id()));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_with(&sql, values);
        mm.dbx().execute(sqlx_query).await?;

        Ok(())
    }

    pub async fn exists(ctx: &Ctx, mm: &ModelManager, filter: ChatMemberFilter) -> Result<bool> {
        base::exists::<Self, _>(ctx, mm, filter).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filter: Option<Vec<ChatMemberFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ChatMember>> {
        base::list::<Self, _, _>(ctx, mm, filter, list_options).await
    }
}
// endregion: ---- ChatMemberBmc
