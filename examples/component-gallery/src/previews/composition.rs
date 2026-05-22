use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn frame_stage_preview() -> Element {
    let frames: [u32; 3] = [0, 90, 179];

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--3col",
            for frame in frames {
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "Frame {frame} / 180" }
                    FrameStage {
                        composition: Composition::new("launch-demo", 1920, 1080, 30, 180),
                        frame: frame,
                        FrameClip { start: 0, duration: 60,
                            FrameLayer { id: "title", depth: 10,
                                h4 { "Dioxus Kinetics" }
                                p { "Frame {frame} / 180" }
                            }
                        }
                    }
                }
            }
        }
    }
}
