use axum::{Router, routing::get};

use crate::state::session_state::{StateType};

pub fn health_create_router() -> Router<StateType> {
   let router = Router::<StateType>::new().route("/", get(|| async {
       return "ok".to_string();
   }));

   return router;
}
