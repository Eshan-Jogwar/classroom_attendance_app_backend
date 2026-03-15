use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock};
use crate::{server::{middleware::debugger::{debugger_middleware}, server::{construct_listener, construct_router}}, state::session_state::AppState};

pub mod server;
pub mod state;
pub mod utils;


#[tokio::main]
async fn main() {
    let router = construct_router().await;
    let listener = construct_listener().await;
    
    let app_state = AppState {
        sessions: RwLock::new(HashMap::new()),
    };
    let router = router.with_state(Arc::new(app_state));
    let router = router.layer(axum::middleware::from_fn(debugger_middleware));
    axum::serve(listener, router).await.unwrap();
}
