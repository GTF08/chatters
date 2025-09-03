use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateChatSchema {
    pub name: String,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct ChatIDSchema {
    pub chat_id: Uuid,
}