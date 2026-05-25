use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_blocks::{LowerThird, LowerThirdAccent};
use ui_composition::ClipFill;

#[component]
pub fn LowerThirdDemoScene() -> Element {
    rsx! {
        Scene {
            id: "lower-third-demo",
            width: 1280,
            height: 720,
            duration_ms: 4_000.0,
            autoplay: Some(true),
            controls: Some(true),
            div { class: "scene-lower-third-backdrop",
                style: "position: relative; width: 100%; height: 100%;",
                Clip { start_ms: 500.0, duration_ms: 3_000.0, fill: ClipFill::HoldEnd,
                    LowerThird {
                        name: "Ada Lovelace".to_string(),
                        role: "Mathematician".to_string(),
                        accent: Some(LowerThirdAccent::Primary),
                    }
                }
            }
        }
    }
}
