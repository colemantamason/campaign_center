mod notification_menu;
mod organization_selector;
mod user_account_menu;

use api::web_app::{Organization, Organizations};
use dioxus::prelude::*;
#[cfg(feature = "web")]
use dioxus::web::WebEventExt;
#[cfg(feature = "web")]
use gloo::events::EventListener;
use notification_menu::NotificationMenu;
use organization_selector::OrganizationSelector;
use user_account_menu::UserAccountMenu;
#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use web_sys::{window, Node};

#[derive(Clone, PartialEq)]
pub enum SidebarMenuType {
    OrganizationSelector,
    NotificationMenu,
    UserAccountMenu,
}

#[derive(Clone, PartialEq, Props)]
pub struct SidebarMenuProps {
    r#type: SidebarMenuType,
    user_first_name: Option<Store<String>>,
    user_last_name: Option<Store<String>>,
    user_avatar_url: Option<Store<Option<String>>>,
    active_organization_id: Option<Store<i32>>,
    active_organization: Option<Store<Organization>>,
    user_role: Option<Store<String>>,
    organizations: Option<Store<Organizations>>,
    notifications: Option<Store<i32>>,
    account_route: Option<String>,
}

#[component]
pub fn SidebarMenu(props: SidebarMenuProps) -> Element {
    let mut show_menu = use_signal(|| false);
    #[cfg(feature = "web")]
    let mut container_node = use_signal(|| None::<Node>);
    #[cfg(feature = "web")]
    let mut click_outside_listener = use_signal(|| None::<EventListener>);

    use_effect(move || {
        #[cfg(feature = "web")]
        if *show_menu.read() {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    let listener = EventListener::new(&document, "click", move |event| {
                        if let Some(target) = event
                            .target()
                            .and_then(|target| target.dyn_into::<Node>().ok())
                        {
                            if let Some(container) = &*container_node.read() {
                                if !container.contains(Some(&target)) {
                                    show_menu.set(false);
                                }
                            }
                        }
                    });
                    click_outside_listener.set(Some(listener));
                }
            }
        } else {
            click_outside_listener.set(None);
        }
    });

    rsx! {
        div {
            onmounted: move |element| {
                #[cfg(feature = "web")]
                {
                    let element = element.data();
                    let node = element.as_web_event().dyn_into::<Node>().ok();
                    container_node.set(node);
                }
            },
            match props.r#type {
                SidebarMenuType::OrganizationSelector => {
                    if let (
                        Some(active_organization_id),
                        Some(active_organization),
                        Some(organizations),
                    ) = (
                        props.active_organization_id,
                        props.active_organization,
                        props.organizations,
                    ) {
                        rsx! {
                            OrganizationSelector {
                                active_organization_id,
                                active_organization,
                                organizations,
                                show_menu,
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
                SidebarMenuType::NotificationMenu => {
                    if let Some(notifications) = props.notifications {
                        rsx! {
                            NotificationMenu { notifications, show_menu }
                        }
                    } else {
                        rsx! {}
                    }
                }
                SidebarMenuType::UserAccountMenu => {
                    if let (
                        Some(user_first_name),
                        Some(user_last_name),
                        Some(user_avatar_url),
                        Some(user_role),
                        Some(account_route),
                    ) = (
                        props.user_first_name,
                        props.user_last_name,
                        props.user_avatar_url,
                        props.user_role,
                        props.account_route,
                    ) {
                        rsx! {
                            UserAccountMenu {
                                user_first_name,
                                user_last_name,
                                user_avatar_url,
                                user_role,
                                account_route,
                                show_menu,
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
            }
        }
    }
}
