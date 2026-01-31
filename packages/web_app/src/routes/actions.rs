use crate::{gate::Gate, routes::Routes};
use api::web_app::PermissionType;
use dioxus::prelude::*;

#[component]
pub fn Actions() -> Element {
    rsx! {
        Gate {
            required_permission: PermissionType::Events,
            permission_fallback_route: Routes::Dashboard {}.to_string(),
            div { class: "w-full",
                h1 { class: "text-primary font-bold text-xl", "Actions" }
                p { "Welcome to the actions page!" }
            }
        }
    }
}
