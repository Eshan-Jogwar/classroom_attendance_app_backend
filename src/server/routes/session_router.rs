use axum::{Router, routing::{get, post}};
use crate::{server::controller::session_controller::{create_session, poll}, state::session_state::{ApiRouter, StateType}};

pub fn session_create_router() -> ApiRouter {
   let router = Router::<StateType>::new();
   let router = router.route("/create", post(create_session));
   let router = router.route("/poll", get(poll));
   return router;
}  
