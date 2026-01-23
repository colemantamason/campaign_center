use dioxus::prelude::*;

#[component]
pub fn Events() -> Element {
    rsx! {
        div { class: "w-full",
            h1 { class: "text-primary font-bold text-xl", "Events" }
            p { "Welcome to the events page!" }
        }
    }
}
