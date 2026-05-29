//! Combobox — typeahead-filtered single-select built on `Popover`.
//!
//! Different from `Select`:
//!   - the trigger is a text `<input>` (not a button) that the user
//!     types into to narrow the visible options;
//!   - the listbox is filtered by `query` via [`filter_options`];
//!   - the consumer owns both `value` (selected option) and `query`
//!     (current text), making the component fully controlled.
//!
//! The input owns a roving active-index over the *filtered* options:
//! ArrowDown/ArrowUp move through the visible matches (wrapping), Enter
//! commits the active match (or the first when none is active yet) via
//! `on_select`, and Escape clears the query / closes the listbox. The
//! active option carries `data-active="true"`, mirrored onto the input
//! via `aria-activedescendant`. A visually-hidden `aria-live="polite"`
//! status sibling announces the visible result count.
//!
//! Use `Select` for short, fixed lists where typeahead is overkill,
//! and `Combobox` for longer lists or when free-text input matters.

use dioxus::prelude::*;

use crate::popover::{Popover, PopoverSide};

/// One option in a `Combobox`. `value` round-trips through
/// `on_select`; `label` is the visible (and filterable) text;
/// `disabled` greys out the row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl ComboboxOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Pure helper: return the subset of `options` whose label contains
/// `query` (case-insensitive). Empty query returns all options
/// unchanged. Used by `Combobox` at render time and exposed so hosts
/// with custom matching (fuzzy, prefix-only, etc.) can reuse the
/// signature.
pub fn filter_options<'a>(options: &'a [ComboboxOption], query: &str) -> Vec<&'a ComboboxOption> {
    if query.is_empty() {
        return options.iter().collect();
    }
    let needle = query.to_lowercase();
    options
        .iter()
        .filter(|opt| opt.label.to_lowercase().contains(&needle))
        .collect()
}

/// Pure helper: the screen-reader status string announcing how many
/// options are currently visible.
fn results_status(count: usize) -> String {
    match count {
        0 => "No results".to_string(),
        1 => "1 result".to_string(),
        n => format!("{n} results"),
    }
}

/// Step the active index by `delta` (±1) across the filtered options,
/// wrapping around and skipping disabled rows. Returns `None` when no
/// visible option is selectable.
fn step_visible(visible: &[&ComboboxOption], from: usize, delta: i32) -> Option<usize> {
    let len = visible.len();
    if len == 0 || visible.iter().all(|opt| opt.disabled) {
        return None;
    }
    let len_i = len as i32;
    let mut idx = from as i32;
    for _ in 0..len {
        idx = (idx + delta).rem_euclid(len_i);
        if !visible[idx as usize].disabled {
            return Some(idx as usize);
        }
    }
    None
}

/// First selectable (enabled) index in the filtered options.
fn first_visible_index(visible: &[&ComboboxOption]) -> Option<usize> {
    visible.iter().position(|opt| !opt.disabled)
}

