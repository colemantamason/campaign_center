mod account;
mod actions;
mod analytics;
mod dashboard;
mod events;
mod exports;
mod groups;
mod login;
mod notifications;
mod page_not_found;
mod settings;
mod team;

use account::{
    devices::DeviceSessions, notifications::NotificationPreferences,
    organizations::OrganizationManagement, Account,
};
use actions::Actions;
use analytics::Analytics;
use api::web_app::get_mock_user_account;
use dashboard::Dashboard;
use dioxus::prelude::*;
use events::Events;
use exports::Exports;
use groups::Groups;
use login::Login;
use notifications::Notifications;
use page_not_found::PageNotFound;
use settings::Settings;
use team::Team;
use ui::web_app::sidebar::{NavRoutes, RouteType, Sidebar, SidebarType};

#[component]
fn Layout() -> Element {
    let user_account = use_store(|| get_mock_user_account());

    let main_sidebar_routes = NavRoutes::from([
        (
            RouteType::Notifications,
            Routes::Notifications {}.to_string(),
        ),
        (RouteType::Dashboard, Routes::Dashboard {}.to_string()),
        (RouteType::Events, Routes::Events {}.to_string()),
        (RouteType::Actions, Routes::Actions {}.to_string()),
        (RouteType::Groups, Routes::Groups {}.to_string()),
        (RouteType::Analytics, Routes::Analytics {}.to_string()),
        (RouteType::Exports, Routes::Exports {}.to_string()),
        (RouteType::Team, Routes::Team {}.to_string()),
        (RouteType::Settings, Routes::Settings {}.to_string()),
        (RouteType::Support, "#".to_string()),
        (RouteType::Account, Routes::Account {}.to_string()),
    ]);

    let account_sidebar_routes = NavRoutes::from([
        (RouteType::Account, Routes::Account {}.to_string()),
        (
            RouteType::OrganizationManagement,
            Routes::OrganizationManagement {}.to_string(),
        ),
        (
            RouteType::NotificationPreferences,
            Routes::NotificationPreferences {}.to_string(),
        ),
        (
            RouteType::DeviceSessions,
            Routes::DeviceSessions {}.to_string(),
        ),
        (RouteType::Dashboard, Routes::Dashboard {}.to_string()),
    ]);

    rsx! {
        div { class: "flex bg-background",
            if !router().full_route_string().starts_with(Routes::Account {}.to_string().as_str()) {
                Sidebar {
                    r#type: SidebarType::Main,
                    user_account,
                    nav_routes: main_sidebar_routes,
                }
            } else {
                Sidebar {
                    r#type: SidebarType::UserAccount,
                    user_account,
                    nav_routes: account_sidebar_routes,
                }
            }
            main { class: "flex-1 p-8 overflow-y-auto", Outlet::<Routes> {} }
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

    #[route("/notifications")]
    Notifications {},

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
