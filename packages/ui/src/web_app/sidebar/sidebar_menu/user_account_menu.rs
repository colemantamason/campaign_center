use crate::shared::{
    Button, ButtonSize, ButtonType, ButtonVariant, Divider, Icon, IconSize, IconVariant,
};
use crate::web_app::{
    Avatar, AvatarVariant, ConfirmationModal, ConfirmationModalType, NavRoutes, UserAccountContext,
};
use api::enums::MemberRole;
use api::state::UserAccountStoreExt;
use dioxus::prelude::*;
use lucide_dioxus::{ChevronsUpDown, LogOut, X};

#[derive(Clone, PartialEq, Props)]
pub struct UserAccountMenuProps {
    user_role: Option<Store<MemberRole>>,
    account_menu_routes: Option<NavRoutes>,
    show_menu: Signal<bool>,
}

#[component]
pub fn UserAccountMenu(mut props: UserAccountMenuProps) -> Element {
    let user_account = use_context::<UserAccountContext>().user_account;
    let mut show_confirmation_modal: Signal<bool> = use_signal(|| false);

    rsx! {
        Button {
            r#type: ButtonType::Button,
            onclick: move |_| props.show_menu.toggle(),
            size: ButtonSize::Full,
            variant: ButtonVariant::Sidebar,
            class: "group",
            Avatar {
                src: user_account.avatar_url().cloned(),
                fallback: {
                    format!(
                        "{}{}",
                        if let Some(first_name) = user_account.first_name().cloned().chars().next() {
                            first_name.to_string()
                        } else {
                            "?".to_string()
                        },
                        if let Some(last_name) = user_account.last_name().cloned().chars().next() {
                            last_name.to_string()
                        } else {
                            "".to_string()
                        },
                    )
                },
                variant: AvatarVariant::Round,
            }
            div { class: "flex flex-col flex-1 text-left gap-1",
                span { class: "text-sm leading-none font-medium text-foreground group-hover:text-accent-foreground truncate",
                    {
                        format!(
                            "{} {}",
                            user_account.first_name().cloned(),
                            user_account.last_name().cloned(),
                        )
                    }
                }
                if let Some(user_role) = &props.user_role {
                    span { class: "text-sm leading-none text-muted-foreground group-hover:text-accent-foreground truncate",
                        {user_role.to_string()}
                    }
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
            div { class: "absolute left-full bottom-2 ml-2 w-60 bg-sidebar border border-border rounded-md shadow-lg z-40 py-2 flex flex-col gap-2",
                div { class: "flex flex-row justify-between items-center px-2",
                    span { class: "text-sm font-medium text-foreground cursor-default",
                        "Your Account"
                    }
                    Button {
                        r#type: ButtonType::Button,
                        onclick: move |_| props.show_menu.set(false),
                        size: ButtonSize::Icon,
                        variant: ButtonVariant::Sidebar,
                        Icon {
                            size: IconSize::Small,
                            variant: IconVariant::Button,
                            X {}
                        }
                    }
                }
                div { class: "px-2 flex flex-col gap-1",
                    if let Some(routes) = props.account_menu_routes {
                        for nav_route in routes {
                            Button {
                                r#type: ButtonType::Link,
                                to: Some(nav_route.route),
                                size: ButtonSize::Full,
                                variant: ButtonVariant::Sidebar,
                                Icon {
                                    size: IconSize::Medium,
                                    variant: IconVariant::Sidebar,
                                    {nav_route.icon}
                                }
                                span { {nav_route.label} }
                            }
                        }
                    }
                    Button {
                        r#type: ButtonType::Button,
                        onclick: move |_| {
                            props.show_menu.set(false);
                            show_confirmation_modal.set(true);
                        },
                        size: ButtonSize::Full,
                        variant: ButtonVariant::Sidebar,
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
                        to: "#",
                        size: ButtonSize::Fit,
                        variant: ButtonVariant::Link,
                        class: "text-xs font-medium text-muted-foreground",
                        span { "Terms & Conditions" }
                    }
                    span { class: "text-xs font-medium text-muted-foreground cursor-default",
                        "v0.1.0"
                    }
                }
            }
        }
        if *show_confirmation_modal.read() {
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
