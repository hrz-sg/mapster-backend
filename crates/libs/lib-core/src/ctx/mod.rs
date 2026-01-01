// region: ---- Modules

mod error;

pub use self::error::{Error, Result};

// endregion: ---- Modules

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: String,
}

// Constructors
impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: "usr_sys_root_0000000000000".to_string() }
    }

    pub fn new(user_id: String) -> Result<Self> {
        if user_id == "usr_sys_root_0000000000000" {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

// Property Accessors
impl Ctx {
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}