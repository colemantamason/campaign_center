use crate::shared::button::{Button, ButtonSize, ButtonType, ButtonVariant};
use crate::shared::icon::{Icon, IconSize, IconVariant};
use dioxus::prelude::*;

pub type CurrentRoute = String;
pub type NavRoute = String;
pub type IconElement = Element;
pub type Label = String;

#[derive(Clone, PartialEq, Props)]
pub struct NavButtonProps {
    current_route: CurrentRoute,
    nav_route: Option<NavRoute>,
    icon: IconElement,
    label: Label,
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
