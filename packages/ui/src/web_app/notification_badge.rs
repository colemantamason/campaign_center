use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct NotificationBadgeProps {
    count: i32,
    class: Option<String>,
}

#[component]
pub fn NotificationBadge(props: NotificationBadgeProps) -> Element {
    let common_classes =
        "rounded-md text-xs px-1.5 py-0.5 font-semibold bg-primary text-primary-foreground";

    let combined_classes = format!(
        "{} {}",
        common_classes,
        if let Some(class) = props.class {
            class
        } else {
            "".to_string()
        }
    );

    rsx! {
        span { class: "{combined_classes}", {props.count.to_string()} }
    }
}
