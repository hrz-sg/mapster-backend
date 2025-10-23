use axum::{Router, routing::get};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_email;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/verify", get(handlers_email::api_verify_email_handler))
        .with_state(mm)
}
