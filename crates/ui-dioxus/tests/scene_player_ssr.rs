use dioxus::prelude::*;
use ui_dioxus::{Scene, SceneState};
use ui_runtime::ReducedMotion;

// `ReducedMotionProvider` in this workspace (see
// `crates/ui-runtime/src/reduced_motion.rs`) auto-detects via
// `prefers-reduced-motion` and is wasm-only for its reactive listener; it
// does not accept a `reduced: bool` prop. To force the reduced-motion
// branch deterministically under SSR we provide the `ReducedMotion`
// context directly — same shape `use_reduced_motion()` consumes. This
// mirrors the pattern in `crates/ui-runtime/tests/hooks_ssr.rs`.
#[component]
fn ReducedMotionContext(value: ReducedMotion, children: Element) -> Element {
    use_context_provider(|| value);
    rsx! { {children} }
}

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
        ReducedMotionContext { value: ReducedMotion(true),
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
