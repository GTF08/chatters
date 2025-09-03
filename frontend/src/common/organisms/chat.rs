use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};

use crate::common::atoms::chats::chats_message_display::ChatMessageDisplay;
use shared::models::{messages::{MessageCreateSchema, MessageUserDTO}, websocket::{MsgTypes, WebSocketMessage}};

pub enum Msg {
    SubmitMessage,
}


#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_chat_id: Uuid,
    pub messages: Rc<RefCell<Vec<MessageUserDTO>>>,
    pub on_send_msg_click: Callback<WebSocketMessage<MsgTypes>>
}

pub struct Chat {
    //audio_recorder_service: AudioRecorderService,
    chat_input: NodeRef,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        // let audio_recorder_service = AudioRecorderService::new(|jsvalue: wasm_bindgen::JsValue| {
        //         ctx.link().callback(Msg::HandleMsg).emit("TEXT".to_string());
        // });

        Self {
            // audio_recorder_service,
            chat_input: NodeRef::default()
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        html! {
            <>
                <div>
                    {
                        ctx.props().messages.borrow().iter().map(|m| {
                            html!{<ChatMessageDisplay message_data={m.clone()}> </ChatMessageDisplay>}
                        }).collect::<Html>()
                    }
                </div>
                <div class="w-full h-14 flex px-3 items-center">
                    <input ref={self.chat_input.clone()} type="text" placeholder="Message" class="block w-full py-2 pl-4 mx-3 bg-gray-100 rounded-full outline-none focus:text-gray-700" name="message" required=true />
                    <button onclick={submit} class="p-3 shadow-sm bg-blue-600 w-10 h-10 rounded-full flex justify-center items-center color-white">
                        <svg fill="#000000" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                            <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                        </svg>
                    </button>
                </div>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    //log::debug!("got input: {:?}", input.value());
                    let new_message_data = MessageCreateSchema {
                        chat_id: _ctx.props().current_chat_id,
                        message_text: input.value()
                    };
                    let message = WebSocketMessage {
                        message_type: MsgTypes::NewMessageRequest(new_message_data),
                        //payload: Some(serde_json::to_string(&new_message_data).unwrap())
                    };

                    _ctx.props().on_send_msg_click.emit(message);
                    // if let Err(e) = self
                    //     .wss
                    //     .tx
                    //     .clone()
                    //     .try_send(Message::Text(serde_json::to_string(&message).unwrap()))
                    // {
                    //     gloo::console::log!("error sending to channel: {:?}", e.to_string());
                    // }
                    input.set_value("");
                };
                false
            }
        }
    }
}