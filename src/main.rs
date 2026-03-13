use std::sync::Arc;

use crate::{server::server::{construct_listener, construct_router}, state::session_state::AppState};

pub mod server;
pub mod state;

#[tokio::main]
async fn main() {
    let router = construct_router().await;
    let listener = construct_listener().await;
    
    let app_state = AppState {
        sessions: Vec::new(),
    };

    let router = router.with_state(Arc::new(app_state));

    axum::serve(listener, router).await.unwrap();
}
