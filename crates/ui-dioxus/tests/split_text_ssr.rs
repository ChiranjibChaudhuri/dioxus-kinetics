use dioxus::prelude::*;
use ui_dioxus::{SplitMode, SplitText};

#[test]
fn split_text_character_mode_emits_per_glyph_spans() {
    let html = dioxus_ssr::render_element(rsx! {
        SplitText { text: "Hi".to_string(), split_by: SplitMode::Character }
    });

    assert!(html.contains("ui-split-text"));
    assert!(html.contains("data-stagger-index=\"0\""));
    assert!(html.contains("data-stagger-index=\"1\""));
    assert!(html.contains(">H<"));
    assert!(html.contains(">i<"));
}

#[test]
fn split_text_default_mode_is_character() {
    let html = dioxus_ssr::render_element(rsx! {
        SplitText { text: "Hi".to_string() }
    });

    assert!(html.contains("data-split-mode=\"character\""));
    assert!(html.contains("data-stagger-index=\"0\""));
    assert!(html.contains("data-stagger-index=\"1\""));
}

#[test]
fn split_text_parent_aria_label_carries_full_text() {
    let html = dioxus_ssr::render_element(rsx! {
        SplitText { text: "Hello world".to_string(), split_by: SplitMode::Character }
    });

    assert!(html.contains("aria-label=\"Hello world\""));
}

#[test]
fn split_text_glyph_spans_are_aria_hidden() {
    let html = dioxus_ssr::render_element(rsx! {
        SplitText { text: "Hi".to_string(), split_by: SplitMode::Character }
    });

    let count = html.matches("aria-hidden=\"true\"").count();
    assert!(
        count >= 2,
        "expected at least 2 aria-hidden spans, got {count}: {html}"
    );
}

#[test]
fn split_text_word_mode_emits_per_word_spans() {
    let html = dioxus_ssr::render_element(rsx! {
        SplitText { text: "Hello world".to_string(), split_by: SplitMode::Word }
    });

    assert!(html.contains("data-split-mode=\"word\""));
    assert!(html.contains("data-stagger-index=\"0\""));
    assert!(html.contains("data-stagger-index=\"1\""));
    assert!(html.contains(">Hello<"));
    assert!(html.contains(">world<"));
}

#[test]
fn split_text_word_mode_preserves_whitespace_between_words() {
    let html = dioxus_ssr::render_element(rsx! {
        SplitText { text: "A B".to_string(), split_by: SplitMode::Word }
    });

    // Whitespace must be preserved as a literal text node between word spans.
    assert!(
        html.contains("</span> <span"),
        "expected literal space between word spans: {html}"
    );
}

#[test]
fn split_text_glyphs_emit_cue_animation_when_inside_scene() {
    use ui_dioxus::{Scene, TimelineScope};
    use ui_runtime::reduced_motion::ReducedMotionProvider;
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 1_000.0,
                autoplay: Some(false),
                TimelineScope { id: "ts".to_string(), autoplay: true,
                    SplitText { text: "Hi".to_string() }
                }
            }
        }
    });
    // Reduced motion settles Scene to 1000ms. With default cue rise-in
    // and stagger step 80ms:
    //   glyph H: index 0 → 1000ms
    //   glyph i: index 1 → 920ms
    assert!(html.contains("animation-name: ui-cue-rise-in"), "{html}");
    assert!(
        html.contains("animation-delay: -1000ms") || html.contains("animation-delay: -920ms"),
        "{html}"
    );
}

#[test]
fn split_text_cue_prop_overrides_default() {
    use ui_dioxus::Scene;
    use ui_runtime::reduced_motion::ReducedMotionProvider;
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 1_000.0,
                autoplay: Some(false),
                SplitText { text: "Hi".to_string(), cue: "fade-in".to_string() }
            }
        }
    });
    assert!(html.contains("animation-name: ui-cue-fade-in"), "{html}");
}
