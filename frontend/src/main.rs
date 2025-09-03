use routes::auth::{login_route, main_page, register_route};

use yew_router::prelude::*;
use yew::prelude::*;

mod common;
mod routes;
mod rtc_network_manager;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/main")]
    UserMain,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
}

// #[function_component(Secure)]
// fn secure() -> Html {
//     let navigator = use_navigator().unwrap();

//     let onclick = Callback::from(move |_| navigator.push(&Route::Home));
//     html! {
//         <div>
//             <h1>{ "Secure" }</h1>
//             <button {onclick}>{ "Go Home" }</button>
//         </div>
//     }
// }

fn switch(routes: Route) -> Html {
    match routes {
        Route::UserMain => main_page(),
        Route::Login => login_route(),
        Route::Register => register_route(),
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}