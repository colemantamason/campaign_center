use crate::auth::AuthContext;
use crate::gate::Gate;
use crate::routes::Routes;
use api::enums::Platform;
use api::interfaces::RegisterRequest;
use api::providers::{get_current_user, register};
use dioxus::prelude::*;

#[component]
pub fn CreateAccount() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut first_name = use_signal(String::new);
    let mut last_name = use_signal(String::new);
    let mut create_error = use_signal(|| None::<String>);
    let mut is_loading = use_signal(|| false);
    let auth_context = use_context::<AuthContext>();

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        let mut auth_context_spawn = auth_context.clone();

        spawn(async move {
            is_loading.set(true);
            create_error.set(None);
            let request = RegisterRequest {
                email: email.read().clone(),
                password: password.read().clone(),
                first_name: first_name.read().clone(),
                last_name: last_name.read().clone(),
                platform: Platform::Web,
            };

            match register(request).await {
                Ok(_response) => {
                    // fetch the full user account after registration
                    match get_current_user().await {
                        Ok(Some(user)) => {
                            auth_context_spawn.set_authenticated(user);
                            // redirect to create organization (new users have no orgs)
                            router().push(Routes::CreateOrganization {}.to_string());
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
                        h1 { class: "text-2xl font-bold text-primary", "Create Account" }
                        p { class: "text-muted-foreground mt-2",
                            "Already have an account? "
                            a {
                                class: "text-primary underline",
                                href: Routes::Login {}.to_string(),
                                "Login"
                            }
                        }
                    }

                    if let Some(error) = create_error.read().as_ref() {
                        div { class: "bg-destructive/10 text-destructive p-3 rounded-md text-sm",
                            "{error}"
                        }
                    }

                    form { class: "space-y-4", onsubmit: handle_submit,
                        div { class: "grid grid-cols-2 gap-4",
                            div {
                                label {
                                    class: "block text-sm font-medium mb-1",
                                    r#for: "first_name",
                                    "First Name"
                                }
                                input {
                                    id: "first_name",
                                    r#type: "text",
                                    required: true,
                                    class: "w-full px-3 py-2 border border-input rounded-md bg-background",
                                    placeholder: "John",
                                    value: "{first_name}",
                                    oninput: move |evt| first_name.set(evt.value()),
                                }
                            }
                            div {
                                label {
                                    class: "block text-sm font-medium mb-1",
                                    r#for: "last_name",
                                    "Last Name"
                                }
                                input {
                                    id: "last_name",
                                    r#type: "text",
                                    required: true,
                                    class: "w-full px-3 py-2 border border-input rounded-md bg-background",
                                    placeholder: "Doe",
                                    value: "{last_name}",
                                    oninput: move |evt| last_name.set(evt.value()),
                                }
                            }
                        }

                        div {
                            label {
                                class: "block text-sm font-medium mb-1",
                                r#for: "email",
                                "Email"
                            }
                            input {
                                id: "email",
                                r#type: "email",
                                required: true,
                                class: "w-full px-3 py-2 border border-input rounded-md bg-background",
                                placeholder: "you@example.com",
                                value: "{email}",
                                oninput: move |evt| email.set(evt.value()),
                            }
                        }

                        div {
                            label {
                                class: "block text-sm font-medium mb-1",
                                r#for: "password",
                                "Password"
                            }
                            input {
                                id: "password",
                                r#type: "password",
                                required: true,
                                class: "w-full px-3 py-2 border border-input rounded-md bg-background",
                                placeholder: "At least 8 characters",
                                value: "{password}",
                                oninput: move |evt| password.set(evt.value()),
                            }
                            p { class: "text-xs text-muted-foreground mt-1",
                                "Must be at least 8 characters with a letter and number"
                            }
                        }

                        button {
                            r#type: "submit",
                            disabled: *is_loading.read(),
                            class: "w-full py-2 px-4 bg-primary text-primary-foreground rounded-md font-medium hover:bg-primary/90 disabled:opacity-50",
                            if *is_loading.read() {
                                "Creating account..."
                            } else {
                                "Create Account"
                            }
                        }
                    }
                }
            }
        }
    }
}
