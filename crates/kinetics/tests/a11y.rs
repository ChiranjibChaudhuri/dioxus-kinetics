//! Accessibility snapshot suite. Gated on the `a11y-tests` feature so it does
//! not run as part of the default test sweep, but is available via
//! `cargo test -p kinetics --features a11y-tests`.

#![cfg(feature = "a11y-tests")]

use dioxus::prelude::*;
use kinetics::prelude::*;

fn render(node: Element) -> String {
    dioxus_ssr::render_element(node)
}

#[test]
fn switch_label_is_associated_via_aria_labelledby() {
    // Switch is a button with role=switch. The visible label cannot use the
    // <label for=...> association (that only binds to form-associated
    // elements), so it must come through aria-labelledby instead.
    let html = render(rsx! {
        Switch {
            id: "auto-renew".to_string(),
            label: "Auto renew".to_string(),
            checked: true,
        }
    });
    assert!(html.contains("role=\"switch\""), "{html}");
    assert!(
        html.contains("aria-labelledby=\"auto-renew-label\""),
        "{html}"
    );
    assert!(html.contains("id=\"auto-renew-label\""), "{html}");
}

#[test]
fn dialog_panel_is_focusable_with_modal_semantics() {
    let html = render(rsx! {
        Dialog {
            open: true,
            title: "Delete workspace".to_string(),
            description: "This action cannot be undone.".to_string(),
        }
    });
    assert!(html.contains("role=\"dialog\""), "{html}");
    assert!(html.contains("aria-modal=\"true\""), "{html}");
    assert!(
        html.contains("aria-labelledby=\"ui-dialog-title\""),
        "{html}"
    );
    assert!(
        html.contains("aria-describedby=\"ui-dialog-description\""),
        "{html}"
    );
    // Panel must be focusable so the keydown listener can receive Escape.
    assert!(html.contains("tabindex=\"-1\""), "{html}");
}

#[test]
fn tabs_only_selected_tab_is_in_tab_sequence() {
    let items = vec![
        TabItem::new("overview", "Overview"),
        TabItem::new("billing", "Billing"),
    ];
    let panels = vec![
        TabPanel::new("overview", "Overview body"),
        TabPanel::new("billing", "Billing body"),
    ];
    let html = render(rsx! {
        Tabs {
            selected: "billing".to_string(),
            items: items,
            panels: panels,
        }
    });
    // Selected tab gets tabindex 0; siblings tabindex -1 so arrow keys drive nav.
    assert!(
        html.contains("aria-selected=\"true\""),
        "expected one selected tab\n{html}"
    );
    assert!(
        html.contains("tabindex=\"0\""),
        "expected at least one focusable tab\n{html}"
    );
    assert!(
        html.contains("tabindex=\"-1\""),
        "expected unselected tabs to be tabindex -1\n{html}"
    );
}

#[test]
fn toast_uses_alert_role_for_danger_tone() {
    let html = render(rsx! {
        Toast {
            tone: ToastTone::Danger,
            title: "Connection lost".to_string(),
        }
    });
    assert!(html.contains("role=\"alert\""), "{html}");
}

#[test]
fn toast_uses_status_role_for_neutral_tone() {
    let html = render(rsx! {
        Toast {
            tone: ToastTone::Neutral,
            title: "Saved".to_string(),
        }
    });
    assert!(html.contains("role=\"status\""), "{html}");
}

#[test]
fn checkbox_in_mixed_state_reports_aria_mixed() {
    let html = render(rsx! {
        Checkbox {
            id: "select-all".to_string(),
            label: "Select all".to_string(),
            indeterminate: true,
        }
    });
    assert!(html.contains("aria-checked=\"mixed\""), "{html}");
}

#[test]
fn icon_button_has_accessible_name_from_label() {
    let html = render(rsx! {
        IconButton {
            label: "Archive".to_string(),
            tone: IconButtonTone::Neutral,
        }
    });
    assert!(html.contains("aria-label=\"Archive\""), "{html}");
}

#[test]
fn text_field_associates_label_help_and_error_via_described_by() {
    let html = render(rsx! {
        TextField {
            id: "workspace-name".to_string(),
            label: "Workspace name".to_string(),
            help_text: "Visible to teammates".to_string(),
            error_text: "Name is required".to_string(),
            invalid: true,
        }
    });
    assert!(html.contains("for=\"workspace-name\""), "{html}");
    assert!(
        html.contains("aria-describedby=\"workspace-name-help workspace-name-error\""),
        "{html}"
    );
    assert!(html.contains("aria-invalid=\"true\""), "{html}");
    assert!(html.contains("role=\"alert\""), "{html}");
}

#[test]
fn command_menu_marks_modal_with_listbox_semantics() {
    let groups = vec![CommandGroup::new(
        "Reports",
        vec![CommandItem::new("export", "Export", "Save as PDF")],
    )];
    let html = render(rsx! {
        CommandMenu {
            open: true,
            query: "rep".to_string(),
            selected_id: "export".to_string(),
            groups: groups,
        }
    });
    assert!(html.contains("role=\"dialog\""), "{html}");
    assert!(html.contains("aria-modal=\"true\""), "{html}");
    assert!(html.contains("role=\"listbox\""), "{html}");
    assert!(html.contains("aria-selected=\"true\""), "{html}");
    assert!(
        html.contains("aria-activedescendant=\"ui-command-item-export\""),
        "{html}"
    );
}
