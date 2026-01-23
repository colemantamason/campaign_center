use dioxus::prelude::*;

#[component]
pub fn Team() -> Element {
    rsx! {
        div { class: "w-full",
            h1 { class: "text-primary font-bold text-xl", "Team" }
            p { "Welcome to the team page!" }
        }
    }
}
