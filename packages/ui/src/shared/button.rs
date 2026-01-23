use crate::shared::form::FormStatus;
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum ButtonType {
    Button,
    Submit,
}

#[derive(PartialEq, Clone)]
pub enum ButtonSize {
    Default,
    Full,
    Fit,
    Icon,
}

#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Destructive,
    Link,
    Sidebar,
    SidebarActive,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default = ButtonType::Button)]
    button_type: ButtonType,
    size: ButtonSize,
    variant: ButtonVariant,
    #[props(default = "".to_string(), into)]
    class: String,
    disabled: Option<Signal<FormStatus>>,
    onclick: Option<EventHandler<MouseEvent>>,
    to: Option<String>,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let common_classes =
        "rounded-md text-sm font-medium disabled:pointer-events-none disabled:opacity-50";

    let size_classes = match props.size {
        ButtonSize::Default => "inline-flex items-center justify-center px-4 py-2",
        ButtonSize::Full => "w-full flex items-center gap-3 p-2",
        ButtonSize::Fit => "p-0",
        ButtonSize::Icon => "inline-flex items-center justify-center p-0.5",
    };

    let variant_classes = match props.variant {
        ButtonVariant::Primary => "bg-primary text-primary-foreground hover:bg-primary/90",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Outline => {
            "bg-background hover:bg-accent text-foreground hover:text-accent-foreground border border-border"
        }
        ButtonVariant::Destructive => "bg-background hover:bg-destructive/10 text-destructive border border-destructive",
        ButtonVariant::Sidebar => "hover:bg-accent text-foreground hover:text-accent-foreground",
        ButtonVariant::SidebarActive => "bg-primary text-primary-foreground",
        ButtonVariant::Link => "text-foreground no-underline hover:underline",
        };

    let combined_classes = format!(
        "{} {} {} {}",
        common_classes, size_classes, variant_classes, props.class
    );

    rsx! {
        if let Some(to_path) = props.to {
            Link { to: to_path, class: "{combined_classes}", {props.children} }
        } else {
            button {
                r#type: match props.button_type {
                    ButtonType::Button => "button",
                    ButtonType::Submit => "submit",
                },
                class: "{combined_classes}",
                disabled: if let Some(status_signal) = &props.disabled { status_signal() == FormStatus::Submitting } else { false },
                onclick: move |evt| {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                },
                {props.children}
            }
        }
    }
}
