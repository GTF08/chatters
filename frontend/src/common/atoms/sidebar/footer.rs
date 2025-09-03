
use gloo::net::http::Request;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub children: Html,
}

#[function_component(Footer)]
pub fn footer(props: &Props) -> Html {
    html! {
        <div class={classes!(
            "footer",
            props.class.clone(),
        )}>
            { props.children.clone() }
        </div>
    }
}