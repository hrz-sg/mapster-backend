use axum::{
    Router,
    routing::get,
};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_user_profile::{
    api_get_user_profile_by_id,
    api_get_my_profile
};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/profile/me", get(api_get_my_profile))
        .route("/api/profile/{user_id}", get(api_get_user_profile_by_id))
        .with_state(mm)
}