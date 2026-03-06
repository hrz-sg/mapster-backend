use crate::model::Result;
use crate::model::base::DbBmc;
use crate::model::user::{UserBmc, UserForPreview, UserPublicIden};
use crate::{ctx::Ctx, model::ModelManager};
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
    pub async fn follow(ctx: &Ctx, mm: &ModelManager, following_id: &str) -> Result<()> {
        let mut query = sea_query::Query::insert();
        query
            .into_table(Self::table_ref())
            .columns([UserFollowIden::FollowerId, UserFollowIden::FollowingId])
            .values([ctx.user_id().into(), following_id.into()])?
            .on_conflict(
                sea_query::OnConflict::columns([UserFollowIden::FollowerId, UserFollowIden::FollowingId])
                    .do_nothing()
                    .to_owned(),
            );

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        mm.dbx().execute(sqlx::query_with(&sql, values)).await?;
        Ok(())
    }

    pub async fn unfollow(ctx: &Ctx, mm: &ModelManager, following_id: &str) -> Result<()> {
        let mut query = sea_query::Query::delete();
        query
            .from_table(Self::table_ref())
            .and_where(Expr::col(UserFollowIden::FollowerId).eq(ctx.user_id()))
            .and_where(Expr::col(UserFollowIden::FollowingId).eq(following_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        mm.dbx().execute(sqlx::query_with(&sql, values)).await?;
        Ok(())
    }

    pub async fn is_following(ctx: &Ctx, mm: &ModelManager, following_id: &str) -> Result<bool> {
        
        let follower_id = ctx.user_id();

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

    pub async fn list_followers(_ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<Vec<UserForPreview>> {
        let mut query = Query::select();

        query
            .columns([UserPublicIden::Id, UserPublicIden::Username, UserPublicIden::AvatarObjectKey])
            .from(UserBmc::table_ref())
            .inner_join(
                Self::table_ref(),
                Expr::col(UserPublicIden::Id).equals(UserFollowIden::FollowerId),
            )
            .and_where(Expr::col(UserFollowIden::FollowingId).eq(user_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, UserForPreview, _>(&sql, values);

        let result = mm.dbx().fetch_all(sqlx_query).await?;
        Ok(result)
    }

    pub async fn list_followings(_ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<Vec<UserForPreview>> {
        let mut query = Query::select();

        query
            .columns([UserPublicIden::Id, UserPublicIden::Username, UserPublicIden::AvatarObjectKey])
            .from(UserBmc::table_ref())
            .inner_join(
                Self::table_ref(),
                Expr::col(UserPublicIden::Id).equals(UserFollowIden::FollowingId),
            )
            .and_where(Expr::col(UserFollowIden::FollowerId).eq(user_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, UserForPreview, _>(&sql, values);

        let result = mm.dbx().fetch_all(sqlx_query).await?;
        Ok(result)
    }

    /// --- Count followers & followings
    pub async fn count_follows(_ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<i64> {
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
            .columns([UserFollowIden::FollowerId, UserFollowIden::FollowingId])
            .from(Self::table_ref())
            .cond_where(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(Expr::col(UserFollowIden::FollowerId).eq(viewer_id))
                            .add(Expr::col(UserFollowIden::FollowingId).is_in(target_user_ids.iter().copied())),
                    )
                    .add(
                        Condition::all()
                            .add(Expr::col(UserFollowIden::FollowingId).eq(viewer_id))
                            .add(Expr::col(UserFollowIden::FollowerId).is_in(target_user_ids.iter().copied())),
                    ),
            );

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, (String, String), _>(&sql, values);

        let result = mm.dbx().fetch_all(sqlx_query).await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;

    #[tokio::test]
    async fn test_follow_and_unfollow_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;

        let fx_follower = "usr_demo1";
        let fx_following = "usr_demo2";

        let ctx_follower = Ctx::new(fx_follower.to_string()).unwrap();
        let root_ctx = Ctx::root_ctx();

        // -- Check if not following
        assert!(!UserFollowBmc::is_following(&root_ctx, &mm, fx_following).await?);

        // -- Follow
        UserFollowBmc::follow(&ctx_follower, &mm, fx_following).await?;
        assert!(UserFollowBmc::is_following(&root_ctx, &mm, fx_following).await?);

        // -- Unfollow
        UserFollowBmc::unfollow(&ctx_follower, &mm, fx_following).await?;
        assert!(!UserFollowBmc::is_following(&root_ctx, &mm, fx_following).await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_list_followers_and_followings_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;

        let fx_user_1 = "usr_demo1";
        let fx_user_2 = "usr_demo2";

        let ctx1 = Ctx::new(fx_user_1.to_string()).unwrap();
        let root_ctx = Ctx::root_ctx();

        // -- Clear before follow
        let _ = UserFollowBmc::unfollow(&ctx1, &mm, fx_user_2).await;

        // -- Follow
        UserFollowBmc::follow(&ctx1, &mm, fx_user_2).await?;

        // -- Check fx_user_2 followers
        let followers = UserFollowBmc::list_followers(&root_ctx, &mm, fx_user_2).await?;
        assert_eq!(followers.len(), 1);
        assert_eq!(followers[0].id, fx_user_1);

        // -- Check fx_user_1 followers
        let followings = UserFollowBmc::list_followings(&root_ctx, &mm, fx_user_1).await?;
        assert_eq!(followings.len(), 1);
        assert_eq!(followings[0].id, fx_user_2);

        Ok(())
    }

    #[tokio::test]
    async fn test_count_follows_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;

        let fx_user_1 = "usr_demo1";
        let fx_user_2 = "usr_demo2";

        let ctx1 = Ctx::new(fx_user_1.to_string()).unwrap();
        let root_ctx = Ctx::root_ctx();

        // -- Clear before follow
        let _ = UserFollowBmc::unfollow(&ctx1, &mm, fx_user_2).await;

        // -- Follow and check count
        UserFollowBmc::follow(&ctx1, &mm, fx_user_2).await?;

        let fx_user_2_followers = UserFollowBmc::count_follows(&root_ctx, &mm, fx_user_2).await?;
        assert_eq!(fx_user_2_followers, 1);

        // -- Unfollow and check count
        UserFollowBmc::unfollow(&ctx1, &mm, fx_user_2).await?;

        let fx_user_2_followers_after = UserFollowBmc::count_follows(&root_ctx, &mm, fx_user_2).await?;
        assert_eq!(fx_user_2_followers_after, 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_follow_relations_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;

        let fx_user_1 = "usr_demo1";
        let fx_user_2 = "usr_demo2";

        let ctx1 = Ctx::new(fx_user_1.to_string()).unwrap();
        let ctx2 = Ctx::new(fx_user_2.to_string()).unwrap();
        let root_ctx = Ctx::root_ctx();

        // -- Clear
        let _ = UserFollowBmc::unfollow(&ctx1, &mm, fx_user_2).await;
        let _ = UserFollowBmc::unfollow(&ctx2, &mm, fx_user_1).await;

        // -- Follow
        UserFollowBmc::follow(&ctx1, &mm, fx_user_2).await?;

        // -- Check relations
        let relations = UserFollowBmc::follow_relations(&root_ctx, &mm, fx_user_1, &[fx_user_2]).await?;
        assert_eq!(relations.len(), 1);
        assert_eq!(relations[0], (fx_user_1.to_string(), fx_user_2.to_string()));

        // -- Mutual follow
        UserFollowBmc::follow(&ctx2, &mm, fx_user_1).await?;

        let relations_mutual = UserFollowBmc::follow_relations(&root_ctx, &mm, fx_user_1, &[fx_user_2]).await?;
        assert_eq!(relations_mutual.len(), 2);
        assert!(relations_mutual.contains(&(fx_user_1.to_string(), fx_user_2.to_string())));
        assert!(relations_mutual.contains(&(fx_user_2.to_string(), fx_user_1.to_string())));

        Ok(())
    }
}
