use axum::Router;
use axum::routing::post;
use lib_core::model::ModelManager;
use lib_web::handlers::{handlers_auth, handlers_tokens};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/register", post(handlers_auth::api_registration_handler))
        .route("/api/login", post(handlers_auth::api_login_handler))
        .route("/api/logout", post(handlers_auth::api_logout_handler))
        .route("/api/auth/refresh", post(handlers_tokens::api_refresh_token_handler))
        .with_state(mm)
}
