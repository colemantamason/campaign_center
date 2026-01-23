use dioxus::prelude::*;

#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen",
            h1 { class: "text-4xl font-bold mb-4", "404 - Page Not Found" }
            p { class: "text-lg", "The requested page could not be found." }
        }
    }
}
