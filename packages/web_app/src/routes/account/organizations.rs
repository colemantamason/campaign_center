use dioxus::prelude::*;

#[component]
pub fn OrganizationManagement() -> Element {
    rsx! {
        div { class: "w-full",
            h1 { class: "text-primary font-bold text-xl", "Organization Management" }
            p { "Welcome to the organization management page!" }
        }
    }
}
