pub mod avatar;
pub mod confirmation_modal;
pub mod notification_badge;
pub mod sidebar;
pub mod toast;

use api::enums::SubscriptionType;
use api::state::{UserAccount, UserAccountStoreExt};
pub use avatar::*;
pub use confirmation_modal::*;
use dioxus::prelude::*;
pub use notification_badge::*;
pub use sidebar::*;
pub use toast::*;

#[derive(Clone, PartialEq)]
pub struct UserAccountContext {
    pub user_account: Store<UserAccount>,
}

impl UserAccountContext {
    pub fn get_active_organization_membership_id(&self) -> Option<i32> {
        // get the active organization ID from the user account
        if let Some(active_organization_membership_id) = self
            .user_account
            .active_organization_membership_id()
            .cloned()
        {
            // check if the active organization ID is valid
            if self
                .user_account
                .organization_memberships()
                .read()
                .contains_key(&active_organization_membership_id)
            {
                return Some(active_organization_membership_id);
            }

            // active organization ID is not valid, try to set it to the first organization
            if let Some(first_org) = self
                .user_account
                .organization_memberships()
                .read()
                .values()
                .next()
            {
                let first_id = first_org.organization_id;
                self.user_account
                    .active_organization_membership_id()
                    .set(Some(first_id));
                return Some(first_id);
            } else {
                // no organizations available, clear the active organization ID
                self.user_account
                    .active_organization_membership_id()
                    .set(None);
                return None;
            }
        } else {
            // no active organization ID set
            return None;
        }
    }

    pub fn has_permission(&self, subscription_type: SubscriptionType) -> bool {
        // get the active organization id
        if let Some(active_organization_membership_id) =
            self.get_active_organization_membership_id()
        {
            // get the active organization
            if let Some(organization) = self
                .user_account
                .organization_memberships()
                .read()
                .get(&active_organization_membership_id)
            {
                // check if the organization has the required permission
                if let Some(permission) = organization.permissions.get(&subscription_type) {
                    return *permission;
                } else {
                    // permission not found
                    return false;
                }
            } else {
                // active organization not found
                return false;
            }
        } else {
            // no active organization ID set
            return false;
        }
    }
}
