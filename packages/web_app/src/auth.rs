use api::interfaces::UserAccountResponse;
use api::providers::get_current_user;
use api::state::{
    Organization, OrganizationMembership, OrganizationMemberships, Permissions, UserAccount,
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum AuthState {
    Loading,
    NotAuthenticated,
    NoOrganizations,
    Ready,
}

#[derive(Clone)]
pub struct AuthContext {
    pub state: Signal<AuthState>,
    pub user_account: Signal<Option<UserAccountResponse>>,
}

impl AuthContext {
    pub fn new() -> Self {
        Self {
            state: Signal::new(AuthState::Loading),
            user_account: Signal::new(None),
        }
    }

    pub fn set_loading(&mut self) {
        self.state.set(AuthState::Loading);
    }

    pub fn set_authenticated(&mut self, user: UserAccountResponse) {
        if user.organization_memberships.is_empty() {
            self.state.set(AuthState::NoOrganizations);
        } else {
            self.state.set(AuthState::Ready);
        }
        self.user_account.set(Some(user));
    }

    pub fn update_user(&mut self, user: UserAccountResponse) {
        if user.organization_memberships.is_empty() {
            self.state.set(AuthState::NoOrganizations);
        } else {
            self.state.set(AuthState::Ready);
        }
        self.user_account.set(Some(user));
    }

    pub fn clear(&mut self) {
        self.state.set(AuthState::NotAuthenticated);
        self.user_account.set(None);
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(
            *self.state.read(),
            AuthState::NoOrganizations | AuthState::Ready
        )
    }

    pub fn has_organizations(&self) -> bool {
        matches!(*self.state.read(), AuthState::Ready)
    }

    pub fn is_loading(&self) -> bool {
        matches!(*self.state.read(), AuthState::Loading)
    }
}

/// Convert UserAccountResponse to UserAccount for UserAccountContext
pub fn user_response_to_account(response: &UserAccountResponse) -> UserAccount {
    let mut organization_memberships: OrganizationMemberships = HashMap::new();

    for (id, membership_info) in &response.organization_memberships {
        let organization = Organization::new(
            membership_info.organization.id,
            membership_info.organization.name.clone(),
            membership_info.organization.avatar_url.clone(),
            membership_info.organization.member_count,
        );

        let permissions: Permissions = membership_info.permissions.clone();

        let membership = OrganizationMembership {
            organization_id: *id,
            organization,
            user_role: membership_info.user_role.clone(),
            permissions,
        };

        organization_memberships.insert(*id, membership);
    }

    UserAccount {
        id: response.id,
        first_name: response.first_name.clone(),
        last_name: response.last_name.clone(),
        avatar_url: response.avatar_url.clone(),
        active_organization_membership_id: response.active_organization_membership_id,
        organization_memberships,
        notifications: HashMap::new(),
    }
}

/// Hook to initialize auth context and check auth state on mount
pub fn use_auth_init() -> AuthContext {
    let auth_context = use_context::<AuthContext>();
    let auth_context_for_effect = auth_context.clone();

    // Check auth state on mount - cookies are sent automatically
    use_effect(move || {
        let mut auth_ctx = auth_context_for_effect.clone();

        spawn(async move {
            // cookies are sent automatically with the request
            match get_current_user().await {
                Ok(Some(user)) => {
                    auth_ctx.set_authenticated(user);
                }
                Ok(None) => {
                    auth_ctx.clear();
                }
                Err(_) => {
                    auth_ctx.clear();
                }
            }
        });
    });

    auth_context
}
