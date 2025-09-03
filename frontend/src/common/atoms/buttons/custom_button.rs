use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct CustomButtonProperty {
    pub label: String,
    pub onclick: Callback<MouseEvent>
}

#[function_component(CustomButton)]
pub fn custom_button(property: &CustomButtonProperty) -> Html {
    html! {
        <button onclick={&property.onclick}> {&property.label} </button>
    }
}