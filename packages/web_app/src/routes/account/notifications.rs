use dioxus::prelude::*;

#[component]
pub fn NotificationPreferences() -> Element {
    rsx! {
        div { class: "w-full",
            h1 { class: "text-primary font-bold text-xl", "Notification Preferences" }
            p { "Welcome to the notification preferences page!" }
        }
    }
}
