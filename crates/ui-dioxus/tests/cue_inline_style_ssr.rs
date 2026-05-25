use dioxus::prelude::*;
use ui_dioxus::{KineticText, Scene};
use ui_runtime::reduced_motion::ReducedMotionProvider;

#[test]
fn kinetic_text_outside_any_clock_renders_static_markup() {
    // No Scene / Sequence / StaggerOffset → no inline animation style.
    let html = dioxus_ssr::render_element(rsx! {
        KineticText { id: "x".to_string(), text: "hi".to_string(), cue: "fade-in".to_string() }
    });
    assert!(html.contains("data-motion-cue=\"fade-in\""), "{html}");
    assert!(!html.contains("animation-name"), "{html}");
}

#[test]
fn kinetic_text_inside_scene_emits_cue_animation_style() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(false),
            Scene {
                id: "test", width: 100, height: 100, duration_ms: 5_000.0,
                autoplay: Some(false),
                KineticText { id: "title".to_string(), text: "hi".to_string(), cue: "rise-in".to_string() }
            }
        }
    });
    // At elapsed_ms = 0, animation-delay should be -0ms.
    assert!(html.contains("animation-name: ui-cue-rise-in"), "{html}");
    assert!(html.contains("animation-delay: -0ms"), "{html}");
    assert!(html.contains("animation-duration: 720ms"), "{html}");
    assert!(html.contains("animation-play-state: paused"), "{html}");
}

#[test]
fn kinetic_text_inside_reduced_motion_scene_renders_at_settled_endpoint() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "test", width: 100, height: 100, duration_ms: 5_000.0,
                autoplay: Some(false),
                KineticText { id: "title".to_string(), text: "hi".to_string(), cue: "fade-in".to_string() }
            }
        }
    });
    // Reduced motion settles at duration_ms (= 5000), so animation-delay
    // is -5000ms which freezes the keyframe at its end state.
    assert!(html.contains("animation-name: ui-cue-fade-in"), "{html}");
    assert!(html.contains("animation-delay: -5000ms"), "{html}");
}
