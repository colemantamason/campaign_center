use dioxus::prelude::*;
#[cfg(feature = "web")]
use dioxus::web::WebEventExt;
use lucide_dioxus::{Check, Square};
#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq, Props)]
pub struct CheckboxProps {
    id: String,
    required: Option<Memo<bool>>,
    value: Signal<bool>,
    label: String,
}

#[component]
pub fn Checkbox(mut props: CheckboxProps) -> Element {
    #[cfg(feature = "web")]
    let mut checkbox_element: Signal<Option<HtmlInputElement>> = use_signal(|| None);

    use_effect(move || {
        #[cfg(feature = "web")]
        if let Some(ref checkbox) = *checkbox_element.read() {
            if props
                .required
                .as_ref()
                .map(|required| *required.read())
                .unwrap_or(false)
                && !*props.value.read()
            {
                checkbox.set_custom_validity("Please select this checkbox");
            } else {
                checkbox.set_custom_validity("");
            }
        }
    });

    rsx! {
        div { class: "relative flex items-center cursor-pointer",
            input {
                class: "absolute left-0 w-6 h-6 opacity-0 z-10 cursor-pointer",
                r#type: "checkbox",
                id: props.id.clone(),
                name: props.id.clone(),
                required: if let Some(required) = props.required { *required.read() } else { false },
                value: props.value.read().to_string(),
                onmounted: move |element| {
                    #[cfg(feature = "web")]
                    {
                        let element = element.data();
                        let checkbox = element.as_web_event().dyn_into::<HtmlInputElement>().ok();
                        checkbox_element.set(checkbox);
                    }
                },
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
                r#for: props.id,
                {props.label}
            }
        }
    }
}
