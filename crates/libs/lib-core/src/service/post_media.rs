use modql::filter::{OpValString, OpValsString};

use crate::ctx::Ctx;
use crate::model::post::PostMediaFilter;
use crate::model::post::post_media::PostMediaBmc;
use crate::model::{ModelManager, Result};

pub struct PostMediaService;

impl PostMediaService {
    pub async fn delete_many(
        ctx: &Ctx, 
        mm: &ModelManager, 
        ids: &[String]
    ) -> Result<()> {

        let id_filter = OpValsString(ids.iter().map(|id| OpValString::Eq(id.clone())).collect());
        let filter = PostMediaFilter {
            id: Some(id_filter),
            ..Default::default()
        };

        let medias = PostMediaBmc::list(ctx, mm, Some(vec![filter]), None).await?;
        let object_keys: Vec<&str> = medias.iter().map(|m| m.object_key.as_str()).collect();

        if !object_keys.is_empty() {
            mm.bucket().delete_many(&object_keys).await?;
        }

        let ids: Vec<&str> = medias.iter().map(|m| m.id.as_str()).collect();
        PostMediaBmc::delete_many(ctx, mm, ids).await?;

        Ok(())
    }

    pub async fn delete_one(
        ctx: &Ctx, 
        mm: &ModelManager, 
        media_id: String
    ) -> Result<()> {
        let media = PostMediaBmc::get(ctx, mm, &media_id).await?;

        mm.bucket().delete(&media.object_key).await?;

        PostMediaBmc::delete(ctx, mm, &media_id).await
    }

    pub async fn next_sort(
        ctx: &Ctx, 
        mm: &ModelManager, 
        post_id: &str
    ) -> Result<i32> {
        let medias = PostMediaBmc::list_by_post(ctx, mm, post_id).await?;
        Ok(medias.iter().map(|m| m.sort_order).max().unwrap_or(-1) + 1)
    }
}
