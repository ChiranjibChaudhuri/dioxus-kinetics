use dioxus::prelude::*;
use ui_dioxus::{KineticText, Scene, TimelineScope};
use ui_runtime::reduced_motion::ReducedMotionProvider;

#[test]
fn timeline_scope_inside_scene_staggers_children_by_step_ms() {
    // Reduced motion settles Scene's elapsed_ms to duration_ms = 2000.
    // step_ms defaults to 80. Per-child elapsed:
    //   index 0 → max(0, 2000 - 0  * 80) = 2000
    //   index 1 → max(0, 2000 - 1  * 80) = 1920
    //   index 2 → max(0, 2000 - 2  * 80) = 1840
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 2_000.0,
                autoplay: Some(false),
                TimelineScope { id: "stagger".to_string(), autoplay: true,
                    KineticText { id: "a".to_string(), text: "A".to_string(), cue: "fade-in".to_string() }
                    KineticText { id: "b".to_string(), text: "B".to_string(), cue: "fade-in".to_string() }
                    KineticText { id: "c".to_string(), text: "C".to_string(), cue: "fade-in".to_string() }
                }
            }
        }
    });
    assert!(html.contains("animation-delay: -2000ms"), "want index 0: {html}");
    assert!(html.contains("animation-delay: -1920ms"), "want index 1: {html}");
    assert!(html.contains("animation-delay: -1840ms"), "want index 2: {html}");
}

#[test]
fn timeline_scope_emits_section_marker_attributes() {
    let html = dioxus_ssr::render_element(rsx! {
        TimelineScope { id: "marker-test".to_string(), autoplay: true,
            span { "stub" }
        }
    });
    assert!(html.contains("data-timeline-id=\"marker-test\""), "{html}");
    assert!(html.contains("data-autoplay=\"true\""), "{html}");
    assert!(html.contains("ui-timeline-scope"), "{html}");
}

#[test]
fn timeline_scope_custom_stagger_step_ms() {
    // step_ms = 200 → index 1 elapsed = 2000 - 200 = 1800.
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 2_000.0,
                autoplay: Some(false),
                TimelineScope { id: "stagger".to_string(), autoplay: true, stagger_step_ms: 200.0,
                    KineticText { id: "a".to_string(), text: "A".to_string(), cue: "fade-in".to_string() }
                    KineticText { id: "b".to_string(), text: "B".to_string(), cue: "fade-in".to_string() }
                }
            }
        }
    });
    assert!(html.contains("animation-delay: -2000ms"), "{html}");
    assert!(html.contains("animation-delay: -1800ms"), "{html}");
}
