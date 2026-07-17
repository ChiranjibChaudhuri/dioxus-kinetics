use dioxus::prelude::*;
use ui_dioxus::{
    visible_window, DataTableColumn, DataTableRow, SortDirection, VirtualizedDataTable,
    WindowedDataTable,
};

fn fifty_rows() -> Vec<DataTableRow> {
    (0..50)
        .map(|i| DataTableRow::new(i.to_string(), vec![format!("Row {i}")]))
        .collect()
}

#[test]
fn virtual_table_renders_only_top_window_at_scroll_zero() {
    let html = dioxus_ssr::render_element(rsx! {
        VirtualizedDataTable {
            columns: vec![DataTableColumn::new("name", "Name")],
            rows: fifty_rows(),
            row_height: 40.0,
            viewport_height: 120.0,
            overscan: 3,
            scroll_top: 0.0,
        }
    });
    // First-window rows are present...
    assert!(html.contains("Row 0"), "{html}");
    assert!(html.contains("Row 8"), "{html}");
    // ...far out-of-window rows are NOT rendered.
    assert!(!html.contains("Row 40"), "{html}");
    // The bottom spacer carries the remaining row height.
    assert!(html.contains("ui-virtual-table-spacer"), "{html}");
    assert!(html.contains("height:"), "{html}");
}

#[test]
fn virtual_table_shifts_window_when_scrolled() {
    let html = dioxus_ssr::render_element(rsx! {
        VirtualizedDataTable {
            columns: vec![DataTableColumn::new("name", "Name")],
            rows: fifty_rows(),
            row_height: 40.0,
            viewport_height: 120.0,
            overscan: 3,
            scroll_top: 1600.0, // row 40
        }
    });
    assert!(html.contains("Row 40"), "{html}");
    assert!(html.contains("Row 42"), "{html}");
    assert!(!html.contains("Row 0"), "{html}");
}

#[test]
fn virtual_table_emits_sort_buttons_for_sortable_columns() {
    let html = dioxus_ssr::render_element(rsx! {
        VirtualizedDataTable {
            columns: vec![DataTableColumn::new("name", "Name").sortable()],
            rows: fifty_rows(),
            sort_key: "name".to_string(),
            sort_direction: SortDirection::Ascending,
        }
    });
    assert!(html.contains("ui-data-table-sort-button"), "{html}");
    assert!(html.contains(r#"aria-sort="ascending""#), "{html}");
}

#[test]
fn windowed_data_table_alias_renders_identically() {
    let cols = vec![DataTableColumn::new("name", "Name")];
    let rows = fifty_rows();
    let a = dioxus_ssr::render_element(rsx! {
        WindowedDataTable { columns: cols.clone(), rows: rows.clone(), row_height: 40.0, viewport_height: 120.0 }
    });
    let b = dioxus_ssr::render_element(rsx! {
        VirtualizedDataTable { columns: cols, rows, row_height: 40.0, viewport_height: 120.0 }
    });
    assert_eq!(a, b);
}

#[test]
fn visible_window_helper_matches_unit_tests() {
    assert_eq!(visible_window(50, 40.0, 120.0, 0.0, 3), (0, 9));
    assert_eq!(visible_window(50, 40.0, 120.0, 1600.0, 3), (37, 46));
}
