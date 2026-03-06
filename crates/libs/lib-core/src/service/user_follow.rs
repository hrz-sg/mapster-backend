// region: --- Modules

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::user::UserForPreview;
use crate::model::user_follow::UserFollowBmc;
use crate::service::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// endregion: --- Modules

pub struct UserFollowService;

impl UserFollowService {
    // --- Check if current user follows another user
    pub async fn is_following(
        ctx: &Ctx, 
        mm: &ModelManager, 
        following_id: String
    ) -> Result<bool> {
        let is_following = UserFollowBmc::is_following(ctx, mm, &following_id).await?;
        Ok(is_following)
    }

    // --- Get list of followers
    pub async fn list_followers(
        ctx: &Ctx,
        mm: &ModelManager,
        user_id: String,
    ) -> Result<FollowListResponse> {
        let viewer_id = ctx.user_id();

        let followers = UserFollowBmc::list_followers(ctx, mm, &user_id).await?;

        let total = UserFollowBmc::count_follows(ctx, mm, &user_id).await?;

        let users_id: Vec<&str> = followers.iter().map(|u| u.id.as_str()).collect();

        let relations = UserFollowBmc::follow_relations(ctx, mm, viewer_id, &users_id).await?;

        let relation_set: HashSet<(String, String)> = relations.into_iter().collect();

        let users = followers
            .into_iter()
            .map(|user| {
                let is_following = relation_set.contains(&(viewer_id.to_string(), user.id.clone()));
                let is_followed_by = relation_set.contains(&(user.id.clone(), viewer_id.to_string()));

                FollowListItem {
                    user,
                    is_following,
                    is_followed_by,
                }
            })
            .collect();

        Ok(FollowListResponse { total, users })
    }

    // --- Get list of followings
    pub async fn list_followings(
        ctx: &Ctx,
        mm: &ModelManager,
        user_id: String,
    ) -> Result<FollowListResponse> {
        let viewer_id = ctx.user_id();

        let followings = UserFollowBmc::list_followings(ctx, mm, &user_id).await?;

        let total = UserFollowBmc::count_follows(ctx, mm, &user_id).await?;

        let users_id: Vec<&str> = followings.iter().map(|u| u.id.as_str()).collect();

        let relations = UserFollowBmc::follow_relations(ctx, mm, viewer_id, &users_id).await?;
        let relation_set: HashSet<(String, String)> = relations.into_iter().collect();

        let users = followings
            .into_iter()
            .map(|user| {
                let is_following = relation_set.contains(&(viewer_id.to_string(), user.id.clone()));
                let is_followed_by = relation_set.contains(&(user.id.clone(), viewer_id.to_string()));

                FollowListItem {
                    user,
                    is_following,
                    is_followed_by,
                }
            })
            .collect();

        Ok(FollowListResponse { total, users })
    }
}

// region: -- Follow List Reques & Response structs
#[derive(Debug, Serialize, Deserialize)]
pub struct FollowListItem {
    pub user: UserForPreview,
    pub is_following: bool,
    pub is_followed_by: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FollowListResponse {
    pub total: i64,
    pub users: Vec<FollowListItem>,
}
// endregion: -- Follow List Reques & Response structs
