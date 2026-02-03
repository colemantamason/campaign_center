use crate::shared::{Button, ButtonSize, ButtonType, ButtonVariant, Icon, IconSize, IconVariant};
use dioxus::prelude::*;
#[cfg(feature = "web")]
use gloo::timers::future::TimeoutFuture;
use lucide_dioxus::{CircleAlert, CircleCheck, Info, X};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum ToastVariant {
    Success,
    Error,
    Info,
}

#[derive(Clone, PartialEq)]
struct ToastData {
    pub id: i32,
    pub title: String,
    pub message: String,
    pub variant: ToastVariant,
}

type Toasts = HashMap<i32, ToastData>;

#[derive(Clone)]
pub struct ToastContext {
    toasts: Signal<Toasts>,
}

impl ToastContext {
    pub fn create(&mut self, title: String, message: String, toast_variant: ToastVariant) -> () {
        // generate a new id for the toast
        let id = if self.toasts.read().is_empty() {
            0
        } else {
            if let Some(last_id) = self.toasts.read().keys().max() {
                last_id + 1
            } else {
                0
            }
        };

        // create the toast data
        let toast = ToastData {
            id,
            title,
            message,
            variant: toast_variant,
        };

        // insert the toast into the toasts map
        self.toasts.write().insert(id, toast);
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ToastProviderProps {
    children: Element,
}

#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    use_context_provider(|| ToastContext {
        toasts: Signal::new(Toasts::new()),
    });

    let toast_context = use_context::<ToastContext>();

    rsx! {
        {props.children}
        div { class: "fixed bottom-4 right-4 z-10 flex flex-col gap-2",
            for (_ , toast) in toast_context.toasts.read().iter() {
                Toast {
                    key: "{toast.id}",
                    toast: toast.clone(),
                    toasts: toast_context.toasts,
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ToastProps {
    toast: ToastData,
    toasts: Signal<Toasts>,
}

#[component]
fn Toast(props: ToastProps) -> Element {
    let toast_id = props.toast.id;
    let mut toasts = props.toasts;
    let duration = 5000;

    // remove the toast after the duration
    use_effect(move || {
        #[cfg(feature = "web")]
        spawn(async move {
            TimeoutFuture::new(duration).await;
            toasts.remove(&toast_id);
        });
    });

    let toast_common_classes =
        "flex items-start gap-3 p-4 rounded-lg shadow-lg border border-border border-l-4 max-w-sm";

    let toast_variant_classes = match props.toast.variant {
        ToastVariant::Success => "bg-background border-primary",
        ToastVariant::Error => "bg-background border-destructive",
        ToastVariant::Info => "bg-background border-muted-foreground",
    };

    let toast_combined_classes = format!("{} {}", toast_common_classes, toast_variant_classes);

    let icon_class = match props.toast.variant {
        ToastVariant::Success => "text-primary",
        ToastVariant::Error => "text-destructive",
        ToastVariant::Info => "text-muted-foreground",
    };

    rsx! {
        div { class: "{toast_combined_classes}",
            div { class: "flex-shrink-0 self-center",
                Icon {
                    size: IconSize::Large,
                    variant: IconVariant::Button,
                    class: icon_class,
                    match props.toast.variant {
                        ToastVariant::Success => rsx! {
                            CircleCheck {}
                        },
                        ToastVariant::Error => rsx! {
                            CircleAlert {}
                        },
                        ToastVariant::Info => rsx! {
                            Info {}
                        },
                    }
                }
            }
            div { class: "flex flex-col flex-1",
                span { class: "text-sm font-medium text-foreground", {props.toast.title} }
                p { class: "text-sm text-muted-foreground", {props.toast.message} }
            }
            Button {
                r#type: ButtonType::Button,
                onclick: move |_| {
                    toasts.remove(&toast_id);
                },
                size: ButtonSize::Icon,
                variant: ButtonVariant::Sidebar,
                Icon { size: IconSize::Small, variant: IconVariant::Muted, X {} }
            }
        }
    }
}
