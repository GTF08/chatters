use axum::{extract::State, response::IntoResponse, Extension};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use redis::AsyncCommands;

use shared::models::messages::{GetMessagesRequestData, MessageCreateSchema, NewVoiceMessageSchema};
use tokio::sync::Mutex;
use std::sync::Arc;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};

use common::{
    appstate::AppState, 
    middlewares::CurrentUser, 
    models::{chat::Chats, messages::Messages}
};
use shared::models::{chats::{ChatIDSchema, CreateChatSchema}, users::FilteredUser};
//use shared::models::messages::ChatMessagesDTO;
use uuid::Uuid;

use shared::models::websocket::{MsgTypes, WebSocketMessage};



// pub async fn get_chat_messages(
//     Extension(current_user): Extension<CurrentUser>,
//     State(database_state): State<AppState>,
//     Path(chat_id) : Path<Uuid>
// ) -> Result<Json<Vec<ChatMessagesDTO>>, StatusCode> {
//     Messages::get_messages_by_chat_id(&database_state, chat_id)
//     .await
//     .map(|value| Json(value))
//     .map_err(|e| {StatusCode::INTERNAL_SERVER_ERROR})
// }

struct RTCConnectionData {
    pub offer: String,
    pub offerIceCandidates: Vec<String>,
    pub answer: String,
    pub answererIceCandidates: Vec<String>
}

pub async fn websocket_handler(
    Extension(current_user): Extension<CurrentUser>,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(current_user, socket, state))
}
async fn handle_socket(
    current_user: CurrentUser,
    ws: WebSocket,
    state: AppState,
) {
    let (sender, reader) = ws.split();
    let sender = Arc::new(Mutex::new(sender));
    //AUTH HERE
    //ADD CONNECTINO TO REDIS
    // let mut con = match state.redis_client().get_connection() {
    //     Ok(value) => {value},
    //     Err(e) => {return;}
    // };

    let mut pubsub = match state.redis_client().get_async_pubsub().await {
        Ok(value) => {value},
        Err(_) => {return;},
    };
    match pubsub.subscribe(current_user.id.to_string()).await {
        Ok(_) => {},
        Err(e) => {return;}
    };

    //Read From Client
    let sender_clone = sender.clone();
    tokio::spawn(async move {
        read_from_client(&state, &current_user, &sender_clone, reader).await
    });
    
    //READ MESSAGES FROM REDIS (ONLY IS IT IS FROM OTHER USER)
    tokio::spawn(async move {
        while let Some(msg) = pubsub.on_message().next().await {
            let payload : String = match msg.get_payload() {
                Ok(value) => {value},
                Err(e) => {return;}
            };
            let websocket_message = serde_json::from_str::<WebSocketMessage<MsgTypes>>(&payload).unwrap();
    
            match websocket_message.message_type {
                MsgTypes::GetMessagesRequest(request_data) => todo!(),
                MsgTypes::Register => todo!(),
                MsgTypes::NewMessageRecieved(ref recieved_message) => {
                    write_to_client(&sender.clone(), websocket_message).await;
                },
                MsgTypes::NewVoiceMessageRecieved(ref voicemsg_schema) => {
                    write_to_client(&sender.clone(), websocket_message).await;
                },

                _ => {}
            }
            println!("channel '{}': {}", msg.get_channel_name(), payload);
        }
    });
   
}


async fn read_from_client(state: &AppState, current_user: &CurrentUser, mut sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>, mut receiver: SplitStream<WebSocket>) {
    while let Some(Ok(message)) = receiver.next().await {
        match message {
            Message::Text(utf8_bytes) => {
                let websocket_message = serde_json::from_str::<WebSocketMessage<MsgTypes>>(&utf8_bytes).unwrap();
                match websocket_message.message_type {
                    MsgTypes::GetMessagesRequest(request_data) => {
                        process_get_messages_request(state, current_user, sender, request_data).await;
                        // write_to_client(sender, wsmessage)
                    },
                    MsgTypes::NewMessageRequest(msg_create_data) => {
                        process_new_message_request(&state, &current_user, msg_create_data).await;
                    },
                    MsgTypes::Users => todo!(),
                    MsgTypes::Register => todo!(),
                    MsgTypes::NewVoiceMessageRequest(voicemsg_schema) => {
                        process_voice_message_request(state, current_user, voicemsg_schema).await;
                    },
                    _ => {}
                }
            },
            Message::Binary(bytes) => {
                println!("BYTES");
            },
            Message::Ping(bytes) => {},
            Message::Pong(bytes) => {},
            Message::Close(close_frame) => todo!(),
        }
    }
}

async fn write_to_client(sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>, wsmessage: WebSocketMessage<MsgTypes>) {
    if let Ok(value) = serde_json::to_string(&wsmessage) {
        sender.lock().await.send(Message::Text(value.into())).await.unwrap();
    }
}

async fn process_get_messages_request(
    state: &AppState,
    current_user: &CurrentUser,
    sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>,
    request_data: GetMessagesRequestData
) {
    //if let Ok(message_request_data) = serde_json::from_str::<GetChatMessagesPageRequest>(&payload) {
        if let Ok(msgs) = Messages::get_messages_page_by_chat_id(
            &state, request_data.chat_id, request_data.page).await {
            // let payload = serde_json::to_string(&GetChatMessagesPageResponse{
            //     chat_id: request_data.chat_id,
            //     messages: msgs,
            // }).unwrap();
            let wsmessage = WebSocketMessage { 
                message_type: MsgTypes::ChatMessagesRecieved(msgs), 
                //payload: Some(payload)
            };
            write_to_client(sender, wsmessage).await;
        }
    //}
}

async fn process_new_message_request(
    state: &AppState, 
    current_user: &CurrentUser,
    msg_create_data: MessageCreateSchema
) {
    //Parse New Message Data
    //if let Ok(message_create_data) = serde_json::from_str::<MessageCreateSchema>(&payload) {
        //If it successfully added to a database send it to everyone
    if let Ok(new_message) = Messages::insert(state, current_user.id, &msg_create_data).await {
        let chat_users : Vec<FilteredUser> = Chats::get_chat_users(state, msg_create_data.chat_id).await.unwrap();
        let mut redis_con = state.redis_client().get_multiplexed_async_connection().await.unwrap();

        let new_message_update = WebSocketMessage {
            message_type: MsgTypes::NewMessageRecieved(new_message),
            //payload: Some(serde_json::to_string(&new_message).unwrap()),
        };
        for user in chat_users.iter() {
            redis_con.publish::<_, String, String>(
                user.user_id.to_string(), 
                serde_json::to_string(&new_message_update).unwrap()
            )
            .await
            .unwrap();
        }
    }
    //}
}

async fn process_voice_message_request(
    state: &AppState,
    current_user: &CurrentUser,
    voicemsg_schema: NewVoiceMessageSchema
) {
    let chat_users : Vec<FilteredUser> = Chats::get_chat_users(state, voicemsg_schema.chat_id).await.unwrap();
    let mut redis_con = state.redis_client().get_multiplexed_async_connection().await.unwrap();

    let voice_message_update = WebSocketMessage {
        message_type: MsgTypes::NewVoiceMessageRecieved(voicemsg_schema)
    };
    for user in chat_users.iter() {
        redis_con.publish::<_, String, String>(
            user.user_id.to_string(), 
            serde_json::to_string(&voice_message_update).unwrap()
        )
        .await
        .unwrap();
    }
}