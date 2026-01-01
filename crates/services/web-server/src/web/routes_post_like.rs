use axum::{
    Router,
    routing::{post, get},
};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_post_likes::{
    api_toggle_like_post_handler, 
    api_get_post_likers_handler, 
    api_post_likes_count_handler, 
};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/posts/{id}/like", post(api_toggle_like_post_handler))
        .route("/api/posts/{id}/likes", get(api_post_likes_count_handler))
        .route("/api/posts/{id}/likers", get(api_get_post_likers_handler))
        .with_state(mm)
}
