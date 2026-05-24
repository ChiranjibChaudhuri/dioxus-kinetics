//! DataTable — minimal viable tabular primitive.
//!
//! Renders a native `<table>` with column headers, body rows, an optional
//! caption, and per-column sort buttons that emit `on_sort(column_key)`.
//! Sorting itself is the caller's responsibility — the component is
//! presentational. Column resize, sticky headers, and virtualization are
//! deferred to a future spec.

use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SortDirection {
    #[default]
    None,
    Ascending,
    Descending,
}

impl SortDirection {
    pub const fn aria(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Ascending => "ascending",
            Self::Descending => "descending",
        }
    }

    pub const fn indicator(self) -> &'static str {
        match self {
            Self::None => "↕",
            Self::Ascending => "↑",
            Self::Descending => "↓",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTableColumn {
    pub key: String,
    pub label: String,
    pub sortable: bool,
}

impl DataTableColumn {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            sortable: false,
        }
    }

    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }
}

/// One body row. `id` is a stable key — Dioxus uses it for diff
/// reordering when the host sorts rows. `cells` is the visible string
/// for each column in declaration order; callers stringify their typed
/// data at the call site.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTableRow {
    pub id: String,
    pub cells: Vec<String>,
}

impl DataTableRow {
    pub fn new(id: impl Into<String>, cells: Vec<String>) -> Self {
        Self {
            id: id.into(),
            cells,
        }
    }
}

#[component]
pub fn DataTable(
    columns: Vec<DataTableColumn>,
    rows: Vec<DataTableRow>,
    #[props(default)] caption: String,
    #[props(default)] sort_key: String,
    #[props(default)] sort_direction: SortDirection,
    on_sort: Option<EventHandler<String>>,
) -> Element {
    rsx! {
        table { class: "ui-data-table",
            if !caption.is_empty() {
                caption { class: "ui-data-table-caption", "{caption}" }
            }
            thead { class: "ui-data-table-head",
                tr {
                    for col in columns.iter() {
                        {
                            let key = col.key.clone();
                            let is_sorted = key == sort_key;
                            let direction = if is_sorted { sort_direction } else { SortDirection::None };
                            let aria_sort = direction.aria();
                            let indicator = direction.indicator();
                            let label_class = if col.sortable {
                                "ui-data-table-th ui-data-table-th--sortable"
                            } else {
                                "ui-data-table-th"
                            };
                            if col.sortable {
                                rsx! {
                                    th {
                                        scope: "col",
                                        "aria-sort": "{aria_sort}",
                                        class: "{label_class}",
                                        button {
                                            class: "ui-data-table-sort-button",
                                            r#type: "button",
                                            "aria-label": "Sort by {col.label}",
                                            onclick: move |_| {
                                                if let Some(handler) = &on_sort {
                                                    handler.call(key.clone());
                                                }
                                            },
                                            "{col.label}"
                                            span { class: "ui-data-table-sort-indicator", "aria-hidden": "true", "{indicator}" }
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    th {
                                        scope: "col",
                                        class: "{label_class}",
                                        "{col.label}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            tbody { class: "ui-data-table-body",
                for row in rows.iter() {
                    tr { key: "{row.id}", class: "ui-data-table-row",
                        for cell in row.cells.iter() {
                            td { class: "ui-data-table-cell", "{cell}" }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_direction_maps_to_aria_value() {
        assert_eq!(SortDirection::None.aria(), "none");
        assert_eq!(SortDirection::Ascending.aria(), "ascending");
        assert_eq!(SortDirection::Descending.aria(), "descending");
    }

    #[test]
    fn column_builder_starts_non_sortable_then_opts_in() {
        let col = DataTableColumn::new("revenue", "Revenue");
        assert!(!col.sortable);
        let sortable = col.sortable();
        assert!(sortable.sortable);
    }

    #[test]
    fn sort_indicators_are_directional() {
        assert_eq!(SortDirection::None.indicator(), "↕");
        assert_eq!(SortDirection::Ascending.indicator(), "↑");
        assert_eq!(SortDirection::Descending.indicator(), "↓");
    }

    #[test]
    fn row_constructor_preserves_id_and_cells() {
        let row = DataTableRow::new("acme", vec!["Acme".into(), "12".into()]);
        assert_eq!(row.id, "acme");
        assert_eq!(row.cells, vec!["Acme".to_string(), "12".to_string()]);
    }
}
