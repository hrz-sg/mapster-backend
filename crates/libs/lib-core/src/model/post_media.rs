use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use crate::model::{Result, ModelManager};
use sqlx::FromRow;
use modql::field::Fields;

// region: --- PostMedia Types
#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize)]
pub struct PostMedia {
    pub id: i64,
    pub post_id: i64,
    pub media_url: String,
    pub media_type: String,  // "image" or "video"
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_size: Option<i64>,
    pub duration: Option<i32>,  // for videos in seconds
    pub sort_order: i32,  // order in carousel
    pub alt_text: Option<String>,
}

#[derive(Fields, Deserialize)]
pub struct PostMediaForCreate {
    pub post_id: i64,
    pub media_url: String,
    pub media_type: String,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_size: Option<i64>,
    pub duration: Option<i32>,
    pub sort_order: i32,
    pub alt_text: Option<String>,
}

#[derive(Fields, Default, Deserialize)]
pub struct PostMediaForUpdate {
    pub media_url: Option<String>,
    pub media_type: Option<String>,
    pub mime_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_size: Option<i64>,
    pub duration: Option<i32>,
    pub sort_order: Option<i32>,
    pub alt_text: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct PostMediaFilter {
    id: Option<OpValsInt64>,
    post_id: Option<OpValsInt64>,
    media_type: Option<OpValsString>,
    mime_type: Option<OpValsString>,
    sort_order: Option<OpValsInt64>,
}

// endregion: --- PostMedia Types

// region: --- PostMediaBmc
pub struct PostMediaBmc;

impl DbBmc for PostMediaBmc {
    const TABLE: &'static str = "post_media";
}

impl PostMediaBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        post_media_c: PostMediaForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, post_media_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<PostMedia> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx, 
        mm: &ModelManager,
        filters: Option<Vec<PostMediaFilter>>,
        list_options: Option<ListOptions>
    ) -> Result<Vec<PostMedia>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: i64, post_media_u: PostMediaForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, post_media_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }

}