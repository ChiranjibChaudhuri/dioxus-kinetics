//! DatePicker — calendar-grid temporal input built on `Popover`.
//!
//! Renders a trigger button that shows the currently-selected ISO date
//! (or placeholder), opens a Popover containing a month-navigable
//! calendar grid, emits `on_select(iso_date)` when a day is clicked or
//! committed via the keyboard.
//!
//! The grid honours the WAI-ARIA grid keyboard contract: Arrow
//! Left/Right move ±1 day, Arrow Up/Down move ∓1 week, PageUp/PageDown
//! move ∓1 month, Home/End jump to the start/end of the focused week,
//! and Enter/Space select the focused day. Day arithmetic rolls over
//! month and year boundaries via the pure [`add_days`] helper, keeping
//! the `view_year` / `view_month` state in sync so the visible grid
//! always frames the focused day.
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

/// Static anchor used when neither `value` nor `today` is a valid ISO
/// date. Hosts that care should always pass `today` so the calendar
/// opens on a sensible month.
pub const DATEPICKER_DEFAULT_ANCHOR: (i32, u32) = (2026, 1);

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

/// Pure date arithmetic: add `delta` days (which may be negative) to
/// `(year, month, day)`, rolling over month and year boundaries.
/// Assumes the input is a valid calendar date.
pub fn add_days(year: i32, month: u32, day: u32, delta: i32) -> (i32, u32, u32) {
    let mut y = year;
    let mut m = month as i32;
    let mut d = day as i32 + delta;

    // Walk backwards over month boundaries.
    while d < 1 {
        m -= 1;
        if m < 1 {
            m = 12;
            y -= 1;
        }
        d += days_in_month(y, m as u32) as i32;
    }
    // Walk forwards over month boundaries.
    loop {
        let dim = days_in_month(y, m as u32) as i32;
        if d <= dim {
            break;
        }
        d -= dim;
        m += 1;
        if m > 12 {
            m = 1;
            y += 1;
        }
    }
    (y, m as u32, d as u32)
}

/// Add (or subtract, when negative) `delta` calendar months to
/// `(year, month)`, clamping `day` into the resulting month's length so
/// e.g. Jan 31 → Feb 28. Rolls the year over as needed.
pub fn add_months(year: i32, month: u32, day: u32, delta: i32) -> (i32, u32, u32) {
    // Convert to a zero-based absolute month count to apply the delta,
    // then back to 1-indexed.
    let total = (year as i64) * 12 + (month as i64 - 1) + delta as i64;
    let new_year = total.div_euclid(12) as i32;
    let new_month = (total.rem_euclid(12) + 1) as u32;
    let clamped_day = day.min(days_in_month(new_year, new_month));
    (new_year, new_month, clamped_day)
}

/// Monday-anchored start of the week containing `(year, month, day)`.
pub fn start_of_week(year: i32, month: u32, day: u32) -> (i32, u32, u32) {
    let dow = day_of_week(year, month, day) as i32; // 0 = Monday
    add_days(year, month, day, -dow)
}

/// Sunday-anchored end of the week containing `(year, month, day)`.
pub fn end_of_week(year: i32, month: u32, day: u32) -> (i32, u32, u32) {
    let dow = day_of_week(year, month, day) as i32; // 0 = Monday … 6 = Sunday
    add_days(year, month, day, 6 - dow)
}

