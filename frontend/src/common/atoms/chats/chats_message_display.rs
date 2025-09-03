use gloo::net::http::Request;
use uuid::Uuid;
use yew::prelude::*;
use shared::models::chats::ChatIDSchema;
use crate::common::atoms::buttons::chat_switch_button::ChatSwitchButton;

use shared::models::messages::MessageUserDTO;

use yew::{function_component, html};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub message_data: MessageUserDTO,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(ChatMessageDisplay)]
pub fn chat_message_display(props: &Props) -> Html {

    html!{
        <div class="message">
            <div class="message-username">{&props.message_data.username}</div><br/>
            <div class="message-text">{&props.message_data.message_text}</div><br/>
            <div class="message-created-at">{&props.message_data.created_at.format("%H:%M:%S").to_string()}</div><br/>
        </div>
    }

}