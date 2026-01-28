use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum AvatarVariant {
    Square,
    Round,
}

pub type Source = String;
pub type Fallback = String;
pub type Class = String;

#[derive(Clone, PartialEq, Props)]
pub struct AvatarProps {
    src: Option<Source>,
    fallback: Fallback,
    variant: AvatarVariant,
    class: Option<Class>,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let common_classes = "bg-primary flex items-center justify-center text-primary-foreground font-medium text-sm overflow-hidden w-8 h-8";

    let variant_classes = match props.variant {
        AvatarVariant::Square => "rounded-md",
        AvatarVariant::Round => "rounded-full",
    };

    let combined_classes = format!(
        "{} {} {}",
        common_classes,
        variant_classes,
        if let Some(class) = props.class {
            class
        } else {
            "".to_string()
        }
    );

    rsx! {
        div { class: "{combined_classes}",
            if let Some(src) = props.src {
                img { src: "{src}", class: "w-full h-full object-cover" }
            } else {
                {props.fallback}
            }
        }
    }
}
