use crate::shared::button::{Button, ButtonSize, ButtonType, ButtonVariant};
use crate::shared::divider::Divider;
use crate::shared::icon::{Icon, IconSize, IconVariant};
use crate::web_app::avatar::{Avatar, AvatarVariant};
use crate::web_app::confirmation_modal::{ConfirmationModal, ConfirmationModalType};
use dioxus::prelude::*;
use lucide_dioxus::{ChevronsUpDown, LogOut, Settings2, X};

#[derive(Clone, PartialEq, Props)]
pub struct UserAccountMenuProps {
    user_first_name: Store<String>,
    user_last_name: Store<String>,
    user_avatar_url: Store<Option<String>>,
    user_role: Store<String>,
    account_route: String,
    show_menu: Signal<bool>,
}

#[component]
pub fn UserAccountMenu(mut props: UserAccountMenuProps) -> Element {
    let mut show_confirmation_modal: Signal<bool> = use_signal(|| false);

    rsx! {
        Button {
            r#type: ButtonType::Button,
            size: ButtonSize::Full,
            variant: ButtonVariant::Sidebar,
            class: "group",
            onclick: move |_| props.show_menu.toggle(),
            Avatar {
                src: props.user_avatar_url.cloned(),
                fallback: {
                    format!(
                        "{}{}",
                        props.user_first_name.read().chars().next().unwrap_or('?'),
                        props.user_last_name.read().chars().next().unwrap_or('?'),
                    )
                },
                variant: AvatarVariant::Round,
            }
            div { class: "flex flex-col flex-1 text-left gap-1",
                span { class: "text-sm leading-none font-medium text-foreground group-hover:text-accent-foreground truncate",
                    {format!("{} {}", props.user_first_name.cloned(), props.user_last_name.cloned())}
                }
                span { class: "text-sm leading-none text-muted-foreground group-hover:text-accent-foreground truncate",
                    {props.user_role.cloned()}
                }
            }
            Icon {
                size: IconSize::Small,
                variant: IconVariant::Muted,
                class: "group-hover:text-accent-foreground",
                ChevronsUpDown {}
            }
        }
        if *props.show_menu.read() {
            div { class: "absolute left-full bottom-2 ml-2 w-60 bg-sidebar border border-border rounded-md shadow-lg z-50 py-2 flex flex-col gap-2",
                div { class: "flex flex-row justify-between items-center px-2",
                    span { class: "text-sm font-medium text-foreground cursor-default",
                        "Your Account"
                    }
                    Button {
                        r#type: ButtonType::Button,
                        size: ButtonSize::Icon,
                        variant: ButtonVariant::Sidebar,
                        onclick: move |_| props.show_menu.set(false),
                        Icon {
                            size: IconSize::Small,
                            variant: IconVariant::Button,
                            X {}
                        }
                    }
                }
                div { class: "px-2 flex flex-col gap-1",
                    Button {
                        r#type: ButtonType::Link,
                        size: ButtonSize::Full,
                        variant: ButtonVariant::Sidebar,
                        to: props.account_route,
                        Icon {
                            size: IconSize::Medium,
                            variant: IconVariant::Sidebar,
                            Settings2 {}
                        }
                        span { "Account Settings" }
                    }
                    Button {
                        r#type: ButtonType::Button,
                        size: ButtonSize::Full,
                        variant: ButtonVariant::Sidebar,
                        onclick: move |_| {
                            props.show_menu.set(false);
                            show_confirmation_modal.set(true);
                        },
                        Icon {
                            size: IconSize::Medium,
                            variant: IconVariant::Sidebar,
                            LogOut {}
                        }
                        span { "Log Out" }
                    }
                }
                Divider {}
                div { class: "flex justify-between px-2 py-1.5",
                    Button {
                        r#type: ButtonType::Link,
                        size: ButtonSize::Fit,
                        variant: ButtonVariant::Link,
                        class: "text-xs font-medium text-muted-foreground",
                        to: "#",
                        span { "Terms & Conditions" }
                    }
                    span { class: "text-xs font-medium text-muted-foreground cursor-default",
                        "v0.1.0"
                    }
                }
            }
        }
        if show_confirmation_modal() {
            ConfirmationModal {
                r#type: ConfirmationModalType::Danger,
                title: "Log Out".to_string(),
                message: "Are you sure you want to log out of your account?".to_string(),
                confirm_text: "Log Out".to_string(),
                cancel_text: "Cancel".to_string(),
                show_modal: show_confirmation_modal,
            }
        }
    }
}
