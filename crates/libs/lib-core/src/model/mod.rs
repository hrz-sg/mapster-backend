// region: ---- Modules
mod base;
mod error;
mod modql_utils;
pub(crate) mod store;

pub mod user;
pub use user::user_follow;
pub use user::user_stats;

pub mod post;
pub use post::post_collection;
pub use post::post_collection_item;
pub use post::post_forward;
pub use post::post_like;
pub use post::post_media;

pub mod comment;

pub mod journey;
pub use journey::journey_collection;
pub use journey::journey_collection_item;
pub use journey::journey_forward;
pub use journey::journey_post;

pub mod chat;

pub use self::error::{Error, Result};
use crate::model::store::new_oss_bucket;
use crate::model::store::oss::Bucket;
use crate::model::store::{dbx::Dbx, new_db_pool};

// endregion: ---- Modules

#[derive(Clone)]
pub struct ModelManager {
    dbx: Dbx,
    bucket: Bucket,
}

// Constructor
impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db_pool = new_db_pool()
            .await
            .map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;

        let dbx = Dbx::new(db_pool, false)?;

        let bucket = new_oss_bucket();

        Ok(ModelManager { dbx, bucket })
    }

    pub fn new_with_txn(&self) -> Result<ModelManager> {
        let dbx = Dbx::new(self.dbx.db().clone(), true)?;

        let bucket = self.bucket.clone();

        Ok(ModelManager { dbx, bucket })
    }

    pub fn dbx(&self) -> &Dbx {
        &self.dbx
    }

    pub fn bucket(&self) -> &Bucket {
        &self.bucket
    }
}
