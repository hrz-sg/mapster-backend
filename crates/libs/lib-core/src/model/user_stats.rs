use crate::{ctx::Ctx, model::ModelManager};
use crate::model::base::DbBmc;
use crate::model::Result;
use modql::field::{Fields, HasSeaFields};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Iden)]
enum UserStatsIden {
    UserId,
}

pub struct UserStatsBmc;

impl DbBmc for UserStatsBmc {
    const TABLE: &'static str = "user_stats";
}

#[derive(Clone, Debug, Fields, FromRow, Serialize)]
pub struct UserProfileStats {
    pub posts_count: i64,
    pub followers_count: i64,
    pub following_count: i64,
}

impl UserStatsBmc {
    pub async fn get_by_user_id(_ctx: &Ctx, mm: &ModelManager, user_id: i64) -> Result<UserProfileStats> {
        let mut query = Query::select();
        query
            .columns(UserProfileStats::sea_column_refs())
            .from(Self::table_ref())
            .and_where(Expr::col(UserStatsIden::UserId).eq(user_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, UserProfileStats, _>(&sql, values);
        let stats = mm.dbx().fetch_one(sqlx_query).await?;
        Ok(stats)
    }
}
