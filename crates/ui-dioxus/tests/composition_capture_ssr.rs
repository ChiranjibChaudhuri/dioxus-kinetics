use dioxus::prelude::*;
use ui_composition::Composition;
use ui_dioxus::{CaptureStage, FrameClip, FrameLayer, FrameStage};

#[test]
fn frame_stage_clip_and_layer_render_deterministic_frame_attributes() {
    let composition = Composition::new("launch-demo", 1920, 1080, 30, 180);
    let html = dioxus_ssr::render_element(rsx! {
        FrameStage { composition, frame: 42,
            FrameClip { start: 0, duration: 60,
                FrameLayer { id: "title", depth: 10,
                    "Dioxus Kinetics"
                }
            }
        }
    });

    assert!(html.contains("ui-frame-stage"));
    assert!(html.contains("data-composition-id=\"launch-demo\""));
    assert!(html.contains("data-frame=\"42\""));
    assert!(html.contains("ui-frame-layer"));
    assert!(html.contains("data-depth=\"10\""));
}

#[test]
fn capture_stage_renders_viewport_and_frame_metadata() {
    let html = dioxus_ssr::render_element(rsx! {
        CaptureStage { id: "component-showcase", viewport: "desktop", frame: 72,
            "Preview"
        }
    });

    assert!(html.contains("ui-capture-stage"));
    assert!(html.contains("data-capture-id=\"component-showcase\""));
    assert!(html.contains("data-viewport=\"desktop\""));
    assert!(html.contains("data-frame=\"72\""));
    assert!(html.contains("Preview"));
    assert!(!html.contains("data-composition-id"));
}
