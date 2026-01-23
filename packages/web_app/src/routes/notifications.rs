use dioxus::prelude::*;

#[component]
pub fn Notifications() -> Element {
    rsx! {
        div { class: "w-full",
            h1 { class: "text-primary font-bold text-xl", "Notifications" }
            p { "Welcome to the notifications page!" }
        }
    }
}
