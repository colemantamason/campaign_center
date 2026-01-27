use crate::shared::button::{Button, ButtonSize, ButtonType, ButtonVariant};
use crate::shared::icon::{Icon, IconSize, IconVariant};
use crate::web_app::avatar::{Avatar, AvatarVariant};
use dioxus::prelude::*;
use lucide_dioxus::{Check, ChevronsUpDown};

#[derive(Clone, PartialEq)]
pub enum OrganizationContainerType {
    Selector,
    Active,
    NonActive,
}

#[derive(Clone, PartialEq, Props)]
pub struct OrganizationContainerProps {
    r#type: OrganizationContainerType,
    selector_avatar_url: Option<Store<Option<String>>>,
    selector_name: Option<Store<String>>,
    selector_member_count: Option<Store<i32>>,
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
    match props.r#type {
        OrganizationContainerType::Selector => rsx! {
            Button {
                r#type: ButtonType::Button,
                onclick: move |_| props.show_menu.unwrap().toggle(),
                size: ButtonSize::Full,
                variant: ButtonVariant::Sidebar,
                class: "group",
                Avatar {
                    src: props.selector_avatar_url.unwrap().cloned(),
                    fallback: props.selector_name.unwrap().read().chars().next().unwrap_or('?').to_string(),
                    variant: AvatarVariant::Square,
                }
                div { class: "flex flex-col flex-1 min-w-0 text-left gap-1",
                    span { class: "text-sm leading-none font-medium text-foreground group-hover:text-accent-foreground truncate",
                        {props.selector_name.unwrap().cloned()}
                    }
                    span { class: "text-sm leading-none text-muted-foreground group-hover:text-accent-foreground truncate",
                        {format!("{} members", props.selector_member_count.unwrap().cloned())}
                    }
                }
                Icon {
                    size: IconSize::Small,
                    variant: IconVariant::Muted,
                    class: "group-hover:text-accent-foreground",
                    ChevronsUpDown {}
                }
            }
        },
        OrganizationContainerType::Active => rsx! {
            div { class: "flex items-center gap-3 w-full px-2 py-2 rounded-md bg-sidebar-accent cursor-default",
                Avatar {
                    src: props.avatar_url,
                    fallback: props.name.as_ref().and_then(|name| name.chars().next()).unwrap_or('?').to_string(),
                    variant: AvatarVariant::Square,
                }
                div { class: "flex flex-col flex-1 min-w-0 text-left gap-1",
                    span { class: "text-sm leading-none font-medium text-foreground truncate",
                        {props.name.as_ref().cloned()}
                    }
                    span { class: "text-sm leading-none text-muted-foreground truncate",
                        {format!("{} members", props.member_count.unwrap())}
                    }
                }
                Icon { size: IconSize::Medium, variant: IconVariant::Primary, Check {} }
            }
        },
        OrganizationContainerType::NonActive => rsx! {
            Button {
                r#type: ButtonType::Button,
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
                    fallback: props.name.as_ref().and_then(|name| name.chars().next()).unwrap_or('?').to_string(),
                    variant: AvatarVariant::Square,
                }
                div { class: "flex flex-col flex-1 min-w-0 text-left gap-1",
                    span { class: "text-sm leading-none font-medium text-foreground group-hover:text-accent-foreground truncate",
                        {props.name.as_ref().cloned()}
                    }
                    span { class: "text-sm leading-none text-muted-foreground group-hover:text-accent-foreground truncate",
                        {format!("{} members", props.member_count.unwrap())}
                    }
                }
            }
        },
    }
}
