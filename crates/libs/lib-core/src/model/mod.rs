// region: ---- Modules

mod base;
mod error;
mod store;
pub mod post;
pub mod user;
pub mod modql_utils;

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
		let db_pool = new_db_pool()
			.await
			.map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;
		let dbx = Dbx::new(db_pool, false)?;
		Ok(ModelManager { dbx })
	}

	pub fn new_with_txn(&self) -> Result<ModelManager> {
		let dbx = Dbx::new(self.dbx.db().clone(), true)?;
		Ok(ModelManager { dbx })
	}

	pub fn dbx(&self) -> &Dbx {
		&self.dbx
	}
}
