use axum::{extract::State, response::IntoResponse, Extension};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use redis::AsyncCommands;

use shared::models::messages::{GetMessagesRequestData, MessageCreateSchema, NewVoiceMessageSchema};
use tokio::sync::Mutex;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::{MediaEngine, MIME_TYPE_OPUS, MIME_TYPE_VP8};
use webrtc::api::APIBuilder;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::data_channel::RTCDataChannel;
use webrtc::ice_transport::ice_candidate::{RTCIceCandidate, RTCIceCandidateInit};
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::rtcp::payload_feedbacks::picture_loss_indication::PictureLossIndication;
use webrtc::rtp_transceiver::rtp_codec::{RTCRtpCodecCapability, RTCRtpHeaderExtensionCapability, RTPCodecType};
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::track::track_local::track_local_static_sample::TrackLocalStaticSample;

use tokio::time::Duration;
use webrtc::track::track_local::{TrackLocal, TrackLocalWriter};
use std::collections::HashMap;
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

//use shared::models::messages::ChatMessagesDTO;
use uuid::Uuid;

use shared::models::websocket::{WebSocketMessage, RTCMessages};



lazy_static::lazy_static! {
    static ref PEER_CONNECTION_MUTEX: Arc<Mutex<Option<Arc<RTCPeerConnection>>>> =
        Arc::new(Mutex::new(None));
    static ref PENDING_CANDIDATES: Arc<Mutex<Vec<RTCIceCandidate>>> = Arc::new(Mutex::new(vec![]));
    static ref ADDRESS: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}


