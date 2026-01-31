use crate::gate::Gate;
use dioxus::prelude::*;

#[component]
pub fn DeviceSessions() -> Element {
    rsx! {
        Gate {
            div { class: "w-full",
                h1 { class: "text-primary font-bold text-xl", "Device Sessions" }
                p { "Welcome to the device sessions page!" }
            }
        }
    }
}
