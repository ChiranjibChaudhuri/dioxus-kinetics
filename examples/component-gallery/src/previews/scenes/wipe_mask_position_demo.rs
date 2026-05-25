use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_blocks::{WipeTransition, WipeVariant};

#[component]
pub fn WipeMaskPositionDemoScene() -> Element {
    rsx! {
        Scene {
            id: "wipe-mask-position-demo",
            width: 1280,
            height: 720,
            duration_ms: 2_500.0,
            autoplay: Some(true),
            controls: Some(true),
            WipeTransition {
                duration_ms: 2_500.0,
                variant: WipeVariant::MaskPosition,
                div {
                    class: "scene-wipe-fill",
                    style: "background: linear-gradient(180deg, #fad84b, #fa6e4b); width: 100%; height: 100%;",
                    h2 { style: "padding: 80px;", "Mask-position wipes sweep horizontally." }
                }
            }
        }
    }
}
