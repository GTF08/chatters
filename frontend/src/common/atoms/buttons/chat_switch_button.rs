use gloo::net::http::Request;
use gloo_timers::callback::Timeout;
use uuid::Uuid;
use yew::{function_component, html, html_nested, use_state, Callback, Html, MouseEvent};
use yew_router::hooks::use_navigator;

use crate::common::atoms::{
    buttons::custom_button::CustomButton, 
    errors::error_overlay_display::ErrorOverlayDisplay
};
use crate::Route;
use yew::Properties;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub chat_id: Uuid,
    pub callback: Callback<Uuid>
}

#[function_component(ChatSwitchButton)]
pub fn chat_switch_button(props: &Props) -> Html {
    //let error_state = use_state(|| Ok(()));
    //let state = error_state.clone();
    let cb = props.callback.clone();
    let id = props.chat_id;
    let onclick = Callback::from(move |_| {
        cb.emit(id);
    });

    html!{<CustomButton label={"Switch"} onclick={onclick}/>}
    // let onclick = {
    //     //let state = state.clone();
    //     Callback::from(
    //         move |e: MouseEvent| {
    //             //let state = state.clone();
    //             {
    //                 let navigator = navigator.clone();
    //                 let state = state.clone();
    //                 wasm_bindgen_futures::spawn_local(async move {
                        
    //                     let _result = 
    //                         Request::get("http://localhost:3000/api/logout")
    //                         .credentials(web_sys::RequestCredentials::Include)
    //                         //.credentials(web_sys::RequestCredentials::Include)
    //                         //.header("Access-Control-Allow-Origin", "http://localhost:3000/")
    //                         .send()
    //                         .await;

    //                     match _result {
    //                         Err(e) => {
    //                             state.set(Err(e.to_string()));
    //                             Timeout::new(3_000, move || {
    //                                 state.set(Ok(()));
    //                             }).forget();
    //                         },
    //                         Ok(response) => {
    //                             if !response.ok() {
    //                                 state.set(Err(response.text().await.unwrap()));
    //                                 Timeout::new(3_000, move || {
    //                                     state.set(Ok(()));
    //                                 }).forget();
    //                             } else {
    //                                 state.set(Ok(()));
    //                                 navigator.push(&Route::Login);
    //                             }
    //                         }
    //                     }
    //                 });
    //             }
    //         }
    //     )
    // };
        
    // match error_state.as_ref() {
    //     Ok(()) => {
    //         html!{
    //             <CustomButton label={"Logout"} onclick={onclick}/>
    //         }
    //     },
    //     Err(e) => {
    //         html!{
    //             <>
    //                 <CustomButton label={"Logout"} onclick={onclick}/>
    //                 <ErrorOverlayDisplay>
    //                     {e}
    //                 </ErrorOverlayDisplay>
    //             </>
    //         }
    //     }
    // }
    //let error = (*error_state).clone();
    // html!({
        
    // })
    // html!{
    //     <div>
    //     <CustomButton label={"Logout"} onclick={onclick}/>
    //     {
    //         if error.is_err() {html!{
    //             <ErrorOverlayDisplay>
    //                 {error.unwrap_err()}
    //             </ErrorOverlayDisplay>
    //         }}
    //         else {
    //             html_nested!{}
    //         }
    //         //if (*error_state).is_some() {html_nested!{<div>{"Test"}</div>}} else {html_nested!{}}
    //     }
    //     </div>
    // }
    // let error = (*error_state).clone();

    // html!{
    //     {if error.is_some() {html_nested!{<div>{error.unwrap()}</div>}} else {html_nested!{<div>{"Successful logout"}</div>}}}
    // }
}