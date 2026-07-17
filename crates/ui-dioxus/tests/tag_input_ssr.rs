use dioxus::prelude::*;
use ui_dioxus::{ChipInput, TagInput};

fn render_component(root: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(root);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

fn tag_input_with_handler() -> Element {
    rsx! {
        TagInput {
            id: "tags",
            label: "Tags".to_string(),
            tags: vec!["rust".to_string(), "dioxus".to_string()],
            on_change: move |_| {},
        }
    }
}

#[test]
fn tag_input_renders_label_chips_and_control() {
    let html = dioxus_ssr::render_element(rsx! {
        TagInput {
            id: "tags",
            label: "Tags".to_string(),
            tags: vec!["rust".to_string(), "dioxus".to_string()],
            placeholder: "Add tag".to_string(),
        }
    });
    assert!(html.contains("ui-tag-input"), "{html}");
    assert!(html.contains("ui-tag-input-chip"), "{html}");
    assert!(html.contains("rust"), "{html}");
    assert!(html.contains("dioxus"), "{html}");
    assert!(html.contains(r#"type="text""#), "{html}");
    assert!(html.contains("Add tag"), "{html}");
    assert!(html.contains("Tags"), "{html}");
}

#[test]
fn tag_input_chips_show_remove_control_when_handler_present() {
    let html = render_component(tag_input_with_handler);
    assert!(html.contains("ui-tag-input-chip-remove"), "{html}");
    assert!(html.contains(r#"aria-label="Remove rust""#), "{html}");
}

#[test]
fn tag_input_disables_control_at_max_limit() {
    let html = dioxus_ssr::render_element(rsx! {
        TagInput {
            id: "t",
            label: "T".to_string(),
            tags: vec!["only".to_string()],
            max: Some(1),
        }
    });
    assert!(html.contains("disabled"), "{html}");
}

#[test]
fn chip_input_alias_renders_identically() {
    let a = dioxus_ssr::render_element(rsx! {
        ChipInput { id: "t".to_string(), label: "L".to_string(), tags: vec!["x".to_string()] }
    });
    let b = dioxus_ssr::render_element(rsx! {
        TagInput { id: "t".to_string(), label: "L".to_string(), tags: vec!["x".to_string()] }
    });
    assert_eq!(a, b);
}
