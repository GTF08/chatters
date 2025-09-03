use yew::prelude::*;
use yew::Properties;


#[derive(Properties, PartialEq)]
pub struct TextInputProperty {
    pub id: AttrValue,
    pub input_type: AttrValue,
    pub name: AttrValue,
    #[prop_or(false)]
    pub required: bool
    //pub onchange: Callback<Event>
}

#[function_component(TextInput)]
pub fn text_input(property: &TextInputProperty) -> Html {
    html! {
        <input type={&property.input_type} id={&property.id} name={&property.name} required={property.required} />//onchange={&property.onchange} />
    }
}