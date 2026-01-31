mod account;
mod actions;
mod analytics;
mod dashboard;
mod events;
mod exports;
mod groups;
mod login;
mod page_not_found;
mod settings;
mod team;

use account::{
    devices::DeviceSessions, notifications::NotificationPreferences,
    organizations::OrganizationManagement, Account,
};
use actions::Actions;
use analytics::Analytics;
use api::web_app::{get_mock_user_account, PermissionType, UserAccountStoreExt};
use dashboard::Dashboard;
use dioxus::prelude::*;
use events::Events;
use exports::Exports;
use groups::Groups;
use login::Login;
use lucide_dioxus::{
    Bell, Building, Calendar, ChartColumn, ContactRound, FileOutput, LayoutGrid, Megaphone,
    MonitorSmartphone, Settings as SettingsIcon, Settings2, User, UsersRound,
};
use page_not_found::PageNotFound;
use settings::Settings;
use team::Team;
use ui::web_app::{
    sidebar::{NavRoute, NavRoutes, Sidebar, SidebarType},
    toast::ToastProvider,
    UserAccountContext,
};

#[component]
fn Layout() -> Element {
    use_context_provider(|| UserAccountContext {
        user_account: Store::new(get_mock_user_account()),
    });

    let user_account_context = use_context::<UserAccountContext>();

    let current_route = router().full_route_string();
    let is_main_sidebar = !current_route.starts_with(&Routes::Account {}.to_string());

    let mut main_menu_routes = NavRoutes::new();
    let mut tools_routes = NavRoutes::new();
    let mut account_menu_routes = NavRoutes::new();
    let support_link = "#".to_string();
    let mut account_settings_routes = NavRoutes::new();
    let dashboard_route = Routes::Dashboard {}.to_string();

    if is_main_sidebar {
        main_menu_routes.push(NavRoute {
            route: Routes::Dashboard {}.to_string(),
            icon: rsx! {
                LayoutGrid {}
            },
            label: "Dashboard".to_string(),
        });

        if user_account_context.has_permission(PermissionType::Events) {
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
                    SettingsIcon {}
                },
                label: "Settings".to_string(),
            },
        ]);

        account_menu_routes.push(NavRoute {
            route: Routes::Account {}.to_string(),
            icon: rsx! {
                Settings2 {}
            },
            label: "Account Settings".to_string(),
        });
    } else {
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

    rsx! {
        ToastProvider {
            div { class: "flex bg-background",
                // only render the sidebar if there's an active organization
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

    #[route("/:..segments")]
    PageNotFound {segments: Vec<String>},
}
