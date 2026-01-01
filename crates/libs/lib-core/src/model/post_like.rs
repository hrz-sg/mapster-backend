use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::user::{UserBmc, UserForPreview, UserPublicIden};
use crate::model::{ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsString};
use sea_query::{Expr, Iden, JoinType, OnConflict, PostgresQueryBuilder, Query};
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

#[derive(Iden)]
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
}

impl PostLikeBmc {
    // Check if user liked the post
    pub async fn user_liked_post(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
    ) -> Result<bool> {
        let user_id = ctx.user_id();

        let filter = PostLikeFilter {
            post_id: Some(post_id.into()),
            user_id: Some(user_id.into()),
        };

        let likes = base::list::<Self, PostLike, _>(
            ctx,
            mm,
            Some(filter),
            Some(ListOptions {
                limit: Some(1),
                offset: None,
                order_bys: None
            })
        ).await?;

        Ok(!likes.is_empty())
    }
    
    // Create like
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
    ) -> Result<()> {

        let user_id = ctx.user_id();
        
        let mut query = Query::insert();
        query
            .into_table(Self::table_ref())
            .columns([PostLikeIden::PostId, PostLikeIden::UserId])
            .values_panic([post_id.into(), user_id.into()])
            .on_conflict(
                OnConflict::columns([PostLikeIden::PostId, PostLikeIden::UserId]) // if like exists
                    .do_nothing()// then do nothing
                    .to_owned()
            );

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_with(&sql, values);
        mm.dbx().execute(sqlx_query).await?;
            
        Ok(())
    }
    
    /// Delete like
    pub async fn delete(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
    ) -> Result<()> {

        let user_id = ctx.user_id();

        let mut query = Query::delete();
        query
            .from_table(Self::table_ref())
            .and_where(Expr::col(PostLikeIden::PostId).eq(post_id))
            .and_where(Expr::col(PostLikeIden::UserId).eq(user_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_with(&sql, values);
        mm.dbx().execute(sqlx_query).await?;
            
        Ok(())
    }
   
    /// List of likes by post
    pub async fn list_by_post(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
    ) -> Result<Vec<PostLike>> {
        let filters = Some(vec![PostLikeFilter {
            post_id: Some(post_id.to_string().into()),
            user_id: None,
        }]);
        
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
                (UserBmc::TABLE, UserPublicIden::AvatarUrl),
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

        let users: Vec<UserForPreview> = mm.dbx()
            .fetch_all(sqlx::query_as_with::<_, UserForPreview, _>(&sql, values))
            .await?;

        Ok(users)
    }

    /// Count likes by post_id
    pub async fn count_likes_by_post_id(
        _ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
    ) -> Result<i64> {
        let (sql, values) = Query::select()
            .expr(Expr::col(PostLikeIden::PostId).count())
            .from(Self::table_ref())
            .and_where(Expr::col(PostLikeIden::PostId).eq(post_id))
            .build_sqlx(PostgresQueryBuilder);

        let (count,) = mm.dbx()
            .fetch_one(sqlx::query_as_with::<_, (i64,), _>(&sql, values))
            .await?;

        Ok(count)
    }
}

// endregion: ---- PostLikeBmc