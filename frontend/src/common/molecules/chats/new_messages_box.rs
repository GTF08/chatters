use uuid::Uuid;
use web_sys::{js_sys, wasm_bindgen::{prelude::Closure, JsCast}, MessageEvent};
use gloo_net::websocket::{futures::WebSocket, Message};
use yew::prelude::*;

// use crate::common::atoms::chats::chats_message_display::ChatMessageDisplay;

use yew::{function_component, html};

// #[derive(PartialEq, Properties)]
// pub struct Props {
//     pub chat_id: Uuid,
//     pub ws: SplitSink<WebSocket, Message>
// }

// #[function_component(NewMessagesBox)]
// pub fn new_messages_box(props: &Props) -> Html {
//     let messages = Vec::<MessageUserDTO>::new();
//     let state = use_state(|| messages);
//     let state_clone = state.clone();
//     let chat_id = props.chat_id.clone();

//     let ws = props.ws.clone();
//     ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

//     let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
//         let mut state = state.clone();
//         if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
//             let array = js_sys::Uint8Array::new(&abuf);
//             gloo::console::log!("I suck so much");
//             let message: MessageUserDTO = serde_json::from_str(&abuf.as_string().unwrap()).unwrap();
//             // here you can for example use Serde Deserialize decode the message
//             // for demo purposes we switch back to Blob-type and send off another binary message
//             state_vec.push(message);
//         } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
//             gloo::console::log!("message event, received blob: {:?}", &blob);
//             // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
//             let fr = web_sys::FileReader::new().unwrap();
//             let fr_c = fr.clone();
//             // create onLoadEnd callback
//             let onloadend_cb = Closure::<dyn FnMut(_)>::new(move |_e: web_sys::ProgressEvent| {
//                 let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
//                 let len = array.byte_length() as usize;
//                 gloo::console::log!("Blob received {}bytes: {:?}", len, array.to_vec());
//                 // here you can for example use the received image/png data
//             });
//             fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
//             fr.read_as_array_buffer(&blob).expect("blob not readable");
//             onloadend_cb.forget();
//         } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
//             gloo::console::log!("message event, received Text: {:?}", &txt);
//             let message: MessageUserDTO = serde_json::from_str::<MessageUserDTO>(&txt.as_string().unwrap()).unwrap();
//             state_vec.push(message);

//         } else {
//             gloo::console::log!("message event, received Unknown: {:?}", e.data());
//         }
//     });

//     ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
//     onmessage_callback.forget();
    

//     html!{
//         <div class="message-box">
//         {
//             state_clone.iter().map(|message| {
//                 html!{ <ChatMessageDisplay message_data={message.clone()} /> }
//             }).collect::<Html>()
//         }
//         </div>
//     }
//     // match state_clone.as_ref() {
//     //     Some(messages) => {
//     //         messages.into_iter().map(|message| {
//     //             html!{ <ChatMessageDisplay message_data={(*message).clone()} /> }
//     //         }).collect::<Html>()
//     //     },
//     //     None => {
//     //         html!{"WTF"}
//     //     }
//     // }

// }