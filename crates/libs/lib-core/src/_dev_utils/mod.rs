// region: ---- Modules
mod dev_db;

use tokio::sync::OnceCell;
use tracing::info;

use crate::{ctx::Ctx, model::{self, post::{Post, PostBmc, PostForCreate}, ModelManager}};

// endregion: ---- Modules

/// Initialize environment for local development.
/// (for early development, will be called from main())
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

/// Init test environment
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();
    
    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}

pub async fn seed_posts(
    ctx: &Ctx,
    mm: &ModelManager,
    titles: &[&str],
    descriptions: &[&str],
) -> model::Result<Vec<Post>> {
    let mut posts = Vec::new();

    for (title, description) in titles.iter().zip(descriptions.iter()) {
        let id = PostBmc::create(
            ctx, 
            mm, 
            PostForCreate {
                title: title.to_string(),
                description: description.to_string(),
                is_published: None,
                cover_media_url: None,
                thumbnail_url: None,
                media_count: None,
                has_video: None,
            }
        )
        .await?;

        let post = PostBmc::get(ctx, mm, id).await?;
        posts.push(post);
    }
    
    Ok(posts)
}