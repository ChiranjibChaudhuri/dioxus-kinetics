use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_composition::ClipFill;

#[component]
pub fn ManualDriverDemoScene() -> Element {
    rsx! {
        Scene {
            id: "manual-driver-demo",
            width: 1280,
            height: 480,
            duration_ms: 5_000.0,
            autoplay: Some(false),
            controls: Some(true),
            driver: Some(SceneDriver::Manual),
            div { style: "padding: 24px; text-align: center;",
                Clip { start_ms: 0.0, duration_ms: 5_000.0, fill: ClipFill::HoldBoth,
                    KineticText {
                        id: "manual-driver-headline",
                        text: "Drag the scrubber. No autoplay.".to_string(),
                        cue: "fade-in",
                    }
                }
                Clip { start_ms: 1_500.0, duration_ms: 3_500.0, fill: ClipFill::HoldEnd,
                    KineticText {
                        id: "manual-driver-body",
                        text: "SceneDriver::Manual disables the rAF loop entirely.".to_string(),
                        cue: "rise-in",
                    }
                }
            }
        }
    }
}
