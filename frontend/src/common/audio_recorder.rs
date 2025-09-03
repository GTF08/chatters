use std::rc::Rc;

use wasm_bindgen::prelude::*;

use web_sys::js_sys;
use web_sys::wasm_bindgen;
//use web_sys::HtmlButtonElement;
use yew::prelude::*;

use crate::common::services::audio_recording_service::AudioRecorderService;

// #[wasm_bindgen(module = "/js/audio_recorder.js")]
// extern "C" {
//     async fn startRecording(callback: &wasm_bindgen::JsValue);
//     async fn stopRecording() -> js_sys::Promise;
// }

pub enum ComponentMsg {
    Start,
    Stop,
    DataRecieved(Vec<u8>)
}

pub struct AudioRecorder {
    active: bool,
    audio_player: NodeRef,
    recording_service: Rc<AudioRecorderService>
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_audio_recorded: Callback<Vec<u8>>
}

impl Component for AudioRecorder {
    type Message = ComponentMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let on_audio = ctx.link().callback(ComponentMsg::DataRecieved);

        let recording_service = Rc::new(AudioRecorderService::new(
            move |jsvalue: wasm_bindgen::JsValue| {
                let buffer: js_sys::ArrayBuffer = jsvalue.dyn_into().unwrap();
                let uint8_array = js_sys::Uint8Array::new(&buffer);
                let uint8vec = uint8_array.to_vec();
                // let uint8array: js_sys::Uint8Array = jsvalue.dyn_into().unwrap();
                // let uint8vec = uint8array.to_vec();
                on_audio.emit(uint8vec);
            }
        ));
        Self {
            active: false,
            audio_player: NodeRef::default(),
            recording_service
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let start = ctx.link().callback(|_| ComponentMsg::Start);
        let stop = ctx.link().callback(|_| ComponentMsg::Stop);
        html! {
            <div>
                <div ref={self.audio_player.clone()}> </div>
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
            ComponentMsg::Start => {
                self.active = true;
                let recording_service = self.recording_service.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    recording_service.start().await;
                });
                true
            }
            ComponentMsg::Stop => {
                self.active = false;
                let recording_service = self.recording_service.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = recording_service.stop().await;
                });
                true
            },
            ComponentMsg::DataRecieved(bytes) => {
                _ctx.props().on_audio_recorded.emit(bytes);
                false
            }
        }
    }
}