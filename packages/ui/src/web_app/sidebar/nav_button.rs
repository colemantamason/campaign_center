use crate::shared::button::{Button, ButtonSize, ButtonVariant};
use crate::shared::icon::{Icon, IconSize, IconVariant};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NavButtonProps {
    nav_route: String,
    current_route: String,
    label: String,
    icon: Element,
}

#[component]
pub fn NavButton(props: NavButtonProps) -> Element {
    rsx! {
        Button {
            size: ButtonSize::Full,
            variant: {
                if props.current_route.ends_with(&props.nav_route) {
                    ButtonVariant::SidebarActive
                } else {
                    ButtonVariant::Sidebar
                }
            },
            to: props.nav_route.clone(),
            Icon {
                size: IconSize::Medium,
                variant: {
                    if props.current_route.ends_with(&props.nav_route) {
                        IconVariant::SidebarActive
                    } else {
                        IconVariant::Sidebar
                    }
                },
                {props.icon}
            }
            span { "{props.label}" }
        }
    }
}
