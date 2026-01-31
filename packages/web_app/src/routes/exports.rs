use crate::gate::Gate;
use dioxus::prelude::*;

#[component]
pub fn Exports() -> Element {
    rsx! {
        Gate {
            div { class: "w-full",
                h1 { class: "text-primary font-bold text-xl", "Exports" }
                p { "Welcome to the exports page!" }
            }
        }
    }
}
