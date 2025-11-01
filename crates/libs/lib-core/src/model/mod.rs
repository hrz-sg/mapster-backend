// region: ---- Modules

mod base;
mod error;
mod store;
mod modql_utils;

pub mod post;
pub mod post_media;
pub mod user;

use crate::model::store::{dbx::Dbx, new_db_pool};
pub use self::error::{Error, Result};

// endregion: ---- Modules

#[derive(Clone)]
pub struct ModelManager {
    dbx: Dbx,
}

// Constructor
impl ModelManager {
    /// Constructor
	pub async fn new() -> Result<Self> {
        println!("DEBUG: ModelManager::new called");
        let db_pool = new_db_pool()
            .await
            .map_err(|ex| {
                println!("DEBUG: Failed to create db pool: {}", ex);
                Error::CantCreateModelManagerProvider(ex.to_string())
            })?;
        println!("DEBUG: Db pool created successfully");
        let dbx = Dbx::new(db_pool, false)?; // Пока используйте false
        println!("DEBUG: Dbx created successfully");
        Ok(ModelManager { dbx })
    }

    pub fn new_with_txn(&self) -> Result<ModelManager> {
        println!("DEBUG: new_with_txn called");
        let dbx = Dbx::new(self.dbx.db().clone(), true)?;
        Ok(ModelManager { dbx })
    }

	pub fn dbx(&self) -> &Dbx {
		&self.dbx
	}
}
