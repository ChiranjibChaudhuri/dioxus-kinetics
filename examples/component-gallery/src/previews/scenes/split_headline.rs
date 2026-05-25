use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn SplitHeadlineScene() -> Element {
    rsx! {
        Scene {
            id: "split-headline",
            width: 1280,
            height: 360,
            duration_ms: 2_500.0,
            autoplay: Some(true),
            controls: Some(true),
            TimelineScope { id: "split-headline-timeline", autoplay: true,
                SplitText {
                    text: "Kinetics typography, glyph by glyph.".to_string(),
                    split_by: Some(SplitMode::Character),
                }
            }
        }
    }
}
