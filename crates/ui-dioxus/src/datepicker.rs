//! DatePicker — calendar-grid temporal input built on `Popover`.
//!
//! Renders a trigger button that shows the currently-selected ISO date
//! (or placeholder), opens a Popover containing a month-navigable
//! calendar grid, emits `on_select(iso_date)` when a day is clicked.
//!
//! Dates are stored as ISO `YYYY-MM-DD` strings to keep this crate
//! dependency-free (no `chrono` / `time` in the workspace). The
//! conversions are pure functions in this module — see
//! `parse_iso_date`, `format_iso_date`, `days_in_month`. Range mode +
//! locale-aware month/day names are out of scope for v1; a future
//! spec layers them on.

use dioxus::prelude::*;

use crate::popover::{Popover, PopoverSide};

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const WEEKDAY_SHORT: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

/// Parse an ISO 8601 `YYYY-MM-DD` date string. Returns `(year, month, day)`
/// with `month` and `day` 1-indexed. Returns `None` for malformed input
/// or out-of-range fields (no leap-year validation; February 30 round-trips).
pub fn parse_iso_date(s: &str) -> Option<(i32, u32, u32)> {
    let bytes = s.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }
    let year: i32 = s.get(0..4)?.parse().ok()?;
    let month: u32 = s.get(5..7)?.parse().ok()?;
    let day: u32 = s.get(8..10)?.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some((year, month, day))
}

/// Format `(year, month, day)` as an ISO 8601 `YYYY-MM-DD` date string.
/// Caller is responsible for valid month (1–12) and day (1–31); no
/// validation is performed.
pub fn format_iso_date(year: i32, month: u32, day: u32) -> String {
    format!("{year:04}-{month:02}-{day:02}")
}

/// Returns the number of days in a given month, honouring Gregorian
/// leap-year rules.
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
            if leap {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Day-of-week index for `(year, month, day)`. Returns 0 for Monday … 6
/// for Sunday (ISO 8601 week). Uses Zeller's congruence.
pub fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
    let (y, m) = if month < 3 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };
    let k = (y % 100 + 100) % 100;
    let j = y.div_euclid(100);
    let h = (day as i32 + (13 * (m as i32 + 1)) / 5 + k + k / 4 + j / 4 + 5 * j).rem_euclid(7);
    // Zeller: 0 = Saturday, 1 = Sunday, 2 = Monday, … 6 = Friday.
    // Re-map to ISO: 0 = Monday … 6 = Sunday.
    match h {
        0 => 5,
        1 => 6,
        n => (n - 2) as u32,
    }
}

