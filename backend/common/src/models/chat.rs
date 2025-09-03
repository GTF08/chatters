use serde::Deserialize;
use shared::models::{chats::ChatIDSchema, users::FilteredUser};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::appstate::AppState;

pub struct Chats {
    pub chat_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

//Chats Users Many-to-Many
pub struct ChatsUsers{
    pub chat_id: Uuid,
    pub user_ud: Uuid
}

impl Chats {
    pub async fn insert(appstate: &AppState, chat_name: String) -> Result<Uuid, sqlx::Error> {
        sqlx::query_scalar(
            "INSERT INTO chats 
            (name) VALUES
            ($1)
            RETURNING chat_id")
            .bind(chat_name)
            .fetch_one(&appstate.db())
            .await
    }

    pub async fn delete(appstate: &AppState, chat_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM chats
                WHERE chat_id=$1")
            .bind(chat_id)
            .execute(&appstate.db())
            .await?;
        Ok(())
    }
    pub async fn join_user(appstate: &AppState, chat_id: Uuid, user_id: Uuid ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO chats_users
                (chat_id, user_id) VALUES
                ($1, $2)")
            .bind(chat_id)
            .bind(user_id)
            .execute(&appstate.db())
            .await?;
        Ok(())
    }

    pub async fn leave_user(appstate: &AppState, chat_id: Uuid, user_id: Uuid ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM chats_users
                 WHERE chat_id=$1
                 AND user_id=$2")
            .bind(chat_id)
            .bind(user_id)
            .execute(&appstate.db())
            .await?;
        Ok(())
    }

    pub async fn get_user_chats(appstate: &AppState, user_id: Uuid) -> Result<Vec<ChatIDSchema>, sqlx::Error> {
        sqlx::query_as(
            "SELECT chat_id from chats_users
            WHERE user_id=$1"
        )
        .bind(user_id)
        .fetch_all(&appstate.db())
        .await
    }

    pub async fn get_chat_users(appstate: &AppState, chat_id: Uuid) -> Result<Vec<FilteredUser>, sqlx::Error> {
        sqlx::query_as(
            "SELECT users.user_id, users.email, users.username, users.created_at, users.updated_at
            FROM users JOIN chats_users ON users.user_id = chats_users.user_id
            WHERE chats_users.chat_id=$1"
        )
        .bind(chat_id)
        .fetch_all(&appstate.db())
        .await
    }
}

