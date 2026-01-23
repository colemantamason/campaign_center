use crate::shared::input::InputType;
use dioxus::prelude::*;
#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use web_sys::{window, HtmlInputElement};

pub fn unmasked_use_effect_hook(
    id: &str,
    input_type: InputType,
    label: &str,
    value: Signal<String>,
    required: Option<Memo<bool>>,
) {
    #[cfg(feature = "web")]
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id(id) {
                if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
                    if value.read().trim().is_empty() {
                        if required.as_ref().map(|r| *r.read()).unwrap_or(false) {
                            let message = if input_type == InputType::Email {
                                "Please enter a valid email address".to_string()
                            } else {
                                format!("Please enter a valid {}", label.to_lowercase())
                            };
                            input.set_custom_validity(&message);
                        } else {
                            input.set_custom_validity("");
                        }
                    } else {
                        input.set_custom_validity("");
                        if input_type == InputType::Email && !input.check_validity() {
                            input.set_custom_validity("Please enter a valid email address");
                        }
                    }
                }
            }
        }
    }
}

pub fn unmasked_oninput_handler(e: Event<FormData>, mut value: Signal<String>) {
    value.set(e.value());
}
