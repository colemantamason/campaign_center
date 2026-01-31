use crate::{gate::Gate, routes::Routes};
use api::web_app::PermissionType;
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        Gate {
            div { class: "w-full",
                h1 { class: "text-primary font-bold text-xl", "Dashboard" }
                p { "Welcome to the dashboard!" }
            }
        }
    }
}
