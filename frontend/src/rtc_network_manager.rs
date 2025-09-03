use std::{cell::RefCell, sync::Arc};

use futures::SinkExt;
use gloo::utils::format::JsValueSerdeExt;
use shared::models::websocket::{RTCMessages, WebSocketMessage};
use web_sys::{
    console, js_sys::JSON, wasm_bindgen::{prelude::Closure, JsCast, JsValue}, HtmlVideoElement, MediaRecorder, MediaStream, MediaStreamTrack, RtcConfiguration, RtcDataChannel, RtcDataChannelEvent, RtcDataChannelInit, RtcDataChannelState, RtcIceCandidate, RtcIceCandidateInit, RtcIceConnectionState, RtcIceGatheringState, RtcIceServer, RtcPeerConnection, RtcPeerConnectionIceEvent, RtcSessionDescriptionInit, RtcTrackEvent
};
use yew::{platform::time, Callback, NodeRef};
use web_sys::HtmlAudioElement;
use web_sys::{
    Navigator,
    MediaStreamConstraints,
    MediaTrackSupportedConstraints,
    DisplayMediaStreamConstraints
};
use gloo_net::websocket::Message;

use web_sys::js_sys::Array;

use wasm_bindgen_futures::JsFuture;

use crate::common::services::websocket::WebsocketService;



pub struct RTCNetworkManager {
    pub local_stream: Option<MediaStream>,
    pub remote_stream: Option<Arc<MediaStream>>,
    pub rtc_peer_connection: Option<Arc<RtcPeerConnection>>,
    wss: WebsocketService,
    //on_rtc_created_callback: Option<Callback<()>>,
    on_signaling_state_changed: Option<Closure<dyn FnMut(JsValue)>>,
    on_ice_candidate: Option<Closure<dyn FnMut(RtcPeerConnectionIceEvent)>>,
    on_track: Option<Closure<dyn FnMut(RtcTrackEvent)>>,
}


impl RTCNetworkManager {
    pub fn new() -> Self {
        Self {
            local_stream: None,
            remote_stream: None,
            rtc_peer_connection: None,
            wss: WebsocketService::new(),
            //on_rtc_created_callback: on_rtc_created_callback,
            on_signaling_state_changed: None,
            on_ice_candidate: None,
            on_track: None
        }
    }

