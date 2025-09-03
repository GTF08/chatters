use gloo::net::http::Request;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{common::atoms::{buttons::custom_button::CustomButton, input::text_input::TextInput}, Route};
use shared::models::users::RegisterUserSchema;


#[function_component(RegisterForm)]
pub fn register_form() -> Html {
    //state holding post result
    let navigator = use_navigator().unwrap();
    let error_state = use_state(|| Ok(()));
    let error_state_clone = error_state.clone();
    
    let onclick = {
        //let state = error_state.clone();
        
        Callback::from(
            move |e: MouseEvent| {
                let navigator = navigator.clone();
                let state = error_state_clone.clone();

                let document = web_sys::window().unwrap().document().unwrap();
                let email_input = document
                                            .get_element_by_id("email_input")
                                            .unwrap()
                                            .dyn_into::<HtmlInputElement>()
                                            .unwrap()
                                            .value();

                let username_input = document
                                            .get_element_by_id("username_input")
                                            .unwrap()
                                            .dyn_into::<HtmlInputElement>()
                                            .unwrap()
                                            .value();
                                                
                let password_input = document
                                            .get_element_by_id("password_input")
                                            .unwrap()
                                            .dyn_into::<HtmlInputElement>()
                                            .unwrap()
                                            .value();

                let password_verify_input = document
                                            .get_element_by_id("password_verify_input")
                                            .unwrap()
                                            .dyn_into::<HtmlInputElement>()
                                            .unwrap()
                                            .value();

                if password_input != password_verify_input {
                    state.set(Err("Passwords do not match".to_string()));
                    return;
                }
                
                wasm_bindgen_futures::spawn_local(async move {
                    let _result = 
                        Request::post("https://192.168.0.3:3000/api/register")
                        .credentials(web_sys::RequestCredentials::Include)
                        .header("Content-Type", "application/json")
                        //.header("Access-Control-Allow-Origin", "http://localhost:8080")
                        .body(serde_json::to_string(
                            &RegisterUserSchema{
                                email: email_input, 
                                username: username_input,
                                password: password_input
                            }
                        ).expect("Error during serialization"))
                        .expect("Error during body initialization")
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
                                navigator.push(&Route::Login);
                            }
                        }
                    }
                });
            }
        )
    };

    let error = (*error_state).clone();

    return html! {
        <div>
            <label for="email"> {"Email:"} </label><br/>
            <TextInput input_type="text" name="email" id="email_input"/>
            <label for="username"> {"Username:"} </label><br/>
            <TextInput input_type="text" name="username" id="username_input"/>
            <label for="password"> {"Password:"} </label><br/>
            <TextInput input_type="password" name="password" id="password_input" required=true/>
            <label for="password_verify"> {"Verify Passford:"} </label><br/>
            <TextInput input_type="password" name="password_verify" id="password_verify_input" required=true/>
            <CustomButton label="Submit" onclick={onclick}/>
            {
                if error.is_err() {html_nested!{<div>{error.unwrap_err()}</div>}} else {html_nested!{}}
                //if (*error_state).is_some() {html_nested!{<div>{"Test"}</div>}} else {html_nested!{}}
            }
        </div>
    }

}