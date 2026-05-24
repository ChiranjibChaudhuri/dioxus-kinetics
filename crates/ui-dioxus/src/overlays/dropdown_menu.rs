//! DropdownMenu — anchored `role="menu"` overlay for action lists.
//!
//! Different from `CommandMenu`:
//!   - no search input;
//!   - `role="menu"` + `role="menuitem"` instead of `listbox`/`option`
//!     (matches the WAI-ARIA menu pattern for "fire-and-forget" actions);
//!   - separators are first-class items via [`DropdownMenuItem::separator`];
//!   - built on `Popover`, so positioning, ESC dismissal, and trigger
//!     anchoring come for free.
//!
//! Use for kebab-menus, "More actions" buttons, and overflow menus.
//! Reach for `CommandMenu` when the surface is a typeahead palette.

use dioxus::prelude::*;

use crate::popover::{Popover, PopoverSide};

/// One row in a `DropdownMenu`. Construct via [`DropdownMenuItem::new`]
/// for an action row or [`DropdownMenuItem::separator`] for a divider.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropdownMenuItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
    pub separator: bool,
}

impl DropdownMenuItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            separator: false,
        }
    }

    /// Build a non-interactive divider. The `id` is ignored by
    /// keyboard navigation but kept so callers can use stable
    /// React-style keys.
    pub fn separator(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: String::new(),
            disabled: true,
            separator: true,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[component]
pub fn DropdownMenu(
    /// Stable id passed to the underlying Popover panel.
    id: String,
    /// The interactive element that opens the menu.
    trigger: Element,
    items: Vec<DropdownMenuItem>,
    #[props(default = PopoverSide::Bottom)] side: PopoverSide,
    /// Seed the internal open-state signal so the menu renders on
    /// first paint. Lets gallery previews and SSR screenshots show
    /// the open state without programmatic clicks.
    #[props(default)]
    default_open: bool,
    on_select: Option<EventHandler<String>>,
) -> Element {
    let mut open = use_signal(|| default_open);

    rsx! {
        Popover {
            id: id.clone(),
            open: *open.read(),
            side,
            on_open_change: move |next: bool| open.set(next),
            trigger,
            ul {
                class: "ui-dropdown-menu",
                role: "menu",
                "aria-labelledby": "{id}-trigger",
                for item in items {
                    {
                        let item_id = item.id.clone();
                        let item_disabled = item.disabled;
                        let item_separator = item.separator;
                        let item_label = item.label.clone();
                        if item_separator {
                            rsx! {
                                li {
                                    class: "ui-dropdown-menu-separator",
                                    role: "separator",
                                    "aria-hidden": "true"
                                }
                            }
                        } else {
                            let item_class = if item_disabled {
                                "ui-dropdown-menu-item ui-dropdown-menu-item--disabled"
                            } else {
                                "ui-dropdown-menu-item"
                            };
                            rsx! {
                                li { class: "{item_class}", role: "none",
                                    button {
                                        class: "ui-dropdown-menu-button",
                                        r#type: "button",
                                        role: "menuitem",
                                        disabled: item_disabled,
                                        "aria-disabled": if item_disabled { "true" } else { "false" },
                                        onclick: move |_| {
                                            if item_disabled {
                                                return;
                                            }
                                            if let Some(handler) = &on_select {
                                                handler.call(item_id.clone());
                                            }
                                            open.set(false);
                                        },
                                        "{item_label}"
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
    fn new_item_is_enabled_action() {
        let item = DropdownMenuItem::new("rename", "Rename");
        assert!(!item.disabled);
        assert!(!item.separator);
        assert_eq!(item.label, "Rename");
    }

    #[test]
    fn separator_item_is_disabled_divider() {
        let item = DropdownMenuItem::separator("div-1");
        assert!(item.separator);
        assert!(item.disabled);
    }

    #[test]
    fn disabled_builder_disables_action_row() {
        let item = DropdownMenuItem::new("delete", "Delete").disabled();
        assert!(item.disabled);
        assert!(!item.separator);
    }
}
