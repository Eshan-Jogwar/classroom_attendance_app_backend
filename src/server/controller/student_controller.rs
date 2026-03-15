use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::{state::session_state::{AttendanceStruct, StateType}, utils::generate_random_string::generate_random_string};

pub async fn get_attendance_token() -> String {
    generate_random_string()
}

#[derive(Deserialize)]
pub struct MarkAttendanceBody {
    pub id: String,
    pub name: String,
    pub attendance_token: String,
    pub session_token: String,
    pub session_id: String
}

pub async fn mark_attendance(
    State(app_state): State<StateType>, 
    Json(body): Json<MarkAttendanceBody>
) -> impl IntoResponse {
    let session_arc = {
        let guard = app_state.sessions.read().await;
        guard.get(&body.session_id).cloned()
    };

    let arc = match session_arc {
        Some(a) => a,
        None => return (StatusCode::NOT_FOUND, "Session not found").into_response(),
    };

    let mut session = arc.lock().await;

    if session.token != body.session_token {
        return (StatusCode::UNAUTHORIZED, "Invalid or expired token").into_response();
    }

    let attendance = AttendanceStruct {
        id: body.id,
        name: body.name,
        timestamp: chrono::Utc::now(),
        attendance_token: body.attendance_token
    };

    session.attendance_records.push(attendance);

    (StatusCode::OK, "Attendance marked").into_response()
}
