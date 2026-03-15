use axum::{Router, routing::{get, post}};

use crate::{server::controller::student_controller::{get_attendance_token, mark_attendance}, state::session_state::{ApiRouter, StateType}};

pub fn student_create_router() -> ApiRouter {
   let router = Router::<StateType>::new();
   let router = router.route("/attendance_token", get(get_attendance_token));
   let router = router.route("/mark", post(mark_attendance)); 

   router
} 
