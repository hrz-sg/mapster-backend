// region: --- Modules

use lib_core::model::journey::{Journey, JourneyFilter, JourneyForUpdate};
use lib_core::model::post::{Post, PostWithAuthor};
use lib_core::service::journey::{CreateJourneyWithNewPostPayload, CreateJourneyWithPostsPayload, DetachPostPayload, JourneyService};
use lib_rpc::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList, Result};
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_rpc::router::RpcRouter;
use lib_rpc::rpc_router;

// endregion: --- Modules

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		create_journey_from_posts,
        create_with_new_post,
        detach_post_from_journey,
        get_journey,
        get_journey_with_posts,
        update_journey,
        delete_journey,
        list_journey
	)
}

pub async fn create_journey_from_posts(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<CreateJourneyWithPostsPayload>
) -> Result<Journey> {
    let ParamsForCreate { data } = params;

    let journey = JourneyService::create_from_posts(&ctx, &mm, data).await?;

    Ok(journey)
}

pub async fn create_with_new_post(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<CreateJourneyWithNewPostPayload>
) -> Result<(Journey, Post)> {
    let ParamsForCreate { data } = params;

    let journey = JourneyService::create_with_new_post(&ctx, &mm, data).await?;

    Ok(journey)
}

pub async fn detach_post_from_journey(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<DetachPostPayload>
) -> Result<()> {
    let ParamsForCreate { data } = params;

    let _ = JourneyService::detach_post_from_journey(&ctx, &mm, data).await?;

    Ok(())
}

pub async fn get_journey(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<Journey> {
    let ParamsIded { id } = params;

    let journey = JourneyService::get(&ctx, &mm, id).await?;

    Ok(journey)
}

pub async fn get_journey_with_posts(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<(Journey, Vec<PostWithAuthor>)> {
    let ParamsIded { id } = params;

    let journey = JourneyService::get_with_posts(&ctx, &mm, id).await?;

    Ok(journey)
}

pub async fn update_journey(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<JourneyForUpdate>
) -> Result<Journey> {
    let ParamsForUpdate { id, data } = params;

    let journey = JourneyService::update(&ctx, &mm, id, data).await?;

    Ok(journey)
}

pub async fn delete_journey(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<()> {
    let ParamsIded { id} = params;

    let result = JourneyService::delete(&ctx, &mm, id).await?;

    Ok(result)
}

pub async fn list_journey(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<JourneyFilter>
) -> Result<Vec<Journey>> {
    let ParamsList { filters, list_options } = params;

    let journeys = JourneyService::list(&ctx, &mm, filters, list_options).await?;

    Ok(journeys)
}
