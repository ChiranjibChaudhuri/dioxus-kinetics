use dioxus::prelude::*;
use ui_dioxus::{IconButton, IconButtonSize, IconButtonTone};
use ui_icons::Close;

#[test]
fn icon_button_renders_button_with_aria_label_and_icon_slot() {
    let html = dioxus_ssr::render_element(rsx! {
        IconButton {
            label: "Close dialog".to_string(),
            Close { size: 16 }
        }
    });

    assert!(html.contains("<button"));
    assert!(html.contains("type=\"button\""));
    assert!(html.contains("aria-label=\"Close dialog\""));
    assert!(html.contains("class=\"ui-icon-button"));
    assert!(html.contains("<svg"));
}

#[test]
fn icon_button_emits_tone_and_size_classes() {
    let html = dioxus_ssr::render_element(rsx! {
        IconButton {
            label: "Delete".to_string(),
            tone: IconButtonTone::Danger,
            size: IconButtonSize::Spacious,
            Close { size: 20 }
        }
    });

    assert!(html.contains("ui-icon-button--danger"));
    assert!(html.contains("ui-icon-button--spacious"));
}

#[test]
fn icon_button_disabled_includes_attribute() {
    let html = dioxus_ssr::render_element(rsx! {
        IconButton {
            label: "Locked".to_string(),
            disabled: true,
            Close { size: 16 }
        }
    });

    assert!(html.contains("disabled"));
}
