// region: --- Modules

use lib_core::model::journey::{AddPostToJourney, JourneyPostForUpdate};
use lib_core::service::journey_post::JourneyPostService;
use lib_rpc::{ParamsForUpdate, Result};
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_rpc::router::RpcRouter;
use lib_rpc::rpc_router;

// endregion: --- Modules

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		add_post_to_journey_end,
		move_post_position,
	)
}

pub async fn add_post_to_journey_end(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<AddPostToJourney>
) -> Result<()> {
    let ParamsForUpdate { id, data } = params;

    let result = JourneyPostService::add_post_to_journey_end(&ctx, &mm, id, data).await?;

    Ok(result)
}

pub async fn move_post_position(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<JourneyPostForUpdate>
) -> Result<()> {
    let ParamsForUpdate { id, data } = params;

    let result = JourneyPostService::move_post_position(&ctx, &mm, id, data).await?;

    Ok(result)
}
