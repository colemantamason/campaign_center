pub mod auth;
pub mod gate;
pub mod routes;

use dioxus::prelude::*;
#[cfg(feature = "server")]
use dotenvy::dotenv;
use routes::Routes;

fn main() {
    // load environment variables from .env file (ignored if file doesn't exist)
    #[cfg(feature = "server")]
    dotenv().ok();

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
