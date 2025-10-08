use axum::{Router, routing::post};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_email;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/register", post(handlers_email::api_registration_handler))
        .route("/api/verify", post(handlers_email::api_verify_email_handler))
        .with_state(mm)
}
