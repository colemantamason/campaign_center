mod organization_container;

use crate::shared::button::{Button, ButtonSize, ButtonType, ButtonVariant};
use crate::shared::divider::Divider;
use crate::shared::icon::{Icon, IconSize, IconVariant};
use crate::shared::input::{Input, InputSize, InputType};
use crate::web_app::confirmation_modal::{ConfirmationModal, ConfirmationModalType};
use api::web_app::{Organization, OrganizationStoreExt, Organizations};
use dioxus::prelude::*;
use lucide_dioxus::{Plus, X};
use organization_container::{OrganizationContainer, OrganizationContainerType};
use std::cmp::Ordering;

#[derive(Clone, PartialEq, Props)]
pub struct OrganizationSelectorProps {
    active_organization_id: Store<i32>,
    organizations: Store<Organizations>,
    active_organization: Store<Organization>,
    show_menu: Signal<bool>,
}

#[component]
pub fn OrganizationSelector(mut props: OrganizationSelectorProps) -> Element {
    let search_text = use_signal(|| "".to_string());
    let mut pending_organization_id = use_signal(|| None::<i32>);
    let show_confirmation_modal: Signal<bool> = use_signal(|| false);

    let mut handle_organization_switch = move |id: i32| {
        props.active_organization_id.set(id);
    };

    rsx! {
        OrganizationContainer {
            r#type: OrganizationContainerType::Selector,
            selector_name: Some(props.active_organization.name().into()),
            selector_avatar_url: Some(props.active_organization.avatar_url().into()),
            selector_member_count: Some(props.active_organization.member_count().into()),
            show_menu: Some(props.show_menu.into()),
        }
        if *props.show_menu.read() {
            div { class: "absolute left-full top-2 ml-2 w-60 z-50 bg-sidebar border border-border rounded-md shadow-lg py-2 gap-2 flex flex-col",
                div { class: "flex flex-col gap-3",
                    div { class: "flex flex-row justify-between items-center px-2",
                        span { class: "text-sm font-medium text-foreground cursor-default",
                            "Switch Organizations"
                        }
                        Button {
                            r#type: ButtonType::Button,
                            onclick: move |_| props.show_menu.set(false),
                            size: ButtonSize::Icon,
                            variant: ButtonVariant::Sidebar,
                            Icon {
                                size: IconSize::Small,
                                variant: IconVariant::Button,
                                X {}
                            }
                        }
                    }
                    div { class: "px-2",
                        Input {
                            r#type: InputType::Text,
                            size: InputSize::Default,
                            id: "organization-search".to_string(),
                            label: "Search...".to_string(),
                            value: search_text,
                        }
                    }
                }
                Divider {}
                div { class: "flex flex-col gap-1 max-h-64 overflow-y-auto",
                    {
                        let search = search_text.read().to_lowercase();
                        let mut visible_organizations: Vec<Organization> = props
                            .organizations
                            .read()
                            .values()
                            .filter(|organization| {
                                organization.name.to_lowercase().contains(&search)
                            })
                            .cloned()
                            .collect();
                        visible_organizations
                            .sort_by(|a, b| {
                                let a_is_active = a.id == props.active_organization.read().id;
                                let b_is_active = b.id == props.active_organization.read().id;
                                if a_is_active && !b_is_active {
                                    Ordering::Less
                                } else if !a_is_active && b_is_active {
                                    Ordering::Greater
                                } else {
                                    a.name.cmp(&b.name)
                                }
                            });
                        if visible_organizations.is_empty() {
                            rsx! {
                                div { class: "px-4 py-3 text-sm text-muted-foreground text-center", "No results found" }
                            }
                        } else {
                            rsx! {
                                for organization in visible_organizations {
                                    div { key: "{organization.id}", class: "w-full px-2",
                                        if organization.id == props.active_organization.read().id {
                                            OrganizationContainer {
                                                r#type: OrganizationContainerType::Active,
                                                name: Some(organization.name.clone().into()),
                                                avatar_url: organization.avatar_url.clone(),
                                                member_count: Some(organization.member_count.into()),
                                            }
                                        } else {
                                            OrganizationContainer {
                                                r#type: OrganizationContainerType::NonActive,
                                                organization_id: Some(organization.id.into()),
                                                name: Some(organization.name.clone().into()),
                                                avatar_url: organization.avatar_url.clone(),
                                                member_count: Some(organization.member_count.into()),
                                                show_menu: Some(props.show_menu.into()),
                                                pending_organization_id: Some(pending_organization_id.into()),
                                                show_confirmation_modal: Some(show_confirmation_modal.into()),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Divider {}
                div { class: "w-full px-2",
                    Button {
                        r#type: ButtonType::Button,
                        size: ButtonSize::Full,
                        variant: ButtonVariant::Sidebar,
                        Icon {
                            size: IconSize::Medium,
                            variant: IconVariant::Sidebar,
                            Plus {}
                        }
                        span { "Add Organization" }
                    }
                }
            }
        }
        if show_confirmation_modal() {
            ConfirmationModal {
                r#type: ConfirmationModalType::Default,
                title: "Switch Organization".to_string(),
                message: {
                    let pending_organization_name = props
                        .organizations
                        .get(pending_organization_id().unwrap())
                        .unwrap()
                        .name()
                        .to_string();
                    format!("Are you sure you want to switch to {}?", pending_organization_name)
                },
                confirm_text: "Switch".to_string(),
                cancel_text: "Cancel".to_string(),
                show_modal: show_confirmation_modal,
                on_confirm: move |_| {
                    handle_organization_switch(pending_organization_id().unwrap());
                    pending_organization_id.set(None);
                },
                on_cancel: move |_| {
                    pending_organization_id.set(None);
                },
            }
        }
    }
}
