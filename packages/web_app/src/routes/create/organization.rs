use crate::auth::AuthContext;
use crate::gate::Gate;
use crate::routes::Routes;
use api::enums::OrganizationType;
use api::interfaces::CreateOrganizationRequest;
use api::providers::{create_organization, get_current_user};
use dioxus::prelude::*;

#[component]
pub fn CreateOrganization() -> Element {
    let mut name = use_signal(String::new);
    let mut organization_type = use_signal(|| OrganizationType::Campaign);
    let mut create_error = use_signal(|| None::<String>);
    let mut is_loading = use_signal(|| false);
    let auth_context = use_context::<AuthContext>();

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
                                option { value: "campaign", selected: *organization_type.read() == OrganizationType::Campaign, "Campaign" }
                                option { value: "organization", selected: *organization_type.read() == OrganizationType::Organization, "Organization" }
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
