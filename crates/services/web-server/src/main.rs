// region:   --- Modules
mod config;
mod error;
mod web;
pub use self::error::{Error, Result};
use axum::response::Html;
use config::web_config;
use lib_web::handlers::mw_req_stamp::mw_req_stamp_resolver;
use lib_web::middleware::mw_auth::mw_ctx_resolver;
use std::net::SocketAddr;
// use lib_web::middleware::mw_res_map::mw_reponse_map;
use crate::web::{routes_auth, routes_email, routes_post, routes_token, routes_user_follow, routes_user_profile};
use axum::routing::get;
use axum::{Router, middleware};
use lib_core::_dev_utils;
use lib_core::model::ModelManager;
use lib_web::routes::routes_static;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing_subscriber::EnvFilter;

// endregion:   --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    // -- Load env
    dotenvy::dotenv().ok();

    // -- Tracing
    tracing_subscriber::fmt()
        .without_time() // For early local development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    // Initialize ModelManager
    let mm = ModelManager::new().await?;

    let routes_hello = Router::new().route("/hello", get(|| async { Html("Hello world") }));
    // .route_layer(middleware::from_fn(mw_ctx_require));

    // -- Define Routes
    let routes_all = Router::new()
        .merge(routes_auth::routes(mm.clone()))
        .merge(routes_email::routes(mm.clone()))
        .merge(routes_post::routes(mm.clone()))
        .merge(routes_user_profile::routes(mm.clone()))
        .merge(routes_user_follow::routes(mm.clone()))
        .merge(routes_token::routes(mm.clone()))
        .merge(routes_hello)
        // .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .layer(middleware::from_fn(mw_req_stamp_resolver))
        .fallback_service(routes_static::serve_dir(&web_config().WEB_FOLDER));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("{:12} - {addr}\n", "LISTENING");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all.into_make_service()).await.unwrap();

    Ok(())
}
