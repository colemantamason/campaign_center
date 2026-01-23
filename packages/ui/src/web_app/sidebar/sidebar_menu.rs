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
use web_sys::Node;

#[derive(Props, Clone, PartialEq)]
pub struct SidebarMenuProps {
    #[props(default = false)]
    is_organization_selector: bool,
    active_organization_id: Option<Store<i32>>,
    organizations: Option<Store<Organizations>>,
    #[props(default = false)]
    is_notification_menu: bool,
    notifications: Option<Store<i32>>,
    #[props(default = false)]
    is_user_account_menu: bool,
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

    #[cfg(feature = "web")]
    use_effect(move || {
        if show_menu() {
            let document = web_sys::window().unwrap().document().unwrap();
            let listener = EventListener::new(&document, "click", move |event| {
                let target = event.target().and_then(|t| t.dyn_into::<Node>().ok());
                if let (Some(container), Some(target)) = (container_node.read().as_ref(), target) {
                    if !container.contains(Some(&target)) {
                        show_menu.set(false);
                    }
                }
            });
            click_outside_listener.set(Some(listener));
        } else {
            click_outside_listener.set(None);
        }
    });

    rsx! {
        div {
            onmounted: move |_element| {
                #[cfg(feature = "web")] container_node.set(Some(_element.as_web_event().into()));
            },
            if props.is_organization_selector {
                OrganizationSelector {
                    active_organization_id: props.active_organization_id.unwrap(),
                    organizations: props.organizations.unwrap(),
                    active_organization: props.active_organization.unwrap(),
                    show_menu,
                }
            } else if props.is_notification_menu {
                NotificationMenu {
                    notifications: props.notifications.unwrap(),
                    show_menu,
                }
            } else if props.is_user_account_menu {
                UserAccountMenu {
                    user_first_name: props.user_first_name.unwrap(),
                    user_last_name: props.user_last_name.unwrap(),
                    user_avatar_url: props.user_avatar_url.unwrap(),
                    user_role: props.user_role.unwrap(),
                    account_route: props.account_route.unwrap(),
                    show_menu,
                }
            }
        }
    }
}
