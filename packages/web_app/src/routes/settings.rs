use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div { class: "w-full",
            h1 { class: "text-primary font-bold text-xl", "Settings" }
            p { "Welcome to the settings page!" }
        }
    }
}
