use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;

use gloo::utils::format::JsValueSerdeExt;
use gloo_net::websocket::Message;
use shared::models::websocket::RTCMessages;
use shared::models::websocket::WebSocketMessage;
use wasm_bindgen::prelude::*;

use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;
use web_sys::js_sys::JSON;
use web_sys::wasm_bindgen;
use web_sys::HtmlAudioElement;
use web_sys::HtmlVideoElement;
use web_sys::MediaStream;
use web_sys::RtcIceCandidate;
use web_sys::RtcIceCandidateInit;
use web_sys::RtcPeerConnection;
use web_sys::RtcPeerConnectionIceEvent;
use web_sys::RtcSessionDescription;
use web_sys::RtcSessionDescriptionInit;
use web_sys::RtcTrackEvent;
//use web_sys::HtmlButtonElement;
use yew::prelude::*;

use crate::rtc_network_manager;
use crate::rtc_network_manager::RTCNetworkManager;

use super::services::websocket::WebsocketService;

// #[wasm_bindgen(module = "/js/audio_recorder.js")]
// extern "C" {
//     async fn startRecording(callback: &wasm_bindgen::JsValue);
//     async fn stopRecording() -> js_sys::Promise;
// }

pub enum ComponentMessage {
    Start,
    Stop,
    WebsocketMessage(String)
}

pub struct WsTest {
    active: bool,
    wss: Arc<RefCell<WebsocketService>>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub offer_sending_callback: Callback<String>
}

impl Component for WsTest {
    type Message = ComponentMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        
        
        Self {
            active: false,
            wss: Arc::new(RefCell::new(WebsocketService::new())),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let start = ctx.link().callback(|_| ComponentMessage::Start);
        let stop = ctx.link().callback(|_| ComponentMessage::Stop);

        html! {
            <div>
                <button onclick={start} disabled={self.active}>{ "Start Recording" }</button>
                <button onclick={stop} disabled={!self.active}>{ "Stop Recording" }</button>
                // if let Some(data_url) = &*audio_data {
                //     <audio controls=true src={data_url.clone()}></audio>
                // }
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ComponentMessage::Start => {
                let wss_clone = Arc::clone(&self.wss);
                let on_message = _ctx.link().callback(ComponentMessage::WebsocketMessage);
                wasm_bindgen_futures::spawn_local(async move {
                    wss_clone.borrow_mut().connect("https://192.168.0.3:3003/ws", on_message).await;
                });
                self.active = true;
                true
            }
            ComponentMessage::Stop => {
                self.active = false;
                
                
                self.wss.borrow_mut().disconnect();
                // wasm_bindgen_futures::spawn_local(async move {
                //     wss_clone.close().await;
                // });
                
                // let recording_service = self.recording_service.clone();
                // wasm_bindgen_futures::spawn_local(async move {
                //     let _ = recording_service.stop().await;
                // });
                true
            },
            _ => {false}
        }
    }
}