use gloo::net::http::Request;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{common::atoms::{
    buttons::custom_button::CustomButton, errors::error_display::ErrorDisplay, input::text_input::TextInput
}, Route};
use shared::models::users::LoginUserSchema;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let navigator = use_navigator().unwrap();

    let email = use_state(|| String::new());
    let password = use_state(|| String::new());

    let on_email_input = {
        let current_email = email.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_email.set(input.value());
        })
    };

    let on_password_input = {
        let current_password = password.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_password.set(input.value());
        })
    };
    //state holding post result
    let error_state = use_state(|| Ok(()));
    
    let on_submit = {
        let state = error_state.clone();
        Callback::from(
            move |e: MouseEvent| {
                let email = email.clone();
                let password = password.clone();

                let navigator = navigator.clone();
                let state = state.clone();

                
                wasm_bindgen_futures::spawn_local(async move {
                    let _result = 
                        Request::post("https://192.168.0.3:3000/api/login")
                        .credentials(web_sys::RequestCredentials::Include)
                        .header("Content-Type", "application/json")
                        //.header("Access-Control-Allow-Origin", "http://localhost:8080")
                        .body(serde_json::to_string(&
                            LoginUserSchema{
                                email: (*email).clone(), 
                                password: (*password).clone()
                            }
                        ).expect("Error during serialization"))
                        .expect("Error duriong body initialization")
                        .send()
                        .await
                        .map_err(|e| e.to_string());

                    match _result {
                        Err(e) => {
                            state.set(Err(e));
                            return;
                        },
                        Ok(response) => {
                            if !response.ok() {
                                state.set(Err(response.text().await.unwrap()));
                            } else {
                                state.set(Ok(()));
                                navigator.push(&Route::UserMain);
                            }
                        }
                    }
                });
            }
        )
    };

    let error = (*error_state).clone();

    return html! {
        <div class="form">
            <label for="Email"> {"Email:"} </label><br/>
            <input type="text" name="email" id="email_input" oninput={on_email_input}/><br/>
            <label for="Password"> {"Password:"} </label><br/>
            <input type="password" name="password" id="password_input" required=true oninput={on_password_input}/><br/>
            <CustomButton label="Submit" onclick={on_submit}/>
            {
                if error.is_err() {html!{
                    <ErrorDisplay>
                        {error.unwrap_err()}
                    </ErrorDisplay>
                }}
                else {
                    html_nested!{}
                }
                //if (*error_state).is_some() {html_nested!{<div>{"Test"}</div>}} else {html_nested!{}}
            }
        </div>
    }

}