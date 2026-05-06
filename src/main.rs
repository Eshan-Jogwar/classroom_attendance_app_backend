use crate::{
    server::{
        db::db_functions::{establish_connection, init_db},
        middleware::debugger::debugger_middleware,
        server::{construct_listener, construct_router},
    },
    state::session_state::AppState,
};
use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

pub mod server;
pub mod state;
pub mod utils;

#[tokio::main]
async fn main() {
    let router = construct_router().await;

    let db = establish_connection().await.unwrap();

    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow any website (Lovable, localhost, etc.)
        .allow_methods(Any) // Allow GET, POST, etc.
        .allow_headers(Any);

    let listener = construct_listener().await;

    let app_state = AppState {
        sessions: RwLock::new(HashMap::new()),
        db: db,
    };
    let router = router.with_state(Arc::new(app_state));
    let router = router.layer(cors);
    let router = router.layer(axum::middleware::from_fn(debugger_middleware));
    axum::serve(listener, router).await.unwrap();
}
