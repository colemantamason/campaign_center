use crate::auth::AuthContext;
use crate::gate::Gate;
use crate::routes::Routes;
use api::enums::Platform;
use api::interfaces::LoginRequest;
use api::providers::{get_current_user, login};
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut login_error = use_signal(|| None::<String>);
    let mut is_loading = use_signal(|| false);
    let auth_context = use_context::<AuthContext>();

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        let mut auth_context_spawn = auth_context.clone();

        spawn(async move {
            is_loading.set(true);
            login_error.set(None);

            let request = LoginRequest {
                email: email.read().clone(),
                password: password.read().clone(),
                platform: Platform::Web,
            };

            match login(request).await {
                Ok(_response) => {
                    // fetch the full user account after login
                    match get_current_user().await {
                        Ok(Some(user)) => {
                            auth_context_spawn.set_authenticated(user);
                            // navigation will happen through Gate
                            router().push(Routes::Dashboard {}.to_string());
                        }
                        Ok(None) => {
                            login_error.set(Some("Failed to fetch user account".to_string()));
                        }
                        Err(error) => {
                            login_error.set(Some(error.to_string()));
                        }
                    }
                }
                Err(error) => {
                    login_error.set(Some(error.to_string()));
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
                        h1 { class: "text-2xl font-bold text-primary", "Login" }
                        p { class: "text-muted-foreground mt-2",
                            "Don't have an account? "
                            a {
                                class: "text-primary underline",
                                href: Routes::CreateAccount {}.to_string(),
                                "Create one"
                            }
                        }
                    }

                    if let Some(error) = login_error.read().as_ref() {
                        div { class: "bg-destructive/10 text-destructive p-3 rounded-md text-sm",
                            "{error}"
                        }
                    }

                    form { class: "space-y-4", onsubmit: handle_submit,
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
                                placeholder: "Your password",
                                value: "{password}",
                                oninput: move |evt| password.set(evt.value()),
                            }
                        }

                        button {
                            r#type: "submit",
                            disabled: *is_loading.read(),
                            class: "w-full py-2 px-4 bg-primary text-primary-foreground rounded-md font-medium hover:bg-primary/90 disabled:opacity-50",
                            if *is_loading.read() {
                                "Logging in..."
                            } else {
                                "Login"
                            }
                        }
                    }
                }
            }
        }
    }
}
