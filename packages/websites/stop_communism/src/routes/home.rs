use dioxus::prelude::*;
use ui::shared::form::sms_opt_in::Form;

const AMERICAN_FLAG: Asset = asset!("assets/american_flag.jpg");
const STATUE_OF_LIBERTY: Asset = asset!("assets/statue_of_liberty.jpg");

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "w-full flex flex-col gap-12 items-center",
            section { class: "relative w-full flex flex-col items-center justify-center text-center py-24",
                img {
                    class: "absolute inset-0 w-full h-full object-cover",
                    src: AMERICAN_FLAG,
                    alt: "American Flag",
                    fetchpriority: "high",
                }
                div { class: "absolute inset-0 bg-primary/70" }
                div { class: "relative z-10 flex flex-col items-center gap-6 px-4 text-primary-foreground",
                    h2 { class: "text-4xl md:text-5xl font-bold", "STOP COMMUNISM PAC" }
                    p { class: "text-lg md:text-xl md:max-w-2xl",
                        "We're dedicated to stopping the spread of Communism in America and supporting candidates who fight for the future of our nation."
                    }
                }
            }
            div { class: "w-full flex flex-col gap-12 items-center justify-center px-4",
                section { class: "w-full max-w-md p-6 border border-border rounded-lg shadow-sm gap-6 flex flex-col items-center justify-center",
                    h3 { class: "text-2xl font-bold text-center uppercase",
                        "Join the "
                        span { class: "p-1 bg-secondary text-primary-foreground rounded",
                            "Fight"
                        }
                    }
                    Form {
                        organization_name: "Stop Communism PAC".to_string(),
                        hidden_source: "website".to_string(),
                    }
                }
                section { class: "relative w-full flex flex-col justify-center py-24 max-w-6xl",
                    img {
                        class: "absolute inset-0 w-full h-full object-cover rounded-lg border",
                        src: STATUE_OF_LIBERTY,
                        alt: "Statue of Liberty",
                    }
                    div { class: "absolute inset-0 bg-primary/70 rounded-lg border" }
                    div { class: "relative z-10 flex flex-col gap-6 md:px-12 text-primary-foreground px-4",
                        h2 { class: "text-4xl md:text-5xl font-bold md:leading-normal leading-tight",
                            "HELP US "
                            span { class: "p-1 bg-secondary text-primary-foreground rounded",
                                "STOP COMMUNISM"
                            }
                            " IN AMERICA BEFORE IT'S TOO LATE"
                        }
                        a {
                            class: "flex items-center justify-center bg-secondary text-primary-foreground font-bold py-3 px-6 w-fit rounded hover:bg-opacity-90 text-xl",
                            href: "#",
                            "DONATE NOW"
                        }
                    }
                }
            }
        }
    }
}
