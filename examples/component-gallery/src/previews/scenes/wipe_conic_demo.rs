use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_blocks::{WipeTransition, WipeVariant};

#[component]
pub fn WipeConicDemoScene() -> Element {
    rsx! {
        Scene {
            id: "wipe-conic-demo",
            width: 1280,
            height: 720,
            duration_ms: 2_500.0,
            autoplay: Some(true),
            controls: Some(true),
            WipeTransition {
                duration_ms: 2_500.0,
                variant: WipeVariant::Conic,
                div {
                    class: "scene-wipe-fill",
                    style: "background: radial-gradient(circle, #ff7ae0, #4bbafa); width: 100%; height: 100%;",
                    h2 { style: "padding: 80px;", "Conic wipes spin around the centre." }
                }
            }
        }
    }
}
