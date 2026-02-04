use crate::auth::AuthContext;
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
    let current_route = router().full_route_string();
    let is_login_page = current_route.starts_with(&Routes::Login {}.to_string());
    let is_create_account_page = current_route.starts_with(&Routes::CreateAccount {}.to_string());
    let is_create_organization_page =
        current_route.starts_with(&Routes::CreateOrganization {}.to_string());

    let auth_context = use_context::<AuthContext>();
    let is_authenticated = auth_context.is_authenticated();

    // check if user is logged in
    if !is_authenticated {
        // allow access to login and create account pages if not logged in
        if !is_login_page && !is_create_account_page {
            // if not authenticated, redirect to login page
            router().push(Routes::Login {}.to_string());
            return rsx! {};
        }
    } else {
        // if authenticated and on login or create account page, redirect to dashboard or create org
        if is_login_page || is_create_account_page {
            if auth_context.has_organizations() {
                router().push(Routes::Dashboard {}.to_string());
            } else {
                router().push(Routes::CreateOrganization {}.to_string());
            }
            return rsx! {};
        }
    }

    let user_account_context = use_context::<UserAccountContext>();

    // check for an active organization (as long as we're not on the create organization, create account, or login page)
    if !is_create_organization_page && !is_create_account_page && !is_login_page && is_authenticated {
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
            router().push(Routes::CreateOrganization {}.to_string());
            return rsx! {};
        }
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
