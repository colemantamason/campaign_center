use crate::shared::input::InputType;
use dioxus::prelude::*;
#[cfg(feature = "web")]
use web_sys::HtmlInputElement;

#[cfg(feature = "web")]
pub fn unmasked_use_effect_hook(
    input: &HtmlInputElement,
    r#type: InputType,
    label: &str,
    value: Signal<String>,
    required: Option<Memo<bool>>,
) {
    if value.read().trim().is_empty() {
        if required
            .as_ref()
            .map(|required| *required.read())
            .unwrap_or(false)
        {
            let message = match r#type {
                InputType::Email => "Please enter a valid email address".to_string(),
                _ => format!("Please enter a valid {}", label.to_lowercase()),
            };
            input.set_custom_validity(&message);
        } else {
            input.set_custom_validity("");
        }
    } else {
        input.set_custom_validity("");
        match r#type {
            InputType::Email if !input.check_validity() => {
                input.set_custom_validity("Please enter a valid email address");
            }
            _ => {}
        }
    }
}

#[cfg(feature = "web")]
pub fn unmasked_oninput_handler(event: Event<FormData>, mut value: Signal<String>) {
    value.set(event.value());
}
