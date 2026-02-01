use crate::ctx::Ctx;
use crate::model::user_stats::{UserProfileStats, UserStatsBmc};
use crate::model::{ModelManager, Result};

pub struct UserStatsService;

impl UserStatsService {
    pub async fn get_by_user_id(ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<UserProfileStats> {
        UserStatsBmc::get_by_user_id(ctx, mm, user_id).await
    }
}