pub async fn webrtc_answer(
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
    let (sender, mut reader) = ws.split();
    let sender = Arc::new(Mutex::new(sender));

    let mut m = MediaEngine::default();
    if let Err(e) =  m.register_default_codecs() {
        panic!("{}", e)
    }

    let mut registry = Registry::new();
    registry = match register_default_interceptors(registry, &mut m) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();

    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:localhost:3478".to_owned()],
            ..Default::default()
        }],
        ..Default::default()
    };

    let peer_connection = Arc::new(
        match api.new_peer_connection(config).await {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }
    );

    let mut output_tracks = HashMap::new();
    let mut media = vec![];
    media.push("audio");
    media.push("video");

    for s in media {
        let output_track = Arc::new(TrackLocalStaticRTP::new(
            RTCRtpCodecCapability {
                mime_type: if s == "video" {
                    MIME_TYPE_VP8.to_owned()
                } else {
                    MIME_TYPE_OPUS.to_owned()
                },
                ..Default::default()
            },
            format!("track-{s}"),
            "webrtc-rs".to_owned(),
        ));

        // Add this newly created track to the PeerConnection
        let rtp_sender = match peer_connection
            .add_track(Arc::clone(&output_track) as Arc<dyn TrackLocal + Send + Sync>)
            .await {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            };
            

        // Read incoming RTCP packets
        // Before these packets are returned they are processed by interceptors. For things
        // like NACK this needs to be called.
        let m = s.to_owned();
        tokio::spawn(async move {
            let mut rtcp_buf = vec![0u8; 1500];
            while let Ok((_, _)) = rtp_sender.read(&mut rtcp_buf).await {}
            println!("{m} rtp_sender.read loop exit");
            Result::<(), webrtc::Error>::Ok(())
        });

        output_tracks.insert(s.to_owned(), output_track);
    }

    let offer = async {
        while let Some(Ok(message)) = reader.next().await {
            match message {
                Message::Text(utf8_bytes) => {
                    let websocket_message = serde_json::from_str::<WebSocketMessage<RTCMessages>>(&utf8_bytes).unwrap();
                    match websocket_message.message_type {
                        RTCMessages::NewOffer(offer_str) => {
                            let sdp = match serde_json::from_str::<RTCSessionDescription>(&offer_str) {
                                Ok(value) => value,
                                Err(e) => panic!("{}", e),
                            };
                            return Some(sdp)
                        },
                        _ => {}
                    }
                },
                Message::Binary(bytes) => {},
                Message::Ping(bytes) => {},
                Message::Pong(bytes) => {},
                Message::Close(close_frame) => {}
            }
        };
        None
    }.await;

    if let None = offer {
        panic!("Failed to read offer from websocker")
    }

    if let Err(e) = peer_connection.set_remote_description(offer.unwrap()).await {
        panic!("{}", e)
    };

    let pc = Arc::downgrade(&peer_connection);
    peer_connection.on_track(Box::new(
        move |track, _, _| {
            let media_ssrc = track.ssrc();

            if track.kind() == RTPCodecType::Video {
                let pc2 = pc.clone();
                tokio::spawn(async move {
                    let mut result = Result::<usize, webrtc::Error>::Ok(0);
                    while result.is_ok() {
                        let timeout = tokio::time::sleep(Duration::from_secs(3));
                        tokio::pin!(timeout);

                        tokio::select! {
                            _ = timeout.as_mut() =>{
                                if let Some(pc) = pc2.upgrade(){
                                    result = pc.write_rtcp(&[Box::new(PictureLossIndication{
                                            sender_ssrc: 0,
                                            media_ssrc,
                                    })]).await.map_err(Into::into);
                                }else{
                                    break;
                                }
                            }
                        };
                    }
                });
            }
            let kind = if track.kind() == RTPCodecType::Audio {
                "audio"
            } else {
                "video"
            };
            let output_track = if let Some(output_track) = output_tracks.get(kind) {
                Arc::clone(output_track)
            } else {
                println!("output_track not found for type = {kind}");
                return Box::pin(async {});
            };

            let output_track2 = Arc::clone(&output_track);
            tokio::spawn(async move {
                println!(
                    "Track has started, of type {}: {}",
                    track.payload_type(),
                    track.codec().capability.mime_type
                );
                // Read RTP packets being sent to webrtc-rs
                while let Ok((rtp, _)) = track.read_rtp().await {
                    if let Err(err) = output_track2.write_rtp(&rtp).await {
                        println!("output track write_rtp got error: {err}");
                        break;
                    }
                }

                println!(
                    "on_track finished, of type {}: {}",
                    track.payload_type(),
                    track.codec().capability.mime_type
                );
            });
            Box::pin(async {})
        }
    ));

    let (done_tx, mut done_rx) = tokio::sync::mpsc::channel::<()>(1);

    peer_connection.on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
        println!("Peer Connection State has changed: {s}");

        if s == RTCPeerConnectionState::Failed {
            // Wait until PeerConnection has had no network activity for 30 seconds or another failure. It may be reconnected using an ICE Restart.
            // Use webrtc.PeerConnectionStateDisconnected if you are interested in detecting faster timeout.
            // Note that the PeerConnection may come back from PeerConnectionStateDisconnected.
            println!("Peer Connection has gone to failed exiting");
            let _ = done_tx.try_send(());
        }

        Box::pin(async {})
    }));

    let answer = match peer_connection.create_answer(None).await {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let mut gather_complete = peer_connection.gathering_complete_promise().await;
    if let Err(e) = peer_connection.set_local_description(answer).await {
        panic!("{}", e)
    };
    let _ = gather_complete.recv().await;

    if let Some(local_desc) = peer_connection.local_description().await {
        let json_str = match serde_json::to_string(&local_desc) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };
        let wsmessage = WebSocketMessage::<RTCMessages> {
            message_type: RTCMessages::NewAnswer(
                json_str
            )
        };
        let wsmsg_json = serde_json::to_string(&wsmessage).unwrap();
        sender.lock().await.send(
            Message::Text(wsmsg_json.into())
        ).await.unwrap()
    } else {
        println!("generate local_description failed!");
    }

    println!("Press ctrl-c to stop");
    //let timeout = tokio::time::sleep(Duration::from_secs(20));
    //tokio::pin!(timeout);

    tokio::select! {
        //_ = timeout.as_mut() => {
        //    println!("received timeout signal!");
        //}
        _ = done_rx.recv() => {
            println!("received done signal!");
        }
        _ = tokio::signal::ctrl_c() => {
            println!();
        }
    };

    if let Err(e) = peer_connection.close().await {
        panic!("{}", e)
    };

    //Ok(())

}


async fn read_from_client(state: &AppState, current_user: &CurrentUser, mut sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>, mut receiver: SplitStream<WebSocket>) {
    while let Some(Ok(message)) = receiver.next().await {
        match message {
            Message::Text(utf8_bytes) => {
                let websocket_message = serde_json::from_str::<WebSocketMessage<RTCMessages>>(&utf8_bytes).unwrap();
                match websocket_message.message_type {
                    RTCMessages::NewOffer(offer) => {
                        let pc = PEER_CONNECTION_MUTEX.lock().await.clone().unwrap();
                        process_offer(sender, pc, offer).await;
                    },
                    RTCMessages::NewIceCandidate(candidate) => {
                        let pc = PEER_CONNECTION_MUTEX.lock().await.clone().unwrap();
                        process_ice_candidate(pc, candidate).await;
                    },
                    RTCMessages::NewAnswer(_) => unreachable!("NewAnswer should never be recieved on the server"),
                }
            },
            Message::Binary(bytes) => {
                println!("BYTES");
            },
            Message::Ping(bytes) => {},
            Message::Pong(bytes) => {},
            Message::Close(close_frame) => {}
        }
    }
}

