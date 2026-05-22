use dioxus::prelude::*;
use ui_dioxus::{SharedElement, SharedLayout};

#[test]
fn shared_layout_renders_wrapper_with_class() {
    let html = dioxus_ssr::render_element(rsx! {
        SharedLayout {
            p { "inner" }
        }
    });
    assert!(html.contains("class=\"ui-shared-layout\""), "got {html}");
    assert!(html.contains("inner"));
}

#[test]
fn shared_element_renders_data_attribute_in_ssr() {
    let html = dioxus_ssr::render_element(rsx! {
        SharedElement { id: "hero".to_string(),
            p { "x" }
        }
    });
    assert!(html.contains("data-shared-id=\"hero\""), "got {html}");
    // SSR must not produce animated inline styles.
    assert!(!html.contains("style=\"opacity"), "got {html}");
    assert!(!html.contains("style=\"transform"), "got {html}");
}

#[test]
fn shared_layout_with_two_shared_elements_renders_correctly() {
    let html = dioxus_ssr::render_element(rsx! {
        SharedLayout {
            SharedElement { id: "x".to_string(), p { "a" } }
        }
        SharedLayout {
            SharedElement { id: "x".to_string(), p { "b" } }
        }
    });
    assert!(html.matches("data-shared-id=\"x\"").count() == 2);
}

#[test]
fn shared_element_outside_shared_layout_uses_default_registry() {
    let html = dioxus_ssr::render_element(rsx! {
        SharedElement { id: "lone".to_string(), p { "x" } }
    });
    assert!(html.contains("data-shared-id=\"lone\""));
}
