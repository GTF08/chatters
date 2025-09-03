use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json};

use common::{
    appstate::AppState, 
    middlewares::CurrentUser, 
    models::{chat::Chats, messages::Messages}
};
use shared::models::chats::{CreateChatSchema, ChatIDSchema};
use uuid::Uuid;

pub async fn create_chat_handler(
    Extension(current_user): Extension<CurrentUser>,
    State(database_state): State<AppState>,
    Json(payload) : Json<CreateChatSchema>
) -> Result<impl IntoResponse, StatusCode> {

    let created_chat_id = 
        Chats::insert(&database_state, payload.name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(
        (StatusCode::CREATED, Json(created_chat_id)).into_response()
    )
}

pub async fn delete_chat_handler(
    Extension(current_user): Extension<CurrentUser>,
    State(database_state): State<AppState>,
    Json(payload) : Json<ChatIDSchema>
) -> Result<impl IntoResponse, StatusCode> {
    
     
    Chats::delete(&database_state, payload.chat_id)
        .await
        .map_err(|e| {println!("{}", e); StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(
        StatusCode::NO_CONTENT
    )
}

pub async fn join_chat_handler(
    Extension(current_user): Extension<CurrentUser>,
    State(database_state): State<AppState>,
    Path(chat_id) : Path<Uuid>
) -> Result<(), StatusCode> {
    Chats::join_user(&database_state, chat_id, current_user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

pub async fn leave_chat_handler(
    Extension(current_user): Extension<CurrentUser>,
    State(database_state): State<AppState>,
    Path(chat_id) : Path<Uuid>
) -> Result<(), StatusCode> {
    Chats::leave_user(&database_state, chat_id, current_user.id)
        .await
        .map_err(|e| {StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(())
}

pub async fn get_user_chats(
    Extension(current_user): Extension<CurrentUser>,
    State(database_state): State<AppState>,
) -> Result<Json<Vec<ChatIDSchema>>, StatusCode> {
    Chats::get_user_chats(&database_state, current_user.id)
        .await
        .map(|value| Json(value))
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)
}