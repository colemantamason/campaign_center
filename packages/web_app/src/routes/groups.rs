use dioxus::prelude::*;

#[component]
pub fn Groups() -> Element {
    rsx! {
        div { class: "w-full",
            h1 { class: "text-primary font-bold text-xl", "Groups" }
            p { "Welcome to the groups page!" }
        }
    }
}
