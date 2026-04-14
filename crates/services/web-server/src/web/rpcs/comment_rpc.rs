// region: --- Imports

use lib_core::model::comment::{Comment, CommentForCreate, CommentForUpdate};
use lib_core::service::comment::CommentService;
use lib_rpc::{ParamsForCreate, ParamsForUpdate, ParamsIded, Result};
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_rpc::router::RpcRouter;
use lib_rpc::rpc_router;

// endregion: --- Imports

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        create_comment,
        update_comment,
        delete_comment,
        list_comments,
        list_replies
    )
}

pub async fn create_comment(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<CommentForCreate>
) -> Result<Comment> {
    let ParamsForCreate { data } = params;
    
    let result = CommentService::create(&ctx, &mm, data).await?;

    Ok(result)
}

pub async fn update_comment(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<CommentForUpdate>
) -> Result<Comment> {
    let ParamsForUpdate { id, data } = params;
    
    let result = CommentService::update(&ctx, &mm, id, data).await?;

    Ok(result)
}

pub async fn delete_comment(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<()> {
    let ParamsIded { id } = params;
    
    let result = CommentService::delete(&ctx, &mm, id).await?;

    Ok(result)
}

pub async fn list_comments(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<Vec<Comment>> {
    let ParamsIded { id } = params;
    
    let result = CommentService::list_comments(&ctx, &mm, id).await?;

    Ok(result)
}

pub async fn list_replies(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<Vec<Comment>> {
    let ParamsIded { id } = params;
    
    let result = CommentService::list_replies(&ctx, &mm, id).await?;

    Ok(result)
}
