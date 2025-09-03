use gloo::net::http::Request;
use uuid::Uuid;
use yew::prelude::*;
use shared::models::chats::ChatIDSchema;
use crate::common::atoms::buttons::chat_switch_button::ChatSwitchButton;

use yew::{function_component, html};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub chat_choose_callback: Callback<Uuid>,
}

#[function_component(ChatChoiceSidebar)]
pub fn chat_choice_sidebar(props: &Props) -> Html {
    let state = use_state(|| None);
    let state_clone = state.clone();
    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_chats : Vec<ChatIDSchema> = 
                Request::get("https://192.168.0.3:3002/chats")
                .credentials(web_sys::RequestCredentials::Include)
                //.credentials(web_sys::RequestCredentials::Include)
                //.header("Access-Control-Allow-Origin", "http://localhost:3000/")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            state.set(Some(fetched_chats));
        });
    });
    

    match state_clone.as_ref() {
        Some(chats) => {
            chats.into_iter().map(|chat| {
                html!{ <ChatSwitchButton chat_id={chat.chat_id} callback = {props.chat_choose_callback.clone()} /> }
            }).collect::<Html>()
        },
        None => {
            html!{}
        }
    }

}