use yew::{classes, function_component, html};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub children: Html,
}

#[function_component(ErrorOverlayDisplay)]
pub fn error_overlay_display(props: &Props) -> Html {
    let Props {
        class,
        children,
    } = props;
    
    html! {
        <div
            class={classes!(
                "overlay",
                class.clone(),
            )}
        >
            { children.clone() }
        </div>
    }
}