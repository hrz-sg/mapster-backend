use axum::{Router, routing::post};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_tokens;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/auth/refresh", post(handlers_tokens::api_refresh_token_handler))
        .with_state(mm)
}
