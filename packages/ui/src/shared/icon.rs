use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum IconSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, PartialEq)]
pub enum IconVariant {
    Primary,
    Muted,
    Sidebar,
    SidebarActive,
    Button,
}

#[derive(Clone, PartialEq, Props)]
pub struct IconProps {
    size: IconSize,
    variant: IconVariant,
    class: Option<String>,
    children: Element,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let common_classes = "inline-flex items-center justify-center [&>svg]:w-full [&>svg]:h-full";

    let size_classes = match props.size {
        IconSize::Small => "w-4 h-4",
        IconSize::Medium => "w-5 h-5",
        IconSize::Large => "w-7 h-7",
    };

    let variant_classes = match props.variant {
        IconVariant::Primary => "text-primary",
        IconVariant::Muted => "text-muted-foreground",
        IconVariant::Sidebar => "",
        IconVariant::SidebarActive => "text-primary-foreground",
        IconVariant::Button => "",
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

    rsx! {
        span { class: "{combined_classes}", {props.children} }
    }
}
