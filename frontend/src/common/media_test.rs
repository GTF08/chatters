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
    //WebsocketMessage(String),
    //RTCInitialized,
    //RtcIceCandidate(String),
}

pub struct MediaTest {
    active: bool,
    local_player: NodeRef,
    remote_player: NodeRef,
    //wss: Arc<RefCell<WebsocketService>>,
    rtc_network_manager: Arc<RwLock<RTCNetworkManager>>
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub offer_sending_callback: Callback<String>
}

impl Component for MediaTest {
    type Message = ComponentMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        //let on_message = ctx.link().callback(ComponentMessage::WebsocketMessage);
        //let on_rtc_creation = ctx.link().callback(|()| ComponentMessage::RTCInitialized);

        Self {
            active: false,
            local_player: NodeRef::default(),
            remote_player: NodeRef::default(),
            //wss: Arc::new(RefCell::new(WebsocketService::new(on_message))),
            rtc_network_manager: Arc::new(RwLock::new(RTCNetworkManager::new()))
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let start = ctx.link().callback(|_| ComponentMessage::Start);
        let stop = ctx.link().callback(|_| ComponentMessage::Stop);

        html! {
            <div>
                <video ref={self.local_player.clone()} muted={true} playsinline={true} autoplay={true} width="240"> </video>
                <video ref={self.remote_player.clone()} id="fgt" muted={true} playsinline={true} autoplay={true} width="240"> </video>
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
                //let wss = self.wss.clone();
                //self.wss.connect("https://192.168.0.3:3003/ws").await;
                self.active = true;
                let local_player = self.local_player.clone().cast::<HtmlVideoElement>().unwrap();
                let remote_player = self.remote_player.clone().cast::<HtmlVideoElement>().unwrap();
                let rtc_network_manager_clone = self.rtc_network_manager.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    rtc_network_manager_clone.write().unwrap().connect().await;
                    local_player.set_src_object(rtc_network_manager_clone.read().unwrap().local_stream.as_ref());
                    remote_player.set_src_object(rtc_network_manager_clone.read().unwrap().remote_stream.as_ref().map(|v| &**v));
                    //JsFuture::from(remote_player.play().unwrap()).await.unwrap();
                });
                true
            }
            ComponentMessage::Stop => {
                self.active = false;
                self.rtc_network_manager.write().unwrap().disconnect();
                self.local_player.cast::<HtmlVideoElement>().unwrap().set_src_object(None);
                self.remote_player.cast::<HtmlVideoElement>().unwrap().set_src_object(None);
                
                true
            },
        }
    }
}