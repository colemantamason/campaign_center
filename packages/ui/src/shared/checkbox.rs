use dioxus::prelude::*;
use lucide_dioxus::{Check, Square};
#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use web_sys::{window, HtmlInputElement};

#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    id: String,
    label: String,
    required: Option<Memo<bool>>,
    value: Signal<bool>,
}

#[component]
pub fn Checkbox(mut props: CheckboxProps) -> Element {
    let id = props.id.clone();

    use_effect(move || {
        #[cfg(feature = "web")]
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.get_element_by_id(&id) {
                    if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
                        if props.required.as_ref().map(|r| *r.read()).unwrap_or(false)
                            && !*props.value.read()
                        {
                            input.set_custom_validity("Please select this checkbox");
                        } else {
                            input.set_custom_validity("");
                        }
                    }
                }
            }
        }
    });

    rsx! {
        div { class: "relative flex items-center cursor-pointer",
            input {
                class: "absolute left-0 w-6 h-6 opacity-0 z-10 cursor-pointer",
                r#type: "checkbox",
                id: "{props.id}",
                name: "{props.id}",
                value: "{props.value}",
                required: props.required.as_ref().map(|r| *r.read()),
                onchange: move |_| {
                    props.value.toggle();
                },
            }
            if *props.value.read() {
                div { class: "relative flex justify-center items-center w-6 h-6",
                    Square { class: "absolute w-6 h-6 fill-primary" }
                    Check { class: "relative z-0 w-4 h-4 text-primary-foreground stroke-[3px]" }
                }
            } else {
                Square { class: "w-6 h-6 text-border" }
            }
            label {
                class: "text-primary text-sm pl-2 cursor-pointer",
                r#for: "{props.id}",
                "{props.label}"
            }
        }
    }
}
