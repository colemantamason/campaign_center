use crate::routes::Routes;
use api::models::SubscriptionType;
use api::state::UserAccountStoreExt;
use dioxus::prelude::*;
use ui::web_app::{
    toast::{ToastContext, ToastVariant},
    UserAccountContext,
};

#[derive(Clone, PartialEq, Props)]
pub struct GateProps {
    required_permission: Option<SubscriptionType>,
    permission_fallback_route: Option<String>,
    children: Element,
}

#[component]
pub fn Gate(props: GateProps) -> Element {
    // TODO: add auth check here

    let user_account_context = use_context::<UserAccountContext>();

    // check for an active organization
    if user_account_context
        .get_active_organization_membership_id()
        .and_then(|id| {
            user_account_context
                .user_account
                .organization_memberships()
                .get(id)
        })
        .is_none()
    {
        // TODO: redirect to organization signup page if no active organization found
        return rsx! {};
    }

    let mut toast_context = use_context::<ToastContext>();

    // if a required permission is specified, check if the user has that permission
    if let Some(required_permission) = props.required_permission {
        if !user_account_context.has_permission(required_permission) {
            // show an error toast if no permission found
            toast_context.create(
                "Access Denied".to_string(),
                "You don't have permissions to view that page for this organization.".to_string(),
                ToastVariant::Error,
            );
            // redirect to the fallback route if no permission found
            if let Some(route) = &props.permission_fallback_route {
                router().push(route.clone());
            } else {
                // if no fallback route provided, redirect to dashboard
                router().push(Routes::Dashboard {}.to_string());
            }
            return rsx! {};
        }
    }

    // render the route if not redirected
    props.children
}
