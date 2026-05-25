use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_blocks::{WipeTransition, WipeVariant};

#[component]
pub fn WipeIrisDemoScene() -> Element {
    rsx! {
        Scene {
            id: "wipe-iris-demo",
            width: 1280,
            height: 720,
            duration_ms: 2_500.0,
            autoplay: Some(true),
            controls: Some(true),
            WipeTransition {
                duration_ms: 2_500.0,
                variant: WipeVariant::Iris,
                div {
                    class: "scene-wipe-fill",
                    style: "background: linear-gradient(45deg, #4bfaaa, #fa4b9c); width: 100%; height: 100%;",
                    h2 { style: "padding: 80px;", "Iris wipes expand from the centre." }
                }
            }
        }
    }
}
