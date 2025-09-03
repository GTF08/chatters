use axum::{middleware, routing::{any, get, post}, Router};


use common::{appstate::AppState, middlewares::auth_guard};
use crate::handlers::{chats::{
    create_chat_handler, delete_chat_handler, get_user_chats, join_chat_handler, leave_chat_handler
}, messages::websocket_handler};

pub fn routes(appstate: AppState) -> Router {
    Router::new()
        .route(
            "/chats",
            get(get_user_chats).
            post(create_chat_handler)
            .delete(delete_chat_handler)
        )
        .route("/chats/{chat_id}/join", get(join_chat_handler))
        .route("/chats/{chat_id}/leave", get(leave_chat_handler))
        //.route("/messages/{chat_id}", get(get_chat_messages))
        //.route("/chats/join", method_router)
        .route("/ws", any(websocket_handler))
        .with_state(appstate.clone())
        
        .route_layer(middleware::from_fn_with_state(appstate, auth_guard))
}