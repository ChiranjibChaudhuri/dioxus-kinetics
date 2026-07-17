//! VirtualizedDataTable — windowed variant of `DataTable`.
//!
//! The original `DataTable` is presentational and renders every row. This
//! variant renders only the rows inside the current scroll window plus an
//! `overscan` guard, padding the top and bottom with spacer rows so the
//! scrollbar reflects the true row count. The component is controlled: the
//! host owns `scroll_top` (typically from an `onscroll` listener on the
//! wrapper) and feeds it back in; the component owns the windowing math via
//! the pure [`visible_window`] helper.

use dioxus::prelude::*;

use crate::data_table::{DataTableColumn, DataTableRow, SortDirection};

/// Half-open `[start, end)` range of row indices that should be rendered.
///
/// `start` is clamped to `0`, pulls in `overscan` rows above the viewport,
/// and `end` is clamped to `total`. `row_height <= 0` or `total == 0` yields
/// `(0, 0)`.
pub fn visible_window(
    total: usize,
    row_height: f32,
    viewport_height: f32,
    scroll_top: f32,
    overscan: usize,
) -> (usize, usize) {
    if total == 0 || row_height <= 0.0 || !scroll_top.is_finite() {
        return (0, 0);
    }
    // Clamp scroll to the valid range so a host that reports a value past the
    // end still resolves to the final page rather than an empty window.
    let max_scroll = (total as f32 * row_height - viewport_height).max(0.0);
    let clamped_scroll = scroll_top.clamp(0.0, max_scroll);
    let first_visible = (clamped_scroll / row_height).floor().max(0.0) as usize;
    let start = first_visible.saturating_sub(overscan);
    let visible_rows = ((viewport_height / row_height).ceil().max(1.0) as usize).max(1);
    let mut end = start + visible_rows + 2 * overscan;
    if end > total {
        end = total;
    }
    let start = start.min(end);
    (start, end)
}

#[component]
pub fn VirtualizedDataTable(
    columns: Vec<DataTableColumn>,
    rows: Vec<DataTableRow>,
    #[props(default)] caption: String,
    #[props(default = 40.0)] row_height: f32,
    #[props(default = 320.0)] viewport_height: f32,
    #[props(default = 3)] overscan: usize,
    #[props(default)] scroll_top: f32,
    #[props(default)] sort_key: String,
    #[props(default)] sort_direction: SortDirection,
    on_sort: Option<EventHandler<String>>,
) -> Element {
    let total = rows.len();
    let (start, end) = visible_window(total, row_height, viewport_height, scroll_top, overscan);
    let top_pad = start as f32 * row_height;
    let bottom_pad = total.saturating_sub(end) as f32 * row_height;
    let window = rows.get(start..end).unwrap_or_default();
    let wrap_style = format!("max-height:{viewport_height}px;overflow-y:auto;");

    rsx! {
        div { class: "ui-virtual-table-wrap", style: "{wrap_style}",
            table { class: "ui-data-table ui-virtual-table",
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
                                if col.sortable {
                                    rsx! {
                                        th {
                                            scope: "col",
                                            "aria-sort": "{aria_sort}",
                                            class: "ui-data-table-th ui-data-table-th--sortable",
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
                                        th { scope: "col", class: "ui-data-table-th", "{col.label}" }
                                    }
                                }
                            }
                        }
                    }
                }
                tbody { class: "ui-data-table-body",
                    if top_pad > 0.0 {
                        tr { class: "ui-virtual-table-spacer", "aria-hidden": "true", style: "height:{top_pad}px", td {} }
                    }
                    for row in window.iter() {
                        tr { key: "{row.id}", class: "ui-data-table-row",
                            for cell in row.cells.iter() {
                                td { class: "ui-data-table-cell", "{cell}" }
                            }
                        }
                    }
                    if bottom_pad > 0.0 {
                        tr { class: "ui-virtual-table-spacer", "aria-hidden": "true", style: "height:{bottom_pad}px", td {} }
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
    fn window_at_top_includes_overscan() {
        // 50 rows, 40px tall, 120px viewport (3 rows), scroll at 0, overscan 3.
        let (start, end) = visible_window(50, 40.0, 120.0, 0.0, 3);
        assert_eq!(start, 0);
        // 3 visible + 2*3 overscan = 9
        assert_eq!(end, 9);
    }

    #[test]
    fn window_mid_scroll_offsets_start() {
        // scrolled to row 40 (1600px): start pulls back by overscan.
        let (start, end) = visible_window(50, 40.0, 120.0, 1600.0, 3);
        assert_eq!(start, 37);
        assert_eq!(end, 46);
    }

    #[test]
    fn window_clamps_end_to_total() {
        // scrolled past the end: end never exceeds total.
        let (start, end) = visible_window(50, 40.0, 120.0, 10_000.0, 3);
        assert_eq!(end, 50);
        assert!(start < end);
    }

    #[test]
    fn window_empty_total_is_empty() {
        assert_eq!(visible_window(0, 40.0, 120.0, 0.0, 3), (0, 0));
    }

    #[test]
    fn window_zero_row_height_is_empty() {
        assert_eq!(visible_window(50, 0.0, 120.0, 0.0, 3), (0, 0));
    }

    #[test]
    fn window_handles_non_finite_scroll() {
        assert_eq!(visible_window(50, 40.0, 120.0, f32::NAN, 3), (0, 0));
    }
}
