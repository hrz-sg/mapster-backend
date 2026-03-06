// region: -- Modules

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::user::{UserBmc, UserForPreview, UserProfile, UserProfileDetails};
use crate::service::error::Result;
use crate::service::post::PostService;
use crate::service::user_follow::UserFollowService;
use crate::service::user_stats::UserStatsService;
use tracing::debug;

// endregion: -- Modules

pub struct UserProfileService;

impl UserProfileService {
    pub async fn get_user_profile(
        ctx: &Ctx, 
        mm: &ModelManager, 
        user_id: String,
    ) -> Result<UserProfile> {

        debug!("Getting user: {} profile", user_id);

        let viewer_id = ctx.user_id();

        let is_my_profile = viewer_id == user_id;

        // -- Get user
        let user = Self::get_profile_details(ctx, mm, &user_id).await?;

        // -- Get stats
        let stats = UserStatsService::get_by_user_id(ctx, mm, &user_id).await?;

        // -- Get posts
        let posts = PostService::list_user_posts(ctx, mm, user_id.clone()).await?;

        // -- Check is subscribed
        let is_following = if !is_my_profile {
            UserFollowService::is_following(ctx, mm, user_id).await?
        } else {
            false
        };

        Ok(UserProfile {
            user,
            stats,
            posts,
            is_my_profile,
            is_following,
        })
    }

    pub async fn get_my_profile(
        ctx: &Ctx, 
        mm: &ModelManager
    ) -> Result<UserProfile> {

        Self::get_user_profile(ctx, mm, ctx.user_id().to_string()).await
    }

    pub async fn get_preview(
        ctx: &Ctx, 
        mm: &ModelManager, 
        user_id: &str
    ) -> Result<UserForPreview> {
        let preview_data = UserBmc::get_preview(ctx, mm, user_id).await?;
        Ok(preview_data)
    }

    pub async fn get_preview_many(
        ctx: &Ctx, 
        mm: &ModelManager, 
        ids: &[String]
    ) -> Result<Vec<UserForPreview>> {
        let previews = UserBmc::list_by_ids(ctx, mm, ids).await?;
        Ok(previews)
    }

    pub async fn get_profile_details(
        ctx: &Ctx, 
        mm: &ModelManager, 
        user_id: &str
    ) -> Result<UserProfileDetails> {
        let preview_details = UserBmc::get_profile_details(ctx, mm, user_id).await?;
        Ok(preview_details)
    }
}
