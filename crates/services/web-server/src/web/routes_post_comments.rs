use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_post_comments::{
    api_create_comment_handler,
    api_get_comment_handler,
    api_get_post_comments_handler,
    api_get_comment_replies_handler,
    api_update_comment_handler,
    api_delete_comment_handler
};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        // Create comment for post
        .route("/api/posts/{post_id}/comments", post(api_create_comment_handler))
        // Get the comment by comment_id
        .route("/api/comments/{comment_id}", get(api_get_comment_handler))
        // Get all the comments for the post
        .route("/api/posts/{post_id}/comments", get(api_get_post_comments_handler))
        // Get replies for the comment
        .route("/api/comments/{comment_id}/replies", get(api_get_comment_replies_handler))
        // Update the comment by id
        .route("/api/comments/{comment_id}", patch(api_update_comment_handler))
        // Delete the comment by id
        .route("/api/comments/{comment_id}", delete(api_delete_comment_handler))
        .with_state(mm)
}
