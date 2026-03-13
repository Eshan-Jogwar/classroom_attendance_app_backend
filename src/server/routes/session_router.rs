use axum::Router;
use crate::state::session_state::{ApiRouter, StateType};

pub fn session_create_router() -> ApiRouter {
   let router = Router::<StateType>::new();
   return router;
}  
