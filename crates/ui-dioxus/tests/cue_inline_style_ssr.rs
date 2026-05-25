use dioxus::prelude::*;
use ui_dioxus::{KineticBox, KineticText, Scene};
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

#[test]
fn kinetic_box_inside_scene_emits_cue_animation_style() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(false),
            Scene {
                id: "test", width: 100, height: 100, duration_ms: 5_000.0,
                autoplay: Some(false),
                KineticBox { id: "block".to_string(), cue: "pop-in".to_string(),
                    p { "child" }
                }
            }
        }
    });
    assert!(html.contains("animation-name: ui-cue-pop-in"), "{html}");
    assert!(html.contains("animation-delay: -0ms"), "{html}");
}

#[test]
fn kinetic_box_outside_clock_renders_static_markup() {
    let html = dioxus_ssr::render_element(rsx! {
        KineticBox { id: "block".to_string(), cue: "fade-in".to_string(),
            p { "child" }
        }
    });
    assert!(!html.contains("animation-name"), "{html}");
}

#[test]
fn sequence_inside_scene_uses_scene_clock_when_no_explicit_clock() {
    use ui_dioxus::Sequence;
    use ui_motion::{Ease, Transition};
    use ui_timeline::{MotionCue, MotionSegment, MotionTarget, Timeline, TimelineTrack};

    let timeline = Timeline::new("test", 1_000.0).with_track(TimelineTrack::new(
        MotionTarget::node("title"),
        vec![MotionSegment::new(
            0.0,
            1_000.0,
            MotionCue::Opacity {
                from: 0.0,
                to: 1.0,
                transition: Transition::Tween {
                    duration_ms: 1_000,
                    ease: Ease::Linear,
                },
            },
        )],
    ));
    // Scene with reduced motion forces elapsed_ms = duration_ms = 2000.
    // Inner Sequence (no explicit clock prop) should pull elapsed_ms = 2000
    // from the Scene, which is past the 1000ms cue duration → opacity = 1.0.
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 2_000.0,
                autoplay: Some(false),
                Sequence {
                    timeline: Some(timeline),
                    KineticBox { id: "title".to_string(), p { "x" } }
                }
            }
        }
    });
    assert!(html.contains("opacity: 1"), "{html}");
}
