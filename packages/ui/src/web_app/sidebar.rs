mod nav_button;
mod nav_label;
mod sidebar_menu;

use crate::shared::button::{Button, ButtonSize, ButtonType, ButtonVariant};
use crate::shared::divider::Divider;
use crate::shared::icon::{Icon, IconSize, IconVariant};
use crate::web_app::UserAccountContext;
use api::web_app::{OrganizationStoreExt, PermissionType, UserAccountStoreExt};
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
    nav_routes: NavRoutes,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    let user_account = use_context::<UserAccountContext>().user_account;

    let current_route = router().full_route_string();

    let active_organization = if let Some(current_organization) = user_account
        .organizations()
        .get(user_account.active_organization_id().cloned())
    {
        Some(current_organization)
    } else if let Some(first_organization) = user_account.organizations().values().next() {
        user_account
            .active_organization_id()
            .set(*first_organization.id().read());
        Some(first_organization)
    } else {
        None
    };

    rsx! {
        if let Some(active_organization) = active_organization {
            aside { class: "w-60 bg-sidebar flex flex-col h-screen border-r border-border",
                div { class: "relative flex flex-col pt-2 gap-2 pb-1",
                    match props.r#type {
                        SidebarType::Main => rsx! {
                            div { class: "px-4",
                                SidebarMenu {
                                    r#type: SidebarMenuType::OrganizationSelector,
                                    active_organization_id: Some(user_account.active_organization_id().into()),
                                    active_organization: Some(active_organization.into()),
                                    organizations: Some(user_account.organizations().into()),
                                }
                            }
                            Divider {}
                            div { class: "px-4",
                                SidebarMenu {
                                    r#type: SidebarMenuType::NotificationMenu,
                                    notifications: Some(user_account.notifications().into()),
                                }
                            }
                        },
                        SidebarType::UserAccount => rsx! {
                            div { class: "px-4",
                                Button {
                                    r#type: ButtonType::Link,
                                    to: if let Some(route) = props.nav_routes.get(&RouteType::Dashboard) { route.clone() },
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
                            NavButton {
                                current_route: current_route.clone(),
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Dashboard) { route.clone() },
                                icon: rsx! {
                                    LayoutGrid {}
                                },
                                label: "Dashboard".to_string(),
                            }
                            if let Some(permission) = active_organization
                                .permissions()
                                .read()
                                .get(&PermissionType::Events)
                            {
                                if *permission {
                                    NavButton {
                                        current_route: current_route.clone(),
                                        nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Events) { route.clone() },
                                        icon: rsx! {
                                            Calendar {}
                                        },
                                        label: "Events".to_string(),
                                    }
                                    NavButton {
                                        current_route: current_route.clone(),
                                        nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Actions) { route.clone() },
                                        icon: rsx! {
                                            Megaphone {}
                                        },
                                        label: "Actions".to_string(),
                                    }
                                }
                            }
                            NavLabel { label: "Tools".to_string() }
                            NavButton {
                                current_route: current_route.clone(),
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Groups) { route.clone() },
                                icon: rsx! {
                                    ContactRound {}
                                },
                                label: "Groups".to_string(),
                            }
                            NavButton {
                                current_route: current_route.clone(),
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Analytics) { route.clone() },
                                icon: rsx! {
                                    ChartColumn {}
                                },
                                label: "Analytics".to_string(),
                            }
                            NavButton {
                                current_route: current_route.clone(),
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Exports) { route.clone() },
                                icon: rsx! {
                                    FileOutput {}
                                },
                                label: "Exports".to_string(),
                            }
                            NavButton {
                                current_route: current_route.clone(),
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Team) { route.clone() },
                                icon: rsx! {
                                    UsersRound {}
                                },
                                label: "Team".to_string(),
                            }
                            NavButton {
                                current_route: current_route.clone(),
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Settings) { route.clone() },
                                icon: rsx! {
                                    Settings {}
                                },
                                label: "Settings".to_string(),
                            }
                        },
                        SidebarType::UserAccount => rsx! {
                            NavLabel { label: "Account Settings".to_string() }
                            NavButton {
                                current_route: current_route.clone(),
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::Account) { route.clone() },
                                icon: rsx! {
                                    User {}
                                },
                                label: "Account".to_string(),
                            }
                            NavButton {
                                current_route: current_route.clone(),
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::OrganizationManagement) { route.clone() },
                                icon: rsx! {
                                    Building {}
                                },
                                label: "Organizations".to_string(),
                            }
                            NavButton {
                                current_route: current_route.clone(),
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::NotificationPreferences) { route.clone() },
                                icon: rsx! {
                                    Bell {}
                                },
                                label: "Notifications".to_string(),
                            }
                            NavButton {
                                current_route: current_route.clone(),
                                nav_route: if let Some(route) = props.nav_routes.get(&RouteType::DeviceSessions) { route.clone() },
                                icon: rsx! {
                                    MonitorSmartphone {}
                                },
                                label: "Devices".to_string(),
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
                                    to: if let Some(route) = props.nav_routes.get(&RouteType::Support) { route.clone() },
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
                                    user_first_name: Some(user_account.first_name().into()),
                                    user_last_name: Some(user_account.last_name().into()),
                                    user_avatar_url: Some(user_account.avatar_url().into()),
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
