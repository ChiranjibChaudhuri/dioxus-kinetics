use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_blocks::Caption;

#[component]
pub fn CaptionDemoScene() -> Element {
    rsx! {
        Scene {
            id: "caption-demo",
            width: 1280,
            height: 360,
            duration_ms: 3_500.0,
            autoplay: Some(true),
            controls: Some(true),
            TimelineScope { id: "caption-timeline", autoplay: true,
                Caption {
                    text: "Built with kinetics ui-blocks.".to_string(),
                    reading_pace_ms_per_word: Some(320.0),
                }
            }
        }
    }
}
