use axum::Router;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

#[derive(Serialize, Deserialize, Clone)]
pub struct AttendanceStruct {
    pub id: String,
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub attendance_token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionState {
    pub session_id: String,
    pub attendance_records: Vec<AttendanceStruct>,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub token_time_out: u32,
}

pub struct AppState {
    pub sessions: RwLock<HashMap<String, Arc<Mutex<SessionState>>>>,
    pub db: PgPool,
}

pub type StateType = Arc<AppState>;
pub type ApiRouter = Router<StateType>;
