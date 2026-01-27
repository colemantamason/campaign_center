mod nav_button;
mod nav_label;
mod sidebar_menu;

use crate::shared::button::{Button, ButtonSize, ButtonType, ButtonVariant};
use crate::shared::divider::Divider;
use crate::shared::icon::{Icon, IconSize, IconVariant};
use api::web_app::{OrganizationStoreExt, UserAccount, UserAccountStoreExt};
use dioxus::prelude::*;
use lucide_dioxus::{
    Bell, Building, Calendar, ChartColumn, ChevronLeft, CircleQuestionMark, ContactRound,
    FileOutput, LayoutGrid, Megaphone, MonitorSmartphone, Settings, User, UsersRound,
};
use nav_button::NavButton;
use nav_label::NavLabel;
use sidebar_menu::{SidebarMenu, SidebarMenuType};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum SidebarType {
    Main,
    UserAccount,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum RouteType {
    Notifications,
    Dashboard,
    Events,
    Actions,
    Groups,
    Analytics,
    Exports,
    Team,
    Settings,
    Support,
    Account,
    OrganizationManagement,
    NotificationPreferences,
    DeviceSessions,
}

pub type NavRoutes = HashMap<RouteType, String>;

#[derive(Clone, PartialEq, Props)]
pub struct SidebarProps {
    r#type: SidebarType,
    user_account: Store<UserAccount>,
    nav_routes: NavRoutes,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    let current_route = router().full_route_string();

    rsx! {
        if let Some(active_organization) = props
            .user_account
            .organizations()
            .get((props.user_account.active_organization_id())())
        {
            aside { class: "w-60 bg-sidebar flex flex-col h-screen border-r border-border",
                div { class: "relative flex flex-col pt-2 gap-2 pb-1",
                    match props.r#type {
                        SidebarType::Main => rsx! {
                            div { class: "px-4",
                                SidebarMenu {
                                    r#type: SidebarMenuType::OrganizationSelector,
                                    active_organization_id: Some(props.user_account.active_organization_id().into()),
                                    organizations: Some(props.user_account.organizations().into()),
                                    active_organization: Some(active_organization.into()),
                                }
                            }
                            Divider {}
                            div { class: "px-4",
                                SidebarMenu {
                                    r#type: SidebarMenuType::NotificationMenu,
                                    notifications: Some(props.user_account.notifications().into()),
                                }
                            }
                        },
                        SidebarType::UserAccount => rsx! {
                            div { class: "px-4",
                                Button {
                                    r#type: ButtonType::Link,
                                    size: ButtonSize::Full,
                                    variant: ButtonVariant::Sidebar,
                                    to: if let Some(route) = props.nav_routes.get(&RouteType::Dashboard) { route.clone() },
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
                            NavButton {
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Dashboard) { route.clone() },
                                current_route: &current_route,
                                label: "Dashboard".to_string(),
                                icon: rsx! {
                                    LayoutGrid {}
                                },
                            }
                            NavButton {
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Events) { route.clone() },
                                current_route: &current_route,
                                label: "Events".to_string(),
                                icon: rsx! {
                                    Calendar {}
                                },
                            }
                            NavButton {
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Actions) { route.clone() },
                                current_route: &current_route,
                                label: "Actions".to_string(),
                                icon: rsx! {
                                    Megaphone {}
                                },
                            }
                            NavLabel { label: "Tools".to_string() }
                            NavButton {
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Groups) { route.clone() },
                                current_route: &current_route,
                                label: "Groups".to_string(),
                                icon: rsx! {
                                    ContactRound {}
                                },
                            }
                            NavButton {
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Analytics) { route.clone() },
                                current_route: &current_route,
                                label: "Analytics".to_string(),
                                icon: rsx! {
                                    ChartColumn {}
                                },
                            }
                            NavButton {
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Exports) { route.clone() },
                                current_route: &current_route,
                                label: "Exports".to_string(),
                                icon: rsx! {
                                    FileOutput {}
                                },
                            }
                            NavButton {
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Team) { route.clone() },
                                current_route: &current_route,
                                label: "Team".to_string(),
                                icon: rsx! {
                                    UsersRound {}
                                },
                            }
                            NavButton {
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Settings) { route.clone() },
                                current_route: &current_route,
                                label: "Settings".to_string(),
                                icon: rsx! {
                                    Settings {}
                                },
                            }
                        },
                        SidebarType::UserAccount => rsx! {
                            NavLabel { label: "Account Settings".to_string() }
                            NavButton {
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Account) { route.clone() },
                                current_route: &current_route,
                                label: "Account".to_string(),
                                icon: rsx! {
                                    User {}
                                },
                            }
                            NavButton {
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::OrganizationManagement) { route.clone() },
                                current_route: &current_route,
                                label: "Organizations".to_string(),
                                icon: rsx! {
                                    Building {}
                                },
                            }
                            NavButton {
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::NotificationPreferences) { route.clone() },
                                current_route: &current_route,
                                label: "Notifications".to_string(),
                                icon: rsx! {
                                    Bell {}
                                },
                            }
                            NavButton {
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::DeviceSessions) { route.clone() },
                                current_route: &current_route,
                                label: "Devices".to_string(),
                                icon: rsx! {
                                    MonitorSmartphone {}
                                },
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
                                    size: ButtonSize::Full,
                                    variant: ButtonVariant::Sidebar,
                                    to: if let Some(route) = props.nav_routes.get(&RouteType::Support) { route.clone() },
                                    Icon { size: IconSize::Medium, variant: IconVariant::Sidebar, CircleQuestionMark {} }
                                    span { "Support" }
                                }
                            }
                            Divider {}
                            div { class: "px-4",
                                SidebarMenu {
                                    r#type: SidebarMenuType::UserAccountMenu,
                                    user_first_name: Some(props.user_account.first_name().into()),
                                    user_last_name: Some(props.user_account.last_name().into()),
                                    user_avatar_url: Some(props.user_account.avatar_url().into()),
                                    user_role: Some(active_organization.user_role().into()),
                                    account_route: if let Some(route) = props.nav_routes.get(&RouteType::Account) { route.clone() },
                                }
                            }
                        }
                    },
                    SidebarType::UserAccount => rsx! {},
                }
            }
        }
    }
}
