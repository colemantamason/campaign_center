use dioxus::prelude::*;

pub type Label = String;

#[derive(Clone, PartialEq, Props)]
pub struct NavLabelProps {
    label: Label,
}

#[component]
pub fn NavLabel(props: NavLabelProps) -> Element {
    rsx! {
        span { class: "pt-2 pb-1 text-sm text-muted-foreground cursor-default", {props.label} }
    }
}
