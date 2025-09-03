use yew::{classes, function_component, html};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &Props) -> Html {
    let Props {
        class,
        children,
    } = props;
    
    html! {
        <div
            class={classes!(
                "sidebar",
                class.clone(),
            )}
        >
            { children.clone() }
        </div>
    }
}