    pub async fn connect(&mut self) {
        self.local_stream = Some(RTCNetworkManager::create_local_stream().await);
        self.remote_stream = Some(Arc::new(RTCNetworkManager::create_remote_stream().await));
        let rtc_peer_conn = Arc::new(RTCNetworkManager::create_peer_connection());

        for track in self.local_stream.as_ref().unwrap().get_tracks().iter() {
            rtc_peer_conn.as_ref().add_track_0(
                &track.dyn_into().unwrap(), 
                self.local_stream.as_ref().unwrap()
            );
        };
        
        rtc_peer_conn.set_onsignalingstatechange(
            match self.on_signaling_state_changed {
                Some(ref v) => Some(v.as_ref().unchecked_ref()),
                None => None,
            }
        );
        rtc_peer_conn.set_onicecandidate(
            match self.on_ice_candidate {
                Some(ref v) => Some(v.as_ref().unchecked_ref()),
                None => None,
            }
        );
        rtc_peer_conn.set_ontrack(
            match self.on_track {
                Some(ref v) => Some(v.as_ref().unchecked_ref()),
                None => None,
            }
        );

        let rtc_peer_conn_clone = Arc::clone(&rtc_peer_conn);
        let on_text = Callback::from(move |msg: String| {
            let msg: WebSocketMessage::<RTCMessages> = serde_json::from_str(&msg).unwrap();
            match msg.message_type {
                RTCMessages::NewOffer(_) => unreachable!("should never happen on client"),
                RTCMessages::NewAnswer(answer) => {
                    // JSON::wasm_bindgen::
                    let descr_obj = JSON::parse(&answer).unwrap();
                    let answer = RtcSessionDescriptionInit::from(descr_obj);

                    let promise = rtc_peer_conn_clone.set_remote_description(&answer);
                    // let promise = self.rtc_network_manager.read().unwrap()
                    //     .rtc_peer_connection.as_ref().unwrap().set_remote_description(&answer);
                    wasm_bindgen_futures::spawn_local(async move {
                        match JsFuture::from(promise).await {
                            Ok(_) => {},
                            Err(e) => {web_sys::console::log_1(&e);},
                        };
                    });
                    
                },
                RTCMessages::NewIceCandidate(candidate) => {
                    let ice_candidate_jsvalue = JSON::parse(&candidate).unwrap();
                    let ice_candidate = RtcIceCandidate::from(ice_candidate_jsvalue);
                    // let ice_candidate_jsvalue = JsValue::from(candidate);
                    // let ice_candidate = RtcIceCandidate::from(ice_candidate_jsvalue);
                    let promise = rtc_peer_conn_clone.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&ice_candidate));
                    // let promise = self.rtc_network_manager.read().unwrap()
                    //     .rtc_peer_connection.as_ref().unwrap()
                    //     .add_ice_candidate_with_opt_rtc_ice_candidate(Some(ice_candidate.as_ref()));
                    wasm_bindgen_futures::spawn_local(async move {
                        JsFuture::from(promise).await.unwrap();
                    });
                },
            }
        });

        self.wss.connect("https://192.168.0.3:3003/ws", on_text).await;
        
        self.on_signaling_state_changed = Some(
            Closure::wrap(Box::new(move |e: JsValue| {
                web_sys::console::log_1(&e);
                }) as Box<dyn FnMut(JsValue)>
            )
        );

        let wswriter = self.wss.get_sender().clone();
        self.on_ice_candidate = Some(
            Closure::wrap(Box::new(move |e: RtcPeerConnectionIceEvent| {
                if let Some(candidate) = e.candidate() {
                    let candidate_string = JSON::stringify(&candidate).unwrap().as_string().unwrap();
                    let mut wswriter = wswriter.clone();
                    //web_sys::console::log_1(&candidate_string.clone().into());
                    wasm_bindgen_futures::spawn_local(async move {
                        let wsmessage = WebSocketMessage::<RTCMessages> {
                            message_type: RTCMessages::NewIceCandidate(candidate_string)
                        };
                        let wsmessage = serde_json::to_string(&wsmessage).unwrap();
                        wswriter.send(Message::Text(wsmessage)).await.unwrap();
                    });
                }
                }) as Box<dyn FnMut(RtcPeerConnectionIceEvent)>
            )
        );

        let rtc_peer_conn_clone2 = Arc::clone(&rtc_peer_conn);
        let remote_stream = self.remote_stream.clone().unwrap();
        self.on_track = Some(
            Closure::wrap(Box::new(move |e: RtcTrackEvent| { 
                /* whatever */
                web_sys::console::log_1(&"NEW TRACK".into());
                for track in e.streams().get(0).dyn_into::<MediaStream>().unwrap().get_tracks().iter() {
                    // let mstrack : MediaStreamTrack = track.into();
                    //rtc_peer_conn_clone2.add_track_0(&track.dyn_into::<MediaStreamTrack>().unwrap(), &remote_stream);
                    let elem : HtmlVideoElement = gloo::utils::document().get_element_by_id("fgt").unwrap().dyn_into().unwrap();
                    elem.set_src_object(Some(e.streams().at(0).dyn_into::<MediaStream>().unwrap()).as_ref());
                    remote_stream.add_track(&track.dyn_into::<MediaStreamTrack>().unwrap());
                    //remote_stream.get_tracks().push(&track);
                }
                web_sys::console::log_1(&"Track added to remote stream".into());
            }) as Box<dyn FnMut(RtcTrackEvent)>)
        );
        

        rtc_peer_conn.set_onsignalingstatechange(
            match self.on_signaling_state_changed {
                Some(ref v) => Some(v.as_ref().unchecked_ref()),
                None => None,
            }
        );
        rtc_peer_conn.set_onicecandidate(
            match self.on_ice_candidate {
                Some(ref v) => Some(v.as_ref().unchecked_ref()),
                None => None
            }
        );
        rtc_peer_conn.set_ontrack(
            match self.on_track {
                Some(ref v) => Some(v.as_ref().unchecked_ref()),
                None => None
            }
        );



        
        let rtc_peer_conn_clone = Arc::clone(&rtc_peer_conn);
        let offer_promise = rtc_peer_conn_clone.create_offer();
        let mut wswriter = self.wss.get_sender().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let offer = JsFuture::from(offer_promise).await.unwrap();
            let offer: RtcSessionDescriptionInit = offer.unchecked_into::<RtcSessionDescriptionInit>();

            let promise = rtc_peer_conn_clone.set_local_description(&offer);
            JsFuture::from(promise).await.unwrap();

            let offer_string = JSON::stringify(&offer).unwrap().as_string().unwrap();
            let wsmessage = WebSocketMessage::<RTCMessages> {
                message_type: RTCMessages::NewOffer(offer_string)
            };
            wswriter
                .try_send(Message::Text(serde_json::to_string(&wsmessage).unwrap())).unwrap();
        });

        self.rtc_peer_connection = Some(rtc_peer_conn);
        // if let Some(cb) = &self.on_rtc_created_callback {
        //     cb.emit(());
        // }

    }

    async fn create_local_stream() -> MediaStream {
        let constraints = MediaStreamConstraints::new();
        constraints.set_audio(&JsValue::from_bool(true));
        constraints.set_video(&JsValue::from_bool(true));

        let stream_promise = web_sys::window().unwrap().navigator()
            .media_devices().unwrap().get_user_media_with_constraints(&constraints).unwrap();
        let stream = JsFuture::from(stream_promise).await.unwrap();
        let local_stream = MediaStream::from(stream);

        local_stream
    }

    async fn create_remote_stream() -> MediaStream {
        MediaStream::new().unwrap()
    }

    fn create_peer_connection() -> RtcPeerConnection {
        let ice_servers = RtcIceServer::new();
        let ice_server_urls = Array::new();
        ice_server_urls.push(&"stun:localhost:3478".into());
        // ice_server_urls.push(&"stun:stun1.l.google.com:19302".into());
        // ice_server_urls.push(&"stun:stun.l.google.com:19302".into());
        // ice_server_urls.push(&"stun:stun.l.google.com:5349".into());
        // ice_server_urls.push(&"stun:stun1.l.google.com:3478".into());
        // ice_server_urls.push(&"stun:stun1.l.google.com:5349".into());
        // ice_server_urls.push(&"stun:stun2.l.google.com:19302".into());
        // ice_server_urls.push(&"stun:stun2.l.google.com:5349".into());
        // ice_server_urls.push(&"stun:stun3.l.google.com:3478".into());
        // ice_server_urls.push(&"stun:stun3.l.google.com:5349".into());
        // ice_server_urls.push(&"stun:stun4.l.google.com:19302".into());
        // ice_server_urls.push(&"stun:stun4.l.google.com:5349".into());        
        ice_servers.set_urls(&ice_server_urls);

        let configuration = RtcConfiguration::new();
        configuration.set_ice_servers(&Array::from(&JsValue::from(&ice_servers)));

        let peer_connection = RtcPeerConnection::new().unwrap();
        peer_connection.set_configuration_with_configuration(&configuration).unwrap();

        peer_connection

        //let audio_element = self.remote_stream_player.cast::<HtmlAudioElement>().unwrap();
        //let remote_player = node.cast::<HtmlAudioElement>().unwrap();
        //remote_player.set_src_object(Some(&self.remote_stream.as_ref().unwrap()));

        // for track in self.local_stream.as_ref().unwrap().get_tracks().iter() {
        //     self.rtc_peer_connection.as_ref().unwrap().add_track_0(&track.dyn_into().unwrap(), self.local_stream.as_ref().unwrap());
        // };
    }

    pub fn disconnect(&mut self) {
        if let Some(v) = self.on_signaling_state_changed.take() {
            drop(v);
        }
        if let Some(v) = self.on_ice_candidate.take() {
            drop(v);
        }
        if let Some(v) = self.on_track.take() {
            drop(v);
        }
        self.wss.disconnect();
        if let Some(rtc) = &self.rtc_peer_connection {
            rtc.close();
        }
        if let Some(local_str) = self.local_stream.take() {

        }
        if let Some(rem_str) = self.remote_stream.take() {
            
        }
    }

}