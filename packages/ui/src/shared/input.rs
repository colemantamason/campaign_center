mod masked_input;
mod unmasked_input;

use dioxus::prelude::*;
#[cfg(feature = "web")]
use dioxus::web::WebEventExt;
#[cfg(feature = "web")]
use masked_input::{
    get_empty_mask, get_max_len, masked_onblur_handler, masked_onfocus_handler,
    masked_oninput_handler, masked_use_effect_hook,
};
#[cfg(feature = "web")]
use unmasked_input::{unmasked_oninput_handler, unmasked_use_effect_hook};
#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use web_sys::HtmlInputElement;

#[derive(Clone, Copy, PartialEq)]
pub enum InputType {
    Text,
    Email,
    Phone,
    Zip,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InputSize {
    Default,
    Form,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InputVariant {
    Default,
    Sidebar,
}

pub type Id = String;
pub type Required = Memo<bool>;
pub type Value = Signal<String>;
pub type Label = String;

#[derive(Clone, PartialEq, Props)]
pub struct InputProps {
    r#type: InputType,
    id: Id,
    required: Option<Required>,
    value: Value,
    label: Label,
    size: InputSize,
    variant: InputVariant,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    #[cfg(feature = "web")]
    let mut input_element: Signal<Option<HtmlInputElement>> = use_signal(|| None);
    let use_effect_label = props.label.clone();
    let masked_pattern = use_signal(|| "".to_string());

    use_effect(move || {
        #[cfg(feature = "web")]
        if let Some(ref input) = input_element() {
            match props.r#type {
                InputType::Text | InputType::Email => unmasked_use_effect_hook(
                    input,
                    props.r#type,
                    if let Some(required) = props.required {
                        required
                    } else {
                        use_memo(|| false)
                    },
                    props.value,
                    use_effect_label.clone(),
                ),
                InputType::Phone | InputType::Zip => masked_use_effect_hook(
                    input,
                    props.r#type,
                    if let Some(required) = props.required {
                        required
                    } else {
                        use_memo(|| false)
                    },
                    props.value,
                    masked_pattern,
                    get_empty_mask(props.r#type),
                ),
            };
        }
    });

    let input_common_classes = "peer w-full rounded-md border border-border text-foreground placeholder-opacity-0 hover:border-foreground focus:hover:border-opacity-0 focus:border-opacity-0 focus:outline focus:outline-primary focus:outline-2 focus:outline-offset-0";

    let input_size_classes = match props.size {
        InputSize::Default => "px-2 py-2 text-sm",
        InputSize::Form => "px-2 py-3 text-base",
    };

    let input_variant_classes = match props.variant {
        InputVariant::Default => "bg-background",
        InputVariant::Sidebar => "bg-sidebar",
    };

    let label_common_classes = "pointer-events-none absolute left-2 px-2 transition-all duration-150 -translate-y-1/2 text-foreground/75 top-0 peer-focus:top-0 peer-focus:text-primary";

    let label_size_classes = match props.size {
        InputSize::Default => "text-xs peer-placeholder-shown:top-1/2 peer-placeholder-shown:text-sm peer-focus:text-xs",
        InputSize::Form => "text-sm peer-placeholder-shown:top-1/2 peer-placeholder-shown:text-base peer-focus:text-sm",
    };

    let label_variant_classes = match props.variant {
        InputVariant::Default => "bg-background peer-focus-shown:bg-background",
        InputVariant::Sidebar => "bg-sidebar peer-focus-shown:bg-sidebar",
    };

    let combined_input_classes = format!(
        "{} {} {}",
        input_common_classes, input_size_classes, input_variant_classes
    );
    let combined_label_classes = format!(
        "{} {} {}",
        label_common_classes, label_size_classes, label_variant_classes
    );

    rsx! {
        div { class: "relative",
            input {
                r#type: match props.r#type {
                    InputType::Text => "text",
                    InputType::Email => "email",
                    InputType::Phone => "tel",
                    InputType::Zip => "text",
                },
                id: props.id.clone(),
                name: props.id.clone(),
                placeholder: " ",
                pattern: match props.r#type {
                    InputType::Phone | InputType::Zip => {
                        if masked_pattern() != "" { Some(masked_pattern()) } else { None }
                    }
                    _ => None,
                },
                required: if let Some(required) = props.required { required() } else { false },
                value: (props.value)(),
                class: "{combined_input_classes}",
                onmounted: move |element| {
                    #[cfg(feature = "web")]
                    {
                        let element = element.data();
                        let input = element.as_web_event().dyn_into::<HtmlInputElement>().ok();
                        input_element.set(input);
                    }
                },
                onfocus: move |_| {
                    match props.r#type {
                        InputType::Phone | InputType::Zip => {
                            #[cfg(feature = "web")]
                            if let Some(ref input) = input_element() {
                                masked_onfocus_handler(
                                    input,
                                    props.r#type,
                                    props.value,
                                    get_empty_mask(props.r#type),
                                );
                            }
                        }
                        _ => {}
                    }
                },
                onblur: move |_| {
                    match props.r#type {
                        InputType::Phone | InputType::Zip => {
                            #[cfg(feature = "web")]
                            masked_onblur_handler(props.value, get_empty_mask(props.r#type));
                        }
                        _ => {}
                    }
                },
                oninput: move |event| {
                    match props.r#type {
                        InputType::Email | InputType::Text => {
                            #[cfg(feature = "web")]
                            unmasked_oninput_handler(event, props.value);
                        }
                        InputType::Phone | InputType::Zip => {
                            #[cfg(feature = "web")]
                            if let Some(ref input) = input_element() {
                                masked_oninput_handler(
                                    event,
                                    input,
                                    props.r#type,
                                    props.value,
                                    get_max_len(props.r#type),
                                );
                            }
                        }
                    }
                },
            }
            label { r#for: props.id, class: "{combined_label_classes}", {props.label} }
        }
    }
}
