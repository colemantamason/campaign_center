use dioxus::prelude::*;

use crate::shared::button::{Button, ButtonSize, ButtonVariant};

#[derive(Props, Clone, PartialEq)]
pub struct ConfirmationModalProps {
    title: String,
    message: String,
    confirm_text: String,
    cancel_text: String,
    #[props(default = false)]
    is_danger: bool,
    show_modal: Signal<bool>,
    on_confirm: Option<EventHandler<()>>,
    on_cancel: Option<EventHandler<()>>,
}

#[component]
pub fn ConfirmationModal(mut props: ConfirmationModalProps) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-foreground/50",
            onclick: move |_| {
                props.show_modal.set(false);
                if let Some(on_cancel) = &props.on_cancel {
                    on_cancel.call(());
                }
            },
            div {
                class: "bg-background border border-border rounded-lg shadow-lg p-6 w-full max-w-sm",
                onclick: move |e| e.stop_propagation(),
                h2 { class: "text-lg font-semibold text-foreground mb-2", "{props.title}" }
                p { class: "text-sm text-muted-foreground mb-6", "{props.message}" }
                div { class: "flex justify-end gap-3",
                    Button {
                        size: ButtonSize::Default,
                        variant: if props.is_danger { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| {
                            props.show_modal.set(false);
                            if let Some(on_cancel) = &props.on_cancel {
                                on_cancel.call(());
                            }
                        },
                        "{props.cancel_text}"
                    }
                    Button {
                        size: ButtonSize::Default,
                        variant: if props.is_danger { ButtonVariant::Destructive } else { ButtonVariant::Primary },
                        onclick: move |_| {
                            props.show_modal.set(false);
                            if let Some(on_confirm) = &props.on_confirm {
                                on_confirm.call(());
                            }
                        },
                        "{props.confirm_text}"
                    }
                }
            }
        }
    }
}
