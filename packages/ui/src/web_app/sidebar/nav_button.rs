use crate::shared::button::{Button, ButtonSize, ButtonType, ButtonVariant};
use crate::shared::icon::{Icon, IconSize, IconVariant};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct NavButtonProps {
    nav_route: Option<String>,
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
                if let Some(route) = &props.nav_route {
                    if props.current_route.ends_with(route) {
                        ButtonVariant::SidebarActive

                    } else {
                        ButtonVariant::Sidebar
                    }
                } else {
                    ButtonVariant::Sidebar
                }
            },
            to: props.nav_route.clone(),
            Icon {
                size: IconSize::Medium,
                variant: {
                    if let Some(route) = &props.nav_route {
                        if props.current_route.ends_with(route) {
                            IconVariant::SidebarActive
                        } else {
                            IconVariant::Sidebar
                        }
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
