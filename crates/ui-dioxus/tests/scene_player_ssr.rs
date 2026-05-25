use dioxus::prelude::*;
use ui_composition::ClipFill;
use ui_dioxus::{Clip, Scene, SceneState};
use ui_runtime::ReducedMotionProvider;

#[test]
fn scene_renders_root_data_attributes() {
    let html = dioxus_ssr::render_element(rsx! {
        Scene {
            id: "intro",
            width: 1920,
            height: 1080,
            duration_ms: 5_000.0,
            fps: Some(60),
            autoplay: Some(false),
            controls: Some(false),
            p { "hello" }
        }
    });
    assert!(html.contains("data-composition-id=\"intro\""), "{html}");
    assert!(html.contains("data-width=\"1920\""), "{html}");
    assert!(html.contains("data-height=\"1080\""), "{html}");
    assert!(html.contains("data-fps=\"60\""), "{html}");
    assert!(html.contains("data-duration-ms=\"5000\""), "{html}");
    assert!(html.contains("data-state=\"paused\""), "{html}");
    assert!(html.contains("data-reduced=\"false\""), "{html}");
    assert!(html.contains("aspect-ratio: 1920 / 1080"), "{html}");
}

#[test]
fn scene_with_reduced_motion_renders_settled() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: true,
            Scene {
                id: "intro",
                width: 100,
                height: 100,
                duration_ms: 1_000.0,
                p { "hi" }
            }
        }
    });
    assert!(html.contains("data-state=\"settled\""), "{html}");
    assert!(html.contains("data-reduced=\"true\""), "{html}");
    assert!(html.contains("data-elapsed-ms=\"1000\""), "{html}");
}

#[test]
fn scene_state_enum_is_re_exported_via_kinetics_prelude() {
    // Sanity: SceneState surfaces correctly.
    let s: SceneState = SceneState::Settled;
    let _ = s;
}

#[test]
fn clip_inside_scene_renders_active_at_settled() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: true,
            Scene {
                id: "intro", width: 100, height: 100, duration_ms: 2_000.0,
                Clip { start_ms: 0.0, duration_ms: 1_000.0, p { "first" } }
                Clip { start_ms: 1_000.0, duration_ms: 1_000.0, p { "second" } }
            }
        }
    });
    // At elapsed = duration = 2000ms, second clip is in range; first is past
    // its end and (default ClipFill::None) inactive.
    assert!(html.contains("data-clip-active=\"false\"") && html.contains("first"), "{html}");
    assert!(html.contains("data-clip-active=\"true\"") && html.contains("second"), "{html}");
}

#[test]
fn clip_with_hold_end_remains_active_after_range() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: true,
            Scene {
                id: "intro", width: 100, height: 100, duration_ms: 5_000.0,
                Clip {
                    start_ms: 0.0,
                    duration_ms: 1_000.0,
                    fill: Some(ClipFill::HoldEnd),
                    p { "held" }
                }
            }
        }
    });
    assert!(html.contains("data-clip-active=\"true\""), "{html}");
}

#[test]
fn clip_outside_scene_renders_with_orphan_flag() {
    // A Clip without a SceneContext renders the children as-is with
    // data-clip-active="true" and a `data-clip-orphan="true"` flag.
    let html = dioxus_ssr::render_element(rsx! {
        Clip { start_ms: 0.0, duration_ms: 1.0, p { "orphan" } }
    });
    assert!(html.contains("data-clip-orphan=\"true\""), "{html}");
}

#[test]
fn scene_with_controls_renders_transport_bar() {
    let html = dioxus_ssr::render_element(rsx! {
        Scene {
            id: "intro", width: 100, height: 100, duration_ms: 5_000.0,
            autoplay: Some(false),
            controls: Some(true),
            p { "body" }
        }
    });
    assert!(html.contains("ui-scene-transport"), "{html}");
    assert!(html.contains("ui-scene-play"), "{html}");
    assert!(html.contains("type=\"range\""), "{html}");
    assert!(html.contains("min=\"0\""), "{html}");
    assert!(html.contains("max=\"5000\""), "{html}");
    assert!(html.contains("ui-scene-time"), "{html}");
}

#[test]
fn scene_transport_marks_scrubber_disabled_under_reduced_motion() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: true,
            Scene {
                id: "intro", width: 100, height: 100, duration_ms: 1_000.0,
                controls: Some(true),
                p { "body" }
            }
        }
    });
    assert!(html.contains("aria-disabled=\"true\""), "{html}");
    assert!(html.contains("ui-scene-reduced-tag"), "{html}");
    assert!(html.contains("Reduced motion · settled state"), "{html}");
}

#[test]
fn scene_provides_adapter_registry_via_context() {
    // Smoke test: SceneContext is accessible inside children.
    #[component]
    fn ContextProbe() -> Element {
        let ctx = try_consume_context::<ui_dioxus::SceneContext>();
        let has = ctx.is_some();
        rsx! { span { "data-probe-has-context": "{has}", "ctx?" } }
    }
    let html = dioxus_ssr::render_element(rsx! {
        Scene {
            id: "intro", width: 1, height: 1, duration_ms: 10.0,
            autoplay: Some(false),
            ContextProbe {}
        }
    });
    assert!(html.contains("data-probe-has-context=\"true\""), "{html}");
}
