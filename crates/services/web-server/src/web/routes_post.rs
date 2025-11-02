use axum::{Router, extract::DefaultBodyLimit, routing::post};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_post;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/posts", post(handlers_post::api_create_post_handler))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .with_state(mm)
}
