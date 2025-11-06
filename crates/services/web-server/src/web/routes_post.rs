use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{delete, get, patch, post},
};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_post::{
    api_create_post_handler, 
    api_delete_post_handler, 
    api_get_post_handler, 
    api_list_posts_handler,
    api_update_post_handler,
};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/posts", post(api_create_post_handler))
        .route("/api/posts", get(api_list_posts_handler))
        .route("/api/posts/{id}", get(api_get_post_handler))
        .route("/api/posts/{id}", patch(api_update_post_handler))
        .route("/api/posts/{id}", delete(api_delete_post_handler))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .with_state(mm)
}
