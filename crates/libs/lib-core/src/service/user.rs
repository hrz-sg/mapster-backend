use crate::ctx::Ctx;
use crate::model::{ModelManager, Result};
use crate::model::user::{UserBmc, UserForPreview, UserProfileDetails};

pub struct UserService;

impl UserService {
    pub async fn _get_preview(ctx: &Ctx, mm: &ModelManager, user_id: i64) -> Result<UserForPreview> {
        UserBmc::get_preview(ctx, mm, user_id).await
    }

    pub async fn get_profile_details(ctx: &Ctx, mm: &ModelManager, user_id: i64) -> Result<UserProfileDetails> {
        UserBmc::get_profile_details(ctx, mm, user_id).await
    }
}
