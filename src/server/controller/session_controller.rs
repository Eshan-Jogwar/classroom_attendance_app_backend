use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::state::session_state::{SessionState, StateType};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CreateSessionResponse {
    session_id: String
}

pub async fn create_session(State(app_state): State<StateType>) -> impl IntoResponse {
    let mut guard = app_state.sessions.lock().await;
   
    let session_id: String = std::iter::repeat_with(fastrand::alphanumeric)
        .take(16)
        .collect();
    let session = SessionState {session_id: session_id, attendance_records: Vec::new()};
    
    guard.push(session.clone());
    
    return Json(session); 
}
