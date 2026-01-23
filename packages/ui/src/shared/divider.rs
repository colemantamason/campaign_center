use dioxus::prelude::*;

#[component]
pub fn Divider() -> Element {
    rsx! {
        div { class: "h-px bg-border" }
    }
}
