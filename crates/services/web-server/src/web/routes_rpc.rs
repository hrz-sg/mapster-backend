use axum::{Router, routing::post};
use lib_rpc::router::RpcRouter;
use lib_web::handlers::handlers_rpc::{RpcState, rpc_axum_handler};
use std::sync::Arc;

use crate::web::rpcs::journey_post_rpc;

use super::rpcs::user_rpc;
use super::rpcs::post_rpc;
use super::rpcs::media_rpc;
use super::rpcs::journey_rpc;
use super::rpcs::chat_rpc;

// Build the combined RpcRouter
pub fn create_rpc_router() -> RpcRouter {
    RpcRouter::new()
        .extend(user_rpc::rpc_router())
        .extend(post_rpc::rpc_router())
        .extend(media_rpc::rpc_router())
        .extend(journey_rpc::rpc_router())
        .extend(journey_post_rpc::rpc_router())
        .extend(chat_rpc::rpc_router())
}

// Axum router for '/api/rpc'
pub fn routes(rpc_state: RpcState, rpc_router: Arc<RpcRouter>) -> Router {

	// Build the Axum Router for '/rpc'
	Router::new()
		.route("/rpc", post(rpc_axum_handler))
		.with_state((rpc_state, rpc_router))
}
