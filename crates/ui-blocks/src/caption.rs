use dioxus::prelude::*;
use ui_dioxus::{SplitMode, SplitText};

/// Subtitle bar with reading-pace word stagger.
///
/// `reading_pace_ms_per_word` is currently advisory only — SP-3 ships
/// `SplitText` with a single workspace-wide stagger pace controlled by
/// the surrounding `TimelineScope`. A future SP will plumb per-Caption
/// pace through to `SplitText`. The value is preserved on the component
/// for forward compatibility.
#[component]
pub fn Caption(text: String, reading_pace_ms_per_word: Option<f32>) -> Element {
    let _ = reading_pace_ms_per_word; // SP-3 limitation: stagger pace consumed by parent TimelineScope. See rustdoc.
    rsx! {
        div {
            class: "ui-block-caption",
            "data-block": "caption",
            SplitText { text: text, split_by: Some(SplitMode::Word) }
        }
    }
}
