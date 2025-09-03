use gloo::net::http::Request;
//use shared::models::messages::MessageTransferDTO;
use uuid::Uuid;
use yew::prelude::*;

// use crate::common::atoms::chats::chats_message_display::ChatMessageDisplay;

use yew::{function_component, html};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub chat_id: Uuid,
}

// #[function_component(OldMessagesBox)]
// pub fn old_messages_box(props: &Props) -> Html {
//     let state = use_state(|| None);
//     let state_clone = state.clone();
//     let chat_id = props.chat_id.clone();
//     use_effect_with((), move |_| {
//         wasm_bindgen_futures::spawn_local(async move {
//             let fetched_messages : Vec<MessageTransferDTO> = 
//                 Request::get(format!("http://localhost:3002/messages/{}", chat_id).as_str())
//                 .credentials(web_sys::RequestCredentials::Include)
//                 //.credentials(web_sys::RequestCredentials::Include)
//                 //.header("Access-Control-Allow-Origin", "http://localhost:3000/")
//                 .send()
//                 .await
//                 .unwrap()
//                 .json()
//                 .await
//                 .unwrap();
//             state.set(Some(fetched_messages));
//         });
//     });
    

//     match state_clone.as_ref() {
//         Some(messages) => {
//             html!{
//                 <div class="message-box">
//                     {
//                         messages.into_iter().map(|message| {
//                             html!{ <ChatMessageDisplay message_data={(*message).clone()} /> }
//                         }).collect::<Html>()
//                     }
//                 </div>
//             }
//         },
//         None => {
//             html!{}
//         }
//     }

// }