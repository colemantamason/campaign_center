use dioxus::prelude::*;

#[component]
pub fn Actions() -> Element {
    rsx! {
        div { class: "w-full",
            h1 { class: "text-primary font-bold text-xl", "Actions" }
            p { "Welcome to the actions page!" }
        }
    }
}
