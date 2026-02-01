// region: -- Modules
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::post::PostFeedItem;
use crate::model::user::{UserBmc, UserForPreview, UserProfileDetails};
use crate::model::user_stats::UserProfileStats;
use crate::service::error::Result;
use crate::service::post::PostService;
use crate::service::user_follow::UserFollowService;
use crate::service::user_stats::UserStatsService;
use serde::Serialize;
// endregion: -- Modules

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub user: UserProfileDetails,
    pub stats: UserProfileStats,
    pub posts: Vec<PostFeedItem>,

    pub is_my_profile: bool,
    pub is_following: bool,
}

pub struct UserProfileService;

impl UserProfileService {
    pub async fn get_user_profile(ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<UserProfile> {
        let viewer_id = ctx.user_id();

        let is_my_profile = viewer_id == user_id;

        // -- Get user
        let user = Self::get_profile_details(ctx, mm, user_id).await?;

        // -- Get stats
        let stats = UserStatsService::get_by_user_id(ctx, mm, user_id).await?;

        // // -- Get posts
        let posts = if is_my_profile {
            PostService::list_user_posts(ctx, mm, viewer_id).await?
        } else {
            PostService::list_user_posts(ctx, mm, user_id).await?
        };

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

    pub async fn get_my_profile(ctx: &Ctx, mm: &ModelManager) -> Result<UserProfile> {
        Self::get_user_profile(ctx, mm, ctx.user_id()).await
    }

    pub(crate) async fn _get_preview(ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<UserForPreview> {
        let preview_data = UserBmc::get_preview(ctx, mm, user_id).await?;
        Ok(preview_data)
    }

    pub(crate) async fn _get_preview_many(ctx: &Ctx, mm: &ModelManager, ids: &[String]) -> Result<Vec<UserForPreview>> {
        let previews = UserBmc::list_by_ids(ctx, mm, ids).await?;
        Ok(previews)
    }

    pub(crate) async fn get_profile_details(ctx: &Ctx, mm: &ModelManager, user_id: &str) -> Result<UserProfileDetails> {
        let preview_details = UserBmc::get_profile_details(ctx, mm, user_id).await?;
        Ok(preview_details)
    }
}
