use crate::gate::Gate;
use dioxus::prelude::*;

#[component]
pub fn Analytics() -> Element {
    rsx! {
        Gate {
            div { class: "w-full",
                h1 { class: "text-primary font-bold text-xl", "Analytics" }
                p { "Welcome to the analytics page!" }
            }
        }
    }
}
