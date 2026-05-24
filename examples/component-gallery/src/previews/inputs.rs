use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn text_field_preview() -> Element {
    rsx! { TextFieldPreviewBody {} }
}

#[component]
fn TextFieldPreviewBody() -> Element {
    let mut value = use_signal(|| "Acme Ops".to_string());
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Default · with help" }
                TextField {
                    id: "workspace-name",
                    label: "Workspace name",
                    value: value.read().clone(),
                    help_text: "Visible to teammates",
                    leading_text: "Org",
                    oninput: move |evt: FormEvent| value.set(evt.value()),
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Invalid · with error" }
                TextField {
                    id: "workspace-slug",
                    label: "Workspace slug",
                    value: "acme ops".to_string(),
                    placeholder: "lowercase, no spaces",
                    invalid: true,
                    error_text: "Slugs cannot contain spaces.",
                    leading_text: "/",
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Disabled" }
                TextField {
                    id: "workspace-region",
                    label: "Region",
                    value: "us-east-1".to_string(),
                    disabled: true,
                    help_text: "Region is fixed per workspace.",
                }
            }
        }
    }
}

pub fn checkbox_preview() -> Element {
    rsx! { CheckboxPreviewBody {} }
}

#[component]
fn CheckboxPreviewBody() -> Element {
    let mut checked = use_signal(|| true);
    rsx! {
        Checkbox {
            id: "weekly-summary",
            label: "Send weekly summary",
            checked: *checked.read(),
            description: "Every Monday morning",
            onchange: move |evt: FormEvent| {
                // FormEvent.value() returns "true"/"false" for checkboxes
                checked.set(evt.value() == "true");
            },
        }
    }
}

pub fn switch_preview() -> Element {
    rsx! { SwitchPreviewBody {} }
}

pub fn slider_preview() -> Element {
    rsx! { SliderPreviewBody {} }
}

pub fn select_preview() -> Element {
    rsx! { SelectPreviewBody {} }
}

pub fn date_picker_preview() -> Element {
    rsx! { DatePickerPreviewBody {} }
}

pub fn data_table_preview() -> Element {
    rsx! { DataTablePreviewBody {} }
}

pub fn combobox_preview() -> Element {
    rsx! { ComboboxPreviewBody {} }
}

pub fn radio_group_preview() -> Element {
    rsx! { RadioGroupPreviewBody {} }
}

#[component]
fn ComboboxPreviewBody() -> Element {
    let mut value = use_signal(|| "ord-2024-12-04".to_string());
    let mut query = use_signal(|| "ord-2024".to_string());
    let options = vec![
        ComboboxOption::new("ord-2024-12-04", "ORD-2024-12-04 — Globex Retail"),
        ComboboxOption::new("ord-2024-12-03", "ORD-2024-12-03 — Acme Ops"),
        ComboboxOption::new("ord-2024-11-30", "ORD-2024-11-30 — Initech R&D"),
        ComboboxOption::new("ord-2024-11-22", "ORD-2024-11-22 — Soylent Foods"),
    ];
    rsx! {
        Combobox {
            id: "ticket-finder",
            label: "Find a ticket",
            value: value.read().clone(),
            query: query.read().clone(),
            options,
            placeholder: "Search by ID or workspace",
            default_open: true,
            on_query: move |next: String| query.set(next),
            on_select: move |selected: String| value.set(selected),
        }
    }
}

#[component]
fn RadioGroupPreviewBody() -> Element {
    let mut value = use_signal(|| "monthly".to_string());
    let options = vec![
        RadioOption::new("monthly", "Monthly")
            .with_description("Billed on the first of every month"),
        RadioOption::new("quarterly", "Quarterly").with_description("Save 8% versus monthly"),
        RadioOption::new("annual", "Annual").with_description("Save 18% versus monthly"),
    ];
    rsx! {
        RadioGroup {
            id: "billing-plan",
            label: "Billing plan",
            name: "billing-plan",
            value: value.read().clone(),
            options,
            description: "Switch plans anytime — prorated automatically.",
            on_change: move |next: String| value.set(next),
        }
    }
}

#[component]
fn DataTablePreviewBody() -> Element {
    let mut sort_key = use_signal(|| "revenue".to_string());
    let mut sort_dir = use_signal(|| SortDirection::Descending);
    let columns = vec![
        DataTableColumn::new("workspace", "Workspace"),
        DataTableColumn::new("revenue", "Revenue").sortable(),
        DataTableColumn::new("seats", "Seats").sortable(),
    ];
    let rows = vec![
        DataTableRow::new(
            "acme",
            vec![
                "Acme Ops".to_string(),
                "$12,400".to_string(),
                "48".to_string(),
            ],
        ),
        DataTableRow::new(
            "globex",
            vec![
                "Globex Retail".to_string(),
                "$9,820".to_string(),
                "32".to_string(),
            ],
        ),
        DataTableRow::new(
            "initech",
            vec![
                "Initech R&D".to_string(),
                "$7,310".to_string(),
                "21".to_string(),
            ],
        ),
    ];
    rsx! {
        DataTable {
            columns,
            rows,
            caption: "Top 3 workspaces this month",
            sort_key: sort_key.read().clone(),
            sort_direction: *sort_dir.read(),
            on_sort: move |key: String| {
                let current = sort_key.read().clone();
                let current_dir = *sort_dir.read();
                if current == key {
                    sort_dir.set(match current_dir {
                        SortDirection::Ascending => SortDirection::Descending,
                        _ => SortDirection::Ascending,
                    });
                } else {
                    sort_key.set(key);
                    sort_dir.set(SortDirection::Ascending);
                }
            },
        }
    }
}

#[component]
fn DatePickerPreviewBody() -> Element {
    let mut value = use_signal(|| "2026-05-23".to_string());
    rsx! {
        DatePicker {
            id: "report-cutoff",
            label: "Report cutoff",
            value: value.read().clone(),
            default_open: true,
            on_select: move |iso: String| value.set(iso),
        }
    }
}

#[component]
fn SelectPreviewBody() -> Element {
    let mut value = use_signal(|| "monthly".to_string());
    let options = vec![
        SelectOption::new("monthly", "Monthly"),
        SelectOption::new("quarterly", "Quarterly"),
        SelectOption::new("annual", "Annual"),
        SelectOption::new("legacy", "Legacy (read-only)").disabled(),
    ];
    rsx! {
        Select {
            id: "billing-cadence",
            label: "Billing cadence",
            selected: value.read().clone(),
            options,
            default_open: true,
            on_select: move |v: String| value.set(v),
        }
    }
}

#[component]
fn SliderPreviewBody() -> Element {
    let mut volume = use_signal(|| 60.0_f32);
    rsx! {
        Slider {
            id: "media-volume",
            label: "Volume",
            value: *volume.read(),
            min: 0.0,
            max: 100.0,
            step: 1.0,
            description: "Live preview volume",
            value_text: format!("{}%", (*volume.read()).round() as i32),
            onchange: move |v: f32| volume.set(v),
        }
    }
}

#[component]
fn SwitchPreviewBody() -> Element {
    let mut checked = use_signal(|| true);
    rsx! {
        Switch {
            id: "auto-renew",
            label: "Auto renew",
            checked: *checked.read(),
            description: "Keep billing active",
            onchange: move |next: bool| checked.set(next),
        }
    }
}
