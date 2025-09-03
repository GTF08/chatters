use yew::{classes, function_component, html};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub children: Html,
}

#[function_component(ErrorDisplay)]
pub fn error_display(props: &Props) -> Html {
    let Props {
        class,
        children,
    } = props;
    
    html! {
        <div
            class={classes!(
                "error-display",
                class.clone(),
            )}
        >
            { children.clone() }
        </div>
    }
}