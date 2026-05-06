use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash, verify};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

use crate::state::session_state::AppState;

pub fn sign_up(client: &mut Client, username: &str, password: &str) -> Result<String, String> {
    let hashed_pw = hash(password, DEFAULT_COST).map_err(|e| format!("Hashing failed: {}", e))?;

    // Note: Postgres uses $1, $2 instead of ?1, ?2
    match client.execute(
        "INSERT INTO users (username, pw_hash) VALUES ($1, $2)",
        &[&username, &hashed_pw],
    ) {
        Ok(_) => Ok("user created successfully".to_string()),
        Err(e) => Err(format!("Database error (maybe username taken?): {}", e)),
    }
}

pub fn sign_in(db: PgPool, student_id: &str, password: &str) -> Result<String, String> {
    // query_opt returns Option<Row>, perfect for fetching a single user
    let response = sqlx::query!("SELECT * FROM users");

    match response {
        Some(record) => {
            // Extract data from the row by index
            let id: String = record;
            let stored_hash: String = row.get(1);

            let is_valid = verify(password, &stored_hash).unwrap_or(false);

            if is_valid {
                Ok(id)
            } else {
                Err("Invalid password".to_string())
            }
        }
        None => Err("User not found".to_string()),
    }
}

pub async fn sign_up_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Move the blocking database operation to a background thread
    let db_result = tokio::task::spawn_blocking(move || {
        // Lock the Mutex to get &mut Client
        let mut client = app_state.db.lock().unwrap();

        // Call your sign_up function
        sign_up(&mut client, &payload.username, &payload.password)
    })
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Thread execution failed".to_string(),
        )
    })?;

    // Return the appropriate HTTP response
    match db_result {
        Ok(success_msg) => Ok((StatusCode::CREATED, success_msg)), // 201 Created
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),               // 400 Bad Request
    }
}

/// POST /signin
pub async fn sign_in_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Move the blocking database operation to a background thread
    let db_result = tokio::task::spawn_blocking(move || {
        // Lock the Mutex to get &mut Client
        let mut client = app_state.db.lock().unwrap();

        // Call your sign_in function
        sign_in(&mut client, &payload.username, &payload.password)
    })
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Thread execution failed".to_string(),
        )
    })?;

    // Return the appropriate HTTP response
    match db_result {
        Ok(user_id) => {
            // For now, we just return the user_id as a string.
            // Later, this is where you would call create_jwt() and return the token!
            Ok((
                StatusCode::OK,
                format!("Login successful! Your ID: {}", user_id),
            ))
        }
        Err(e) => Err((StatusCode::UNAUTHORIZED, e)), // 401 Unauthorized
    }
}
