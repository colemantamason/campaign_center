use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum AvatarVariant {
    Square,
    Round,
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    src: Option<String>,
    fallback: String,
    variant: AvatarVariant,
    #[props(default = "".to_string(), into)]
    class: String,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let common_classes = "bg-primary flex items-center justify-center text-primary-foreground font-medium text-sm overflow-hidden w-8 h-8";

    let variant_classes = match props.variant {
        AvatarVariant::Square => "rounded-md",
        AvatarVariant::Round => "rounded-full",
    };

    let combined_classes = format!("{} {} {}", common_classes, variant_classes, props.class);

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
