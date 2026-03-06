use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, TimestampType};
use crate::model::{Error, ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsString};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- PostForward Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct PostForward {
    pub post_id: String,
    pub user_id: String,
    pub chat_id: String,
}

#[derive(Fields, Deserialize)]
pub struct PostForwardForCreate {
    pub post_id: String,
    pub chat_id: String,
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct PostForwardFilter {
    pub post_id: Option<OpValsString>,
    pub user_id: Option<OpValsString>,
}
// endregion: ---- PostForward Types

// region: ---- PostForwardIden
#[derive(Iden, Clone)]
pub enum PostForwardIden {
    Table,
    PostId,
    UserId,
    ChatId,
    Ctime,
}
// endregion: ---- PostForwardIden

// region: ---- PostForwardBmc
pub struct PostForwardBmc;

impl DbBmc for PostForwardBmc {
    const TABLE: &'static str = "post_forward";

    fn has_user_id() -> bool {
        true
    }

    fn timestamp_fields() -> TimestampType {
        TimestampType::CtimeOnly
    }
}

impl PostForwardBmc {
    pub async fn exists(ctx: &Ctx, mm: &ModelManager, post_id: &str, user_id: &str) -> Result<bool> {
        let filter = vec![PostForwardFilter {
            post_id: Some(post_id.into()),
            user_id: Some(user_id.into()),
        }];
        base::exists::<Self, _>(ctx, mm, filter).await
    }

    pub async fn create_on_conflict(
        ctx: &Ctx,
        mm: &ModelManager,
        post_forawrd_c: PostForwardForCreate,
    ) -> Result<bool> {
        base::create_on_conflict::<Self, _, _>(
            ctx,
            mm,
            post_forawrd_c,
            &[PostForwardIden::PostId, PostForwardIden::UserId, PostForwardIden::ChatId],
        )
        .await
    }

    // --- Delete for current user (ctx.user_id())
    pub async fn delete(ctx: &Ctx, mm: &ModelManager, journey_id: &str) -> Result<()> {
        let mut query = Query::delete();
        query
            .from_table(Self::table_ref())
            .and_where(Expr::col(PostForwardIden::PostId).eq(journey_id))
            .and_where(Expr::col(PostForwardIden::UserId).eq(ctx.user_id()));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_with(&sql, values);
        let count = mm.dbx().execute(sqlx_query).await?;

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: Self::TABLE,
                id: format!("{journey_id}:{}", ctx.user_id()),
            });
        }

        Ok(())
    }

    pub async fn count(ctx: &Ctx, mm: &ModelManager, filter: Option<Vec<PostForwardFilter>>) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filter).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filter: Option<Vec<PostForwardFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<PostForward>> {
        base::list::<Self, PostForward, _>(ctx, mm, filter, list_options).await
    }
}
// endregion: ---- PostForwardBmc
