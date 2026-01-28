use crate::shared::button::{Button, ButtonSize, ButtonType, ButtonVariant};
use crate::shared::icon::{Icon, IconSize, IconVariant};
use crate::web_app::notification_badge::NotificationBadge;
use dioxus::prelude::*;
use lucide_dioxus::{Bell, X};

pub type Notifications = Store<i32>;
pub type ShowMenu = Signal<bool>;

#[derive(Clone, PartialEq, Props)]
pub struct NotificationMenuProps {
    notifications: Notifications,
    show_menu: ShowMenu,
}

#[component]
pub fn NotificationMenu(mut props: NotificationMenuProps) -> Element {
    rsx! {
        Button {
            r#type: ButtonType::Button,
            onclick: move |_| props.show_menu.toggle(),
            size: ButtonSize::Full,
            variant: ButtonVariant::Sidebar,
            class: if (props.notifications)() > 0 { "group" } else { "" },
            Icon { size: IconSize::Medium, variant: IconVariant::Sidebar, Bell {} }
            span { "Notifications" }
            if (props.notifications)() > 0 {
                div { class: "flex flex-1 justify-end",
                    if (props.notifications)() <= 99 {
                        NotificationBadge {
                            count: (props.notifications)(),
                            class: "group-hover:bg-accent-foreground",
                        }
                    } else if (props.notifications)() > 99 {
                        NotificationBadge {
                            count: 99,
                            class: "group-hover:bg-accent-foreground",
                        }
                    }
                }
            }
        }
        if (props.show_menu)() {
            div { class: "absolute left-full top-2 ml-2 w-80 bg-sidebar border border-border rounded-md shadow-lg z-50 py-2 flex flex-col gap-2",
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
