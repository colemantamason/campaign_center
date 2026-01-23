use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NotificationBadgeProps {
    count: i32,
    #[props(default = "".to_string(), into)]
    class: String,
}

#[component]
pub fn NotificationBadge(props: NotificationBadgeProps) -> Element {
    let common_classes =
        "rounded-md text-xs px-1.5 py-0.5 font-semibold bg-primary text-primary-foreground";

    let combined_classes = format!("{} {}", common_classes, props.class);

    rsx! {
        span { class: "{combined_classes}", "{props.count}" }
    }
}
