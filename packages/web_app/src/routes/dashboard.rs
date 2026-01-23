use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "w-full",
            h1 { class: "text-primary font-bold text-xl", "Dashboard" }
            p { "Welcome to the dashboard!" }
        }
    }
}
