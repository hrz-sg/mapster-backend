// region:    --- Modules

mod crud_fns;
pub(crate) mod ids;
mod macros;
mod utils;

// -- Flatten hierarchy for user code.
pub use crud_fns::*;
pub use utils::*;

use modql::SIden;
use sea_query::{Iden, IntoIden, TableRef};

// endregion: --- Modules

// region:    --- Consts

const LIST_LIMIT_DEFAULT: i64 = 1000;
const LIST_LIMIT_MAX: i64 = 5000;

// endregion: --- Consts

// region:    --- SeaQuery Idens

#[derive(Iden)]
pub enum CommonIden {
    Id,
    OwnerId,
    UserId,
}

#[derive(Iden)]
pub enum TimestampIden {
    Cid,
    Ctime,
    Mid,
    Mtime,
}

// Types for timesteps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimestampType {
    Full,       // cid, ctime, mid, mtime
    CtimeOnly,  // only ctime
    CtimeMtime, // ctime, mtime
    None,       // no timesteps at all
}

// endregion: --- SeaQuery Idens

/// The DbBmc trait must be implemented for the Bmc struct of an entity.
/// It specifies meta information such as the table name,
/// whether the table has timestamp columns (cid, ctime, mid, mtime), and more as the
/// code evolves.
///
/// Note: This trait should not be confused with the BaseCrudBmc trait, which provides
///       common default CRUD BMC functions for a given Bmc/Entity.
pub trait DbBmc {
    const TABLE: &'static str;

    fn table_ref() -> TableRef {
        TableRef::Table(SIden(Self::TABLE).into_iden())
    }

    /// Specifies that the table for this Bmc has timestamps (cid, ctime, mid, mtime) columns.
    /// This will allow the code to update those as needed.
    /// Type of timestamp fields (only used if has_timestamps() == true).
    /// default: Full
    fn timestamp_fields() -> TimestampType {
        TimestampType::Full // all fields by default
    }

    /// Specifies if the entity table managed by this BMC
    /// has an `owner_id` column that needs to be set on create (by default ctx.user_id).
    ///
    /// default: false
    fn has_owner_id() -> bool {
        false
    }

    /// Specifies if the entity table
    /// has an `user_id` column that needs to be set on create (by default ctx.user_id).
    ///
    /// default: false
    fn has_user_id() -> bool {
        false
    }

    /// Specifies that the table for this Bmc has id column.
    /// This will allow the code to generate id as needed.
    ///
    /// default: true
    fn has_id() -> bool {
        true
    }
}
