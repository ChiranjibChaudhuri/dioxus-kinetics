use dioxus::prelude::*;
use ui_dioxus::{
    Checkbox, EmptyState, MetricCard, MetricTone, Sidebar, SidebarItem, SidebarSection, Switch,
    TabItem, TabPanel, Tabs, TextField, Toolbar,
};

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

#[test]
fn tabs_render_selected_tab_and_panel() {
    let html = dioxus_ssr::render_element(rsx! {
        Tabs {
            selected: "billing",
            items: vec![
                TabItem::new("overview", "Overview"),
                TabItem::new("billing", "Billing"),
            ],
            panels: vec![
                TabPanel::new("overview", "Overview content"),
                TabPanel::new("billing", "Billing content"),
            ],
        }
    });

    assert!(html.contains("role=\"tablist\""));
    assert!(html.contains("aria-selected=\"true\""));
    assert!(html.contains("Billing content"));
    assert!(!html.contains("Overview content"));
}

#[test]
fn toolbar_sidebar_and_display_components_render_semantic_structure() {
    let toolbar = dioxus_ssr::render_element(rsx! {
        Toolbar {
            primary: vec!["New".to_string(), "Filter".to_string()],
            secondary: "Updated now",
        }
    });
    let sidebar = dioxus_ssr::render_element(rsx! {
        Sidebar {
            collapsed: false,
            selected: "settings",
            sections: vec![SidebarSection::new(
                "Workspace",
                vec![
                    SidebarItem::new("home", "Home", "#home"),
                    SidebarItem::new("settings", "Settings", "#settings"),
                ],
            )],
        }
    });
    let metric = dioxus_ssr::render_element(rsx! {
        MetricCard {
            label: "Net revenue",
            value: "$128.4k",
            delta: "+12.5%",
            tone: MetricTone::Success,
        }
    });
    let empty = dioxus_ssr::render_element(rsx! {
        EmptyState {
            title: "No reports yet",
            description: "Create a report to share performance with your team.",
            action_label: "Create report",
        }
    });

    assert!(toolbar.contains("role=\"toolbar\""));
    assert!(toolbar.contains("ui-toolbar-group"));
    assert!(sidebar.contains("ui-sidebar"));
    assert!(sidebar.contains("aria-current=\"page\""));
    assert!(metric.contains("ui-metric-card--success"));
    assert!(metric.contains("ui-metric-card-sparkline"));
    assert!(empty.contains("ui-empty-state"));
    assert!(empty.contains("Create report"));
}
