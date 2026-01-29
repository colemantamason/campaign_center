pub mod avatar;
pub mod confirmation_modal;
pub mod notification_badge;
pub mod sidebar;
pub mod toast;

use api::web_app::UserAccount;
use dioxus::prelude::*;

#[derive(Clone)]
pub struct UserAccountContext {
    pub user_account: Store<UserAccount>,
}
