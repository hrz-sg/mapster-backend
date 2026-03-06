use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, TimestampType};
use crate::model::{Error, ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- Journey Types

#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize, PartialEq)]
#[sqlx(type_name = "journey_status")]
pub enum JourneyStatus {
    Draft,
    Published,
}

// Covert custom JourneyStatus into sea_query::Value
impl From<JourneyStatus> for sea_query::Value {
    fn from(val: JourneyStatus) -> Self {
        val.to_string().into()
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Journey {
    pub id: String,
    pub owner_id: String,
    pub title: String,
    pub description: String,
    pub cover_object_key: Option<String>,
    #[field(cast_as = "journey_status")]
    pub status: JourneyStatus,
    pub total_likes: i64,
    pub save_count: i64,
    pub forward_count: i64,
}

#[derive(Debug, Serialize)]
pub struct JourneyWithStats {
    pub journey: Journey,
    pub post_like_sum: i64, // likes sum of posts insde this current journey
    pub save_count: i64,
    pub forward_count: i64,
    pub current_user_saved: bool,
    pub current_user_forwarded: bool,
}

#[derive(Fields, Deserialize)]
pub struct JourneyForCreate {
    pub title: String,
    pub description: String,
    pub cover_object_key: Option<String>,
    #[field(cast_as = "journey_status")]
    pub status: JourneyStatus,
}

#[derive(Fields, Deserialize)]
pub struct JourneyForUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    #[field(cast_as = "journey_status")]
    pub status: JourneyStatus,
    pub cover_object_key: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct JourneyFilter {
    pub id: Option<OpValsString>,
    pub owner_id: Option<OpValsString>,
    pub title: Option<OpValsString>,
    pub description: Option<OpValsString>,
    #[modql(cast_as = "journey_status")]
    pub status: Option<JourneyStatus>,
    pub total_likes: Option<OpValsInt64>,
    pub save_count: Option<OpValsInt64>,
}

// endregion: ---- Journey Types

// region: ---- JourneyIden
#[derive(Iden, Clone, Copy)]
pub enum JourneyIden {
    #[iden = "id"]
    Id,
    #[iden = "like_count"]
    LikeCount,
    #[iden = "save_count"]
    SaveCount,
    #[iden = "forward_count"]
    ForwardCount,
    #[iden = "total_likes"]
    TotalLikes,
}
// endregion: ---- JourneyIden

// region: ---- CounterType enum
pub enum CounterType {
    SaveCount,
    ForwardCount,
    TotalLikes,
}
// endregion: ---- CounterType enum

// region: ---- JourneyBmc
pub struct JourneyBmc;

impl DbBmc for JourneyBmc {
    const TABLE: &'static str = "journey";

    fn has_owner_id() -> bool {
        true
    }

    fn timestamp_fields() -> TimestampType {
        TimestampType::Full
    }
}

impl JourneyBmc {
    // --- Check if journey exists by journey_id
    pub async fn exists(ctx: &Ctx, mm: &ModelManager, journey_id: &str) -> Result<bool> {
        let filter = vec![JourneyFilter {
            id: Some(journey_id.into()),
            ..Default::default()
        }];

        base::exists::<Self, _>(ctx, mm, filter).await
    }

    // --- Create journey with no posts inside
    pub async fn create(ctx: &Ctx, mm: &ModelManager, journey_c: JourneyForCreate) -> Result<String> {
        base::create::<Self, _>(ctx, mm, journey_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, journey_id: &str) -> Result<Journey> {
        base::get::<Self, _>(ctx, mm, journey_id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<JourneyFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Journey>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: &str, journey_u: JourneyForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, journey_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }

    pub async fn count(ctx: &Ctx, mm: &ModelManager, filter: Option<Vec<JourneyFilter>>) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filter).await
    }

    async fn update_counter(
        _ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
        counter_type: CounterType,
        delta: i32, // +1 or -1
    ) -> Result<()> {
        let mut query = Query::update();
        query.table(Self::table_ref());

        match counter_type {
            CounterType::SaveCount => {
                query.value(JourneyIden::SaveCount, Expr::col(JourneyIden::SaveCount).add(delta));
                // protect column from negative values
                if delta < 0 {
                    query.and_where(Expr::col(JourneyIden::SaveCount).gt(0));
                }
            }
            CounterType::ForwardCount => {
                query.value(
                    JourneyIden::ForwardCount,
                    Expr::col(JourneyIden::ForwardCount).add(delta),
                );
                if delta < 0 {
                    query.and_where(Expr::col(JourneyIden::ForwardCount).gt(0));
                }
            }
            CounterType::TotalLikes => {
                query.value(JourneyIden::TotalLikes, Expr::col(JourneyIden::TotalLikes).add(delta));
                if delta < 0 {
                    query.and_where(Expr::col(JourneyIden::TotalLikes).gt(0));
                }
            }
        }

        query.and_where(Expr::col(JourneyIden::Id).eq(journey_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_with(&sql, values);
        let count = mm.dbx().execute(sqlx_query).await?;

        // -- Check result
        if count == 0 {
            Err(Error::EntityNotFound {
                entity: Self::TABLE,
                id: journey_id.to_string(),
            })
        } else {
            Ok(())
        }
    }

    pub async fn increment_save_count(ctx: &Ctx, mm: &ModelManager, journey_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, journey_id, CounterType::SaveCount, 1).await
    }

    pub async fn decrement_save_count(ctx: &Ctx, mm: &ModelManager, journey_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, journey_id, CounterType::SaveCount, -1).await
    }

    pub async fn increment_forward_count(ctx: &Ctx, mm: &ModelManager, journey_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, journey_id, CounterType::ForwardCount, 1).await
    }

    pub async fn decrement_forward_count(ctx: &Ctx, mm: &ModelManager, journey_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, journey_id, CounterType::ForwardCount, -1).await
    }

    pub async fn increment_total_likes(ctx: &Ctx, mm: &ModelManager, journey_id: &str, amount: i32) -> Result<()> {
        Self::update_counter(ctx, mm, journey_id, CounterType::TotalLikes, amount).await
    }

    pub async fn decrement_total_likes(ctx: &Ctx, mm: &ModelManager, journey_id: &str, amount: i32) -> Result<()> {
        Self::update_counter(ctx, mm, journey_id, CounterType::TotalLikes, -amount).await
    }

    pub async fn set_total_likes(_ctx: &Ctx, mm: &ModelManager, journey_id: &str, total_likes: i64) -> Result<()> {
        use sea_query::{Expr, Query};

        let mut query = Query::update();
        query
            .table(Self::table_ref())
            .value(JourneyIden::TotalLikes, total_likes)
            .and_where(Expr::col(JourneyIden::Id).eq(journey_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_with(&sql, values);
        let count = mm.dbx().execute(sqlx_query).await?;

        // -- Check result
        if count == 0 {
            Err(Error::EntityNotFound {
                entity: Self::TABLE,
                id: journey_id.to_string(),
            })
        } else {
            Ok(())
        }
    }
}

// endregion: ---- JourneyBmc
