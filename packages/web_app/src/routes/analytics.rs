use dioxus::prelude::*;

#[component]
pub fn Analytics() -> Element {
    rsx! {
        div { class: "w-full",
            h1 { class: "text-primary font-bold text-xl", "Analytics" }
            p { "Welcome to the analytics page!" }
        }
    }
}
