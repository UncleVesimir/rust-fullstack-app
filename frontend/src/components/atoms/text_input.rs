use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub placeholder: String,
    pub input_type: String,
    pub onchange_handler: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let onchange = {
        let onchange_handler = props.onchange_handler.clone();
        Callback::from(move |event: Event| {

            let target = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                onchange_handler.emit(input.value());
            }
        })
    };

    html!(
        <input placeholder={props.placeholder.clone()} type={props.input_type.clone()} onchange={onchange} />
    )
}
