use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{delete, get, patch, post},
};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_post::{
    api_create_comment_handler, api_create_post_handler, api_delete_comment_handler, api_delete_post_handler,
    api_get_comment_replies_handler, api_get_post_comments_handler, api_get_post_detail_by_id,
    api_get_post_likers_handler, api_list_feed_posts_handler, api_post_likes_count_handler,
    api_toggle_like_post_handler, api_update_comment_handler, api_update_post_handler,
};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .nest(
            "/api/posts",
            Router::new()
                .route("/", post(api_create_post_handler))
                .route("/feed", get(api_list_feed_posts_handler))
                .route("/{id}", get(api_get_post_detail_by_id))
                .route("/{id}", patch(api_update_post_handler))
                .route("/{id}", delete(api_delete_post_handler))
                .route("/{id}/like", post(api_toggle_like_post_handler))
                .route("/{id}/likes", get(api_post_likes_count_handler))
                .route("/{id}/likers", get(api_get_post_likers_handler))
                .route("/{post_id}/comments", post(api_create_comment_handler))
                .route("/{post_id}/comments", get(api_get_post_comments_handler)),
        )
        .route(
            "/api/comments/{comment_id}/replies",
            get(api_get_comment_replies_handler),
        )
        .route("/api/comments/{comment_id}", patch(api_update_comment_handler))
        .route("/api/comments/{comment_id}", delete(api_delete_comment_handler))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .with_state(mm)
}
