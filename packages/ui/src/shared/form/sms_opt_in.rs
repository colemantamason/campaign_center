use crate::shared::{
    button::{Button, ButtonSize, ButtonType, ButtonVariant},
    checkbox::Checkbox,
    form::FormStatus,
    input::{masked_input::get_empty_mask, Input, InputSize, InputType, InputVariant},
};
use dioxus::prelude::*;
#[cfg(feature = "web")]
use gloo::net::http::Request;
use lucide_dioxus::CircleCheck;
#[cfg(feature = "web")]
use web_sys::{FormData, RequestMode};

#[derive(Clone, PartialEq, Props)]
pub struct FormProps {
    organization_name: String,
    hidden_source: String,
}

#[component]
pub fn Form(props: FormProps) -> Element {
    let mut status = use_signal(|| FormStatus::Idle);
    let full_name_required = use_memo(move || true);
    let full_name_value = use_signal(|| "".to_string());
    let email_required = use_memo(move || true);
    let email_value = use_signal(|| "".to_string());
    let zip_code_required = use_memo(move || true);
    let zip_code_value = use_signal(|| "".to_string());
    let opt_in_value = use_signal(|| false);
    let phone_required = use_memo(move || opt_in_value());
    let phone_value = use_signal(|| "".to_string());
    let opt_in_required =
        use_memo(move || !phone_value.read().is_empty() && !phone_value.read().contains('_'));
    let submit_disabled = use_memo(move || *status.read() == FormStatus::Processing);

    // handle form submission
    let onsubmit_handler = move |event: FormEvent| {
        event.prevent_default();
        status.set(FormStatus::Processing);

        let full_name = full_name_value.cloned();
        let email = email_value.cloned();
        let phone = if *phone_value.read() != get_empty_mask(InputType::Phone) {
            phone_value.cloned()
        } else {
            "".to_string()
        };
        let zip = if *zip_code_value.read() != get_empty_mask(InputType::Zip) {
            zip_code_value.cloned()
        } else {
            "".to_string()
        };
        let opt_in = opt_in_value.cloned();
        let hidden_source = props.hidden_source.clone();

        // submit the form data
        #[cfg(feature = "web")]
        spawn(async move {
            let result = async {
                let form_data = FormData::new().ok()?;
                form_data.append_with_str("full_name", &full_name).ok()?;
                form_data.append_with_str("email_address", &email).ok()?;
                form_data.append_with_str("mobile_phone", &phone).ok()?;
                form_data.append_with_str("zip_code", &zip).ok()?;
                form_data.append_with_str("opt_in", &opt_in.to_string()).ok()?;
                form_data.append_with_str("source", &hidden_source).ok()?;

                let url = "https://script.google.com/macros/s/AKfycbwTkDiqEJ46wU6E_MiXaqJmOilv89-Z3kHWx_RfrQ92c2EEkcxhHVH7AOy62wBIjvj1hg/exec";
                Request::post(url)
                    .mode(RequestMode::NoCors)
                    .body(form_data).ok()?
                    .send().await.ok()
            }.await;

            if result.is_some() {
                status.set(FormStatus::Success);
            } else {
                status.set(FormStatus::Error);
            }
        });
    };

    match status.cloned() {
        FormStatus::Idle | FormStatus::Processing => rsx! {
            form { class: "flex flex-col gap-6", onsubmit: onsubmit_handler,
                Input {
                    r#type: InputType::Text,
                    id: "full_name".to_string(),
                    required: Some(full_name_required),
                    value: full_name_value,
                    label: "Full Name".to_string(),
                    size: InputSize::Form,
                    variant: InputVariant::Default,
                }
                Input {
                    r#type: InputType::Email,
                    id: "email_address".to_string(),
                    required: Some(email_required),
                    value: email_value,
                    label: "Email Address".to_string(),
                    size: InputSize::Form,
                    variant: InputVariant::Default,
                }
                Input {
                    r#type: InputType::Phone,
                    id: "mobile_phone".to_string(),
                    required: Some(phone_required),
                    value: phone_value,
                    label: "Mobile Phone".to_string(),
                    size: InputSize::Form,
                    variant: InputVariant::Default,
                }
                Input {
                    r#type: InputType::Zip,
                    id: "zip_code".to_string(),
                    required: Some(zip_code_required),
                    value: zip_code_value,
                    label: "Zip Code".to_string(),
                    size: InputSize::Form,
                    variant: InputVariant::Default,
                }
                div { class: "flex flex-col gap-2",
                    Checkbox {
                        id: "opt_in".to_string(),
                        required: Some(opt_in_required),
                        value: opt_in_value,
                        label: "Opt-in to receive text messages".to_string(),
                    }
                    div {
                        p { class: "text-xs text-primary/75 leading-tight",
                            "By entering your phone number and selecting to opt in, you consent to join a recurring SMS/MMS text messaging program that will provide alerts, donation requests, updates, and other important information. By participating, you agree to the terms & privacy policy for auto dialed messages to the phone number you provide. Msg&data rates may apply. Msg frequency varies. Reply HELP for help or STOP to opt-out at any time. SMS information is not rented, sold, or shared. View "
                            a { href: "/terms", class: "hover:underline", "Terms of Service" }
                            " & "
                            a { href: "/privacy", class: "hover:underline", "Privacy Policy" }
                            "."
                        }
                    }
                }
                Button {
                    r#type: ButtonType::Submit,
                    disabled: Some(submit_disabled),
                    size: ButtonSize::FormFull,
                    variant: ButtonVariant::Primary,
                    match *status.read() {
                        FormStatus::Idle | FormStatus::Success | FormStatus::Error => "SIGN UP",
                        FormStatus::Processing => "SIGNING UP...",
                    }
                }
            }
        },
        FormStatus::Success => rsx! {
            div { class: "flex flex-col items-center justify-center text-center gap-4 bg-background p-6 rounded-lg",
                div { class: "text-green-500 flex items-center justify-center",
                    CircleCheck { class: "w-12 h-12" }
                }
                p { class: "text-2xl font-bold text-foreground", "Thank You!" }
                p { class: "text-foreground",
                    {
                        format!(
                            "You've successfully joined {}. We'll keep you updated on how you can help make a difference.",
                            props.organization_name,
                        )
                    }
                }
            }
        },
        FormStatus::Error => rsx! {
            div { class: "flex flex-col items-center justify-center text-center gap-4 bg-background p-6 rounded-lg",
                p { class: "text-2xl font-bold text-destructive", "Oops!" }
                p { class: "text-foreground",
                    "Something went wrong. Please refresh the page and try again."
                }
            }
        },
    }
}
