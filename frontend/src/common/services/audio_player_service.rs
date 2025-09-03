use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{js_sys::{self, Uint8Array}, wasm_bindgen};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/audio_player.js")]
extern "C" {
    pub async fn play_audio(bytes: Uint8Array);
    fn destroy();
}
