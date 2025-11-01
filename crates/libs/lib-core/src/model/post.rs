use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use crate::model::{Result, ModelManager};
use sqlx::FromRow;
use modql::field::Fields;

// region: ---- Post Types

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub is_published: bool,
    pub cover_media_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub media_count: i32,
    pub has_video: bool,
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
    pub is_published: Option<bool>
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct PostFilter {
    id: Option<OpValsInt64>,
    title: Option<OpValsString>,
    is_published: Option<OpValsBool>,
    has_video: Option<OpValsBool>,
    media_count: Option<OpValsInt64>,
}

// endregion: ---- Post Types

// region: ---- PostBmc
pub struct PostBmc;

impl DbBmc for PostBmc {
    const TABLE: &'static str = "post";
}

impl PostBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        post_c: PostForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, post_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Post> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx, 
        mm: &ModelManager,
        filters: Option<Vec<PostFilter>>,
        list_options: Option<ListOptions>
    ) -> Result<Vec<Post>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: i64, post_u: PostForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, post_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
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
        let fx_cover_media_url: Option<String> = Some(String::from("https://plus.unsplash.com/premium_photo-1759484628323-142ec8547fb9?ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&q=80&w=774"));
        let fx_thumbnail_url: Option<String> = Some(String::from("https://plus.unsplash.com/premium_photo-1759484628323-142ec8547fb9?ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&q=80&w=774"));
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
        let post = PostBmc::get(&ctx, &mm, id).await?;
        assert_eq!(post.title, fx_title);
        assert_eq!(post.description, fx_description);

        // -- Clean
        PostBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        // -- Exec
        let res = PostBmc::get(&ctx, &mm, fx_id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound { 
                    entity: "post",
                    id: 100 
                })
            ),
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
            PostBmc::delete(&ctx, &mm, post.id).await?;
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
        "test_list_by_filter_ok-post 03"
        ];
       let fx_descriptions = &[
        "test_list_by_filter_ok-post 01.a", 
        "test_list_by_filter_ok-post 01.b", 
        "test_list_by_filter_ok-post 02.a", 
        "test_list_by_filter_ok-post 02.b", 
        "test_list_by_filter_ok-post 03"
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
        "order_bys": "!id"
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
            PostBmc::delete(&ctx, &mm, post.id).await?;
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
            fx_post.id, 
            PostForUpdate { 
                title: Some(fx_title_new.to_string()), 
                description: Some(fx_description_new.to_string()),
                ..Default::default()
            },
        )
        .await?;

        // -- Check
        let post = PostBmc::get(&ctx, &mm, fx_post.id).await?;
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
        let fx_id = 100;

        // -- Exec
        let res = PostBmc::delete(&ctx, &mm, fx_id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound { 
                    entity: "post",
                    id: 100 
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
// endregion: ---- Test