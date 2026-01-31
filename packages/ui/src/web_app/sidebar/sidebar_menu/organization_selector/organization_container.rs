use crate::shared::{
    button::{Button, ButtonSize, ButtonType, ButtonVariant},
    icon::{Icon, IconSize, IconVariant},
};
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
    name: Option<String>,
    avatar_url: Option<String>,
    member_count: Option<i32>,
    show_menu: Option<Signal<bool>>,
    organization_id: Option<i32>,
    selected_organization_membership_id: Option<Signal<Option<i32>>>,
    show_confirmation_modal: Option<Signal<bool>>,
}

#[component]
pub fn OrganizationContainer(props: OrganizationContainerProps) -> Element {
    match props.r#type {
        OrganizationContainerType::Selector => rsx! {
            Button {
                r#type: ButtonType::Button,
                onclick: move |_| {
                    if let Some(mut show_menu) = props.show_menu {
                        show_menu.toggle();
                    }
                },
                size: ButtonSize::Full,
                variant: ButtonVariant::Sidebar,
                class: "group",
                Avatar {
                    src: if let Some(selector_avatar_url) = props.selector_avatar_url { selector_avatar_url.cloned() } else { None },
                    fallback: if let Some(name) = props.selector_name { if let Some(character) = name.read().chars().next() {
                        character.to_string()
                    } else {
                        "?".to_string()
                    } } else { "?".to_string() },
                    variant: AvatarVariant::Square,
                }
                div { class: "flex flex-col flex-1 min-w-0 text-left gap-1",
                    span { class: "text-sm leading-none font-medium text-foreground group-hover:text-accent-foreground truncate",
                        if let Some(selector_name) = props.selector_name {
                            {selector_name.cloned()}
                        }
                    }
                    span { class: "text-sm leading-none text-muted-foreground group-hover:text-accent-foreground truncate",
                        if let Some(selector_member_count) = props.selector_member_count {
                            {format!("{} members", selector_member_count.read())}
                        }
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
                    fallback: if let Some(name) = &props.name { if let Some(character) = name.chars().next() {
                        character.to_string()
                    } else {
                        "?".to_string()
                    } } else { "?".to_string() },
                    variant: AvatarVariant::Square,
                }
                div { class: "flex flex-col flex-1 min-w-0 text-left gap-1",
                    span { class: "text-sm leading-none font-medium text-foreground truncate",
                        if let Some(name) = &props.name {
                            {name.clone()}
                        } else {
                            ""
                        }
                    }
                    span { class: "text-sm leading-none text-muted-foreground truncate",
                        if let Some(member_count) = props.member_count {
                            {format!("{} members", member_count)}
                        } else {
                            ""
                        }
                    }
                }
                Icon { size: IconSize::Medium, variant: IconVariant::Primary, Check {} }
            }
        },
        OrganizationContainerType::NonActive => rsx! {
            Button {
                r#type: ButtonType::Button,
                onclick: move |_| {
                    if let Some(mut show_menu) = props.show_menu {
                        show_menu.set(false);
                    }
                    if let Some(mut selected_organization_membership_id) = props
                        .selected_organization_membership_id
                    {
                        selected_organization_membership_id.set(props.organization_id);
                    }
                    if let Some(mut show_confirmation_modal) = props.show_confirmation_modal {
                        show_confirmation_modal.set(true);
                    }
                },
                size: ButtonSize::Full,
                variant: ButtonVariant::Sidebar,
                class: "group",
                Avatar {
                    src: props.avatar_url,
                    fallback: if let Some(name) = &props.name { if let Some(character) = name.chars().next() {
                        character.to_string()
                    } else {
                        "?".to_string()
                    } } else { "?".to_string() },
                    variant: AvatarVariant::Square,
                }
                div { class: "flex flex-col flex-1 min-w-0 text-left gap-1",
                    span { class: "text-sm leading-none font-medium text-foreground group-hover:text-accent-foreground truncate",
                        if let Some(name) = &props.name {
                            {name.clone()}
                        } else {
                            ""
                        }
                    }
                    span { class: "text-sm leading-none text-muted-foreground group-hover:text-accent-foreground truncate",
                        if let Some(member_count) = props.member_count {
                            {format!("{} members", member_count)}
                        } else {
                            ""
                        }
                    }
                }
            }
        },
    }
}
