//! Select — single-select dropdown built on `Popover`.
//!
//! The trigger button shows the current selection (or placeholder);
//! clicking opens a popover-anchored listbox, clicking an option emits
//! `on_select(value)` and closes the popover. The trigger owns a roving
//! active-index model so the full WAI-ARIA listbox keyboard contract is
//! honoured: ArrowUp/Down move the active option (wrapping), Home/End
//! jump to the first/last, Enter/Space commit the active option,
//! Escape closes, and printable keys typeahead to the first option
//! whose label starts with the typed buffer. The active option carries
//! `data-active="true"` and is mirrored onto the trigger via
//! `aria-activedescendant`. Full Combobox (typeahead *filter* + async
//! loading) lives in `Combobox`.

use dioxus::prelude::*;

use crate::popover::{Popover, PopoverSide};

/// One option in a `Select`. `value` round-trips through `on_select`;
/// `label` is the visible text; `disabled` greys out the row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
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

#[component]
pub fn Select(
    /// Stable id; also passed to the underlying Popover as its panel id.
    id: String,
    label: String,
    /// Currently-selected `value`. Empty string means nothing is
    /// selected and the placeholder is shown.
    selected: String,
    options: Vec<SelectOption>,
    #[props(default = "Choose…".to_string())] placeholder: String,
    #[props(default)] disabled: bool,
    /// Seed the internal open-state signal so the listbox renders on
    /// first paint. Lets gallery previews and SSR screenshots show the
    /// open state without programmatic clicks.
    #[props(default)]
    default_open: bool,
    on_select: Option<EventHandler<String>>,
) -> Element {
    let mut open = use_signal(|| default_open);
    // Index of the roving-active option. Seeds to the selected option so
    // arrow keys start from the current value; falls back to the first
    // enabled option.
    let initial_active = options
        .iter()
        .position(|opt| opt.value == selected && !opt.disabled)
        .or_else(|| first_enabled_index(&options))
        .unwrap_or(0);
    let mut active = use_signal(|| initial_active);
    // Typeahead buffer of recently-typed characters.
    let mut typed = use_signal(String::new);

    let selected_label = options
        .iter()
        .find(|opt| opt.value == selected)
        .map(|opt| opt.label.clone())
        .unwrap_or_else(|| placeholder.clone());
    let has_selection = !selected.is_empty();
    let trigger_class = if has_selection {
        "ui-select-trigger"
    } else {
        "ui-select-trigger ui-select-trigger--placeholder"
    };
    let label_id = format!("{id}-label");
    let popover_id = format!("{id}-popover");

    let active_index = *active.read();
    let active_descendant = options
        .get(active_index)
        .filter(|_| *open.read())
        .map(|opt| format!("{id}-option-{}", opt.value))
        .unwrap_or_default();

    // Snapshots captured for the keydown closure (cannot borrow props by
    // reference inside the move closure).
    let key_options = options.clone();
    let key_id = id.clone();

    rsx! {
        div { class: "ui-select",
            label { id: "{label_id}", class: "ui-select-label", "{label}" }
            Popover {
                id: popover_id.clone(),
                open: *open.read(),
                side: PopoverSide::Bottom,
                on_open_change: move |next: bool| open.set(next),
                trigger: rsx! {
                    button {
                        class: "{trigger_class}",
                        r#type: "button",
                        role: "combobox",
                        "aria-labelledby": "{label_id}",
                        "aria-haspopup": "listbox",
                        "aria-expanded": if *open.read() { "true" } else { "false" },
                        "aria-controls": "{popover_id}",
                        "aria-activedescendant": "{active_descendant}",
                        disabled,
                        onkeydown: move |evt| {
                            let len = key_options.len();
                            if len == 0 {
                                return;
                            }
                            let current = *active.read();
                            match evt.key() {
                                Key::ArrowDown => {
                                    evt.prevent_default();
                                    if !*open.read() {
                                        open.set(true);
                                    }
                                    if let Some(next) = step_enabled(&key_options, current, 1) {
                                        active.set(next);
                                    }
                                }
                                Key::ArrowUp => {
                                    evt.prevent_default();
                                    if !*open.read() {
                                        open.set(true);
                                    }
                                    if let Some(next) = step_enabled(&key_options, current, -1) {
                                        active.set(next);
                                    }
                                }
                                Key::Home => {
                                    evt.prevent_default();
                                    if let Some(next) = first_enabled_index(&key_options) {
                                        active.set(next);
                                    }
                                }
                                Key::End => {
                                    evt.prevent_default();
                                    if let Some(next) = last_enabled_index(&key_options) {
                                        active.set(next);
                                    }
                                }
                                Key::Enter => {
                                    if let Some(value) = committable(&key_options, current) {
                                        evt.prevent_default();
                                        if let Some(handler) = &on_select {
                                            handler.call(value);
                                        }
                                        open.set(false);
                                    }
                                }
                                Key::Character(ref c) if c.as_str() == " " => {
                                    // Space commits the active option (listbox
                                    // convention) rather than acting as typeahead.
                                    if let Some(value) = committable(&key_options, current) {
                                        evt.prevent_default();
                                        if let Some(handler) = &on_select {
                                            handler.call(value);
                                        }
                                        open.set(false);
                                    }
                                }
                                Key::Escape => {
                                    if *open.read() {
                                        evt.stop_propagation();
                                        open.set(false);
                                    }
                                }
                                Key::Character(ref c)
                                    if c.chars().next().map(|ch| !ch.is_control()).unwrap_or(false) =>
                                {
                                    // Accumulate into the typeahead buffer; if the
                                    // grown buffer matches nothing (e.g. the user
                                    // moved on to a new initial after a pause),
                                    // fall back to just the latest character so a
                                    // repeated key cycles through that initial.
                                    let mut buffer = typed.read().clone();
                                    buffer.push_str(c);
                                    let mut target = typeahead_index(&key_options, &buffer);
                                    if target.is_none() {
                                        buffer = c.clone();
                                        target = typeahead_index(&key_options, &buffer);
                                    }
                                    if let Some(next) = target {
                                        if !*open.read() {
                                            open.set(true);
                                        }
                                        active.set(next);
                                    }
                                    typed.set(buffer);
                                }
                                _ => {}
                            }
                        },
                        "{selected_label}"
                        span { class: "ui-select-chevron", "aria-hidden": "true", "▾" }
                    }
                },
                ul {
                    class: "ui-select-listbox",
                    role: "listbox",
                    "aria-labelledby": "{label_id}",
                    for (idx, option) in options.into_iter().enumerate() {
                        {
                            let value = option.value.clone();
                            let is_selected = value == selected;
                            let is_active = idx == active_index;
                            let opt_class = if option.disabled {
                                "ui-select-option ui-select-option--disabled"
                            } else if is_selected {
                                "ui-select-option ui-select-option--selected"
                            } else {
                                "ui-select-option"
                            };
                            let option_dom_id = format!("{key_id}-option-{value}");
                            rsx! {
                                li {
                                    id: "{option_dom_id}",
                                    class: "{opt_class}",
                                    role: "option",
                                    "aria-selected": if is_selected { "true" } else { "false" },
                                    "aria-disabled": if option.disabled { "true" } else { "false" },
                                    "data-active": if is_active && !option.disabled { "true" } else { "false" },
                                    onclick: move |_| {
                                        if option.disabled {
                                            return;
                                        }
                                        active.set(idx);
                                        if let Some(handler) = &on_select {
                                            handler.call(value.clone());
                                        }
                                        open.set(false);
                                    },
                                    "{option.label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// First index whose option is enabled, or `None` if every option is
/// disabled / the list is empty.
fn first_enabled_index(options: &[SelectOption]) -> Option<usize> {
    ui_core::roving::first_focusable(options.len(), |i| !options[i].disabled)
}

/// Last index whose option is enabled, or `None` if none are.
fn last_enabled_index(options: &[SelectOption]) -> Option<usize> {
    ui_core::roving::last_focusable(options.len(), |i| !options[i].disabled)
}

/// Step the active index by `delta` (±1), wrapping around the list and
/// skipping disabled options. Returns `None` only when no option is
/// enabled.
fn step_enabled(options: &[SelectOption], from: usize, delta: i32) -> Option<usize> {
    ui_core::roving::step_focusable(options.len(), from, delta, |i| !options[i].disabled)
}

/// Typeahead: first enabled option whose lowercased label starts with
/// the lowercased `buffer`. Returns `None` when nothing matches so the
/// caller can keep the current active option.
fn typeahead_index(options: &[SelectOption], buffer: &str) -> Option<usize> {
    ui_core::roving::typeahead_index(
        options.len(),
        buffer,
        |i| !options[i].disabled,
        |i| options[i].label.clone(),
    )
}

/// The `value` of the option at `index`, or `None` when the index is out
/// of range or the option is disabled. Shared by Enter and Space commit.
fn committable(options: &[SelectOption], index: usize) -> Option<String> {
    options
        .get(index)
        .filter(|opt| !opt.disabled)
        .map(|opt| opt.value.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> Vec<SelectOption> {
        vec![
            SelectOption::new("apple", "Apple"),
            SelectOption::new("apricot", "Apricot"),
            SelectOption::new("banana", "Banana"),
        ]
    }

    #[test]
    fn step_enabled_moves_forward_and_wraps() {
        let o = opts();
        assert_eq!(step_enabled(&o, 0, 1), Some(1));
        assert_eq!(step_enabled(&o, 2, 1), Some(0));
    }

    #[test]
    fn step_enabled_moves_backward_and_wraps() {
        let o = opts();
        assert_eq!(step_enabled(&o, 1, -1), Some(0));
        assert_eq!(step_enabled(&o, 0, -1), Some(2));
    }

    #[test]
    fn step_enabled_skips_disabled() {
        let o = vec![
            SelectOption::new("a", "A"),
            SelectOption::new("b", "B").disabled(),
            SelectOption::new("c", "C"),
        ];
        assert_eq!(step_enabled(&o, 0, 1), Some(2));
        assert_eq!(step_enabled(&o, 2, 1), Some(0));
        assert_eq!(step_enabled(&o, 0, -1), Some(2));
    }

    #[test]
    fn step_enabled_all_disabled_is_none() {
        let o = vec![
            SelectOption::new("a", "A").disabled(),
            SelectOption::new("b", "B").disabled(),
        ];
        assert_eq!(step_enabled(&o, 0, 1), None);
    }

    #[test]
    fn first_and_last_enabled_skip_disabled_edges() {
        let o = vec![
            SelectOption::new("a", "A").disabled(),
            SelectOption::new("b", "B"),
            SelectOption::new("c", "C"),
            SelectOption::new("d", "D").disabled(),
        ];
        assert_eq!(first_enabled_index(&o), Some(1));
        assert_eq!(last_enabled_index(&o), Some(2));
    }

    #[test]
    fn typeahead_matches_label_prefix_case_insensitive() {
        let o = opts();
        assert_eq!(typeahead_index(&o, "ban"), Some(2));
        assert_eq!(typeahead_index(&o, "AP"), Some(0));
    }

    #[test]
    fn typeahead_skips_disabled_and_empty_buffer() {
        let o = vec![
            SelectOption::new("apple", "Apple").disabled(),
            SelectOption::new("apricot", "Apricot"),
        ];
        assert_eq!(typeahead_index(&o, "ap"), Some(1));
        assert_eq!(typeahead_index(&o, ""), None);
        assert_eq!(typeahead_index(&o, "z"), None);
    }

    #[test]
    fn committable_returns_value_for_enabled_only() {
        let o = vec![
            SelectOption::new("apple", "Apple"),
            SelectOption::new("apricot", "Apricot").disabled(),
        ];
        assert_eq!(committable(&o, 0).as_deref(), Some("apple"));
        assert_eq!(committable(&o, 1), None); // disabled
        assert_eq!(committable(&o, 9), None); // out of range
    }
}
