use std::sync::{Arc};
use tokio::sync::Mutex;
use axum::Router;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AttendanceStruct {
    pub id: String,
    pub name: String,
    pub timestamp: DateTime<Utc> 
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionState{
    pub session_id: String,
    pub attendance_records: Vec<AttendanceStruct> 
}

pub struct AppState {
    pub sessions: Mutex<Vec<SessionState>> 
}

pub type StateType = Arc<AppState>;
pub type ApiRouter = Router<StateType>;
