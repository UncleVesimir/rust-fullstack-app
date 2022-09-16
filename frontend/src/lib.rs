use std::ops::Deref;
use reqwasm::http::Request;
use serde_json::json;
use stylist::{style, yew::styled_component};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;
use gloo::console::log;
use serde::{Serialize, Deserialize};
mod components;

// use common::model::support_request::{SupportRequest};

use components::atoms::text_input::TextInput;

#[derive(Serialize, Deserialize, Default, Clone)]
struct SupportRequestFormData {
    pub req_name: String,
    pub req_email: String,
    pub support_type: String,
    pub sup_message: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    //handlers
    let state = use_state(|| SupportRequestFormData::default());
    //handlers
    let name_onchange = {
        let state = state.clone();
        Callback::from(move |value| {
            state.set(SupportRequestFormData {
                req_name: value,
                ..state.deref().clone()
            })
        })
    };

    let email_onchange = {
        let state = state.clone();
        Callback::from(move |value| {
            state.set(SupportRequestFormData {
                req_email: value,
                ..state.deref().clone()
            })
        })
    };

    let supp_type_onchange = {
        let state = state.clone();
        Callback::from(move |event: Event| {
            let select = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
        
            if let Some(select) = select {
                state.set(SupportRequestFormData {
                    support_type: select.value(),
                    ..state.deref().clone()
                });
            }
        })
    };

    let message_onchange = {
        let state = state.clone();
        Callback::from(move |event: Event| {
            let select = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            if let Some(select) = select {
                state.set(SupportRequestFormData {
                    sup_message: select.value(),
                    ..state.deref().clone()
                });
            }
        })
    };

    let onsubmit = {
        let state = state.clone();
        Callback::from(move |event:FocusEvent| {
            event.prevent_default();
            let body = serde_json::to_string(&*state).ok();
            spawn_local(async move {
                let response = Request::post("/api/create_request")
                .header("content-type", "application/json")
                .body(body)
                .send()
                .await
                .unwrap();
            });
            

            // log!(serde_json::to_string_pretty(&(*state)).unwrap());
        })
    };
    //style
    let style = style!(
        r#"
        input, select, textarea {
            display: block;
        }
    "#
    )
    .unwrap_or_else(|_| style!("").unwrap());

    html!(
        <div class={style}>
            <h1>{"THIS IS A PAGE!"}</h1>
            <form onsubmit={onsubmit}>
            <TextInput placeholder="Full name" input_type={"text"} onchange_handler={name_onchange}/>
            <TextInput placeholder="Email" input_type={"email"} onchange_handler={email_onchange}/>
            <label />
            <select name="support_types" id="support_type_select" onchange={supp_type_onchange}>
                <option value="">{"--Please choose an option--"}</option>
                <option value="general_help">{"Help with Product"}</option>
                <option value="refund">{"Request a Refund"}</option>
                <option value="subscription">{"Cancel Subscription"}</option>
            </select>
            <textarea maxlength="1024" onchange={message_onchange}/>
            <button>{"Submit"}</button>
            </form>
        </div>
    )
}
