use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::appstate::AppState;
use shared::models::messages::{MessageCreateSchema, MessageUserDTO};

#[derive(Deserialize, Serialize, FromRow)]
pub struct Messages {
    pub message_id: Uuid,
    pub chat_id: Uuid,
    pub user_id: Uuid,
    pub message_text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Messages {
    pub async fn get_messages_page_by_chat_id(appstate: &AppState, chat_id: Uuid, page_number: i64) -> Result<Vec<MessageUserDTO>, sqlx::Error> {
        sqlx::query_as(
            r#"SELECT chat_id, users.username, messages.message_text, messages.created_at, messages.updated_at 
                FROM messages JOIN users ON messages.user_id = users.user_id
                WHERE messages.chat_id = $1
                ORDER BY messages.created_at DESC
                LIMIT 50
                OFFSET 50 * ($2 - 1)"#
        )
        .bind(chat_id)
        .bind(page_number)
        .fetch_all(&appstate.db())
        .await
    }

    pub async fn insert(appstate: &AppState, sender_id: Uuid, message_create_schema: &MessageCreateSchema) -> Result<MessageUserDTO, sqlx::Error> {
        sqlx::query_as(
            "WITH new_message AS (
	                INSERT INTO messages
	                (chat_id, user_id, message_text)
	                VALUES ($1, $2, $3)
	                RETURNING chat_id, user_id, message_text, created_at, updated_at
                ) 
                SELECT chat_id, users.username, new_message.message_text, new_message.created_at, new_message.updated_at
                FROM new_message JOIN users 
                ON new_message.user_id = users.user_id"
        )
        .bind(message_create_schema.chat_id)
        .bind(sender_id)
        .bind(&message_create_schema.message_text)
        .fetch_one(&appstate.db())
        .await
    }
}