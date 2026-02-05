mod organization_container;

use crate::shared::{
    Button, ButtonSize, ButtonType, ButtonVariant, Divider, Icon, IconSize, IconVariant, Input,
    InputSize, InputType, InputVariant,
};
use crate::web_app::{ConfirmationModal, ConfirmationModalType, UserAccountContext};
use api::state::{
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
    let mut selected_organization_membership_id = use_signal(|| None::<i32>);
    let show_confirmation_modal = use_signal(|| false);

    let handle_organization_switch = move |id: Option<i32>| match id {
        // invalid id, do nothing
        None => return,
        // valid id, switch organization
        Some(id) => user_account
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
                            // filter organizations based on search text
                            let search = search_text.read().to_lowercase();
                            // Keep full membership info (not just Organization) to preserve membership_id
                            // Clone memberships into a vec to own the data (avoids borrow issues)
                            let mut visible_memberships: Vec<_> = user_account
                                .organization_memberships()
                                .read()
                                .values()
                                .filter(|membership| {
                                    membership.organization.name.to_lowercase().contains(&search)
                                })
                                .cloned()
                                .collect();
                            visible_memberships
                                .sort_by(|a, b| {
                                    let a_is_active = a.organization.id == active_organization.read().id;
                                    let b_is_active = b.organization.id == active_organization.read().id;
                                    if a_is_active && !b_is_active {
                                        Ordering::Less
                                    } else if !a_is_active && b_is_active {
                                        Ordering::Greater
                                    } else {
                                        a.organization.name.cmp(&b.organization.name)
                                    }
                                });
                            if visible_memberships.is_empty() {
                                rsx! {
                                    div { class: "px-4 py-3 text-sm text-muted-foreground text-center", "No results found" }
                                }
                            } else {
                                rsx! {
                                    for membership in visible_memberships {
                                        div { key: "{membership.id}", class: "w-full px-2",
                                            if membership.organization.id == active_organization.read().id {
                                                OrganizationContainer {
                                                    r#type: OrganizationContainerType::Active,
                                                    name: Some(membership.organization.name.clone().into()),
                                                    avatar_url: membership.organization.avatar_url.clone(),
                                                    member_count: Some(membership.organization.member_count.into()),
                                                }
                                            } else {
                                                OrganizationContainer {
                                                    r#type: OrganizationContainerType::NonActive,
                                                    name: Some(membership.organization.name.clone().into()),
                                                    avatar_url: membership.organization.avatar_url.clone(),
                                                    member_count: Some(membership.organization.member_count.into()),
                                                    show_menu: Some(props.show_menu.into()),
                                                    organization_membership_id: Some(membership.id.into()),
                                                    selected_organization_membership_id: Some(selected_organization_membership_id.into()),
                                                    show_confirmation_modal: Some(show_confirmation_modal.into()),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // TODO: implement organization creation flow or remove this button
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
                        if let Some(selected_organization_membership) = user_account
                            .organization_memberships()
                            .get(
                                // unwrap the selected organization membership id or use -1 as a fallback if somehow missing
                                if let Some(selected_organization_membership_id) = *selected_organization_membership_id
                                    .read()
                                {
                                    selected_organization_membership_id
                                } else {
                                    -1
                                },
                            )
                        {
                            format!(
                                "Are you sure you want to switch to {}?",
                                *selected_organization_membership.organization().name().read(),
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
                            if let Some(selected_organization_membership_id) = *selected_organization_membership_id
                                .read()
                            {
                                Some(selected_organization_membership_id)
                            } else {
                                None
                            },
                        );
                        selected_organization_membership_id.set(None);
                    },
                    on_cancel: move |_| {
                        selected_organization_membership_id.set(None);
                    },
                }
            }
        }
    }
}
