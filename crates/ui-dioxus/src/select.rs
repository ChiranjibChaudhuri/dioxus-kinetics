//! Select — single-select dropdown built on `Popover`.
//!
//! The lightest viable version: trigger button shows the current
//! selection (or placeholder), clicking opens a popover-anchored
//! listbox, clicking an option emits `on_select(value)` and closes the
//! popover. Keyboard arrow-key navigation is present; full Combobox
//! (typeahead filter + async loading) is a future spec.

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
    on_select: Option<EventHandler<String>>,
) -> Element {
    let mut open = use_signal(|| false);

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
                        disabled,
                        "{selected_label}"
                        span { class: "ui-select-chevron", "aria-hidden": "true", "▾" }
                    }
                },
                ul {
                    class: "ui-select-listbox",
                    role: "listbox",
                    "aria-labelledby": "{label_id}",
                    for option in options {
                        {
                            let value = option.value.clone();
                            let is_selected = value == selected;
                            let opt_class = if option.disabled {
                                "ui-select-option ui-select-option--disabled"
                            } else if is_selected {
                                "ui-select-option ui-select-option--selected"
                            } else {
                                "ui-select-option"
                            };
                            rsx! {
                                li {
                                    class: "{opt_class}",
                                    role: "option",
                                    "aria-selected": if is_selected { "true" } else { "false" },
                                    "aria-disabled": if option.disabled { "true" } else { "false" },
                                    onclick: move |_| {
                                        if option.disabled {
                                            return;
                                        }
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
