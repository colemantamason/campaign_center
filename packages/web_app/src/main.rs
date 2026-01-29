pub mod routes;

use dioxus::prelude::*;
use routes::Routes;
use ui::web_app::toast::ToastProvider;

fn main() {
    dioxus::launch(App);
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const STYLESHEET: Asset = asset!("/assets/style.css");

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: STYLESHEET }
        ToastProvider { Router::<Routes> {} }
    }
}
