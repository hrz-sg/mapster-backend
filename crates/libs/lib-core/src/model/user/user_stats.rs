// region: --- Modules
use crate::model::base::{DbBmc, TimestampType};
use crate::model::{Error, Result};
use crate::{ctx::Ctx, model::ModelManager};
use modql::field::{Fields, HasSeaFields};
use sea_query::{Expr, Iden, OnConflict, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::Serialize;
use sqlx::FromRow;
// endregion: --- Modules

// region:    --- UserSrats Types
#[derive(Iden)]
enum UserStatsIden {
    OwnerId,
    PostsCount,
    FollowersCount,
    FollowingCount,
}

#[derive(Clone, Debug, Fields, FromRow, Serialize)]
pub struct UserProfileStats {
    pub posts_count: i64,
    pub followers_count: i64,
    pub following_count: i64,
}

enum StatsCounter {
    Posts,
    Followers,
    Following,
}

// endregion:    --- UserSrats Types

// region:    --- UserSratsBmc
pub struct UserStatsBmc;

impl DbBmc for UserStatsBmc {
    const TABLE: &'static str = "user_stats";

    fn has_id() -> bool {
        false
    }

    fn has_owner_id() -> bool {
        true
    }

    fn timestamp_fields() -> TimestampType {
        TimestampType::None
    }
}

impl UserStatsBmc {
    pub async fn create_for_user(_ctx: &Ctx, mm: &ModelManager, owner_id: &str) -> Result<()> {
        let mut query = Query::insert();

        query
            .into_table(Self::table_ref())
            .columns([
                UserStatsIden::OwnerId,
                UserStatsIden::PostsCount,
                UserStatsIden::FollowersCount,
                UserStatsIden::FollowingCount,
            ])
            .values([owner_id.into(), 0.into(), 0.into(), 0.into()])? // <-- дефолтные значения
            .on_conflict(OnConflict::columns([UserStatsIden::OwnerId]).do_nothing().to_owned());

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        mm.dbx().execute(sqlx::query_with(&sql, values)).await?;

        Ok(())
    }

    pub async fn get_by_user_id(_ctx: &Ctx, mm: &ModelManager, owner_id: &str) -> Result<UserProfileStats> {
        let mut query = Query::select();
        query
            .columns(UserProfileStats::sea_column_refs())
            .from(Self::table_ref())
            .and_where(Expr::col(UserStatsIden::OwnerId).eq(owner_id))
            .limit(1);

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, UserProfileStats, _>(&sql, values);
        let stats = mm.dbx().fetch_optional(sqlx_query).await?;

        stats.ok_or(Error::EntityNotFound {
            entity: Self::TABLE,
            id: owner_id.to_string(),
        })
    }

    async fn update_counter(
        _ctx: &Ctx,
        mm: &ModelManager,
        owner_id: &str,
        counter: StatsCounter,
        delta: i64, // +1 / -1
    ) -> Result<()> {
        let mut query = Query::update();
        query.table(Self::table_ref());

        match counter {
            StatsCounter::Posts => {
                query.value(
                    UserStatsIden::PostsCount,
                    Expr::col(UserStatsIden::PostsCount).add(delta),
                );
                if delta < 0 {
                    query.and_where(Expr::col(UserStatsIden::PostsCount).gt(0));
                }
            }
            StatsCounter::Followers => {
                query.value(
                    UserStatsIden::FollowersCount,
                    Expr::col(UserStatsIden::FollowersCount).add(delta),
                );
                if delta < 0 {
                    query.and_where(Expr::col(UserStatsIden::FollowersCount).gt(0));
                }
            }
            StatsCounter::Following => {
                query.value(
                    UserStatsIden::FollowingCount,
                    Expr::col(UserStatsIden::FollowingCount).add(delta),
                );
                if delta < 0 {
                    query.and_where(Expr::col(UserStatsIden::FollowingCount).gt(0));
                }
            }
        }

        query.and_where(Expr::col(UserStatsIden::OwnerId).eq(owner_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let count = mm.dbx().execute(sqlx::query_with(&sql, values)).await?;

        if count == 0 {
            Err(Error::EntityNotFound {
                entity: Self::TABLE,
                id: owner_id.to_string(),
            })
        } else {
            Ok(())
        }
    }

    pub async fn increment_posts(ctx: &Ctx, mm: &ModelManager, owner_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, owner_id, StatsCounter::Posts, 1).await
    }

    pub async fn decrement_posts(ctx: &Ctx, mm: &ModelManager, owner_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, owner_id, StatsCounter::Posts, -1).await
    }

    pub async fn increment_followers(ctx: &Ctx, mm: &ModelManager, owner_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, owner_id, StatsCounter::Followers, 1).await
    }

    pub async fn decrement_followers(ctx: &Ctx, mm: &ModelManager, owner_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, owner_id, StatsCounter::Followers, -1).await
    }

    pub async fn increment_following(ctx: &Ctx, mm: &ModelManager, owner_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, owner_id, StatsCounter::Following, 1).await
    }

    pub async fn decrement_following(ctx: &Ctx, mm: &ModelManager, owner_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, owner_id, StatsCounter::Following, -1).await
    }
}
// endregion:    --- UserStatsBmc

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;

    async fn setup() -> Result<(Ctx, ModelManager, String)> {
        let mm = _dev_utils::init_test().await;
        let ctx = crate::ctx::Ctx::root_ctx();
        let fx_user_id = "usr_demo1";

        UserStatsBmc::create_for_user(&ctx, &mm, &fx_user_id).await?;

        Ok((ctx, mm, fx_user_id.to_string()))
    }

    #[tokio::test]
    async fn create_stats_for_user_ok() -> crate::model::Result<()> {
        let (ctx, mm, fx_user_id) = setup().await?;

        // -- fetch
        let stats = UserStatsBmc::get_by_user_id(&ctx, &mm, &fx_user_id).await?;

        // -- assert defaults
        assert_eq!(stats.posts_count, 0);
        assert_eq!(stats.followers_count, 0);
        assert_eq!(stats.following_count, 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_stats_ok() -> crate::model::Result<()> {
        let (ctx, mm, fx_user_id) = setup().await?;

        let stats = UserStatsBmc::get_by_user_id(&ctx, &mm, &fx_user_id).await?;
        assert_eq!(stats.posts_count, 0);

        assert!(stats.posts_count >= 0);
        assert!(stats.followers_count >= 0);
        assert!(stats.following_count >= 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_increment_decrement_posts() -> Result<()> {
        let (ctx, mm, fx_user_id) = setup().await?;

        UserStatsBmc::increment_posts(&ctx, &mm, &fx_user_id).await?;
        UserStatsBmc::increment_posts(&ctx, &mm, &fx_user_id).await?;

        let stats = UserStatsBmc::get_by_user_id(&ctx, &mm, &fx_user_id).await?;
        assert_eq!(stats.posts_count, 2);

        UserStatsBmc::decrement_posts(&ctx, &mm, &fx_user_id).await?;

        let stats = UserStatsBmc::get_by_user_id(&ctx, &mm, &fx_user_id).await?;
        assert_eq!(stats.posts_count, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_followers_and_following() -> Result<()> {
        let (ctx, mm, fx_user_id) = setup().await?;

        UserStatsBmc::increment_followers(&ctx, &mm, &fx_user_id).await?;
        UserStatsBmc::increment_following(&ctx, &mm, &fx_user_id).await?;

        let stats = UserStatsBmc::get_by_user_id(&ctx, &mm, &fx_user_id).await?;
        assert_eq!(stats.followers_count, 1);
        assert_eq!(stats.following_count, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_update_non_existing_user_stats_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = crate::ctx::Ctx::root_ctx();
        let fx_fake_user_id = "usr_not_exists";

        let res = UserStatsBmc::increment_posts(&ctx, &mm, fx_fake_user_id).await;
        assert!(matches!(res, Err(Error::EntityNotFound { .. })));

        Ok(())
    }
}

// endregion:    --- Tests
