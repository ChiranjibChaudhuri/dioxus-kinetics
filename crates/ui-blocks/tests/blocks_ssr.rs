use dioxus::prelude::*;
use ui_blocks::{LowerThird, LowerThirdAccent};

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
