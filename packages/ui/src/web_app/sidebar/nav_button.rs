use crate::shared::button::{Button, ButtonSize, ButtonType, ButtonVariant};
use crate::shared::icon::{Icon, IconSize, IconVariant};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
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
            r#type: ButtonType::Link,
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
            span { {props.label} }
        }
    }
}
