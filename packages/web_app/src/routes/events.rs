use crate::routes::Routes;
use api::web_app::{OrganizationStoreExt, PermissionType, UserAccountStoreExt};
use dioxus::prelude::*;
use ui::web_app::toast::{ToastContext, ToastVariant};
use ui::web_app::UserAccountContext;

#[component]
pub fn Events() -> Element {
    let mut toast_context = use_context::<ToastContext>();
    let user_account = use_context::<UserAccountContext>().user_account;

    if let Some(active_organization) = user_account
        .organizations()
        .get(user_account.active_organization_id().cloned())
    {
        if let Some(permission) = active_organization
            .permissions()
            .get(PermissionType::Events)
        {
            if !permission() {
                toast_context.create(
                    "Access Denied".to_string(),
                    "You don't have permissions to view that page for this organization."
                        .to_string(),
                    ToastVariant::Error,
                );
                router().push(Routes::Dashboard {});
            }
        }
    }

    rsx! {
        div { class: "w-full",
            h1 { class: "text-primary font-bold text-xl", "Events" }
            p { "Welcome to the events page!" }
        }
    }
}
