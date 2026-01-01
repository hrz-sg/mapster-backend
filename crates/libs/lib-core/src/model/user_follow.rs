use crate::model::user::{UserBmc, UserForPreview, UserPublicIden};
use crate::{ctx::Ctx, model::ModelManager};
use crate::model::base::DbBmc;
use crate::model::Result;
use sea_query::{Condition, Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;

pub struct UserFollowBmc;

#[derive(Iden)]
enum UserFollowIden {
    FollowerId,
    FollowingId,
}

impl DbBmc for UserFollowBmc {
    const TABLE: &'static str = "user_follow";

    fn has_id() -> bool {
        false
    }
}

impl UserFollowBmc {
    pub async fn is_following(
        _ctx: &Ctx,
        mm: &ModelManager,
        follower_id: &str,
        following_id: &str,
    ) -> Result<bool> {

        let mut query = Query::select();
        query
            .expr(Expr::val(1))
            .from(Self::table_ref())
            .and_where(Expr::col(UserFollowIden::FollowerId).eq(follower_id))
            .and_where(Expr::col(UserFollowIden::FollowingId).eq(following_id))
            .limit(1);

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, (i32,), _>(&sql, values);
        let result = mm.dbx().fetch_optional(sqlx_query).await?;

        Ok(result.is_some())
    }

    pub async fn list_followers(
        _ctx: &Ctx,
        mm: &ModelManager,
        user_id: &str,
    ) -> Result<Vec<UserForPreview>> {
        
        let mut query = Query::select();
        
        query
            .columns([UserPublicIden::Id, UserPublicIden::Username, UserPublicIden::AvatarUrl])
            .from(UserBmc::table_ref())
            .inner_join(
                Self::table_ref(),
                Expr::col(UserPublicIden::Id)
                    .equals(UserFollowIden::FollowerId)
            )
            .and_where(
                Expr::col(UserFollowIden::FollowingId).eq(user_id)
            );

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, UserForPreview, _>(&sql, values);

        let result = mm.dbx().fetch_all(sqlx_query).await?;
        Ok(result)
    }

    pub async fn list_followings(
        _ctx: &Ctx,
        mm: &ModelManager,
        user_id: &str,
    ) -> Result<Vec<UserForPreview>> {
        
        let mut query = Query::select();
        
        query
            .columns([UserPublicIden::Id, UserPublicIden::Username, UserPublicIden::AvatarUrl])
            .from(UserBmc::table_ref())
            .inner_join(
                Self::table_ref(),
                Expr::col(UserPublicIden::Id)
                    .equals(UserFollowIden::FollowingId)
            )
            .and_where(
                Expr::col(UserFollowIden::FollowerId).eq(user_id)
            );

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, UserForPreview, _>(&sql, values);

        let result = mm.dbx().fetch_all(sqlx_query).await?;
        Ok(result)
    }

    /// --- Count followers & followings
    pub async fn count_follows(
        _ctx: &Ctx,
        mm: &ModelManager,
        user_id: &str,
    ) -> Result<i64> {

        let mut query = Query::select();

        query
            .expr(Expr::col(UserFollowIden::FollowerId).count())
            .from(Self::table_ref())
            .and_where(Expr::col(UserFollowIden::FollowingId).eq(user_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (result,) = mm.dbx().fetch_one(sqlx_query).await?;

        Ok(result)
    }

    /// --- Check if users mutually subscribed
    pub async fn follow_relations(
        _ctx: &Ctx,
        mm: &ModelManager,
        viewer_id: &str,
        target_user_ids: &[&str],
    ) -> Result<Vec<(String, String)>> {

        if target_user_ids.is_empty() {
            return Ok(vec![]);
        }

        let mut query = Query::select();

        query
            .columns([
                UserFollowIden::FollowerId,
                UserFollowIden::FollowingId,
            ])
            .from(Self::table_ref())
            .cond_where(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(Expr::col(UserFollowIden::FollowerId).eq(viewer_id))
                            .add(
                                Expr::col(UserFollowIden::FollowingId)
                                    .is_in(target_user_ids.iter().copied())
                            )
                    )
                    .add(
                        Condition::all()
                            .add(Expr::col(UserFollowIden::FollowingId).eq(viewer_id))
                            .add(
                                Expr::col(UserFollowIden::FollowerId)
                                    .is_in(target_user_ids.iter().copied())
                            )
                    )
            );

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query =
            sqlx::query_as_with::<_, (String, String), _>(&sql, values);

        let result = mm.dbx().fetch_all(sqlx_query).await?;
        Ok(result)
    }
}