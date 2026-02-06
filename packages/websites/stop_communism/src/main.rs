mod routes;

use dioxus::prelude::*;
use routes::Routes;
use std::env::current_exe;

// SSG launch configuration
fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder()
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        .static_dir(
                            current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

const STYLESHEET: Asset = asset!("/assets/style.css");
const FAVICON: Asset = asset!("/assets/favicon.png");

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Stop Communism PAC" }
        document::Link { rel: "stylesheet", href: STYLESHEET, fetchpriority: "high" }
        document::Link {
            rel: "icon",
            sizes: "32x32",
            r#type: "image/png",
            href: FAVICON,
        }
        document::Meta {
            name: "description",
            content: "Stop Communism PAC is dedicated to stopping the spread of Communism in America and supporting candidates who fight for the future of our nation.",
        }
        Router::<Routes> {}
    }
}
