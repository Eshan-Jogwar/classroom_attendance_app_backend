use axum::{Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};

use crate::state::session_state::StateType;

// The database health check handler
async fn db_health_handler(State(state): State<StateType>) -> impl IntoResponse {
    // Ping the database with a simple query
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => {
            // DB is reachable
            (StatusCode::OK, "Database connection is healthy".to_string())
        }
        Err(e) => {
            // DB is unreachable
            // In a production app, you might want to log this error using `tracing::error!`
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection failed: {}", e),
            )
        }
    }
}

pub fn health_create_router() -> Router<StateType> {
    let router = Router::<StateType>::new()
        .route(
            "/",
            get(|| async {
                return "ok".to_string();
            }),
        )
        .route("/db", get(db_health_handler));

    return router;
}
