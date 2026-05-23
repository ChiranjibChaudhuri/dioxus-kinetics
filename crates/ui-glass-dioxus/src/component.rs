use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LiquidSurfaceProps {
    #[props(default = 320)]
    pub width: u32,
    #[props(default = 200)]
    pub height: u32,
}

#[component]
pub fn LiquidSurface(props: LiquidSurfaceProps) -> Element {
    let style = format!("width: {}px; height: {}px; display: inline-block;", props.width, props.height);
    rsx! {
        div {
            class: "ui-liquid-surface",
            style: "{style}",
        }
    }
}
