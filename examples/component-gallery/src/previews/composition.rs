use crate::demo_frame::{ScrubElapsedMs, ScrubFps, ScrubFrame};
use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn frame_stage_preview() -> Element {
    rsx! {
        ScrubFrame {
            duration_ms: 6000.0,
            fps: Some(30),
            label: "FrameStage",
            children: rsx! { FrameStageBody {} },
        }
    }
}

#[component]
fn FrameStageBody() -> Element {
    let elapsed = use_context::<ScrubElapsedMs>().0;
    let fps = use_context::<ScrubFps>().0;
    let frame = ((*elapsed.read() / 1000.0) * fps as f32).round() as u32;
    rsx! {
        FrameStage {
            composition: Composition::new("launch-demo", 1920, 1080, 30, 180),
            frame,
            FrameClip { start: 0, duration: 60,
                FrameLayer { id: "title", depth: 10,
                    h4 { "Dioxus Kinetics" }
                    p { "Frame {frame} / 180" }
                }
            }
        }
    }
}
