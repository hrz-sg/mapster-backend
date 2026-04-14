// region: ---- Imports

use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::{ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValString, OpValsInt64, OpValsString};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
// endregion: ---- Imports

// region: --- PostMedia Types

#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "media_type")]
pub enum MediaType {
    Image,
    Video,
}

// Covert custom MediaType into sea_query::Value
impl From<MediaType> for sea_query::Value {
    fn from(val: MediaType) -> Self {
        val.to_string().into()
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize)]
pub struct PostMedia {
    pub id: String,
    pub post_id: String,
    pub object_key: String,
    pub media_type: MediaType, // "image" or "video"
    pub mime_type: String,
    pub sort_order: i32, // order in carousel
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize)]
pub struct PostMediaForDisplay {
    pub id: String,
    pub post_id: String,
    pub url: String,
    pub media_type: MediaType, // "image" or "video"
    pub mime_type: String,
    pub sort_order: i32, // order in carousel
}

#[derive(Debug, Fields, Deserialize)]
pub struct PostMediaForCreate {
    pub post_id: String,
    pub object_key: String,
    #[field(cast_as = "media_type")]
    pub media_type: MediaType,
    pub mime_type: String,
    pub etag: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_size: i64,
    pub duration: Option<i32>,
    pub sort_order: i32,
}

#[derive(Fields, Default, Deserialize)]
pub struct PostMediaForUpdate {
    pub object_key: Option<String>,
    pub media_type: Option<String>,
    pub mime_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_size: Option<i64>,
    pub duration: Option<i32>,
    pub sort_order: Option<i32>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct PostMediaFilter {
    pub id: Option<OpValsString>,
    pub post_id: Option<OpValsString>,
    pub media_type: Option<OpValsString>,
    pub mime_type: Option<OpValsString>,
    pub sort_order: Option<OpValsInt64>,
}

#[derive(Iden)]
enum PostMediaIden {
    PostId,
}

// endregion: --- PostMedia Types

// region: --- PostMediaBmc
pub struct PostMediaBmc;

impl DbBmc for PostMediaBmc {
    const TABLE: &'static str = "post_media";
}

impl PostMediaBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, post_media_c: PostMediaForCreate) -> Result<String> {
        base::create::<Self, _>(ctx, mm, post_media_c).await
    }

    pub async fn create_many(ctx: &Ctx, mm: &ModelManager, post_medias_c: Vec<PostMediaForCreate>) -> Result<Vec<String>> {
        base::create_many::<Self, _>(ctx, mm, post_medias_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<PostMedia> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<PostMediaFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<PostMedia>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn list_by_post(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<Vec<PostMedia>> {
        // -- Build query
        let filter = PostMediaFilter {
            post_id: Some(OpValsString(vec![OpValString::Eq(post_id.to_string())])),
            ..Default::default()
        };

        base::list::<Self, _, _>(ctx, mm, Some(vec![filter]), None).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: &str, post_media_u: PostMediaForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, post_media_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
    
    pub async fn delete_many(ctx: &Ctx, mm: &ModelManager, ids: Vec<&str>) -> Result<u64> {
        base::delete_many::<Self>(ctx, mm, ids).await
    }

    pub async fn delete_by_post(_ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<()> {
        // -- Build query
        let mut query = Query::delete();
        query
            .from_table(Self::table_ref())
            .and_where(Expr::col(PostMediaIden::PostId).eq(post_id));

        // -- Execute query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_with(&sql, values);
        mm.dbx().execute(sqlx_query).await?;

        Ok(())
    }

    pub async fn count(ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<i64> {
        base::count::<Self, _>(
            ctx, 
            mm, 
            Some(PostMediaFilter { 
                post_id: Some(OpValsString(vec![OpValString::Eq(id.to_string())])),
            ..Default::default()
        })).await
    }
}

// endregion: --- PostMediaBmc
