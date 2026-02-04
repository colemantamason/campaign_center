mod account;
mod actions;
mod analytics;
mod create;
mod dashboard;
mod events;
mod exports;
mod groups;
mod login;
mod settings;
mod team;

use crate::auth::{user_response_to_account, AuthContext, AuthState};
use account::{
    devices::DeviceSessions, notifications::NotificationPreferences,
    organizations::OrganizationManagement, Account,
};
use actions::Actions;
use analytics::Analytics;
use api::models::SubscriptionType;
use api::providers::get_current_user;
use api::state::{UserAccount, UserAccountStoreExt};
use create::{CreateAccount, CreateOrganization};
use dashboard::Dashboard;
use dioxus::prelude::*;
use events::Events;
use exports::Exports;
use groups::Groups;
use login::Login;
use lucide_dioxus::{
    Bell, Building, Calendar, ChartColumn, ContactRound, FileOutput, LayoutGrid, Megaphone,
    MonitorSmartphone, Settings as Settings1, Settings2, User, UsersRound,
};
use settings::Settings;
use std::collections::HashMap;
use team::Team;
use ui::web_app::{
    sidebar::{NavRoute, NavRoutes, Sidebar, SidebarType},
    toast::ToastProvider,
    UserAccountContext,
};

#[component]
fn Layout() -> Element {
    use_context_provider(|| AuthContext::new());
    let auth_context = use_context::<AuthContext>();
    let auth_context_for_effect = auth_context.clone();

    use_effect(move || {
        let mut auth_context_for_effect = auth_context_for_effect.clone();
        spawn(async move {
            // cookies are sent automatically with the request
            match get_current_user().await {
                Ok(Some(user)) => {
                    auth_context_for_effect.set_authenticated(user);
                }
                Ok(None) => {
                    auth_context_for_effect.clear();
                }
                Err(_) => {
                    auth_context_for_effect.clear();
                }
            }
        });
    });

    // create an empty UserAccount for when not authenticated
    let empty_user_account = UserAccount {
        id: 0,
        first_name: String::new(),
        last_name: String::new(),
        avatar_url: None,
        active_organization_membership_id: None,
        organization_memberships: HashMap::new(),
        notifications: HashMap::new(),
    };

    // get the actual user account if authenticated
    let user_account = auth_context
        .user_account
        .read()
        .as_ref()
        .map(user_response_to_account)
        .unwrap_or(empty_user_account);

    // provide the UserAccountContext
    use_context_provider(|| UserAccountContext {
        user_account: Store::new(user_account),
    });

    let user_account_context = use_context::<UserAccountContext>();

    let current_route = router().full_route_string();
    let is_main_sidebar = !current_route.ends_with(&Routes::Login {}.to_string())
        || !current_route.ends_with(&Routes::CreateAccount {}.to_string())
        || !current_route.ends_with(&Routes::CreateOrganization {}.to_string())
        || !current_route.starts_with(&Routes::Account {}.to_string());
    let is_account_sidebar = current_route.starts_with(&Routes::Account {}.to_string());

    let mut main_menu_routes = NavRoutes::new();
    let mut tools_routes = NavRoutes::new();
    let mut account_menu_routes = NavRoutes::new();
    let mut support_link = "".to_string();
    let mut account_settings_routes = NavRoutes::new();
    let mut dashboard_route = "".to_string();

    // populate the routes based on the sidebar type and user permissions
    if is_main_sidebar {
        main_menu_routes.push(NavRoute {
            route: Routes::Dashboard {}.to_string(),
            icon: rsx! {
                LayoutGrid {}
            },
            label: "Dashboard".to_string(),
        });

        if user_account_context.has_permission(SubscriptionType::Events) {
            main_menu_routes.extend([
                NavRoute {
                    route: Routes::Events {}.to_string(),
                    icon: rsx! {
                        Calendar {}
                    },
                    label: "Events".to_string(),
                },
                NavRoute {
                    route: Routes::Actions {}.to_string(),
                    icon: rsx! {
                        Megaphone {}
                    },
                    label: "Actions".to_string(),
                },
            ]);
        }

        tools_routes.extend([
            NavRoute {
                route: Routes::Events {}.to_string(),
                icon: rsx! {
                    ContactRound {}
                },
                label: "Groups".to_string(),
            },
            NavRoute {
                route: Routes::Analytics {}.to_string(),
                icon: rsx! {
                    ChartColumn {}
                },
                label: "Analytics".to_string(),
            },
            NavRoute {
                route: Routes::Exports {}.to_string(),
                icon: rsx! {
                    FileOutput {}
                },
                label: "Exports".to_string(),
            },
            NavRoute {
                route: Routes::Team {}.to_string(),
                icon: rsx! {
                    UsersRound {}
                },
                label: "Team".to_string(),
            },
            NavRoute {
                route: Routes::Settings {}.to_string(),
                icon: rsx! {
                    Settings1 {}
                },
                label: "Settings".to_string(),
            },
        ]);

        support_link = "https://support.campaigncenter.com".to_string();

        account_menu_routes.push(NavRoute {
            route: Routes::Account {}.to_string(),
            icon: rsx! {
                Settings2 {}
            },
            label: "Account Settings".to_string(),
        });
    }

    if is_account_sidebar {
        dashboard_route = Routes::Dashboard {}.to_string();

        account_settings_routes.extend([
            NavRoute {
                route: Routes::Account {}.to_string(),
                icon: rsx! {
                    User {}
                },
                label: "Account".to_string(),
            },
            NavRoute {
                route: Routes::OrganizationManagement {}.to_string(),
                icon: rsx! {
                    Building {}
                },
                label: "Organizations".to_string(),
            },
            NavRoute {
                route: Routes::NotificationPreferences {}.to_string(),
                icon: rsx! {
                    Bell {}
                },
                label: "Notifications".to_string(),
            },
            NavRoute {
                route: Routes::DeviceSessions {}.to_string(),
                icon: rsx! {
                    MonitorSmartphone {}
                },
                label: "Devices".to_string(),
            },
        ]);
    }

    // show skeleton while loading
    let auth_state = auth_context.state.read().clone();
    if matches!(auth_state, AuthState::Loading) {
        return rsx! {
            div { class: "flex bg-background min-h-screen",
                // sidebar skeleton
                div { class: "w-64 bg-muted animate-pulse" }
                // main content skeleton
                main { class: "flex-1 p-8",
                    div { class: "space-y-4",
                        div { class: "h-8 w-48 bg-muted rounded animate-pulse" }
                        div { class: "h-4 w-96 bg-muted rounded animate-pulse" }
                        div { class: "h-32 w-full bg-muted rounded animate-pulse" }
                    }
                }
            }
        };
    }

    rsx! {
        ToastProvider {
            div { class: "flex bg-background min-h-screen",
                // only render the sidebar if there's an active organization membership
                if let Some(active_organization_membership) = user_account_context
                    .get_active_organization_membership_id()
                    .and_then(|id| {
                        user_account_context.user_account.organization_memberships().get(id)
                    })
                {
                    if is_main_sidebar {
                        Sidebar {
                            r#type: SidebarType::Main,
                            active_organization_membership,
                            current_route,
                            main_menu_routes,
                            tools_routes,
                            support_link,
                            account_menu_routes,
                        }
                    } else {
                        Sidebar {
                            r#type: SidebarType::UserAccount,
                            active_organization_membership,
                            current_route,
                            dashboard_route,
                            account_settings_routes,
                        }
                    }
                }
                main { class: "flex-1 p-8 overflow-y-auto", Outlet::<Routes> {} }
            }
        }
    }
}

#[derive(Clone, PartialEq, Routable)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
pub enum Routes {
    #[layout(Layout)]

    #[route("/login")]
    Login {},

    #[route("/create/account")]
    CreateAccount {},

    #[route("/create/organization")]
    CreateOrganization {},

    #[route("/")]
    Dashboard {},

    #[route("/events")]
    Events {},

    #[route("/actions")]
    Actions {},

    #[route("/groups")]
    Groups {},

    #[route("/analytics")]
    Analytics {},

    #[route("/exports")]
    Exports {},

    #[route("/team")]
    Team {},

    #[route("/settings")]
    Settings {},
    
    #[route("/account")]
    Account {},

    #[route("/account/devices")]
    DeviceSessions {},

    #[route("/account/notifications")]
    NotificationPreferences {},

    #[route("/account/organizations")]
    OrganizationManagement {},
}
