pub mod auth;
pub mod gate;
pub mod routes;

#[cfg(feature = "server")]
use api::{http::session_middleware, initialize_databases};
#[cfg(feature = "server")]
use axum::middleware;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::server::router;
#[cfg(feature = "server")]
use dotenvy::dotenv;
use routes::Routes;

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        dotenv().ok();
        initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

        Ok(router(App).layer(middleware::from_fn(session_middleware)))
    });
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
