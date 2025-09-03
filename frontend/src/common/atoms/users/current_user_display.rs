use gloo::net::http::Request;
use yew::prelude::*;
use shared::models::users::FilteredUser;

#[function_component(CurrentUserDisplay)]
pub fn current_user_display() -> Html {
    let state = use_state(|| None);
    let state_clone = state.clone();
    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_user : FilteredUser = 
                Request::get("https://192.168.0.3:3001/me")
                .credentials(web_sys::RequestCredentials::Include)
                //.credentials(web_sys::RequestCredentials::Include)
                //.header("Access-Control-Allow-Origin", "http://localhost:3000/")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            state.set(Some(fetched_user));
        });
    });
    

    match state_clone.as_ref() {
        Some(user) => {
            html!{
                <h3>
                    {user.username.clone()}
                </h3>
            }
        },
        None => {
            html!{<h3>{"No data"}</h3>}
        }
    }

}