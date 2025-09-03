
use gloo::net::http::Request;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(Body)]
pub fn body(props: &Props) -> Html {
    html! {
        <div class={classes!(
            "body",
            props.class.clone(),
        )}>
            { props.children.clone() }
        </div>
    }
}