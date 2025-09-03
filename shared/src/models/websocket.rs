use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::messages::{MessageUserDTO, MessageCreateSchema, GetMessagesRequestData, NewVoiceMessageSchema};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    //Server Side
    GetMessagesRequest(GetMessagesRequestData),
    NewMessageRequest(MessageCreateSchema),
    NewVoiceMessageRequest(NewVoiceMessageSchema),
    Users,
    Register,
    //Client Side
    NewMessageRecieved(MessageUserDTO),
    NewVoiceMessageRecieved(NewVoiceMessageSchema),
    ChatMessagesRecieved(Vec<MessageUserDTO>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RTCMessages {
    //RTC
    NewOffer(String),
    NewAnswer(String),
    NewIceCandidate(String)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketMessage<T> {
    pub message_type: T,
    // pub payload: Option<String>,
}



// #[derive(Deserialize, Serialize, FromRow, PartialEq, Clone)]
// pub struct GetChatMessagesPageRequest {
//     pub chat_id: Uuid,
//     pub page: i64
// }

// #[derive(Deserialize, Serialize, FromRow, PartialEq, Clone)]
// pub struct GetChatMessagesPageResponse {
//     pub chat_id: Uuid,
//     pub messages: Vec<MessageUserDTO>
// }
