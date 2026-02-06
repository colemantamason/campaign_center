use crate::auth::{user_response_to_account, AuthContext};
use crate::gate::Gate;
use crate::routes::Routes;
use api::enums::OrganizationType;
use api::interfaces::CreateOrganizationRequest;
use api::providers::{create_organization, get_current_user};
use api::state::UserAccountStoreExt;
use dioxus::prelude::*;
use ui::web_app::UserAccountContext;

#[component]
pub fn CreateOrganization() -> Element {
    let mut name = use_signal(String::new);
    let mut organization_type = use_signal(|| OrganizationType::Campaign);
    let mut create_error = use_signal(|| None::<String>);
    let mut is_loading = use_signal(|| false);
    let auth_context = use_context::<AuthContext>();
    let user_account_context = use_context::<UserAccountContext>();

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        let mut auth_context_spawn = auth_context.clone();

        spawn(async move {
            is_loading.set(true);
            create_error.set(None);
            let request = CreateOrganizationRequest {
                name: name.read().clone(),
                slug: None,
                description: None,
                organization_type: *organization_type.read(),
            };

            match create_organization(request).await {
                Ok(_org) => {
                    // refresh the user account to include the new organization
                    match get_current_user().await {
                        Ok(Some(user)) => {
                            // update UserAccountContext BEFORE navigation so Gate sees the new org
                            let updated_account = user_response_to_account(&user);
                            user_account_context
                                .user_account
                                .id()
                                .set(updated_account.id);
                            user_account_context
                                .user_account
                                .first_name()
                                .set(updated_account.first_name);
                            user_account_context
                                .user_account
                                .last_name()
                                .set(updated_account.last_name);
                            user_account_context
                                .user_account
                                .avatar_url()
                                .set(updated_account.avatar_url);
                            user_account_context
                                .user_account
                                .active_organization_membership_id()
                                .set(updated_account.active_organization_membership_id);
                            user_account_context
                                .user_account
                                .organization_memberships()
                                .set(updated_account.organization_memberships);
                            user_account_context
                                .user_account
                                .notifications()
                                .set(updated_account.notifications);

                            // update auth context
                            auth_context_spawn.set_authenticated(user);
                            // redirect to dashboard
                            router().push(Routes::Dashboard {}.to_string());
                        }
                        Ok(None) => {
                            create_error.set(Some("Failed to fetch user account".to_string()));
                        }
                        Err(error) => {
                            create_error.set(Some(error.to_string()));
                        }
                    }
                }
                Err(error) => {
                    create_error.set(Some(error.to_string()));
                }
            }
            is_loading.set(false);
        });
    };

    rsx! {
        Gate {
            div { class: "flex min-h-screen items-center justify-center",
                div { class: "w-full max-w-md space-y-6 p-8",
                    div { class: "text-center",
                        h1 { class: "text-2xl font-bold text-primary", "Create Organization" }
                        p { class: "text-muted-foreground mt-2",
                            "Set up your campaign or organization to get started."
                        }
                    }

                    if let Some(error) = create_error.read().as_ref() {
                        div { class: "bg-destructive/10 text-destructive p-3 rounded-md text-sm",
                            "{error}"
                        }
                    }

                    form { class: "space-y-4", onsubmit: handle_submit,
                        div {
                            label {
                                class: "block text-sm font-medium mb-1",
                                r#for: "org-type",
                                "Type"
                            }
                            select {
                                id: "org-type",
                                class: "w-full px-3 py-2 border border-input rounded-md bg-background",
                                onchange: move |evt| {
                                    if let Some(org_type) = OrganizationType::from_str(&evt.value()) {
                                        organization_type.set(org_type);
                                    }
                                },
                                option {
                                    value: "campaign",
                                    selected: *organization_type.read() == OrganizationType::Campaign,
                                    "Campaign"
                                }
                                option {
                                    value: "organization",
                                    selected: *organization_type.read() == OrganizationType::Organization,
                                    "Organization"
                                }
                            }
                        }

                        div {
                            label {
                                class: "block text-sm font-medium mb-1",
                                r#for: "name",
                                "Name"
                            }
                            input {
                                id: "name",
                                r#type: "text",
                                required: true,
                                class: "w-full px-3 py-2 border border-input rounded-md bg-background",
                                placeholder: "My Campaign",
                                value: "{name}",
                                oninput: move |evt| name.set(evt.value()),
                            }
                        }

                        button {
                            r#type: "submit",
                            disabled: *is_loading.read(),
                            class: "w-full py-2 px-4 bg-primary text-primary-foreground rounded-md font-medium hover:bg-primary/90 disabled:opacity-50",
                            if *is_loading.read() {
                                "Creating organization..."
                            } else {
                                "Create Organization"
                            }
                        }
                    }
                }
            }
        }
    }
}
