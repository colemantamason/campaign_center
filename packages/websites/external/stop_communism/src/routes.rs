mod home;
mod privacy;
mod terms;

#[cfg(feature = "web")]
use chrono::{Datelike, Utc};
use dioxus::prelude::*;
use home::Home;
use privacy::Privacy;
use terms::Terms;

#[component]
fn Layout() -> Element {
    #[cfg(feature = "web")]
    let year = Utc::now().year().to_string();

    rsx! {
        div { class: "flex flex-col min-h-screen bg-background text-foreground font-sans items-center w-full",
            main { class: "flex-grow flex flex-col items-center w-full", Outlet::<Routes> {} }
            footer { class: "mt-12 py-8 px-4 border-t border-border bg-primary text-primary-foreground w-full",
                div { class: "flex flex-col items-center justify-center gap-4 text-sm text-center",
                    p {
                        {
                            #[cfg(feature = "web")]
                            format!("© {} Stop Communism PAC. All rights reserved.", year)
                        }
                    }
                    div { class: "flex gap-4",
                        Link { class: "hover:underline", to: Routes::Privacy {}, "Privacy Policy" }
                        Link { class: "hover:underline", to: Routes::Terms {}, "Terms of Service" }
                    }
                    div { class: "text-xs opacity-75 max-w-lg text-center border-background border p-2",
                        p { "Paid for by Stop Communism PAC." }
                        p { "Not authorized by any candidate or candidate's committee." }
                    }
                }
            }
        }
    }
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
pub enum Routes {
    #[layout(Layout)]

    #[route("/")]
    Home {},

    #[route("/privacy")]
    Privacy {},

    #[route("/terms")]
    Terms {},
}

#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Routes::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}
