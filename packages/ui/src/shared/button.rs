use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Button,
    Link,
    Submit,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Default,
    Full,
    Icon,
    Fit,
    FormDefault,
    FormFull,
}

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Destructive,
    Link,
    Sidebar,
    SidebarActive,
}

#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
    r#type: ButtonType,
    disabled: Option<Memo<bool>>,
    onclick: Option<EventHandler<MouseEvent>>,
    to: Option<String>,
    size: ButtonSize,
    variant: ButtonVariant,
    class: Option<String>,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let common_classes = "rounded-md disabled:pointer-events-none disabled:opacity-50";

    let size_classes = match props.size {
        ButtonSize::Default => {
            "inline-flex items-center justify-center px-4 py-2 text-sm font-medium"
        }
        ButtonSize::Full => "w-full flex items-center gap-3 p-2 text-sm font-medium",
        ButtonSize::Fit => "p-0 text-sm font-medium",
        ButtonSize::Icon => "inline-flex items-center justify-center p-0.5 text-sm font-medium",
        ButtonSize::FormDefault => "inline-flex items-center justify-center px-4 py-2 font-bold",
        ButtonSize::FormFull => "w-full flex items-center justify-center p-3 font-bold",
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
        common_classes,
        size_classes,
        variant_classes,
        if let Some(class) = props.class {
            class
        } else {
            "".to_string()
        }
    );

    match props.r#type {
        ButtonType::Button | ButtonType::Submit => {
            rsx!(
                button {
                    r#type: match props.r#type {
                        ButtonType::Button => "button",
                        ButtonType::Submit => "submit",
                        _ => "",
                    },
                    disabled: if let Some(disabled) = props.disabled { *disabled.read() } else { false },
                    onclick: move |event| {
                        if let Some(handler) = props.onclick {
                            handler.call(event);
                        }
                    },
                    class: "{combined_classes}",
                    {props.children}
                }
            )
        }
        ButtonType::Link => {
            rsx!(
                Link {
                    to: if let Some(to) = props.to { to } else { "#".to_string() },
                    class: "{combined_classes}",
                    {props.children}
                }
            )
        }
    }
}
