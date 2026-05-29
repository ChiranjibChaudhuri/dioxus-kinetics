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
//! The menu owns a roving DOM-focus engine matching the WAI-ARIA menu
//! pattern: ArrowUp/Down move focus across the enabled items (wrapping),
//! Home/End jump to the first/last, printable keys typeahead to the next
//! item whose label starts with the buffer, Enter/Space activate the
//! focused item, and Escape closes. Focus moves via
//! `document::eval`-driven `element.focus()` (mirroring
//! `navigation::focus_tab`); the focused item also carries
//! `data-active="true"`.
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

    /// Whether this row participates in roving focus (enabled action row).
    fn is_focusable(&self) -> bool {
        !self.separator && !self.disabled
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
    // Index (into `items`) of the roving-focused row. Seeds to the first
    // focusable item.
    let initial_active = first_focusable_index(&items).unwrap_or(0);
    let mut active = use_signal(|| initial_active);
    let mut typed = use_signal(String::new);

    let active_index = *active.read();

    // Snapshots for the keydown closure (the render loop keeps `key_id`).
    let key_items = items.clone();
    let key_id = id.clone();
    let key_id_for_handler = id.clone();

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
                onkeydown: move |evt| {
                    let current = *active.read();
                    match evt.key() {
                        Key::ArrowDown => {
                            evt.prevent_default();
                            if let Some(next) = step_focusable(&key_items, current, 1) {
                                active.set(next);
                                focus_menu_item(&key_id_for_handler, &key_items[next].id);
                            }
                        }
                        Key::ArrowUp => {
                            evt.prevent_default();
                            if let Some(next) = step_focusable(&key_items, current, -1) {
                                active.set(next);
                                focus_menu_item(&key_id_for_handler, &key_items[next].id);
                            }
                        }
                        Key::Home => {
                            evt.prevent_default();
                            if let Some(next) = first_focusable_index(&key_items) {
                                active.set(next);
                                focus_menu_item(&key_id_for_handler, &key_items[next].id);
                            }
                        }
                        Key::End => {
                            evt.prevent_default();
                            if let Some(next) = last_focusable_index(&key_items) {
                                active.set(next);
                                focus_menu_item(&key_id_for_handler, &key_items[next].id);
                            }
                        }
                        Key::Enter => {
                            if let Some(item) = key_items.get(current) {
                                if item.is_focusable() {
                                    evt.prevent_default();
                                    if let Some(handler) = &on_select {
                                        handler.call(item.id.clone());
                                    }
                                    open.set(false);
                                }
                            }
                        }
                        Key::Character(ref c) if c.as_str() == " " => {
                            if let Some(item) = key_items.get(current) {
                                if item.is_focusable() {
                                    evt.prevent_default();
                                    if let Some(handler) = &on_select {
                                        handler.call(item.id.clone());
                                    }
                                    open.set(false);
                                }
                            }
                        }
                        Key::Escape => {
                            // Let it also bubble to Popover, but close here too.
                            open.set(false);
                        }
                        Key::Character(ref c)
                            if c.chars().next().map(|ch| !ch.is_control()).unwrap_or(false) =>
                        {
                            // Accumulate into the typeahead buffer; if the grown
                            // buffer matches nothing, retry with just the latest
                            // character so a repeated key cycles that initial.
                            let mut buffer = typed.read().clone();
                            buffer.push_str(c);
                            let mut target = typeahead_index(&key_items, &buffer);
                            if target.is_none() {
                                buffer = c.clone();
                                target = typeahead_index(&key_items, &buffer);
                            }
                            if let Some(next) = target {
                                active.set(next);
                                focus_menu_item(&key_id_for_handler, &key_items[next].id);
                            }
                            typed.set(buffer);
                        }
                        _ => {}
                    }
                },
                for (idx, item) in items.into_iter().enumerate() {
                    {
                        let item_id = item.id.clone();
                        let item_disabled = item.disabled;
                        let item_separator = item.separator;
                        let item_label = item.label.clone();
                        let is_active = idx == active_index && item.is_focusable();
                        let button_dom_id = format!("{key_id}-item-{item_id}");
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
                            // Roving tabindex: only the active row is in the
                            // tab sequence; arrows move focus among the rest.
                            let tabindex = if is_active { "0" } else { "-1" };
                            let on_select_for_click = on_select;
                            rsx! {
                                li { class: "{item_class}", role: "none",
                                    button {
                                        id: "{button_dom_id}",
                                        class: "ui-dropdown-menu-button",
                                        r#type: "button",
                                        role: "menuitem",
                                        tabindex: "{tabindex}",
                                        disabled: item_disabled,
                                        "aria-disabled": if item_disabled { "true" } else { "false" },
                                        "data-active": if is_active { "true" } else { "false" },
                                        onclick: move |_| {
                                            if item_disabled {
                                                return;
                                            }
                                            active.set(idx);
                                            if let Some(handler) = &on_select_for_click {
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

/// First index whose row is focusable (enabled, non-separator).
fn first_focusable_index(items: &[DropdownMenuItem]) -> Option<usize> {
    items.iter().position(|item| item.is_focusable())
}

/// Last index whose row is focusable.
fn last_focusable_index(items: &[DropdownMenuItem]) -> Option<usize> {
    items.iter().rposition(|item| item.is_focusable())
}

/// Step focus by `delta` (±1) across focusable rows, wrapping and
/// skipping separators / disabled rows. Returns `None` when nothing is
/// focusable.
fn step_focusable(items: &[DropdownMenuItem], from: usize, delta: i32) -> Option<usize> {
    let len = items.len();
    if len == 0 || !items.iter().any(|item| item.is_focusable()) {
        return None;
    }
    let len_i = len as i32;
    let mut idx = from as i32;
    for _ in 0..len {
        idx = (idx + delta).rem_euclid(len_i);
        if items[idx as usize].is_focusable() {
            return Some(idx as usize);
        }
    }
    None
}

/// Typeahead: first focusable row whose lowercased label starts with the
/// lowercased `buffer`.
fn typeahead_index(items: &[DropdownMenuItem], buffer: &str) -> Option<usize> {
    if buffer.is_empty() {
        return None;
    }
    let needle = buffer.to_lowercase();
    items
        .iter()
        .position(|item| item.is_focusable() && item.label.to_lowercase().starts_with(&needle))
}

/// Move DOM focus to the menu button with id `{menu_id}-item-{item_id}`.
/// Mirrors `navigation::focus_tab`: only identifier-safe characters are
/// interpolated into the JS literal.
fn focus_menu_item(menu_id: &str, item_id: &str) {
    let safe = |s: &str| {
        s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ':' || c == '.')
    };
    if !safe(menu_id) || !safe(item_id) {
        return;
    }
    let _ = dioxus::document::eval(&format!(
        "const el = document.getElementById('{menu_id}-item-{item_id}'); if (el) el.focus();"
    ));
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
        assert!(item.is_focusable());
    }

    #[test]
    fn separator_item_is_disabled_divider() {
        let item = DropdownMenuItem::separator("div-1");
        assert!(item.separator);
        assert!(item.disabled);
        assert!(!item.is_focusable());
    }

    #[test]
    fn disabled_builder_disables_action_row() {
        let item = DropdownMenuItem::new("delete", "Delete").disabled();
        assert!(item.disabled);
        assert!(!item.separator);
        assert!(!item.is_focusable());
    }

    fn menu() -> Vec<DropdownMenuItem> {
        vec![
            DropdownMenuItem::new("rename", "Rename"),
            DropdownMenuItem::separator("div-1"),
            DropdownMenuItem::new("duplicate", "Duplicate"),
            DropdownMenuItem::new("delete", "Delete").disabled(),
            DropdownMenuItem::new("move", "Move"),
        ]
    }

    #[test]
    fn step_focusable_skips_separators_and_disabled() {
        let m = menu();
        // From "rename" (0) forward → "duplicate" (2), skipping the separator.
        assert_eq!(step_focusable(&m, 0, 1), Some(2));
        // From "duplicate" (2) forward → "move" (4), skipping disabled delete.
        assert_eq!(step_focusable(&m, 2, 1), Some(4));
        // From "move" (4) forward → wraps to "rename" (0).
        assert_eq!(step_focusable(&m, 4, 1), Some(0));
        // From "rename" (0) backward → wraps to "move" (4).
        assert_eq!(step_focusable(&m, 0, -1), Some(4));
    }

    #[test]
    fn first_and_last_focusable_indices() {
        let m = menu();
        assert_eq!(first_focusable_index(&m), Some(0));
        assert_eq!(last_focusable_index(&m), Some(4));
    }

    #[test]
    fn step_focusable_all_unfocusable_is_none() {
        let m = vec![
            DropdownMenuItem::separator("a"),
            DropdownMenuItem::new("b", "B").disabled(),
        ];
        assert_eq!(step_focusable(&m, 0, 1), None);
        assert_eq!(first_focusable_index(&m), None);
    }

    #[test]
    fn typeahead_matches_focusable_label_prefix() {
        let m = menu();
        assert_eq!(typeahead_index(&m, "du"), Some(2));
        assert_eq!(typeahead_index(&m, "mo"), Some(4));
        // "delete" is disabled, so a "de" prefix does not match it.
        assert_eq!(typeahead_index(&m, "de"), None);
        assert_eq!(typeahead_index(&m, ""), None);
    }
}
