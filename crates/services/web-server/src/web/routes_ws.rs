use axum::{routing::get, Router};
use lib_core::model::ModelManager;
use lib_rpc::router::RpcRouter;
use lib_web::handlers::handlers_ws;
use std::sync::Arc;

pub fn routes(mm: ModelManager, rpc_router: Arc<RpcRouter>) -> Router {
    Router::new()
        .route("/chat", get(handlers_ws::handler))
        .with_state((mm, rpc_router))
}