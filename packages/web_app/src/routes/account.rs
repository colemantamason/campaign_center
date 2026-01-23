pub mod devices;
pub mod notifications;
pub mod organizations;

use dioxus::prelude::*;

#[component]
pub fn Account() -> Element {
    rsx! {
        div { class: "w-full",
            h1 { class: "text-primary font-bold text-xl", "Account" }
            p { "Welcome to the account page!" }
        }
    }
}