#[component]
pub fn DatePicker(
    /// Stable id; passed to the underlying Popover.
    id: String,
    label: String,
    /// Currently-selected date as ISO `YYYY-MM-DD`. Empty string means
    /// nothing is selected and the placeholder is shown.
    value: String,
    #[props(default = "Select date".to_string())] placeholder: String,
    #[props(default)] disabled: bool,
    on_select: Option<EventHandler<String>>,
) -> Element {
    let mut open = use_signal(|| false);

    // Decide the calendar's anchor month. Defaults to today's month
    // (which on wasm is approximate; we just default to value's month
    // when set, otherwise to January of the current year-equivalent).
    // For a fully accurate "today" we'd need js_sys::Date — out of
    // scope for the crate's no-wasm-binding policy on this module.
    let (anchor_year, anchor_month) = match parse_iso_date(&value) {
        Some((y, m, _)) => (y, m),
        None => (2026, 1),
    };
    let mut view_year = use_signal(|| anchor_year);
    let mut view_month = use_signal(|| anchor_month);

    let trigger_label = if value.is_empty() {
        placeholder.clone()
    } else {
        value.clone()
    };
    let trigger_class = if value.is_empty() {
        "ui-datepicker-trigger ui-datepicker-trigger--placeholder"
    } else {
        "ui-datepicker-trigger"
    };
    let label_id = format!("{id}-label");
    let popover_id = format!("{id}-popover");

    let year_now = *view_year.read();
    let month_now = *view_month.read();
    let total_days = days_in_month(year_now, month_now);
    let first_day_idx = day_of_week(year_now, month_now, 1);
    let value_clone = value.clone();

    rsx! {
        div { class: "ui-datepicker",
            label { id: "{label_id}", class: "ui-datepicker-label", "{label}" }
            Popover {
                id: popover_id.clone(),
                open: *open.read(),
                side: PopoverSide::Bottom,
                on_open_change: move |next: bool| open.set(next),
                trigger: rsx! {
                    button {
                        class: "{trigger_class}",
                        r#type: "button",
                        "aria-labelledby": "{label_id}",
                        "aria-haspopup": "dialog",
                        "aria-expanded": if *open.read() { "true" } else { "false" },
                        "aria-controls": "{popover_id}",
                        disabled,
                        "{trigger_label}"
                        span { class: "ui-datepicker-icon", "aria-hidden": "true", "📅" }
                    }
                },
                div { class: "ui-datepicker-calendar",
                    div { class: "ui-datepicker-nav",
                        button {
                            class: "ui-datepicker-nav-button",
                            r#type: "button",
                            "aria-label": "Previous month",
                            onclick: move |_| {
                                let m = *view_month.read();
                                let y = *view_year.read();
                                if m == 1 {
                                    view_month.set(12);
                                    view_year.set(y - 1);
                                } else {
                                    view_month.set(m - 1);
                                }
                            },
                            "‹"
                        }
                        strong { class: "ui-datepicker-title",
                            "{MONTH_NAMES[(month_now - 1) as usize]} {year_now}"
                        }
                        button {
                            class: "ui-datepicker-nav-button",
                            r#type: "button",
                            "aria-label": "Next month",
                            onclick: move |_| {
                                let m = *view_month.read();
                                let y = *view_year.read();
                                if m == 12 {
                                    view_month.set(1);
                                    view_year.set(y + 1);
                                } else {
                                    view_month.set(m + 1);
                                }
                            },
                            "›"
                        }
                    }
                    div {
                        class: "ui-datepicker-grid",
                        role: "grid",
                        "aria-labelledby": "{label_id}",
                        for label in WEEKDAY_SHORT.iter() {
                            div { class: "ui-datepicker-weekday", role: "columnheader", "{label}" }
                        }
                        // Empty cells before the first of the month.
                        for _idx in 0u32..first_day_idx {
                            div { class: "ui-datepicker-cell ui-datepicker-cell--empty" }
                        }
                        // Day cells.
                        for day in 1u32..=total_days {
                            {
                                let iso = format_iso_date(year_now, month_now, day);
                                let is_selected = iso == value_clone;
                                let cell_class = if is_selected {
                                    "ui-datepicker-cell ui-datepicker-cell--selected"
                                } else {
                                    "ui-datepicker-cell"
                                };
                                rsx! {
                                    button {
                                        class: "{cell_class}",
                                        r#type: "button",
                                        role: "gridcell",
                                        "aria-label": "{iso}",
                                        "aria-selected": if is_selected { "true" } else { "false" },
                                        onclick: move |_| {
                                            if let Some(handler) = &on_select {
                                                handler.call(iso.clone());
                                            }
                                            open.set(false);
                                        },
                                        "{day}"
                                    }
                                }
                            }
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
    fn parse_iso_date_roundtrips() {
        assert_eq!(parse_iso_date("2026-05-23"), Some((2026, 5, 23)));
        assert_eq!(format_iso_date(2026, 5, 23), "2026-05-23");
    }

    #[test]
    fn parse_iso_date_rejects_malformed() {
        assert_eq!(parse_iso_date("2026/05/23"), None);
        assert_eq!(parse_iso_date("2026-13-01"), None);
        assert_eq!(parse_iso_date("2026-12-32"), None);
        assert_eq!(parse_iso_date("not a date"), None);
    }

    #[test]
    fn days_in_month_handles_leap_year() {
        assert_eq!(days_in_month(2024, 2), 29); // leap
        assert_eq!(days_in_month(2025, 2), 28); // not leap
        assert_eq!(days_in_month(2000, 2), 29); // century leap
        assert_eq!(days_in_month(1900, 2), 28); // century non-leap
        assert_eq!(days_in_month(2026, 12), 31);
        assert_eq!(days_in_month(2026, 4), 30);
    }

    #[test]
    fn day_of_week_known_dates() {
        // 2026-05-23 is a Saturday → ISO index 5.
        assert_eq!(day_of_week(2026, 5, 23), 5);
        // 2026-01-01 is a Thursday → ISO index 3.
        assert_eq!(day_of_week(2026, 1, 1), 3);
        // 2000-01-01 is a Saturday → ISO index 5.
        assert_eq!(day_of_week(2000, 1, 1), 5);
    }
}
