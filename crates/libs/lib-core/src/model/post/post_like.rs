use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, TimestampType};
use crate::model::user::{UserBmc, UserForPreview, UserPublicIden};
use crate::model::{ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, OpValsString};
use sea_query::{Expr, Iden, JoinType, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- PostLike Types

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct PostLike {
    pub post_id: String,
    pub user_id: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct PostLikeFilter {
    pub post_id: Option<OpValsString>,
    pub user_id: Option<OpValsString>,
}

#[derive(Debug, Fields, Deserialize)]
pub struct PostLikeForCreate {
    pub post_id: String,
    pub user_id: String,
}

#[derive(Iden, Clone)]
enum PostLikeIden {
    #[iden = "post_id"]
    PostId,
    #[iden = "user_id"]
    UserId,
    #[iden = "ctime"]
    Ctime,
}

// endregion: ---- PostLike Types

// region: ---- PostLikeBmc
pub struct PostLikeBmc;

impl DbBmc for PostLikeBmc {
    const TABLE: &'static str = "post_like";

    fn has_id() -> bool {
        false
    }

    fn timestamp_fields() -> TimestampType {
        TimestampType::CtimeOnly
    }
}

impl PostLikeBmc {
    pub async fn exists(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<bool> {
        let filter = PostLikeFilter {
            post_id: Some(post_id.into()),
            user_id: Some(ctx.user_id().into()),
        };

        base::exists::<Self, _>(ctx, mm, filter).await
    }

    // Check if user liked the post
    pub async fn user_liked_post(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<bool> {
        Self::exists(ctx, mm, post_id).await
    }

    // Create like
    pub async fn create(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<()> {
        let item_c = PostLikeForCreate {
            post_id: post_id.to_string(),
            user_id: ctx.user_id().to_owned(),
        };

        base::create_on_conflict::<Self, _, _>(ctx, mm, item_c, &[PostLikeIden::PostId, PostLikeIden::UserId]).await?;

        Ok(())
    }

    /// Delete like
    pub async fn delete(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<()> {
        let user_id = ctx.user_id();

        let filter = PostLikeFilter {
            post_id: Some(post_id.into()),
            user_id: Some(user_id.into()),
        };

        if let Some(like) = base::first_by_composite_key::<Self, PostLike, _>(ctx, mm, Some(filter)).await? {
            // Delete through custom SQL, as the table doesn't have ID
            let mut query = Query::delete();
            query
                .from_table(Self::table_ref())
                .and_where(Expr::col(PostLikeIden::PostId).eq(&like.post_id))
                .and_where(Expr::col(PostLikeIden::UserId).eq(&like.user_id));

            let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
            mm.dbx().execute(sqlx::query_with(&sql, values)).await?;
        }

        Ok(())
    }

    /// Delete like
    pub async fn delete_simple(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<()> {
        let user_id = ctx.user_id();

        let mut query = Query::delete();
        query
            .from_table(Self::table_ref())
            .and_where(Expr::col(PostLikeIden::PostId).eq(post_id))
            .and_where(Expr::col(PostLikeIden::UserId).eq(user_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        mm.dbx().execute(sqlx::query_with(&sql, values)).await?;

        Ok(())
    }

    /// List of likes by post
    pub async fn list_by_post(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<Vec<PostLike>> {
        let filters = Some(PostLikeFilter {
            post_id: Some(post_id.to_string().into()),
            user_id: None,
        });

        base::list::<Self, _, _>(ctx, mm, filters, None).await
    }

    /// Get list of users liked the post
    pub async fn get_likers_with_user_preview(
        _ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
        limit: Option<u32>,
    ) -> Result<Vec<UserForPreview>> {
        // -- Build query with JOIN
        let mut query = Query::select();
        query
            .columns([
                (UserBmc::TABLE, UserPublicIden::Id),
                (UserBmc::TABLE, UserPublicIden::Username),
                (UserBmc::TABLE, UserPublicIden::AvatarObjectKey),
            ])
            .from(UserBmc::table_ref())
            .join(
                JoinType::InnerJoin,
                Self::table_ref(),
                Expr::col((UserBmc::TABLE, UserPublicIden::Id)).equals((Self::TABLE, PostLikeIden::UserId)),
            )
            .and_where(Expr::col((Self::TABLE, PostLikeIden::PostId)).eq(post_id))
            .order_by((Self::TABLE, PostLikeIden::Ctime), sea_query::Order::Desc);

        if let Some(limit) = limit {
            query.limit(limit as u64);
        }

        // -- Execute query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let users: Vec<UserForPreview> = mm
            .dbx()
            .fetch_all(sqlx::query_as_with::<_, UserForPreview, _>(&sql, values))
            .await?;

        Ok(users)
    }

    /// Count likes by post_id
    pub async fn count_likes_by_post_id(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<i64> {
        let filter = PostLikeFilter {
            post_id: Some(post_id.into()),
            user_id: None,
        };

        base::count::<Self, _>(ctx, mm, Some(filter)).await
    }
}

// endregion: ---- PostLikeBmc
