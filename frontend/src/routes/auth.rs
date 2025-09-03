use yew::prelude::*;

use crate::common::pages::login_page::LoginPage;
use crate::common::molecules::forms::register_form::RegisterForm;
use crate::common::pages::main_page::MainPage;


pub fn login_route() -> Html {
    html!{
        <LoginPage/>
    }
}

pub fn register_route() -> Html {
    html!{<RegisterForm />}
}

pub fn main_page() -> Html {
    html!{<MainPage />}
}