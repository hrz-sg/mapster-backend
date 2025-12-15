use axum::{
    Router,
    routing::get,
};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_user_follow::{
    api_get_my_followers,
    api_get_user_followers,
    api_get_my_followings,
    api_get_user_followings
};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        // My Profile Followers & Followings
        .route("/api/me/followers", get(api_get_my_followers))
        .route("/api/me/followings", get(api_get_my_followings))
        // Other User Profile Followers & Followings
        .route("/api/users/{user_id}/followers", get(api_get_user_followers))
        .route("/api/users/{user_id}/followings", get(api_get_user_followings))
        .with_state(mm)
}