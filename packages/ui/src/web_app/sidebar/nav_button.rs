use crate::shared::{
    button::{Button, ButtonSize, ButtonType, ButtonVariant},
    icon::{Icon, IconSize, IconVariant},
};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct NavButtonProps {
    current_route: String,
    nav_route: Option<String>,
    icon: Element,
    label: String,
}

#[component]
pub fn NavButton(props: NavButtonProps) -> Element {
    rsx! {
        Button {
            r#type: ButtonType::Link,
            to: props.nav_route.clone(),
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
