// region: --- Modules

use lib_core::model::user::UserProfile;
use lib_core::service::user_follow::FollowListResponse;
use lib_core::service::user_follow::UserFollowService;
use lib_core::service::user_profile::UserProfileService;
use lib_rpc::ParamsIded;
use lib_rpc::Result;
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_rpc::router::RpcRouter;
use lib_rpc::rpc_router;

// endregion: --- Modules

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		is_following,
		list_followers,
		list_followings,
		get_my_profile,
		get_user_profile,
	)
}

pub async fn get_my_profile(
    ctx: Ctx,
    mm: ModelManager,
) -> Result<UserProfile> {

    let profile = UserProfileService::get_my_profile(&ctx, &mm).await?;

    Ok(profile)
}

pub async fn get_user_profile(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<UserProfile> {
    let ParamsIded { id } = params;

    let profile = UserProfileService::get_user_profile(&ctx, &mm, id).await?;

    Ok(profile)
}

pub async fn is_following(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<bool> {
    let ParamsIded { id } = params;

    let resp = UserFollowService::is_following(&ctx, &mm, id).await?;

    Ok(resp)
}

pub async fn list_followers(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<FollowListResponse> {
    let ParamsIded { id } = params;

    let resp = UserFollowService::list_followers(&ctx, &mm, id).await?;

    Ok(resp)
}

pub async fn list_followings(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<FollowListResponse> {
    let ParamsIded { id } = params;

    let resp = UserFollowService::list_followings(&ctx, &mm, id).await?;

    Ok(resp)
}
