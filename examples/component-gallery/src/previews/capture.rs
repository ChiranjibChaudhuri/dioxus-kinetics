use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn capture_stage_preview() -> Element {
    let profiles: [(&str, &str, u32); 3] = [
        ("mobile", "Mobile · 360 × 640", 24),
        ("tablet", "Tablet · 768 × 1024", 48),
        ("desktop", "Desktop · 1440 × 900", 72),
    ];

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--3col",
            for (viewport, caption, frame) in profiles {
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "{caption}" }
                    CaptureStage {
                        id: "capture-{viewport}",
                        viewport: viewport.to_string(),
                        frame: frame,
                        p { "Frame {frame}" }
                    }
                }
            }
        }
    }
}
