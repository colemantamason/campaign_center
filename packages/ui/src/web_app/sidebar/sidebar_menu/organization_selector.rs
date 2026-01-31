mod organization_container;

use crate::shared::button::{Button, ButtonSize, ButtonType, ButtonVariant};
use crate::shared::divider::Divider;
use crate::shared::icon::{Icon, IconSize, IconVariant};
use crate::shared::input::{Input, InputSize, InputType, InputVariant};
use crate::web_app::confirmation_modal::{ConfirmationModal, ConfirmationModalType};
use crate::web_app::UserAccountContext;
use api::web_app::{
    Organization, OrganizationMembershipStoreExt, OrganizationStoreExt, UserAccountStoreExt,
};
use dioxus::prelude::*;
use lucide_dioxus::{Plus, X};
use organization_container::{OrganizationContainer, OrganizationContainerType};
use std::cmp::Ordering;

#[derive(Clone, PartialEq, Props)]
pub struct OrganizationSelectorProps {
    active_organization: Option<Store<Organization>>,
    show_menu: Signal<bool>,
}

#[component]
pub fn OrganizationSelector(mut props: OrganizationSelectorProps) -> Element {
    let user_account = use_context::<UserAccountContext>().user_account;
    let search_text = use_signal(|| "".to_string());
    let mut pending_organization_membership_id = use_signal(|| None::<i32>);
    let show_confirmation_modal = use_signal(|| false);

    let handle_organization_switch = move |id: i32| match id {
        -1 => return,
        _ => user_account
            .active_organization_membership_id()
            .set(Some(id)),
    };

    rsx! {
        if let Some(active_organization) = &props.active_organization {
            OrganizationContainer {
                r#type: OrganizationContainerType::Selector,
                selector_name: Some(active_organization.name().into()),
                selector_avatar_url: Some(active_organization.avatar_url().into()),
                selector_member_count: Some(active_organization.member_count().into()),
                show_menu: Some(props.show_menu.into()),
            }
            if *props.show_menu.read() {
                div { class: "absolute left-full top-2 ml-2 w-60 z-40 bg-sidebar border border-border rounded-md shadow-lg py-2 gap-2 flex flex-col",
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
                                id: "organization-search".to_string(),
                                value: search_text,
                                label: "Search...".to_string(),
                                size: InputSize::Default,
                                variant: InputVariant::Sidebar,
                            }
                        }
                    }
                    Divider {}
                    div { class: "flex flex-col gap-1 max-h-64 overflow-y-auto",
                        {
                            let search = search_text.read().to_lowercase();
                            let mut visible_organizations: Vec<Organization> = user_account
                                .organization_memberships()
                                .read()
                                .values()
                                .filter(|membership| {
                                    membership.organization.name.to_lowercase().contains(&search)
                                })
                                .map(|membership| membership.organization.clone())
                                .collect();
                            visible_organizations
                                .sort_by(|a, b| {
                                    let a_is_active = a.id == active_organization.read().id;
                                    let b_is_active = b.id == active_organization.read().id;
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
                                            if organization.id == active_organization.read().id {
                                                OrganizationContainer {
                                                    r#type: OrganizationContainerType::Active,
                                                    name: Some(organization.name.clone().into()),
                                                    avatar_url: organization.avatar_url.clone(),
                                                    member_count: Some(organization.member_count.into()),
                                                }
                                            } else {
                                                OrganizationContainer {
                                                    r#type: OrganizationContainerType::NonActive,
                                                    name: Some(organization.name.clone().into()),
                                                    avatar_url: organization.avatar_url.clone(),
                                                    member_count: Some(organization.member_count.into()),
                                                    show_menu: Some(props.show_menu.into()),
                                                    organization_id: Some(organization.id.into()),
                                                    pending_organization_membership_id: Some(pending_organization_membership_id.into()),
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
            if *show_confirmation_modal.read() {
                ConfirmationModal {
                    r#type: ConfirmationModalType::Default,
                    title: "Switch Organization".to_string(),
                    message: {
                        if let Some(pending_organization_membership) = user_account
                            .organization_memberships()
                            .get(
                                if let Some(pending_organization_membership_id) = *pending_organization_membership_id
                                    .read()
                                {
                                    pending_organization_membership_id
                                } else {
                                    -1
                                },
                            )
                        {
                            format!(
                                "Are you sure you want to switch to {}?",
                                *pending_organization_membership.organization().name().read(),
                            )
                        } else {
                            "There was an error trying to switch organizations. Please refresh the page and try again."
                                .to_string()
                        }
                    },
                    confirm_text: "Switch".to_string(),
                    cancel_text: "Cancel".to_string(),
                    show_modal: show_confirmation_modal,
                    on_confirm: move |_| {
                        handle_organization_switch(
                            if let Some(pending_organization_membership_id) = *pending_organization_membership_id
                                .read()
                            {
                                pending_organization_membership_id
                            } else {
                                -1
                            },
                        );
                        pending_organization_membership_id.set(None);
                    },
                    on_cancel: move |_| {
                        pending_organization_membership_id.set(None);
                    },
                }
            }
        }
    }
}
