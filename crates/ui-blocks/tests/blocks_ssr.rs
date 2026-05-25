use dioxus::prelude::*;
use ui_blocks::{
    Caption, LowerThird, LowerThirdAccent, MetricCounter, SocialOverlay, SocialPlatform,
    WipeTransition,
};

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

#[test]
fn wipe_transition_emits_mask_image_kinetic_box() {
    let html = dioxus_ssr::render_element(rsx! {
        WipeTransition { duration_ms: 1_000.0, p { "covered content" } }
    });
    assert!(html.contains("ui-block-wipe-transition"), "{html}");
    assert!(html.contains("data-block=\"wipe-transition\""), "{html}");
    assert!(html.contains("covered content"), "{html}");
    assert!(
        html.contains("mask-image") || html.contains("-webkit-mask-image"),
        "{html}",
    );
}

#[test]
fn metric_counter_renders_three_kinetic_text_lines() {
    let html = dioxus_ssr::render_element(rsx! {
        MetricCounter {
            label: "Active users".to_string(),
            value: "1,287".to_string(),
            delta_text: Some("+24% w/w".to_string()),
        }
    });
    assert!(html.contains("Active users"), "{html}");
    assert!(html.contains("1,287"), "{html}");
    assert!(html.contains("+24% w/w"), "{html}");
    assert!(html.contains("ui-block-metric-counter"), "{html}");
}

#[test]
fn metric_counter_without_delta_omits_third_line() {
    let html = dioxus_ssr::render_element(rsx! {
        MetricCounter {
            label: "Loose".to_string(),
            value: "42".to_string(),
        }
    });
    assert!(html.contains("Loose"), "{html}");
    assert!(html.contains("42"), "{html}");
    // No delta -> no delta KineticText id reference.
    assert!(!html.contains("metric-delta"), "{html}");
}

#[test]
fn social_overlay_renders_platform_accent_class() {
    let html = dioxus_ssr::render_element(rsx! {
        SocialOverlay {
            platform: SocialPlatform::Instagram,
            handle: "@kineticsui".to_string(),
            message: "Just followed you!".to_string(),
        }
    });
    assert!(
        html.contains("ui-block-social-overlay--instagram"),
        "{html}"
    );
    assert!(html.contains("@kineticsui"), "{html}");
    assert!(html.contains("Just followed you!"), "{html}");
}

#[test]
fn social_overlay_twitter_variant() {
    let html = dioxus_ssr::render_element(rsx! {
        SocialOverlay {
            platform: SocialPlatform::Twitter,
            handle: "@dx".to_string(),
            message: "Replied to your post.".to_string(),
        }
    });
    assert!(html.contains("ui-block-social-overlay--twitter"), "{html}");
}
