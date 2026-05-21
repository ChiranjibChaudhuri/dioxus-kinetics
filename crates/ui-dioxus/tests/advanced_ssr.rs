use dioxus::prelude::*;
use ui_dioxus::{Checkbox, Switch, TextField};

#[test]
fn text_field_renders_label_value_help_and_invalid_state() {
    let html = dioxus_ssr::render_element(rsx! {
        TextField {
            id: "workspace-name",
            label: "Workspace name",
            value: "Acme Ops",
            placeholder: "Enter workspace",
            help_text: "Visible to teammates",
            error_text: "Use at least 3 characters",
            invalid: true,
        }
    });

    assert!(html.contains("ui-text-field"));
    assert!(html.contains("ui-field--invalid"));
    assert!(html.contains("for=\"workspace-name\""));
    assert!(html.contains("aria-invalid=\"true\""));
    assert!(html.contains("Visible to teammates"));
    assert!(html.contains("Use at least 3 characters"));
}

#[test]
fn checkbox_renders_checked_and_mixed_states() {
    let checked = dioxus_ssr::render_element(rsx! {
        Checkbox {
            id: "weekly-summary",
            label: "Send weekly summary",
            checked: true,
            description: "Every Monday morning",
        }
    });
    let mixed = dioxus_ssr::render_element(rsx! {
        Checkbox {
            id: "partial-selection",
            label: "Select visible rows",
            indeterminate: true,
        }
    });

    assert!(checked.contains("ui-checkbox"));
    assert!(checked.contains("checked"));
    assert!(checked.contains("Every Monday morning"));
    assert!(mixed.contains("aria-checked=\"mixed\""));
    assert!(mixed.contains("ui-checkbox--mixed"));
}

#[test]
fn switch_uses_switch_role_and_checked_state() {
    let html = dioxus_ssr::render_element(rsx! {
        Switch {
            id: "auto-renew",
            label: "Auto renew",
            checked: true,
            description: "Keep billing active",
        }
    });

    assert!(html.contains("ui-switch"));
    assert!(html.contains("role=\"switch\""));
    assert!(html.contains("aria-checked=\"true\""));
    assert!(html.contains("Keep billing active"));
}