async fn write_to_client(sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>, wsmessage: WebSocketMessage<RTCMessages>) {
    if let Ok(value) = serde_json::to_string(&wsmessage) {
        sender.lock().await.send(Message::Text(value.into())).await.unwrap();
    }
}

async fn create_peer_connection(

) -> Arc<RTCPeerConnection> {
    // Prepare the configuration
    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec![
                "stun:localhost:3478".to_owned(),
                // "stun:stun1.l.google.com:19302".to_owned(),
                // "stun:stun.l.google.com:19302".to_owned(),
                // "stun:stun.l.google.com:5349".to_owned(),
                // "stun:stun1.l.google.com:3478".to_owned(),
                // "stun:stun1.l.google.com:5349".to_owned(),
                // "stun:stun2.l.google.com:19302".to_owned(),
                // "stun:stun2.l.google.com:5349".to_owned(),
                // "stun:stun3.l.google.com:3478".to_owned(),
                // "stun:stun3.l.google.com:5349".to_owned(),
                // "stun:stun4.l.google.com:19302".to_owned(),
                // "stun:stun4.l.google.com:5349".to_owned(),
            ],
            ..Default::default()
        }],
        ..Default::default()
    };

    // Create a MediaEngine object to configure the supported codec
    let mut m = MediaEngine::default();
    if let Err(e) = m.register_default_codecs() {
        panic!("{}", e);
    }

    for extension in [
        "urn:ietf:params:rtp-hdrext:sdes:mid",
        "urn:ietf:params:rtp-hdrext:sdes:rtp-stream-id",
        "urn:ietf:params:rtp-hdrext:sdes:repaired-rtp-stream-id",
    ] {
        m.register_header_extension(
            RTCRtpHeaderExtensionCapability {
                uri: extension.to_owned(),
            },
            RTPCodecType::Video,
            None,
        ).unwrap();
    }

    let mut registry = Registry::new();
    registry = match register_default_interceptors(registry, &mut m) {
        Ok(value) => value,
        Err(e) => panic!("{}", e),
    };

    // Create the API object with the MediaEngine
    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();

    // Create a new RTCPeerConnection
    let peer_connection = Arc::new(
        match api.new_peer_connection(config).await {
            Ok(value) => value,
            Err(e) => panic!("{}", e),
        });

    peer_connection.add_transceiver_from_kind(
        webrtc::rtp_transceiver::rtp_codec::RTPCodecType::Video, None
    ).await.unwrap();

    return peer_connection;
}

async fn signal_ice_candidate(
    sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>,
    wsmessage: WebSocketMessage<RTCMessages>
) {
    write_to_client(sender, wsmessage).await;
}

async fn process_ice_candidate(
    pc: Arc<RTCPeerConnection>,
    candidate: String,
) {
    println!("{:?}", candidate);
    if let Err(e) = pc
        .add_ice_candidate(RTCIceCandidateInit{
            candidate,
            ..Default::default()
        })
        .await 
    {
        panic!("{}", e);
    }
}

async fn process_offer(
    sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>,
    pc: Arc<RTCPeerConnection>,
    offer: String
) {
    let sdp = match serde_json::from_str::<RTCSessionDescription>(&offer) {
        Ok(value) => value,
        Err(e) => panic!("{}", e),
    };

    if let Err(e) = pc.set_remote_description(sdp).await {
        panic!("{}", e);
    }

    let answer = match pc.create_answer(None).await {
        Ok(value) => value,
        Err(e) => panic!("{}", e),
    };

    if let Err(e) = pc.set_local_description(answer.clone()).await {
        panic!("{}", e);
    }

    let wsmessage = WebSocketMessage::<RTCMessages> {
        message_type: RTCMessages::NewAnswer(
            match serde_json::to_string(&answer) {
                Ok(value) => value,
                Err(e) => panic!("{}", e)
            }
        )
    };

    write_to_client(sender, wsmessage).await;
}