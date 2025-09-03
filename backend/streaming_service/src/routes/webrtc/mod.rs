use axum::{middleware, routing::{any, get, post}, Router};


use common::{appstate::AppState, middlewares::auth_guard};
use crate::handlers::webrtc::webrtc_answer;

pub fn routes(appstate: AppState) -> Router {
    Router::new()
        //.route("/messages/{chat_id}", get(get_chat_messages))
        //.route("/chats/join", method_router)
        .route("/ws", any(webrtc_answer))
        .with_state(appstate.clone())
        
        .route_layer(middleware::from_fn_with_state(appstate, auth_guard))
}