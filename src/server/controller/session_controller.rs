use std::sync::Arc;

use tokio::sync::Mutex;
use axum::{Json, extract::{Query, State}, http::StatusCode, response::IntoResponse};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{server::auth::check_key::verify_admin_key, state::session_state::{SessionState, StateType}, utils::generate_random_string::generate_random_string};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CreateSessionResponse {
    session_id: String
}

#[derive(Deserialize)]
pub struct CreateSessionBody {
    pub token_time_out: u32,
    pub password: String
}

pub async fn create_session(State(app_state): State<StateType>, Json(body): Json<CreateSessionBody>) -> impl IntoResponse {
    
    if !verify_admin_key(&body.password) {
       return (StatusCode::UNAUTHORIZED).into_response(); 
    }
   
    let session_id: String = generate_random_string(); 
    let token: String = generate_random_string(); 

    let expire_at = Utc::now() + Duration::seconds(body.token_time_out.into());
    let session = SessionState {
        session_id: session_id.clone(), 
        attendance_records: Vec::new(), 
        token: token, 
        token_time_out: body.token_time_out,
        expires_at: expire_at
    };

    let shared_session = Arc::new(Mutex::new(session));    

    let mut map_guard = app_state.sessions.write().await;

    map_guard.insert(session_id.clone(), shared_session);

    return (StatusCode::OK, Json(CreateSessionResponse {session_id: session_id})).into_response(); 
}

#[derive(Deserialize)]
pub struct PollQueryParams {
    session_id: String,
}

pub async fn poll(
    State(app_state): State<StateType>,
    Query(params): Query<PollQueryParams>,
) -> impl IntoResponse {
    let session_arc = {
        let guard = app_state.sessions.read().await;
        guard.get(&params.session_id).cloned()
    };

    let arc = match session_arc {
        Some(a) => a,
        None => return (StatusCode::NOT_FOUND, "Not Found").into_response(),
    };

    let mut session = arc.lock().await;

    if chrono::Utc::now() >= session.expires_at {
        session.token = generate_random_string();
        session.expires_at = chrono::Utc::now() + chrono::Duration::seconds(session.token_time_out.into());
    }

    let data_to_return = session.clone();

    (StatusCode::OK, Json(data_to_return)).into_response()
}
