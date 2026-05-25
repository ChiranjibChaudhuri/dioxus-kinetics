use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_blocks::{SocialOverlay, SocialPlatform};
use ui_composition::ClipFill;

#[component]
pub fn SocialOverlayDemoScene() -> Element {
    rsx! {
        Scene {
            id: "social-overlay-demo",
            width: 1280,
            height: 720,
            duration_ms: 3_000.0,
            autoplay: Some(true),
            controls: Some(true),
            Clip { start_ms: 200.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                SocialOverlay {
                    platform: SocialPlatform::Instagram,
                    handle: "@kineticsui".to_string(),
                    message: "Just followed you!".to_string(),
                }
            }
        }
    }
}