#[component]
pub fn DatePicker(
    /// Stable id; passed to the underlying Popover.
    id: String,
    label: String,
    /// Currently-selected date as ISO `YYYY-MM-DD`. Empty string means
    /// nothing is selected and the placeholder is shown.
    value: String,
    /// Today's date in ISO `YYYY-MM-DD`, supplied by the host so this
    /// crate stays free of `js_sys::Date` / `chrono`. Used as the
    /// calendar anchor when `value` is empty. Empty string falls back
    /// to the static epoch in [`DATEPICKER_DEFAULT_ANCHOR`].
    #[props(default)]
    today: String,
    #[props(default = "Select date".to_string())] placeholder: String,
    #[props(default)] disabled: bool,
    /// Seed the internal open-state signal so the calendar grid renders
    /// on first paint. Lets gallery previews and SSR screenshots show
    /// the open state without programmatic clicks.
    #[props(default)]
    default_open: bool,
    on_select: Option<EventHandler<String>>,
) -> Element {
    let mut open = use_signal(|| default_open);

    let (anchor_year, anchor_month, anchor_day) = parse_iso_date(&value)
        .or_else(|| parse_iso_date(&today))
        .unwrap_or((DATEPICKER_DEFAULT_ANCHOR.0, DATEPICKER_DEFAULT_ANCHOR.1, 1));
    let mut view_year = use_signal(|| anchor_year);
    let mut view_month = use_signal(|| anchor_month);
    // Day-of-month that holds keyboard focus within the visible grid.
    let mut focused_day = use_signal(|| anchor_day);

    // Deferred keyboard focus: keydown writes the new view_year/view_month/
    // focused_day signals, but the target cell only exists after the new
    // month re-renders. Mirroring the command-menu scroll-into-view effect,
    // this effect re-runs whenever the open state or focused position
    // changes and moves DOM focus once the new cells are in the DOM.
    let focus_grid_id = id.clone();
    use_effect(use_reactive(
        (
            &*open.read(),
            &*view_year.read(),
            &*view_month.read(),
            &*focused_day.read(),
        ),
        move |(is_open, year, month, day)| {
            if !is_open {
                return;
            }
            focus_day_cell(&focus_grid_id, year, month, day);
        },
    ));

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
    // Clamp the focused day into the visible month (it can dangle after a
    // month change shortens the month, e.g. Jan 31 → Feb).
    let focused_now = (*focused_day.read()).clamp(1, total_days);
    let grid_id = id.clone();
    // Lay the month out as calendar weeks for the `role="row"` wrappers:
    // `first_day_idx` leading fillers (None), then each day-of-month, then
    // trailing fillers to pad the final week to seven cells. Chunking by
    // seven keeps the grid > row > gridcell ARIA structure well-formed.
    let weeks: Vec<Vec<Option<u32>>> = {
        let mut cells: Vec<Option<u32>> = Vec::new();
        for _ in 0..first_day_idx {
            cells.push(None);
        }
        for day in 1u32..=total_days {
            cells.push(Some(day));
        }
        while !cells.len().is_multiple_of(7) {
            cells.push(None);
        }
        cells.chunks(7).map(|chunk| chunk.to_vec()).collect()
    };

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
                        onkeydown: move |evt| {
                            let y = *view_year.read();
                            let m = *view_month.read();
                            let d = (*focused_day.read()).clamp(1, days_in_month(y, m));
                            let next = match evt.key() {
                                Key::ArrowLeft => Some(add_days(y, m, d, -1)),
                                Key::ArrowRight => Some(add_days(y, m, d, 1)),
                                Key::ArrowUp => Some(add_days(y, m, d, -7)),
                                Key::ArrowDown => Some(add_days(y, m, d, 7)),
                                Key::PageUp => Some(add_months(y, m, d, -1)),
                                Key::PageDown => Some(add_months(y, m, d, 1)),
                                Key::Home => Some(start_of_week(y, m, d)),
                                Key::End => Some(end_of_week(y, m, d)),
                                Key::Enter => {
                                    evt.prevent_default();
                                    let iso = format_iso_date(y, m, d);
                                    if let Some(handler) = &on_select {
                                        handler.call(iso);
                                    }
                                    open.set(false);
                                    None
                                }
                                Key::Character(ref c) if c.as_str() == " " => {
                                    evt.prevent_default();
                                    let iso = format_iso_date(y, m, d);
                                    if let Some(handler) = &on_select {
                                        handler.call(iso);
                                    }
                                    open.set(false);
                                    None
                                }
                                _ => None,
                            };
                            if let Some((ny, nm, nd)) = next {
                                evt.prevent_default();
                                // Write the new month/day; DOM focus is applied
                                // by the deferred use_effect above once the new
                                // month cells have rendered.
                                view_year.set(ny);
                                view_month.set(nm);
                                focused_day.set(nd);
                            }
                        },
                        // Weekday header row.
                        div { role: "row",
                            for label in WEEKDAY_SHORT.iter() {
                                div { class: "ui-datepicker-weekday", role: "columnheader", "{label}" }
                            }
                        }
                        // One `role="row"` per calendar week; leading/trailing
                        // filler cells keep every row seven cells wide.
                        for (week_idx, week) in weeks.iter().enumerate() {
                            div { key: "week-{week_idx}", role: "row",
                                for (cell_idx, cell) in week.iter().enumerate() {
                                    if let Some(day) = *cell {
                                        {
                                            let iso = format_iso_date(year_now, month_now, day);
                                            let is_selected = iso == value_clone;
                                            let is_focused = day == focused_now;
                                            let cell_class = if is_selected {
                                                "ui-datepicker-cell ui-datepicker-cell--selected"
                                            } else {
                                                "ui-datepicker-cell"
                                            };
                                            let cell_dom_id = format!("{grid_id}-day-{iso}");
                                            // Roving tabindex: only the focused day is in
                                            // the tab order; arrows move focus among the rest.
                                            let tabindex = if is_focused { "0" } else { "-1" };
                                            rsx! {
                                                button {
                                                    id: "{cell_dom_id}",
                                                    class: "{cell_class}",
                                                    r#type: "button",
                                                    role: "gridcell",
                                                    tabindex: "{tabindex}",
                                                    "aria-label": "{iso}",
                                                    "aria-selected": if is_selected { "true" } else { "false" },
                                                    "data-active": if is_focused { "true" } else { "false" },
                                                    onclick: move |_| {
                                                        focused_day.set(day);
                                                        if let Some(handler) = &on_select {
                                                            handler.call(iso.clone());
                                                        }
                                                        open.set(false);
                                                    },
                                                    "{day}"
                                                }
                                            }
                                        }
                                    } else {
                                        div {
                                            key: "filler-{week_idx}-{cell_idx}",
                                            class: "ui-datepicker-cell ui-datepicker-cell--empty"
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
}

/// Move DOM focus to the gridcell with id `{grid_id}-day-{iso}`.
/// Mirrors `navigation::focus_tab`: the ISO date is digit/`-` only, and
/// the grid id is validated before interpolating into the JS literal.
fn focus_day_cell(grid_id: &str, year: i32, month: u32, day: u32) {
    let safe = grid_id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ':' || c == '.');
    if !safe {
        return;
    }
    let iso = format_iso_date(year, month, day);
    let _ = dioxus::document::eval(&format!(
        "const el = document.getElementById('{grid_id}-day-{iso}'); if (el) el.focus();"
    ));
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

    #[test]
    fn add_days_within_month() {
        assert_eq!(add_days(2026, 5, 10, 5), (2026, 5, 15));
        assert_eq!(add_days(2026, 5, 10, -3), (2026, 5, 7));
        assert_eq!(add_days(2026, 5, 10, -9), (2026, 5, 1));
    }

    #[test]
    fn add_days_rolls_over_month_forward() {
        // 2026-01-31 + 1 → 2026-02-01.
        assert_eq!(add_days(2026, 1, 31, 1), (2026, 2, 1));
        // +7 over a month edge.
        assert_eq!(add_days(2026, 1, 28, 7), (2026, 2, 4));
    }

    #[test]
    fn add_days_rolls_over_month_backward() {
        // 2026-03-01 - 1 → 2026-02-28.
        assert_eq!(add_days(2026, 3, 1, -1), (2026, 2, 28));
        // 2024 is a leap year: 2024-03-01 - 1 → 2024-02-29.
        assert_eq!(add_days(2024, 3, 1, -1), (2024, 2, 29));
    }

    #[test]
    fn add_days_rolls_over_year() {
        assert_eq!(add_days(2026, 12, 31, 1), (2027, 1, 1));
        assert_eq!(add_days(2027, 1, 1, -1), (2026, 12, 31));
    }

    #[test]
    fn add_months_clamps_day_into_short_month() {
        // Jan 31 - 1 month → Feb 28 (2026 not leap).
        assert_eq!(add_months(2026, 1, 31, 1), (2026, 2, 28));
        // Mar 31 - 1 month → Feb 28.
        assert_eq!(add_months(2026, 3, 31, -1), (2026, 2, 28));
    }

    #[test]
    fn add_months_rolls_over_year() {
        assert_eq!(add_months(2026, 12, 15, 1), (2027, 1, 15));
        assert_eq!(add_months(2026, 1, 15, -1), (2025, 12, 15));
        // Multi-year jump.
        assert_eq!(add_months(2026, 6, 10, -18), (2024, 12, 10));
    }

    #[test]
    fn week_edges_anchor_monday_and_sunday() {
        // 2026-05-23 is a Saturday (ISO 5).
        // Start of week (Monday) → 2026-05-18; end (Sunday) → 2026-05-24.
        assert_eq!(start_of_week(2026, 5, 23), (2026, 5, 18));
        assert_eq!(end_of_week(2026, 5, 23), (2026, 5, 24));
        // A Monday is its own start of week.
        assert_eq!(start_of_week(2026, 5, 18), (2026, 5, 18));
    }
}
