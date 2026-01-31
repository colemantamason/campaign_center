pub mod gate;
pub mod routes;

use dioxus::prelude::*;
use routes::Routes;

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
        Router::<Routes> {}
    }
}
