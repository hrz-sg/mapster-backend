use crate::ctx::Ctx;
use crate::model::{ModelManager, Result};
use crate::model::user_stats::{UserStatsBmc, UserProfileStats};

pub struct UserStatsService;

impl UserStatsService {
    pub async fn get_by_user_id(ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<UserProfileStats> {
        UserStatsBmc::get_by_user_id(ctx, mm, user_id).await
    }
}