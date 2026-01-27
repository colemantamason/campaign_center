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
    active_organization_id: Option<Store<i32>>,
    organizations: Option<Store<Organizations>>,
    notifications: Option<Store<i32>>,
    user_first_name: Option<Store<String>>,
    user_last_name: Option<Store<String>>,
    user_avatar_url: Option<Store<Option<String>>>,
    user_role: Option<Store<String>>,
    account_route: Option<String>,
    active_organization: Option<Store<Organization>>,
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
        if show_menu() {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    let listener = EventListener::new(&document, "click", move |event| {
                        if let Some(target) = event
                            .target()
                            .and_then(|target| target.dyn_into::<Node>().ok())
                        {
                            if let Some(container) = container_node.read().as_ref() {
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
                SidebarMenuType::OrganizationSelector => rsx! {
                    OrganizationSelector {
                        active_organization_id: props.active_organization_id.unwrap(),
                        organizations: props.organizations.unwrap(),
                        active_organization: props.active_organization.unwrap(),
                        show_menu,
                    }
                },
                SidebarMenuType::NotificationMenu => rsx! {
                    NotificationMenu { notifications: props.notifications.unwrap(), show_menu }
                },
                SidebarMenuType::UserAccountMenu => rsx! {
                    UserAccountMenu {
                        user_first_name: props.user_first_name.unwrap(),
                        user_last_name: props.user_last_name.unwrap(),
                        user_avatar_url: props.user_avatar_url.unwrap(),
                        user_role: props.user_role.unwrap(),
                        account_route: props.account_route.unwrap(),
                        show_menu,
                    }
                },
            }
        }
    }
}
