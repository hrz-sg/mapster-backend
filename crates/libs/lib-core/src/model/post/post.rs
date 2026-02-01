use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::post::post_media::PostMedia;
use crate::model::user::{UserForPreview, UserPublicIden};
use crate::model::{Error, ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- Post Types

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Post {
    pub id: String,
    pub owner_id: String,
    pub title: String,
    pub description: String,
    pub is_published: bool,
    pub cover_media_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub media_count: i32,
    pub like_count: i64,
    pub comment_count: i64,
    pub save_count: i64,
    pub forward_count: i64,
    pub has_video: bool,
}

#[derive(Debug, Serialize)]
pub struct PostFeedItem {
    pub id: String,
    pub title: String,
    pub author: UserForPreview,
    pub thumbnail_url: Option<String>,
    pub media_count: i32,
    pub has_video: bool,
    pub like_count: i64,
}

#[derive(Debug, Serialize)]
pub struct PostDetail {
    pub id: String,
    pub title: String,
    pub description: String,
    pub author: UserForPreview,
    pub thumbnail_url: Option<String>,
    pub medias: Vec<PostMedia>,
    pub stats: PostStats,
}

#[derive(Fields, Deserialize)]
pub struct PostForCreate {
    pub title: String,
    pub description: String,
    pub is_published: Option<bool>,
    pub cover_media_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub media_count: Option<i32>,
    pub has_video: Option<bool>,
}

#[derive(Fields, Default, Deserialize)]
pub struct PostForUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_published: Option<bool>,
    pub media_count: Option<i32>,
    pub has_video: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PostStats {
    pub like_count: i64,
    pub comment_count: i64,
    pub save_count: i64,
    pub forward_count: i64,
    pub user_liked: bool,
    pub user_saved: bool,
    pub user_forwarded: bool,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct PostFilter {
    id: Option<OpValsString>,
    owner_id: Option<OpValsString>,
    title: Option<OpValsString>,
    is_published: Option<OpValsBool>,
    has_video: Option<OpValsBool>,
    media_count: Option<OpValsInt64>,
}

impl PostFilter {
    pub fn by_user(owner_id: &str) -> Self {
        Self {
            owner_id: Some(owner_id.into()),
            id: None,
            title: None,
            is_published: None,
            has_video: None,
            media_count: None,
        }
    }
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PostWithUser {
    // Post
    pub id: String,
    pub title: String,
    pub thumbnail_url: Option<String>,
    pub media_count: i32,
    pub has_video: bool,
    pub like_count: i64,
    pub comment_count: i64,
    pub save_count: i64,

    // User
    pub user_id: String,
    pub username: String,
    pub avatar_url: Option<String>,
}
// endregion: ---- Post Types

// region: ---- PostIden
#[derive(Iden)]
pub enum PostIden {
    #[iden = "post"]
    Table,
    #[iden = "id"]
    Id,
    #[iden = "user_id"]
    UserId,
    #[iden = "title"]
    Title,
    #[iden = "description"]
    Description,
    #[iden = "cover_media_url"]
    CoverMediaUrl,
    #[iden = "media_count"]
    MediaCount,
    #[iden = "like_count"]
    LikeCount,
    #[iden = "forward_count"]
    ForwardCount,
    #[iden = "save_count"]
    SaveCount,
    #[iden = "comment_count"]
    CommentCount,
}

// endregion: ---- PostIden

// region: ---- CounterType enum
pub enum CounterType {
    LikeCount,
    SaveCount,
    ForwardCount,
    CommentCount,
}
// endregion: ---- CounterType enum

// region: ---- PostBmc
pub struct PostBmc;

impl DbBmc for PostBmc {
    const TABLE: &'static str = "post";

    fn has_owner_id() -> bool {
        true
    }
}

impl PostBmc {
    pub async fn exists(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<bool> {
        let filter = vec![PostFilter {
            id: Some(post_id.into()),
            ..Default::default()
        }];

        base::exists::<Self, _>(ctx, mm, filter).await
    }

    pub async fn create(ctx: &Ctx, mm: &ModelManager, post_c: PostForCreate) -> Result<String> {
        base::create::<Self, _>(ctx, mm, post_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<Post> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    /// --- Get multiple posts by ids
    pub async fn get_many_with_users(_ctx: &Ctx, mm: &ModelManager, ids: Vec<&str>) -> Result<Vec<PostWithUser>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let mut query = Query::select();

        query
            .columns([
                (PostIden::Table, PostIden::Id),
                (PostIden::Table, PostIden::Title),
                (PostIden::Table, PostIden::MediaCount),
                (PostIden::Table, PostIden::UserId),
            ])
            .expr(Expr::col((UserPublicIden::Table, UserPublicIden::Username)))
            .expr(Expr::col((UserPublicIden::Table, UserPublicIden::AvatarUrl)))
            .from(PostIden::Table)
            .inner_join(
                UserPublicIden::Table,
                Expr::col((PostIden::Table, PostIden::UserId)).equals((UserPublicIden::Table, UserPublicIden::Id)),
            )
            .and_where(Expr::col((Self::TABLE, PostIden::Id)).is_in(ids.iter().copied()));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, PostWithUser, _>(&sql, values);
        let posts = mm.dbx().fetch_all(sqlx_query).await?;

        Ok(posts)
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<PostFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Post>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: &str, post_u: PostForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, post_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }

    pub async fn increment_like_count(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, post_id, CounterType::LikeCount, 1).await
    }

    pub async fn decrement_like_count(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, post_id, CounterType::LikeCount, -1).await
    }

    pub async fn increment_save_count(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, post_id, CounterType::SaveCount, 1).await
    }

    pub async fn decrement_save_count(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, post_id, CounterType::SaveCount, -1).await
    }

    pub async fn increment_forward_count(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, post_id, CounterType::ForwardCount, 1).await
    }

    pub async fn decrement_forward_count(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, post_id, CounterType::ForwardCount, -1).await
    }

    pub async fn increment_comment_count(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, post_id, CounterType::CommentCount, 1).await
    }

    pub async fn decrement_comment_count(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<()> {
        Self::update_counter(ctx, mm, post_id, CounterType::CommentCount, -1).await
    }

    pub async fn get_counts(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<(i64, i64, i64, i64)> {
        // like, comment, saved, forward
        let post = Self::get(ctx, mm, post_id).await?;
        Ok((post.like_count, post.comment_count, post.save_count, post.forward_count))
    }

    async fn update_counter(
        _ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
        counter_type: CounterType,
        delta: i32, // +1 or -1
    ) -> Result<()> {
        let mut query = Query::update();
        query.table(Self::table_ref());

        match counter_type {
            CounterType::SaveCount => {
                query.value(PostIden::SaveCount, Expr::col(PostIden::SaveCount).add(delta));
                // protect column from negative values
                if delta < 0 {
                    query.and_where(Expr::col(PostIden::SaveCount).gt(0));
                }
            }
            CounterType::ForwardCount => {
                query.value(PostIden::ForwardCount, Expr::col(PostIden::ForwardCount).add(delta));
                if delta < 0 {
                    query.and_where(Expr::col(PostIden::ForwardCount).gt(0));
                }
            }
            CounterType::LikeCount => {
                query.value(PostIden::LikeCount, Expr::col(PostIden::LikeCount).add(delta));
                if delta < 0 {
                    query.and_where(Expr::col(PostIden::LikeCount).gt(0));
                }
            }
            CounterType::CommentCount => {
                query.value(PostIden::CommentCount, Expr::col(PostIden::CommentCount).add(delta));
                if delta < 0 {
                    query.and_where(Expr::col(PostIden::CommentCount).gt(0));
                }
            }
        }

        query.and_where(Expr::col(PostIden::Id).eq(post_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_with(&sql, values);
        let count = mm.dbx().execute(sqlx_query).await?;

        // -- Check result
        if count == 0 {
            Err(Error::EntityNotFound {
                entity: Self::TABLE,
                id: post_id.to_string(),
            })
        } else {
            Ok(())
        }
    }
}

// endregion: ---- PostBmc

// region: ---- Test
#[cfg(test)]
mod tests {
    #[allow(unused)]
    use crate::_dev_utils;
    use crate::model::Error;

    use super::*;
    use anyhow::{Ok, Result};
    // type Error = Box<dyn std::error::Error>;
    // type Result<T> = core::result::Result<T, Error>;
    use serde_json::json;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title: &'static str = "test_create_ok title";
        let fx_description: &'static str = "test_create_ok description";
        let fx_is_published: Option<bool> = Some(true);
        let fx_cover_media_url: Option<String> = Some(String::from(
            "https://plus.unsplash.com/premium_photo-1759484628323-142ec8547fb9?ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&q=80&w=774",
        ));
        let fx_thumbnail_url: Option<String> = Some(String::from(
            "https://plus.unsplash.com/premium_photo-1759484628323-142ec8547fb9?ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&q=80&w=774",
        ));
        let fx_media_count: Option<i32> = Some(1);
        let fx_has_video: Option<bool> = Some(false);

        // -- Exec
        let post_c = PostForCreate {
            title: fx_title.to_string(),
            description: fx_description.to_string(),
            is_published: fx_is_published,
            cover_media_url: fx_cover_media_url,
            thumbnail_url: fx_thumbnail_url,
            media_count: fx_media_count,
            has_video: fx_has_video,
        };

        let id = PostBmc::create(&ctx, &mm, post_c).await?;

        // -- Check
        let post = PostBmc::get(&ctx, &mm, &id).await?;
        assert_eq!(post.title, fx_title);
        assert_eq!(post.description, fx_description);
        assert_eq!(post.owner_id, ctx.user_id());

        // -- Clean
        PostBmc::delete(&ctx, &mm, &id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = "pst_000000000000000000000";

        // -- Exec
        let res = PostBmc::get(&ctx, &mm, fx_id).await;

        // -- Check
        assert!(
            matches!(res, Err(Error::EntityNotFound { entity: "post", id: _ })),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_all_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_list_all_ok-post 01", "test_list_all_ok-post 02"];
        let fx_descriptions = &["test_list_all_ok-post 01", "test_list_all_ok-post 02"];
        _dev_utils::seed_posts(&ctx, &mm, fx_titles, fx_descriptions).await?;

        // -- Exec
        let posts = PostBmc::list(&ctx, &mm, None, None).await?;

        // -- Check
        let posts: Vec<Post> = posts
            .into_iter()
            .filter(|t| t.title.starts_with("test_list_all_ok-post"))
            .collect();
        assert_eq!(posts.len(), 2, "number of seeded posts.");

        // -- Clean
        for post in posts.iter() {
            PostBmc::delete(&ctx, &mm, &post.id).await?;
        }

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_filter_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &[
            "test_list_by_filter_ok-post 01.a",
            "test_list_by_filter_ok-post 01.b",
            "test_list_by_filter_ok-post 02.a",
            "test_list_by_filter_ok-post 02.b",
            "test_list_by_filter_ok-post 03",
        ];
        let fx_descriptions = &[
            "test_list_by_filter_ok-post 01.a",
            "test_list_by_filter_ok-post 01.b",
            "test_list_by_filter_ok-post 02.a",
            "test_list_by_filter_ok-post 02.b",
            "test_list_by_filter_ok-post 03",
        ];
        _dev_utils::seed_posts(&ctx, &mm, fx_titles, fx_descriptions).await?;

        // -- Exec
        let filters: Vec<PostFilter> = serde_json::from_value(json!([
        {
        "title": {
            "$endsWith": ".a",
            "$containsAny": ["01", "02"]
        }
        },
        {
            "title": {"$contains": "03"}
        }
        ]))?;
        let list_options = serde_json::from_value(json!({
         "order_bys": "!ctime"
        }))?;
        let posts = PostBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        // -- Check
        assert_eq!(posts.len(), 3);
        assert!(posts[0].title.ends_with("03"));
        assert!(posts[1].title.ends_with("02.a"));
        assert!(posts[2].title.ends_with("01.a"));

        // -- Clean
        let posts = PostBmc::list(
            &ctx,
            &mm,
            Some(serde_json::from_value(json!([{
                "title": {"$startsWith": "test_list_by_filter_ok"}
            }]))?),
            None,
        )
        .await?;
        assert_eq!(posts.len(), 5);
        for post in posts.iter() {
            PostBmc::delete(&ctx, &mm, &post.id).await?;
        }

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_update_ok - post 01";
        let fx_title_new = "test_update_ok - post 01 - new title";
        let fx_description = "test_update_ok - post 01";
        let fx_description_new = "test_update_ok - post 01 - new description";
        let fx_post = _dev_utils::seed_posts(&ctx, &mm, &[fx_title], &[fx_description])
            .await?
            .remove(0);

        // -- Exec
        PostBmc::update(
            &ctx,
            &mm,
            &fx_post.id,
            PostForUpdate {
                title: Some(fx_title_new.to_string()),
                description: Some(fx_description_new.to_string()),
                ..Default::default()
            },
        )
        .await?;

        // -- Check
        let post = PostBmc::get(&ctx, &mm, &fx_post.id).await?;
        assert_eq!(post.title, fx_title_new);
        assert_eq!(post.description, fx_description_new);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = "pst_000000000000000000000";

        // -- Exec
        let res = PostBmc::delete(&ctx, &mm, fx_id).await;

        // -- Check
        assert!(
            matches!(res, Err(Error::EntityNotFound { entity: "post", id: _ })),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
// endregion: ---- Test
