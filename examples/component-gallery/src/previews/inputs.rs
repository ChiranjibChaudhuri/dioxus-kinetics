use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn text_field_preview() -> Element {
    rsx! { TextFieldPreviewBody {} }
}

#[component]
fn TextFieldPreviewBody() -> Element {
    let mut value = use_signal(|| "Acme Ops".to_string());
    rsx! {
        TextField {
            id: "workspace-name",
            label: "Workspace name",
            value: value.read().clone(),
            help_text: "Visible to teammates",
            leading_text: "Org",
            oninput: move |evt: FormEvent| value.set(evt.value()),
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
        vec![
            "Acme Ops".to_string(),
            "$12,400".to_string(),
            "48".to_string(),
        ],
        vec![
            "Globex Retail".to_string(),
            "$9,820".to_string(),
            "32".to_string(),
        ],
        vec![
            "Initech R&D".to_string(),
            "$7,310".to_string(),
            "21".to_string(),
        ],
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
