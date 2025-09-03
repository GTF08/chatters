use std::{cell::RefCell, rc::Rc, sync::Arc};

use gloo_net::websocket::Message;
use uuid::Uuid;
use web_sys::{js_sys::Uint8Array, wasm_bindgen, HtmlInputElement};
use yew::{html, Callback, Component, Context, ContextProvider, Html, NodeRef, Properties};

use crate::common::{
    atoms::{
        buttons::logout_button::LogoutButton, 
        chats::chats_choice_sidebar::ChatChoiceSidebar, 
        sidebar::{body::Body, footer::Footer, header::Header}, 
        users::current_user_display::CurrentUserDisplay}, audio_recorder::AudioRecorder, media_test::MediaTest, organisms::{chat::Chat, sidebar::Sidebar}, services::websocket::WebsocketService
};
use shared::models::{messages::{MessageCreateSchema, MessageUserDTO, NewVoiceMessageSchema}, websocket::{MsgTypes, WebSocketMessage}};

use crate::common::services::audio_player_service::play_audio;
use crate::common::wstest::WsTest;

pub enum ComponentMessage {
    SendMessageButtonClick(WebSocketMessage::<MsgTypes>),
    ChatChange(Uuid),
    HandleMsg(String),
}


pub struct MainPage {
    wss: Arc<RefCell<WebsocketService>>,
    //audio_recorder_service: AudioRecorderService,
    messages: Rc<RefCell<Vec<MessageUserDTO>>>,
    current_chat_id: Option<Uuid>
}


impl Component for MainPage {
    type Message = ComponentMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let on_message = ctx.link().callback(ComponentMessage::HandleMsg);

        let wss = Arc::new(RefCell::new(WebsocketService::new()));
        let wss_clone = wss.clone();
        wasm_bindgen_futures::spawn_local(async move {
            wss_clone.borrow_mut().connect("https://192.168.0.3:3002/ws", on_message).await;
        });
        

        // let audio_recorder_service = AudioRecorderService::new(|jsvalue: wasm_bindgen::JsValue| {
        //         ctx.link().callback(Msg::HandleMsg).emit("TEXT".to_string());
        // });

        Self {
            messages: Rc::new(RefCell::new(vec![])),
            // audio_recorder_service,
            wss,
            current_chat_id: None
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        //let on_chat_change = ctx.link().callback(|_| Msg::SubmitMessage);
        let on_chat_change = ctx.link().callback(ComponentMessage::ChatChange);
        let on_send_msg_click = ctx.link().callback(ComponentMessage::SendMessageButtonClick);
        //let on_audio_recorded = ctx.link().callback(ComponentMessage::AudioRecorded);
        
        html!{
            <>
                <Sidebar>
                    <Sidebar class="sidebar-chats">
                        <ChatChoiceSidebar chat_choose_callback={on_chat_change}/>
                    </Sidebar>
                    <Sidebar class = "sidebar-content">
                        <Header></Header>
                        <Body>
                            //<MediaTest/>
                            // <WsTest />
                        </Body>
                        <Footer>
                            //<AudioRecorder on_audio_recorded={on_audio_recorded}/>
                            <CurrentUserDisplay />
                            <LogoutButton />
                        </Footer>
                    </Sidebar>
                </Sidebar>
                <div class="main">
                    <MediaTest/>
                    {
                        match self.current_chat_id {
                            Some(value) => {
                                html!{
                                    <>
                                    <Chat current_chat_id={value} messages={self.messages.clone()} on_send_msg_click={on_send_msg_click}/>
                                    </>
                                }
                            },
                            None => {
                                html!{}
                            }
                        }
                    }
                </div>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ComponentMessage::HandleMsg(s) => {
                let msg: WebSocketMessage::<MsgTypes> = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::NewMessageRecieved(message_data) => {
                        self.messages.borrow_mut().push(message_data);
                        //web_sys::console::log_1(&format!("MSG LEN {}", self.messages.borrow().len()).into());
                        return true;
                    },
                    MsgTypes::NewVoiceMessageRecieved(voice_msg_schema) => {
                        //web_sys::console::log_1(&"I AM TIRED".into());
                        wasm_bindgen_futures::spawn_local(async move {
                            let uint8array = Uint8Array::from(voice_msg_schema.bytes.as_slice());
                            play_audio(uint8array).await;
                        });
                        return false;
                    },
                    _ => {
                        return false;
                    }
                }
            },
            // ComponentMessage::AudioRecorded(bytes) => {
            //     if let Err(e) = self
            //         .wss
            //         .tx
            //         .clone()
            //         .try_send(Message::Text(
            //             serde_json::to_string(
            //                 &WebSocketMessage{
            //                     message_type: MsgTypes::NewVoiceMessageRequest(NewVoiceMessageSchema {
            //                         chat_id: self.current_chat_id.unwrap(),
            //                         bytes: bytes
            //                     })
            //                 }
            //             ).unwrap()
            //         )) {
            //             gloo::console::log!("error sending to channel: {:?}", e.to_string());
            //         }
            //     //web_sys::console::log_1(&"AUDIO RECORDED".into());
            //     false
            // },
            ComponentMessage::ChatChange(id) => {
                self.current_chat_id = Some(id);
                true
            },
            ComponentMessage::SendMessageButtonClick(wsmessage) => {
                if let Err(e) = self
                        .wss
                        .borrow_mut()
                        .get_sender()
                        .clone()
                        .try_send(Message::Text(serde_json::to_string(&wsmessage).unwrap()))
                {
                    gloo::console::log!("error sending to channel: {:?}", e.to_string());
                }
                false
            },
            // ComponentMessage::RTCOfferReady(offer) => {
            //     let wsmessage = WebSocketMessage {
            //         message_type: MsgTypes::NewOffer(offer)
            //     };
            //     if let Err(e) = self
            //         .wss
            //         .tx
            //         .clone()
            //         .try_send(Message::Text(serde_json::to_string(&wsmessage).unwrap()))
            //     {
            //         gloo::console::log!("error sending to channel: {:?}", e.to_string());
            //     }
            //     true
            // }
        }
    }
}