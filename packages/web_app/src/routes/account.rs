pub mod devices;
pub mod notifications;
pub mod organizations;

use crate::{gate::Gate, routes::Routes};
use api::web_app::PermissionType;
use dioxus::prelude::*;

#[component]
pub fn Account() -> Element {
    rsx! {
        Gate {
            div { class: "w-full",
                h1 { class: "text-primary font-bold text-xl", "Account" }
                p { "Welcome to the account page!" }
            }
        }
    }
}
