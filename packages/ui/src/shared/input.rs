mod masked_input;
mod unmasked_input;

use dioxus::prelude::*;
use masked_input::{
    get_empty_mask, get_max_len, masked_onblur_handler, masked_onfocus_handler,
    masked_oninput_handler, masked_use_effect_hook,
};
use unmasked_input::{unmasked_oninput_handler, unmasked_use_effect_hook};

#[derive(Clone, PartialEq, Copy)]
pub enum InputType {
    Text,
    Email,
    Phone,
    Zip,
}

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    input_type: InputType,
    id: String,
    label: String,
    required: Option<Memo<bool>>,
    value: Signal<String>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let is_masked = props.input_type == InputType::Phone || props.input_type == InputType::Zip;
    let masked_pattern = use_signal(|| "");
    let empty_mask = get_empty_mask(props.input_type);
    let max_len = get_max_len(props.input_type);

    let input_type = props.input_type;
    let id = props.id.clone();
    let label = props.label.clone();
    let required = props.required;
    let value = props.value;

    use_effect(move || {
        if is_masked {
            masked_use_effect_hook(&id, input_type, value, required, masked_pattern, empty_mask);
        } else {
            unmasked_use_effect_hook(&id, input_type, &label, value, required);
        }
    });

    rsx! {
        div { class: "relative",
            input {
                class: "peer w-full rounded-md border border-border bg-background px-2 py-3 text-foreground placeholder-opacity-0 hover:border-foreground focus:hover:border-opacity-0 focus:border-opacity-0 focus:outline focus:outline-primary focus:outline-2 focus:outline-offset-0",
                r#type: match props.input_type {
                    InputType::Text => "text",
                    InputType::Email => "email",
                    InputType::Phone => "tel",
                    InputType::Zip => "text",
                },
                id: "{props.id}",
                name: "{props.id}",
                placeholder: " ",
                required: props.required.as_ref().map(|r| *r.read()).unwrap_or(false),
                pattern: if is_masked { Some(masked_pattern()) } else { None },
                value: "{value.read()}",
                onfocus: move |e| {
                    if is_masked {
                        masked_onfocus_handler(e, input_type, value, empty_mask);
                    }
                },
                onblur: move |_| {
                    if is_masked {
                        masked_onblur_handler(value, empty_mask);
                    }
                },
                oninput: move |e| {
                    if is_masked {
                        masked_oninput_handler(e, input_type, value, max_len);
                    } else {
                        unmasked_oninput_handler(e, value);
                    }
                },
            }
            label {
                class: "pointer-events-none absolute left-2 px-2 transition-all duration-150 -translate-y-1/2 text-sm bg-background text-foreground/75 top-0
                        peer-placeholder-shown:top-1/2 peer-placeholder-shown:text-base peer-placeholder-shown:text-foreground/75
                        peer-focus:top-0 peer-focus:text-sm peer-focus-shown:bg-background peer-focus:text-primary",
                r#for: "{props.id}",
                "{props.label}"
            }
        }
    }
}
