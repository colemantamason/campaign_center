use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NavLabelProps {
    label: String,
}

#[component]
pub fn NavLabel(props: NavLabelProps) -> Element {
    rsx! {
        span { class: "pt-2 pb-1 text-sm text-muted-foreground cursor-default", "{props.label}" }
    }
}
