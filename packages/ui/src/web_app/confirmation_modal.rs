use crate::shared::button::{Button, ButtonSize, ButtonType, ButtonVariant};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ConfirmationModalType {
    Default,
    Danger,
}

pub type Title = String;
pub type Message = String;
pub type ConfirmText = String;
pub type CancelText = String;
pub type ShowModal = Signal<bool>;
pub type OnConfirm = EventHandler<()>;
pub type OnCancel = EventHandler<()>;

#[derive(Clone, PartialEq, Props)]
pub struct ConfirmationModalProps {
    r#type: ConfirmationModalType,
    title: Title,
    message: Message,
    confirm_text: ConfirmText,
    cancel_text: CancelText,
    on_confirm: Option<OnConfirm>,
    on_cancel: Option<OnCancel>,
    show_modal: ShowModal,
}

#[component]
pub fn ConfirmationModal(mut props: ConfirmationModalProps) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-foreground/50",
            onclick: move |_| {
                props.show_modal.set(false);
                if let Some(on_cancel) = props.on_cancel {
                    on_cancel.call(());
                }
            },
            div {
                class: "bg-background border border-border rounded-lg shadow-lg p-6 w-full max-w-sm",
                onclick: move |event| event.stop_propagation(),
                h2 { class: "text-lg font-semibold text-foreground mb-2", {props.title} }
                p { class: "text-sm text-muted-foreground mb-6", {props.message} }
                div { class: "flex justify-end gap-3",
                    Button {
                        r#type: ButtonType::Button,
                        onclick: move |_| {
                            props.show_modal.set(false);
                            if let Some(on_cancel) = props.on_cancel {
                                on_cancel.call(());
                            }
                        },
                        size: ButtonSize::Default,
                        variant: match props.r#type {
                            ConfirmationModalType::Default => ButtonVariant::Primary,
                            ConfirmationModalType::Danger => ButtonVariant::Outline,
                        },
                        {props.cancel_text}
                    }
                    Button {
                        r#type: ButtonType::Button,
                        onclick: move |_| {
                            props.show_modal.set(false);
                            if let Some(on_confirm) = props.on_confirm {
                                on_confirm.call(());
                            }
                        },
                        size: ButtonSize::Default,
                        variant: match props.r#type {
                            ConfirmationModalType::Default => ButtonVariant::Primary,
                            ConfirmationModalType::Danger => ButtonVariant::Destructive,
                        },
                        {props.confirm_text}
                    }
                }
            }
        }
    }
}
