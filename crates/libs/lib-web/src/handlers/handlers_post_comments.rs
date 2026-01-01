use modql::filter::{ListOptions, OrderBy, OrderBys};
use crate::error::Result;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use lib_core::{
    ctx::Ctx, model::{ModelManager, post_comment::PostComment}, 
    service::post_comment::PostCommentService
};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Deserialize, Default)]
pub struct GetCommentsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    #[serde(rename = "order_bys")]
    pub order_bys: Option<String>,
}

impl GetCommentsQuery {
    pub fn to_list_options(&self) -> ListOptions {
        ListOptions {
            limit: self.limit,
            offset: self.offset,
            order_bys: self.create_order_bys(),
        }
    }
    
    fn create_order_bys(&self) -> Option<OrderBys> {
        self.order_bys.as_ref().map(|s| {
            // Split string by ","
            let orders: Vec<OrderBy> = s
                .split(',')
                .filter(|part| !part.trim().is_empty())
                .map(|part| OrderBy::from(part.trim()))
                .collect();
            
            // Create OrderBys
            OrderBys::new(orders)
        })
    }
}

#[derive(Deserialize)]
pub struct CreatePostCommentPayload {
    pub text: String,
    pub parent_id: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePostCommentPayload {
    pub text: String,
}

#[derive(Serialize)]
pub struct CreatePostCommentResponse {
    pub success: bool,
    pub comment: PostComment,
}

#[derive(Serialize)]
pub struct DeletePostCommentResponse {
    pub success: bool,
    pub comment_id: String,
}

#[derive(Serialize)]
pub struct UpdatePostCommentResponse {
    pub success: bool,
    pub comment: PostComment,
}

#[derive(Serialize)]
pub struct GetPostCommentResponse {
    pub success: bool,
    pub comment: PostComment,
}

#[derive(Serialize)]
pub struct GetPostCommentsResponse {
    pub success: bool,
    pub comments: Vec<PostComment>,
    pub total: i64,
}

#[derive(Serialize)]
pub struct GetPostCommentReplies {
    pub success: bool,
    pub replies: Vec<PostComment>,
}

pub async fn api_create_comment_handler(
    // ctx: &Ctx,
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
    Json(payload): Json<CreatePostCommentPayload>,
) -> Result<Json<CreatePostCommentResponse>> {
    debug!("{:<12} - api_create_comment_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    let comment = PostCommentService::create(
        &ctx, 
        &mm, 
        &post_id,
        payload.text,
        payload.parent_id // parent id
    ).await?;

    Ok(Json(CreatePostCommentResponse {
        success: true,
        comment,
    }))
}

pub async fn api_get_comment_handler(
    // ctx: &Ctx,
    State(mm): State<ModelManager>,
    Path(comment_id): Path<String>,
) -> Result<Json<GetPostCommentResponse>> {
    debug!("{:<12} - api_get_comment_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    let comment = PostCommentService::get_comment(&ctx, &mm, &comment_id).await?;

    Ok(Json(GetPostCommentResponse {
        success: true,
        comment,
    }))
}

pub async fn api_get_post_comments_handler(
    // ctx: &Ctx,
    State(mm): State<ModelManager>,
    Path(post_id): Path<String>,
    Query(query): Query<GetCommentsQuery>
) -> Result<Json<GetPostCommentsResponse>> {
    debug!("{:<12} - api_get_post_comments_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    let list_options = query.to_list_options();

    let comments = PostCommentService::get_post_comments(&ctx, &mm, &post_id, Some(list_options)).await?;

    let total = PostCommentService::get_count_post_comments(&ctx, &mm, &post_id).await?;

    Ok(Json(GetPostCommentsResponse {
        success: true,
        comments,
        total,
    }))
}

pub async fn api_get_comment_replies_handler(
    // ctx: Ctx,
    State(mm): State<ModelManager>,
    Path(comment_id): Path<String>,
    Query(query): Query<GetCommentsQuery>
) -> Result<Json<GetPostCommentReplies>> {
    debug!("{:<12} - api_get_comment_replies_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    let list_options = query.to_list_options();

    let replies = PostCommentService::get_comment_replies(&ctx, &mm, &comment_id, Some(list_options)).await?;

    Ok(Json(GetPostCommentReplies {
        success: true,
        replies,
    }))
}

pub async fn api_update_comment_handler(
    // ctx: &Ctx,
    State(mm): State<ModelManager>,
    Path(comment_id): Path<String>,
    Json(payload): Json<UpdatePostCommentPayload>,
) -> Result<Json<UpdatePostCommentResponse>> {
    debug!("{:<12} - api_update_comment_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    let comment = PostCommentService::update(
        &ctx, 
        &mm, 
        &comment_id,
        payload.text,
    ).await?;

    Ok(Json(UpdatePostCommentResponse {
        success: true,
        comment,
    }))
}

pub async fn api_delete_comment_handler(
    // ctx: Ctx,
    State(mm): State<ModelManager>,
    Path(comment_id): Path<String>,
) -> Result<Json<DeletePostCommentResponse>> {
    debug!("{:<12} - api_delete_comment_handler", "HANDLER");

    // DEV ONLY!!!
    let ctx = Ctx::root_ctx();

    PostCommentService::delete(&ctx, &mm, &comment_id).await?;

    Ok(Json(DeletePostCommentResponse {
        success: true,
        comment_id,
    }))
}