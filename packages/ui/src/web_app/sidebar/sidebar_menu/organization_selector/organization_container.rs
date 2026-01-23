use crate::shared::button::{Button, ButtonSize, ButtonVariant};
use crate::shared::icon::{Icon, IconSize, IconVariant};
use crate::web_app::avatar::{Avatar, AvatarVariant};
use dioxus::prelude::*;
use lucide_dioxus::{Check, ChevronsUpDown};

#[derive(Props, Clone, PartialEq)]
pub struct OrganizationContainerProps {
    #[props(default = false)]
    is_selector_container: bool,
    selector_avatar_url: Option<Store<Option<String>>>,
    selector_name: Option<Store<String>>,
    selector_member_count: Option<Store<i32>>,
    #[props(default = false)]
    is_active_container: bool,
    #[props(default = false)]
    is_non_active_container: bool,
    organization_id: Option<i32>,
    avatar_url: Option<String>,
    name: Option<String>,
    member_count: Option<i32>,
    show_menu: Option<Signal<bool>>,
    pending_organization_id: Option<Signal<Option<i32>>>,
    show_confirmation_modal: Option<Signal<bool>>,
}

#[component]
pub fn OrganizationContainer(props: OrganizationContainerProps) -> Element {
    rsx! {
        if props.is_selector_container {
            Button {
                onclick: move |_| props.show_menu.unwrap().toggle(),
                size: ButtonSize::Full,
                variant: ButtonVariant::Sidebar,
                class: "group",
                Avatar {
                    src: props.selector_avatar_url.unwrap().read().clone(),
                    fallback: props.selector_name.unwrap().read().chars().next().unwrap_or('?').to_string(),
                    variant: AvatarVariant::Square,
                }
                div { class: "flex flex-col flex-1 min-w-0 text-left gap-1",
                    span { class: "text-sm leading-none font-medium text-foreground group-hover:text-accent-foreground truncate",
                        "{props.selector_name.unwrap().read().clone()}"
                    }
                    span { class: "text-sm leading-none text-muted-foreground group-hover:text-accent-foreground truncate",
                        "{props.selector_member_count.unwrap().read().clone()} members"
                    }
                }
                Icon {
                    size: IconSize::Small,
                    variant: IconVariant::Muted,
                    class: "group-hover:text-accent-foreground",
                    ChevronsUpDown {}
                }
            }
        } else if props.is_active_container {
            div { class: "flex items-center gap-3 w-full px-2 py-2 rounded-md bg-sidebar-accent cursor-default",
                Avatar {
                    src: props.avatar_url,
                    fallback: props.name.as_ref().and_then(|s| s.chars().next()).unwrap_or('?').to_string(),
                    variant: AvatarVariant::Square,
                }
                div { class: "flex flex-col flex-1 min-w-0 text-left gap-1",
                    span { class: "text-sm leading-none font-medium text-foreground truncate",
                        "{props.name.as_ref().unwrap()}"
                    }
                    span { class: "text-sm leading-none text-muted-foreground truncate",
                        "{props.member_count.unwrap()} members"
                    }
                }
                Icon { size: IconSize::Medium, variant: IconVariant::Primary, Check {} }
            }
        } else if props.is_non_active_container {
            Button {
                onclick: move |_| {
                    props.show_menu.unwrap().set(false);
                    props.pending_organization_id.unwrap().set(props.organization_id);
                    props.show_confirmation_modal.unwrap().set(true);
                },
                size: ButtonSize::Full,
                variant: ButtonVariant::Sidebar,
                class: "group",
                Avatar {
                    src: props.avatar_url,
                    fallback: props.name.as_ref().and_then(|s| s.chars().next()).unwrap_or('?').to_string(),
                    variant: AvatarVariant::Square,
                }
                div { class: "flex flex-col flex-1 min-w-0 text-left gap-1",
                    span { class: "text-sm leading-none font-medium text-foreground group-hover:text-accent-foreground truncate",
                        "{props.name.as_ref().unwrap()}"
                    }
                    span { class: "text-sm leading-none text-muted-foreground group-hover:text-accent-foreground truncate",
                        "{props.member_count.unwrap()} members"
                    }
                }
            }
        }
    }
}
