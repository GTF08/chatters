use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{js_sys, wasm_bindgen};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/audio_recorder.js")]
extern "C" {
    async fn startRecording(callback: &wasm_bindgen::JsValue);
    async fn stopRecording() -> js_sys::Promise;
    fn destroy();
}

pub struct AudioRecorderService {
    callback: Rc<RefCell<Closure<dyn Fn(JsValue) + 'static>>>,
}

impl AudioRecorderService {
    pub fn new(f: impl Fn(JsValue) + 'static) -> AudioRecorderService {
        AudioRecorderService {
            callback : Rc::new(RefCell::new(Closure::wrap(Box::new(f) as Box<dyn Fn(wasm_bindgen::JsValue)>))),
        }
    }

    pub async fn start(&self) {
        startRecording(&*self.callback.borrow().as_ref().unchecked_ref()).await;
    }

    pub async fn stop(&self) -> js_sys::Promise {
        stopRecording().await
    }
}

impl Drop for AudioRecorderService {
    fn drop(&mut self) {
        destroy();
    }
}