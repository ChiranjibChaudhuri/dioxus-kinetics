use dioxus::prelude::*;
use ui_dioxus::{
    Checkbox, CommandGroup, CommandItem, CommandMenu, Dialog, DialogAction, EmptyState, MetricCard,
    MetricTone, Sidebar, SidebarItem, SidebarSection, Switch, TabItem, TabPanel, Tabs, TextField,
    Toast, ToastTone, Toolbar, Tooltip,
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

#[test]
fn dialog_toast_and_tooltip_render_overlay_semantics() {
    let dialog = dioxus_ssr::render_element(rsx! {
        Dialog {
            open: true,
            title: "Delete workspace",
            description: "This action cannot be undone.",
            body: "All reports and settings will be archived.",
            actions: vec![
                DialogAction::ghost("cancel", "Cancel"),
                DialogAction::danger("delete", "Delete"),
            ],
        }
    });
    let closed_dialog = dioxus_ssr::render_element(rsx! {
        Dialog {
            open: false,
            title: "Hidden",
        }
    });
    let toast = dioxus_ssr::render_element(rsx! {
        Toast {
            tone: ToastTone::Success,
            title: "Report exported",
            description: "The PDF is ready.",
            action_label: "Open",
            dismiss_label: "Dismiss",
        }
    });
    let tooltip = dioxus_ssr::render_element(rsx! {
        Tooltip {
            id: "revenue-tip",
            visible: true,
            trigger_label: "Net revenue",
            content: "Revenue after refunds and credits.",
        }
    });

    assert!(dialog.contains("role=\"dialog\""));
    assert!(dialog.contains("aria-modal=\"true\""));
    assert!(dialog.contains("ui-dialog-backdrop"));
    assert!(!closed_dialog.contains("ui-dialog-panel"));
    assert!(toast.contains("ui-toast--success"));
    assert!(toast.contains("role=\"status\""));
    assert!(tooltip.contains("role=\"tooltip\""));
    assert!(tooltip.contains("aria-describedby=\"revenue-tip\""));
}

#[test]
fn command_menu_renders_grouped_items_and_empty_state() {
    let menu = dioxus_ssr::render_element(rsx! {
        CommandMenu {
            open: true,
            query: "rep",
            selected_id: "reports",
            empty_text: "No commands",
            groups: vec![CommandGroup::new(
                "Navigation",
                vec![
                    CommandItem::new("dashboard", "Open dashboard", "Go to overview"),
                    CommandItem::new("reports", "Open reports", "Review exports"),
                ],
            )],
        }
    });
    let empty = dioxus_ssr::render_element(rsx! {
        CommandMenu {
            open: true,
            query: "zzz",
            empty_text: "No commands",
            groups: vec![],
        }
    });

    assert!(menu.contains("ui-command-menu"));
    assert!(menu.contains("role=\"dialog\""));
    assert!(menu.contains("role=\"listbox\""));
    assert!(menu.contains("aria-selected=\"true\""));
    assert!(menu.contains("Open reports"));
    assert!(empty.contains("No commands"));
}
