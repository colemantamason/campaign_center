use crate::shared::{Button, ButtonSize, ButtonType, ButtonVariant, Icon, IconSize, IconVariant};
use crate::web_app::{notification_badge::NotificationBadge, UserAccountContext};
use api::state::UserAccountStoreExt;
use dioxus::prelude::*;
use lucide_dioxus::{Bell, X};

#[derive(Clone, PartialEq, Props)]
pub struct NotificationMenuProps {
    show_menu: Signal<bool>,
}

#[component]
pub fn NotificationMenu(mut props: NotificationMenuProps) -> Element {
    let user_account = use_context::<UserAccountContext>().user_account;
    let unread_count = user_account
        .notifications()
        .read()
        .values()
        .filter(|notification| !notification.read)
        .count() as i32;

    rsx! {
        Button {
            r#type: ButtonType::Button,
            onclick: move |_| props.show_menu.toggle(),
            size: ButtonSize::Full,
            variant: ButtonVariant::Sidebar,
            class: if unread_count > 0 { "group" } else { "" },
            Icon { size: IconSize::Medium, variant: IconVariant::Sidebar, Bell {} }
            span { "Notifications" }
            if unread_count > 0 {
                div { class: "flex flex-1 justify-end",
                    if unread_count <= 99 {
                        NotificationBadge {
                            count: unread_count,
                            class: "group-hover:bg-accent-foreground",
                        }
                    } else if unread_count > 99 {
                        NotificationBadge {
                            count: 99,
                            class: "group-hover:bg-accent-foreground",
                        }
                    }
                }
            }
        }
        if *props.show_menu.read() {
            div { class: "absolute left-full top-2 ml-2 w-80 bg-sidebar border border-border rounded-md shadow-lg z-40 py-2 flex flex-col gap-2",
                div { class: "flex flex-row justify-between items-center px-2",
                    span { class: "text-sm font-medium text-foreground cursor-default",
                        "Notifications"
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
                div { class: "px-2 h-80 overflow-y-auto flex flex-col align-center",
                    span { class: "text-sm text-foreground cursor-default", "No new notifications." }
                }
            }
        }
    }
}
