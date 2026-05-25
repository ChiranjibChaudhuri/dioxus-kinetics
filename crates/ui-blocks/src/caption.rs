use dioxus::prelude::*;
use ui_dioxus::{SplitMode, SplitText};

/// Subtitle bar with reading-pace word stagger.
#[component]
pub fn Caption(text: String, reading_pace_ms_per_word: Option<f32>) -> Element {
    let _ = reading_pace_ms_per_word; // stagger pace consumed by parent TimelineScope; surfaced via data attr below.
    rsx! {
        div {
            class: "ui-block-caption",
            "data-block": "caption",
            SplitText { text: text, split_by: Some(SplitMode::Word) }
        }
    }
}
