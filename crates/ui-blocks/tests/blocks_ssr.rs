use dioxus::prelude::*;
use ui_blocks::{Caption, LowerThird, LowerThirdAccent};

#[test]
fn lower_third_emits_aria_label_with_name_and_role() {
    let html = dioxus_ssr::render_element(rsx! {
        LowerThird { name: "Ada Lovelace".to_string(), role: "Mathematician".to_string() }
    });
    assert!(html.contains("Ada Lovelace"), "{html}");
    assert!(html.contains("Mathematician"), "{html}");
    assert!(
        html.contains("aria-label=\"Ada Lovelace, Mathematician\""),
        "{html}",
    );
}

#[test]
fn lower_third_accent_primary_is_default() {
    let html = dioxus_ssr::render_element(rsx! {
        LowerThird { name: "x".to_string(), role: "y".to_string() }
    });
    assert!(html.contains("ui-block-lower-third--primary"), "{html}");
}

#[test]
fn lower_third_accent_secondary_renders_modifier_class() {
    let html = dioxus_ssr::render_element(rsx! {
        LowerThird {
            name: "x".to_string(),
            role: "y".to_string(),
            accent: Some(LowerThirdAccent::Secondary),
        }
    });
    assert!(html.contains("ui-block-lower-third--secondary"), "{html}");
}

#[test]
fn caption_emits_per_word_split_text_spans() {
    let html = dioxus_ssr::render_element(rsx! {
        Caption { text: "Built with kinetics.".to_string() }
    });
    // Caption uses SplitText { split_by: Word }, which emits per-word
    // spans with data-stagger-index.
    assert!(html.contains("data-stagger-index=\"0\""), "{html}");
    assert!(html.contains("data-stagger-index=\"1\""), "{html}");
    assert!(html.contains("data-stagger-index=\"2\""), "{html}");
    assert!(
        html.contains("aria-label=\"Built with kinetics.\""),
        "{html}",
    );
}
