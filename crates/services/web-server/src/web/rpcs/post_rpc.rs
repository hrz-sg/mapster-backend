// region: --- Modules

use lib_core::model::comment::Comment;
use lib_core::model::comment::CommentForUpdate;
use lib_core::model::post::PostFeedItem;
use lib_core::model::post::PostFilter;
use lib_core::model::post::PostProfileItem;
use lib_core::model::post_forward::PostForwardForCreate;
use lib_core::model::user::UserForPreview;
use lib_core::service::post::CreatePostCommentPayload;
use lib_core::service::post::CreatePostPayload;
use lib_core::service::post::SavePostToCollectionPayload;
use lib_core::service::post::UnsavePostFromCollectionPayload;
use lib_core::service::post::UpdatePostPayload;
use lib_rpc::ParamsForUpdate;
use lib_rpc::ParamsList;
use lib_rpc::Result;
use lib_core::ctx::Ctx;
use lib_core::model::post::PostDetail;
use lib_core::model::ModelManager;
use lib_core::service::post::PostService;
use lib_rpc::ParamsForCreate;
use lib_rpc::ParamsIded;
use lib_rpc::router::RpcRouter;
use lib_rpc::rpc_router;

// endregion: --- Modules

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		create_post_with_media_metadata,
		get_post_detail,
        list_feed_posts,
        list_user_posts,
        create_comment,
        update_comment,
        delete_comment,
        list_comments,
        list_comment_replies,
        toggle_like,
        get_likers,
        get_like_count,
        save_to_collection,
        unsave_from_collection,
        forward_post,
        update_post_with_media_meta,
        delete_post,
	)
}

pub async fn create_post_with_media_metadata(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<CreatePostPayload>
) -> Result<String> {
    let ParamsForCreate { data } = params;

    let post_id = PostService::create_with_media_meta(&ctx, &mm, data).await?;
    
    Ok(post_id)
}

pub async fn get_post_detail(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<PostDetail> {

    let ParamsIded { id} = params;

    let post = PostService::get_post_detail(&ctx, &mm, &id).await?;

    Ok(post)
}

pub async fn list_feed_posts(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<PostFilter>,
) -> Result<Vec<PostFeedItem>> {

    let ParamsList { filters, list_options} = params;

    let posts = PostService::list_feed_posts(&ctx, &mm, filters, list_options).await?;

    Ok(posts)
}

pub async fn list_user_posts(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<Vec<PostProfileItem>> {

    let ParamsIded { id} = params;

    let posts = PostService::list_user_posts(&ctx, &mm, id).await?;

    Ok(posts)
}

pub async fn create_comment(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<CreatePostCommentPayload>
) -> Result<Comment> {
    let ParamsForCreate { data } = params;

    let comment = PostService::create_comment(&ctx, &mm, data).await?;
    
    Ok(comment)
}

pub async fn update_comment(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<CommentForUpdate>
) -> Result<Comment> {
    let ParamsForUpdate { id,  data } = params;

    let comment = PostService::update_comment(&ctx, &mm, id, data).await?;
    
    Ok(comment)
}

pub async fn delete_comment(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<()> {
    let ParamsIded { id } = params;

    let _ = PostService::delete_comment(&ctx, &mm, id).await?;
    
    Ok(())
}

pub async fn list_comments(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<Vec<Comment>> {

    let ParamsIded { id } = params;

    let comments = PostService::list_comments(&ctx, &mm, id).await?;

    Ok(comments)
}

pub async fn list_comment_replies(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<Vec<Comment>> {

    let ParamsIded { id } = params;

    let comments = PostService::list_comments(&ctx, &mm, id).await?;

    Ok(comments)
}

pub async fn toggle_like(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<(bool, i64)> {

    let ParamsIded { id } = params;

    let result = PostService::toggle_like(&ctx, &mm, id).await?;

    Ok(result)
}

pub async fn get_likers(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<Vec<UserForPreview>> {

    let ParamsIded { id } = params;

    let likers = PostService::get_likers(&ctx, &mm, id).await?;

    Ok(likers)
}

pub async fn get_like_count(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<i64> {

    let ParamsIded { id } = params;

    let likes = PostService::get_like_count(&ctx, &mm, id).await?;

    Ok(likes)
}

pub async fn save_to_collection(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<SavePostToCollectionPayload>,
) -> Result<String> {

    let ParamsForCreate { data } = params;

    let collection_id = PostService::save_to_collection(&ctx, &mm, data).await?;

    Ok(collection_id)
}

pub async fn unsave_from_collection(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<UnsavePostFromCollectionPayload>,
) -> Result<()> {

    let ParamsForCreate { data } = params;

    let result = PostService::unsave_from_collection(&ctx, &mm, data).await?;

    Ok(result)
}

pub async fn forward_post(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<PostForwardForCreate>,
) -> Result<()> {

    let ParamsForCreate { data } = params;

    let result = PostService::forward_post(&ctx, &mm, data).await?;

    Ok(result)
}

pub async fn update_post_with_media_meta(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<UpdatePostPayload>,
) -> Result<()> {

    let ParamsForUpdate { id, data } = params;

    let result = PostService::update_post_with_media_meta(&ctx, &mm, id, data).await?;

    Ok(result)
}

pub async fn delete_post(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<()> {

    let ParamsIded { id } = params;

    let result = PostService::delete_post(&ctx, &mm, id).await?;

    Ok(result)
}