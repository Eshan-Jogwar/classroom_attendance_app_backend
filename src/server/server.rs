use axum::Router;
use tokio::net::TcpListener;
use std::net::SocketAddr;

use crate::{server::health_router::health_create_router, state::session_state::{StateType}};

pub async fn construct_router() -> Router<StateType> {
    let router = Router::<StateType>::new();
    let router = router.nest("/health", health_create_router());
    return router;
}


pub async fn construct_listener() -> TcpListener {
    // 1. Load the "PORT" variable or default to "8080" if not set
    let port_str = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // 2. Parse the string into a u16 integer
    let port: u16 = port_str
        .parse()
        .expect("PORT environment variable must be a valid number");

    // 3. Construct the address (0.0.0.0 is required for Render/Cloud)
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // 4. Bind and return the listener
    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to port {}", port));

    println!("Listening on {}", addr);
    listener
}
