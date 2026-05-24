//! Combobox — typeahead-filtered single-select built on `Popover`.
//!
//! Different from `Select`:
//!   - the trigger is a text `<input>` (not a button) that the user
//!     types into to narrow the visible options;
//!   - the listbox is filtered by `query` via [`filter_options`];
//!   - the consumer owns both `value` (selected option) and `query`
//!     (current text), making the component fully controlled.
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

    let label_id = format!("{id}-label");
    let popover_id = format!("{id}-popover");
    let listbox_id = format!("{id}-listbox");
    let visible = filter_options(&options, &query);
    let has_matches = !visible.is_empty();

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
                        disabled,
                        oninput: move |evt| {
                            open.set(true);
                            if let Some(handler) = &on_query {
                                handler.call(evt.value());
                            }
                        },
                        onfocus: move |_| open.set(true),
                    }
                },
                if has_matches {
                    ul {
                        id: "{listbox_id}",
                        class: "ui-combobox-listbox",
                        role: "listbox",
                        "aria-labelledby": "{label_id}",
                        for option in visible.iter().copied().cloned() {
                            {
                                let option_value = option.value.clone();
                                let is_selected = option_value == value;
                                let opt_class = if option.disabled {
                                    "ui-combobox-option ui-combobox-option--disabled"
                                } else if is_selected {
                                    "ui-combobox-option ui-combobox-option--selected"
                                } else {
                                    "ui-combobox-option"
                                };
                                let disabled_opt = option.disabled;
                                rsx! {
                                    li {
                                        class: "{opt_class}",
                                        role: "option",
                                        "aria-selected": if is_selected { "true" } else { "false" },
                                        "aria-disabled": if disabled_opt { "true" } else { "false" },
                                        onclick: move |_| {
                                            if disabled_opt {
                                                return;
                                            }
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
}
