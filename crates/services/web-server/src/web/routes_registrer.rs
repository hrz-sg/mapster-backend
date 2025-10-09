use axum::{Router, routing::post};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_register;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/register", post(handlers_register::api_registration_handler))
        .with_state(mm)
}