#[component]
pub fn Combobox(
    /// Stable id; also passed to the underlying Popover as its panel id.
    id: String,
    label: String,
    /// Currently-selected `value`. Empty string means nothing is selected.
    #[props(default)]
    value: String,
    /// Free-text query the user has typed. Consumer-controlled so the
    /// host can clear/set it programmatically.
    #[props(default)]
    query: String,
    options: Vec<ComboboxOption>,
    #[props(default = "Search…".to_string())] placeholder: String,
    #[props(default = "No matches".to_string())] empty_text: String,
    #[props(default)] disabled: bool,
    /// Seed the internal open-state signal so the filtered listbox
    /// renders on first paint. Lets gallery previews and SSR screenshots
    /// show the open state without programmatic clicks.
    #[props(default)]
    default_open: bool,
    on_query: Option<EventHandler<String>>,
    on_select: Option<EventHandler<String>>,
) -> Element {
    let mut open = use_signal(|| default_open);
    // Roving active index over the *filtered* options. Clamped against
    // the visible length each render so query changes never leave a
    // stale out-of-range cursor.
    let mut active = use_signal(|| 0usize);

    let label_id = format!("{id}-label");
    let popover_id = format!("{id}-popover");
    let listbox_id = format!("{id}-listbox");
    let status_id = format!("{id}-status");
    let visible = filter_options(&options, &query);
    let has_matches = !visible.is_empty();

    // Clamp the active index into the current visible range.
    let active_index = if visible.is_empty() {
        0
    } else {
        (*active.read()).min(visible.len() - 1)
    };
    let active_descendant = visible
        .get(active_index)
        .filter(|_| *open.read() && has_matches)
        .map(|opt| format!("{id}-option-{}", opt.value))
        .unwrap_or_default();
    let status_text = results_status(visible.len());

    // Snapshots for the keydown closure.
    let key_options = options.clone();
    let key_query = query.clone();

    rsx! {
        div { class: "ui-combobox",
            label { id: "{label_id}", class: "ui-combobox-label", r#for: "{id}", "{label}" }
            Popover {
                id: popover_id.clone(),
                open: *open.read(),
                side: PopoverSide::Bottom,
                on_open_change: move |next: bool| open.set(next),
                trigger: rsx! {
                    input {
                        id: "{id}",
                        class: "ui-combobox-input",
                        r#type: "text",
                        role: "combobox",
                        value: "{query}",
                        placeholder: "{placeholder}",
                        autocomplete: "off",
                        "aria-labelledby": "{label_id}",
                        "aria-autocomplete": "list",
                        "aria-haspopup": "listbox",
                        "aria-expanded": if *open.read() { "true" } else { "false" },
                        "aria-controls": "{listbox_id}",
                        "aria-activedescendant": "{active_descendant}",
                        disabled,
                        oninput: move |evt| {
                            open.set(true);
                            // A fresh query reframes the result set; restart
                            // the cursor at the first match.
                            active.set(0);
                            if let Some(handler) = &on_query {
                                handler.call(evt.value());
                            }
                        },
                        onfocus: move |_| open.set(true),
                        onkeydown: move |evt| {
                            let current_visible = filter_options(&key_options, &key_query);
                            let current = *active.read();
                            match evt.key() {
                                Key::ArrowDown => {
                                    evt.prevent_default();
                                    if !*open.read() {
                                        open.set(true);
                                    }
                                    if let Some(next) = step_visible(&current_visible, current, 1) {
                                        active.set(next);
                                    }
                                }
                                Key::ArrowUp => {
                                    evt.prevent_default();
                                    if !*open.read() {
                                        open.set(true);
                                    }
                                    if let Some(next) = step_visible(&current_visible, current, -1) {
                                        active.set(next);
                                    }
                                }
                                Key::Enter => {
                                    // Commit the active match, or the first
                                    // selectable match when the cursor has not
                                    // moved yet.
                                    let target = current_visible
                                        .get(current)
                                        .filter(|opt| !opt.disabled)
                                        .map(|_| current)
                                        .or_else(|| first_visible_index(&current_visible));
                                    if let Some(idx) = target {
                                        if let Some(opt) = current_visible.get(idx) {
                                            evt.prevent_default();
                                            if let Some(handler) = &on_select {
                                                handler.call(opt.value.clone());
                                            }
                                            open.set(false);
                                        }
                                    }
                                }
                                Key::Escape => {
                                    evt.stop_propagation();
                                    active.set(0);
                                    if !key_query.is_empty() {
                                        if let Some(handler) = &on_query {
                                            handler.call(String::new());
                                        }
                                    }
                                    open.set(false);
                                }
                                _ => {}
                            }
                        },
                    }
                },
                // Screen-reader status announcing the live result count.
                // `aria-live="polite"` makes this a live region without the
                // explicit `role="status"` (which would collide with the
                // empty-state `<p role="status">` for single-element locators).
                div {
                    id: "{status_id}",
                    class: "visually-hidden",
                    "aria-live": "polite",
                    "aria-atomic": "true",
                    "{status_text}"
                }
                if has_matches {
                    ul {
                        id: "{listbox_id}",
                        class: "ui-combobox-listbox",
                        role: "listbox",
                        "aria-labelledby": "{label_id}",
                        for (idx, option) in visible.iter().copied().cloned().enumerate() {
                            {
                                let option_value = option.value.clone();
                                let is_selected = option_value == value;
                                let is_active = idx == active_index;
                                let opt_class = if option.disabled {
                                    "ui-combobox-option ui-combobox-option--disabled"
                                } else if is_selected {
                                    "ui-combobox-option ui-combobox-option--selected"
                                } else {
                                    "ui-combobox-option"
                                };
                                let disabled_opt = option.disabled;
                                let option_dom_id = format!("{id}-option-{option_value}");
                                rsx! {
                                    li {
                                        id: "{option_dom_id}",
                                        class: "{opt_class}",
                                        role: "option",
                                        "aria-selected": if is_selected { "true" } else { "false" },
                                        "aria-disabled": if disabled_opt { "true" } else { "false" },
                                        "data-active": if is_active && !disabled_opt { "true" } else { "false" },
                                        onclick: move |_| {
                                            if disabled_opt {
                                                return;
                                            }
                                            active.set(idx);
                                            if let Some(handler) = &on_select {
                                                handler.call(option_value.clone());
                                            }
                                            open.set(false);
                                        },
                                        "{option.label}"
                                    }
                                }
                            }
                        }
                    }
                } else {
                    p {
                        id: "{listbox_id}",
                        class: "ui-combobox-empty",
                        role: "status",
                        "{empty_text}"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> Vec<ComboboxOption> {
        vec![
            ComboboxOption::new("apple", "Apple"),
            ComboboxOption::new("apricot", "Apricot"),
            ComboboxOption::new("banana", "Banana"),
        ]
    }

    #[test]
    fn empty_query_returns_all_options() {
        let all = opts();
        let result = filter_options(&all, "");
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn filter_is_case_insensitive_substring() {
        let all = opts();
        let result = filter_options(&all, "AP");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].value, "apple");
        assert_eq!(result[1].value, "apricot");
    }

    #[test]
    fn filter_returns_empty_for_no_match() {
        let all = opts();
        let result = filter_options(&all, "xyz");
        assert!(result.is_empty());
    }

    #[test]
    fn disabled_builder_sets_flag() {
        let opt = ComboboxOption::new("apple", "Apple").disabled();
        assert!(opt.disabled);
    }

    #[test]
    fn results_status_singular_plural_and_empty() {
        assert_eq!(results_status(0), "No results");
        assert_eq!(results_status(1), "1 result");
        assert_eq!(results_status(3), "3 results");
    }

    #[test]
    fn step_visible_wraps_and_skips_disabled() {
        let all = vec![
            ComboboxOption::new("a", "A"),
            ComboboxOption::new("b", "B").disabled(),
            ComboboxOption::new("c", "C"),
        ];
        let visible = filter_options(&all, "");
        assert_eq!(step_visible(&visible, 0, 1), Some(2));
        assert_eq!(step_visible(&visible, 2, 1), Some(0));
        assert_eq!(step_visible(&visible, 0, -1), Some(2));
    }

    #[test]
    fn step_visible_empty_or_all_disabled_is_none() {
        let empty: Vec<&ComboboxOption> = Vec::new();
        assert_eq!(step_visible(&empty, 0, 1), None);
        let all = vec![ComboboxOption::new("a", "A").disabled()];
        let visible = filter_options(&all, "");
        assert_eq!(step_visible(&visible, 0, 1), None);
    }

    #[test]
    fn first_visible_index_skips_leading_disabled() {
        let all = vec![
            ComboboxOption::new("a", "A").disabled(),
            ComboboxOption::new("b", "B"),
        ];
        let visible = filter_options(&all, "");
        assert_eq!(first_visible_index(&visible), Some(1));
    }
}
