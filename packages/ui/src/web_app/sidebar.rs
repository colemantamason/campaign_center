mod nav_button;
mod nav_label;
mod sidebar_menu;

use crate::shared::{
    Button, ButtonSize, ButtonType, ButtonVariant, Divider, Icon, IconSize, IconVariant,
};
use api::state::{OrganizationMembership, OrganizationMembershipStoreExt};
use dioxus::prelude::*;
use lucide_dioxus::{ChevronLeft, CircleQuestionMark};
use nav_button::NavButton;
use nav_label::NavLabel;
use sidebar_menu::{SidebarMenu, SidebarMenuType};

#[derive(Clone, PartialEq)]
pub enum SidebarType {
    Main,
    UserAccount,
}

#[derive(Clone, PartialEq)]
pub struct NavRoute {
    pub route: String,
    pub icon: Element,
    pub label: String,
}

pub type NavRoutes = Vec<NavRoute>;

#[derive(Clone, PartialEq, Props)]
pub struct SidebarProps {
    r#type: SidebarType,
    active_organization_membership: Store<OrganizationMembership>,
    current_route: String,
    main_menu_routes: Option<NavRoutes>,
    tools_routes: Option<NavRoutes>,
    support_link: Option<String>,
    account_menu_routes: Option<NavRoutes>,
    dashboard_route: Option<String>,
    account_settings_routes: Option<NavRoutes>,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    rsx! {
        aside { class: "w-60 bg-sidebar flex flex-col h-screen border-r border-border",
            div { class: "relative flex flex-col pt-2 gap-2 pb-1",
                match props.r#type {
                    SidebarType::Main => rsx! {
                        div { class: "px-4",
                            SidebarMenu {
                                r#type: SidebarMenuType::OrganizationSelector,
                                active_organization: Some(props.active_organization_membership.organization().into()),
                            }
                        }
                        Divider {}
                        div { class: "px-4",
                            SidebarMenu { r#type: SidebarMenuType::NotificationMenu }
                        }
                    },
                    SidebarType::UserAccount => rsx! {
                        div { class: "px-4",
                            Button {
                                r#type: ButtonType::Link,
                                to: props.dashboard_route.clone(),
                                size: ButtonSize::Full,
                                variant: ButtonVariant::Sidebar,
                                Icon { size: IconSize::Medium, variant: IconVariant::Sidebar, ChevronLeft {} }
                                span { "Back" }
                            }
                        }
                        Divider {}
                    },
                }
            }
            nav { class: "flex flex-col flex-1 px-4 gap-1",
                match props.r#type {
                    SidebarType::Main => rsx! {
                        NavLabel { label: "Main Menu".to_string() }
                        if let Some(routes) = &props.main_menu_routes {
                            for nav_route in routes {
                                NavButton {
                                    current_route: props.current_route.clone(),
                                    nav_route: Some(nav_route.route.clone()),
                                    icon: nav_route.icon.clone(),
                                    label: nav_route.label.clone(),
                                }
                            }
                        }
                        NavLabel { label: "Tools".to_string() }
                        if let Some(routes) = &props.tools_routes {
                            for nav_route in routes {
                                NavButton {
                                    current_route: props.current_route.clone(),
                                    nav_route: Some(nav_route.route.clone()),
                                    icon: nav_route.icon.clone(),
                                    label: nav_route.label.clone(),
                                }
                            }
                        }
                    },
                    SidebarType::UserAccount => rsx! {
                        NavLabel { label: "Account Settings".to_string() }
                        if let Some(routes) = &props.account_settings_routes {
                            for nav_route in routes {
                                NavButton {
                                    current_route: props.current_route.clone(),
                                    nav_route: Some(nav_route.route.clone()),
                                    icon: nav_route.icon.clone(),
                                    label: nav_route.label.clone(),
                                }
                            }
                        }
                    },
                }
            }
            match props.r#type {
                SidebarType::Main => rsx! {
                    div { class: "flex flex-col relative pb-2 gap-2",
                        div { class: "px-4",
                            Button {
                                r#type: ButtonType::Link,
                                to: props.support_link.clone(),
                                size: ButtonSize::Full,
                                variant: ButtonVariant::Sidebar,
                                Icon { size: IconSize::Medium, variant: IconVariant::Sidebar, CircleQuestionMark {} }
                                span { "Support" }
                            }
                        }
                        Divider {}
                        div { class: "px-4",
                            SidebarMenu {
                                r#type: SidebarMenuType::UserAccountMenu,
                                user_role: Some(props.active_organization_membership.user_role().into()),
                                account_menu_routes: props.account_menu_routes,
                            }
                        }
                    }
                },
                SidebarType::UserAccount => rsx! {},
            }
        }
    }
}
