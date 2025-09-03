use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};



// #[derive(Deserialize, Serialize, FromRow, PartialEq, Clone)]
// pub struct ChatMessageDTO {
//     pub username: String,
//     pub message_text: String,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }

#[derive(Debug, Deserialize, Serialize, FromRow, PartialEq, Clone)]
pub struct MessageUserDTO {
    pub chat_id: Uuid,
    pub username: String,
    pub message_text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct MessagesPageDTO {
    pub chat_id: Uuid,
    pub total: i64,
}

#[derive(Debug, Deserialize, Serialize, FromRow, PartialEq, Clone)]
pub struct MessageCreateSchema {
    pub chat_id: Uuid,
    pub message_text: String
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct GetMessagesRequestData {
    pub chat_id: Uuid,
    pub page: i64
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct NewVoiceMessageSchema {
    pub chat_id: Uuid,
    pub bytes: Vec<u8>
}