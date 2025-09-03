
use gloo::net::http::Request;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    html! {
        <div class={classes!(
            "header",
            props.class.clone(),
        )}>
            { props.children.clone() }
        </div>
    }
}