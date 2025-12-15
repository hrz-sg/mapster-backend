use std::collections::HashSet;

use serde::Serialize;

use crate::ctx::Ctx;
use crate::model::user::UserForPreview;
use crate::model::user_follow::UserFollowBmc;
use crate::model::{ModelManager, Result};

pub struct UserFollowService;

#[derive(Debug, Serialize)]
pub struct FollowListItem {
    pub user: UserForPreview,
    pub is_following: bool,
    pub is_followed_by: bool,
}

pub struct FollowListResult {
    pub total: i64,
    pub users: Vec<FollowListItem>,
}

impl UserFollowService {
    pub async fn is_following(
        ctx: &Ctx,
        mm: &ModelManager,
        following_id: i64,
    ) -> Result<bool> {
        let viewer_id = ctx.user_id();
        UserFollowBmc::is_following(ctx, mm, viewer_id, following_id).await
    }
    
    pub async fn list_followers(
        ctx: &Ctx,
        mm: &ModelManager,
        target_user_id: Option<i64>,
    ) -> Result<FollowListResult> {

        let viewer_id = ctx.user_id();
        let user_id = target_user_id.unwrap_or(viewer_id);

        let followers = UserFollowBmc::list_followers(ctx, mm, user_id).await?;

        let total = UserFollowBmc::count_follows(ctx, mm, user_id).await?;

        let user_ids: Vec<i64> = followers.iter().map(|u| u.id).collect();

        let relations =
            UserFollowBmc::follow_relations(ctx, mm, viewer_id, &user_ids).await?;

        let relation_set: HashSet<(i64, i64)> =
            relations.into_iter().collect();

        let users = followers
            .into_iter()
            .map(|user| {
                let is_following =
                    relation_set.contains(&(viewer_id, user.id));
                let is_followed_by =
                    relation_set.contains(&(user.id, viewer_id));

                FollowListItem {
                    user,
                    is_following,
                    is_followed_by,
                }
            })
            .collect();

        Ok(FollowListResult { total, users })
    }

    pub async fn list_followings(
        ctx: &Ctx,
        mm: &ModelManager,
        target_user_id: Option<i64>,
    ) -> Result<FollowListResult> {

        let viewer_id = ctx.user_id();
        let user_id = target_user_id.unwrap_or(viewer_id);

        let followings =
            UserFollowBmc::list_followings(ctx, mm, user_id).await?;

        let total =
            UserFollowBmc::count_follows(ctx, mm, user_id).await?;

        let user_ids: Vec<i64> = followings.iter().map(|u| u.id).collect();

        let relations =
            UserFollowBmc::follow_relations(ctx, mm, viewer_id, &user_ids).await?;

        let relation_set: HashSet<(i64, i64)> =
            relations.into_iter().collect();

        let users = followings
            .into_iter()
            .map(|user| {
                let is_following =
                    relation_set.contains(&(viewer_id, user.id));
                let is_followed_by =
                    relation_set.contains(&(user.id, viewer_id));

                FollowListItem {
                    user,
                    is_following,
                    is_followed_by,
                }
            })
            .collect();

        Ok(FollowListResult { total, users })
    }

}
