//! Keyboard-driven command palette with grouped items and arrow navigation.
//!
//! Rendered as a true modal spotlight: a dimmed `.ui-command-menu-backdrop`
//! sits behind a glass panel, focus is pulled into the search input on
//! mount, Tab is trapped inside the panel, and the opener is restored on
//! dismiss. Hover and keyboard selection are unified through
//! `on_selection_change`, the active row carries `data-active="true"`, and
//! a visually-hidden `aria-live` region announces the result count.

use dioxus::prelude::*;

use super::focus_trap;

/// Number of skeleton rows rendered while `loading` is true.
const SKELETON_ROW_COUNT: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandItem {
    pub id: String,
    pub label: String,
    pub description: String,
    /// Optional leading icon, expressed as the `d` attribute of an inline
    /// SVG `<path>`. Empty means no icon. Set via [`CommandItem::with_icon`].
    pub icon_path: String,
    /// Optional trailing keyboard shortcut hint (e.g. `"⌘K"`). Empty means
    /// none. Set via [`CommandItem::with_shortcut`].
    pub shortcut: String,
}

impl CommandItem {
    /// Constructs a command item. Signature is intentionally unchanged
    /// (3 args) so every existing call site keeps compiling; icon and
    /// shortcut are opt-in via the builder methods below.
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: description.into(),
            icon_path: String::new(),
            shortcut: String::new(),
        }
    }

    /// Attaches a leading icon (the `d` of an inline SVG path).
    pub fn with_icon(mut self, path: impl Into<String>) -> Self {
        self.icon_path = path.into();
        self
    }

    /// Attaches a trailing keyboard-shortcut hint.
    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = shortcut.into();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGroup {
    pub label: String,
    pub items: Vec<CommandItem>,
}

impl CommandGroup {
    pub fn new(label: impl Into<String>, items: Vec<CommandItem>) -> Self {
        Self {
            label: label.into(),
            items,
        }
    }
}

#[component]
pub fn CommandMenu(
    /// Stable id base for the palette. The panel derives `{id}-panel` and
    /// the listbox `{id}-list`. Defaults to `"ui-command-menu"`.
    #[props(default = "ui-command-menu".to_string())]
    id: String,
    #[props(default)] open: bool,
    #[props(default)] query: String,
    #[props(default)] selected_id: String,
    #[props(default = "No commands found".to_string())] empty_text: String,
    /// When true, render skeleton rows and mark the list `aria-busy`;
    /// `empty_text` is suppressed so a transient empty result during an
    /// async fetch does not flash "No commands found".
    #[props(default)]
    loading: bool,
    #[props(default)] groups: Vec<CommandGroup>,
    on_query: Option<EventHandler<String>>,
    on_select: Option<EventHandler<String>>,
    on_selection_change: Option<EventHandler<String>>,
    on_dismiss: Option<EventHandler<()>>,
) -> Element {
    // Hook must run on every render (Dioxus keys hooks by call order), so
    // it lives before the `!open` early return. It scrolls the selected
    // row into view whenever the selection changes while the menu is open.
    // `open`/`selected_id` are props (not signals), so `use_reactive`
    // re-runs the effect whenever either changes — closing or reselecting.
    use_effect(use_reactive(
        (&open, &selected_id),
        move |(open, selected)| {
            if !open || selected.is_empty() {
                return;
            }
            let _ = dioxus::document::eval(&build_scroll_into_view_script(&selected));
        },
    ));

    if !open {
        return rsx! {};
    }

    let panel_id = format!("{id}-panel");
    let list_id = format!("{id}-list");

    let item_count = count_items(&groups);
    let has_items = item_count > 0;
    let active_descendant = if selected_id.is_empty() {
        String::new()
    } else {
        format!("ui-command-item-{selected_id}")
    };
    let result_announcement = result_count_label(item_count, loading);

    // Flatten visible item ids so arrow keys can step through them.
    let flat_ids: Vec<String> = groups
        .iter()
        .flat_map(|group| group.items.iter().map(|item| item.id.clone()))
        .collect();
    let flat_ids_for_key = flat_ids.clone();
    let selected_for_key = selected_id.clone();

    let panel_attr = panel_id.clone();
    let panel_for_mount = panel_id.clone();
    let panel_for_dismiss = panel_id.clone();

    rsx! {
        div {
            class: "ui-command-menu",
            role: "dialog",
            "aria-modal": "true",
            onkeydown: move |evt| {
                let key = evt.key();
                match key {
                    Key::Escape => {
                        evt.stop_propagation();
                        focus_trap::restore_opener(&panel_for_dismiss);
                        if let Some(handler) = &on_dismiss {
                            handler.call(());
                        }
                    }
                    Key::Enter => {
                        if !selected_for_key.is_empty() {
                            if let Some(handler) = &on_select {
                                evt.prevent_default();
                                handler.call(selected_for_key.clone());
                            }
                        }
                    }
                    Key::ArrowDown => {
                        if let Some(handler) = &on_selection_change {
                            let next = step_selection(&flat_ids_for_key, &selected_for_key, 1);
                            if let Some(next_id) = next {
                                evt.prevent_default();
                                handler.call(next_id);
                            }
                        }
                    }
                    Key::ArrowUp => {
                        if let Some(handler) = &on_selection_change {
                            let next = step_selection(&flat_ids_for_key, &selected_for_key, -1);
                            if let Some(next_id) = next {
                                evt.prevent_default();
                                handler.call(next_id);
                            }
                        }
                    }
                    _ => {}
                }
            },
            div {
                class: "ui-command-menu-backdrop",
                onclick: move |_evt| {
                    focus_trap::restore_opener(&panel_id);
                    if let Some(handler) = &on_dismiss {
                        handler.call(());
                    }
                },
            }
            div {
                id: "{panel_attr}",
                class: "ui-command-menu-panel",
                "data-state": "open",
                onmounted: move |_evt| {
                    focus_trap::capture_opener(&panel_for_mount);
                    focus_trap::install_trap(&panel_for_mount);
                },
                input {
                    class: "ui-command-menu-input",
                    value: "{query}",
                    placeholder: "Search commands",
                    "aria-label": "Search commands",
                    "aria-autocomplete": "list",
                    "aria-controls": "{list_id}",
                    "aria-activedescendant": "{active_descendant}",
                    autocomplete: "off",
                    onmounted: move |evt| {
                        spawn(async move {
                            let _ = evt.set_focus(true).await;
                        });
                    },
                    oninput: move |evt| {
                        if let Some(handler) = &on_query {
                            handler.call(evt.value());
                        }
                    },
                }
                // Visually-hidden live region announcing the result count.
                div {
                    class: "visually-hidden",
                    role: "status",
                    "aria-live": "polite",
                    "aria-atomic": "true",
                    "{result_announcement}"
                }
                if loading {
                    div {
                        id: "{list_id}",
                        class: "ui-command-menu-list",
                        role: "listbox",
                        "aria-busy": "true",
                        for row in 0..SKELETON_ROW_COUNT {
                            div {
                                key: "skeleton-{row}",
                                class: "ui-command-menu-item ui-command-menu-item--skeleton",
                                "aria-hidden": "true",
                                span { class: "ui-skeleton ui-command-menu-skeleton-line" }
                            }
                        }
                    }
                } else if has_items {
                    div {
                        id: "{list_id}",
                        class: "ui-command-menu-list",
                        role: "listbox",
                        "aria-busy": "false",
                        for group in groups {
                            div { class: "ui-command-menu-group",
                                p { class: "ui-command-menu-group-label", "{group.label}" }
                                for item in group.items {
                                    {
                                        let item_id = item.id.clone();
                                        let hover_id = item.id.clone();
                                        let is_selected = item.id == selected_id;
                                        let has_icon = !item.icon_path.is_empty();
                                        let has_shortcut = !item.shortcut.is_empty();
                                        rsx! {
                                            div {
                                                id: "ui-command-item-{item.id}",
                                                class: if is_selected {
                                                    "ui-command-menu-item ui-command-menu-item--selected"
                                                } else {
                                                    "ui-command-menu-item"
                                                },
                                                role: "option",
                                                "aria-selected": if is_selected { "true" } else { "false" },
                                                "data-active": if is_selected { "true" } else { "false" },
                                                onclick: move |_evt| {
                                                    if let Some(handler) = &on_select {
                                                        handler.call(item_id.clone());
                                                    }
                                                },
                                                onmouseenter: move |_evt| {
                                                    if let Some(handler) = &on_selection_change {
                                                        handler.call(hover_id.clone());
                                                    }
                                                },
                                                if has_icon {
                                                    span {
                                                        class: "ui-command-menu-item-icon",
                                                        "aria-hidden": "true",
                                                        svg {
                                                            "viewBox": "0 0 24 24",
                                                            width: "16",
                                                            height: "16",
                                                            fill: "none",
                                                            stroke: "currentColor",
                                                            "stroke-width": "2",
                                                            "stroke-linecap": "round",
                                                            "stroke-linejoin": "round",
                                                            path { d: "{item.icon_path}" }
                                                        }
                                                    }
                                                }
                                                span { class: "ui-command-menu-item-body",
                                                    strong { "{item.label}" }
                                                    span { "{item.description}" }
                                                }
                                                if has_shortcut {
                                                    kbd {
                                                        class: "ui-command-menu-item-shortcut",
                                                        "{item.shortcut}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    p { class: "ui-command-menu-empty", "{empty_text}" }
                }
            }
        }
    }
}

/// Counts the total number of items across all groups.
fn count_items(groups: &[CommandGroup]) -> usize {
    groups.iter().map(|group| group.items.len()).sum()
}

/// Builds the visually-hidden live-region message announcing the current
/// result count (or a loading state).
fn result_count_label(count: usize, loading: bool) -> String {
    if loading {
        return "Loading commands".to_string();
    }
    match count {
        0 => "No results".to_string(),
        1 => "1 result".to_string(),
        n => format!("{n} results"),
    }
}

/// Builds the script that scrolls the selected command row into view with
/// `block: 'nearest'`, avoiding a jarring jump when arrow-keying through a
/// long list.
fn build_scroll_into_view_script(selected_id: &str) -> String {
    format!(
        r#"
        (function() {{
            const el = document.getElementById('ui-command-item-{selected_id}');
            if (el && typeof el.scrollIntoView === 'function') {{
                el.scrollIntoView({{ block: 'nearest' }});
            }}
        }})();
        "#,
        selected_id = selected_id,
    )
}

fn step_selection(ids: &[String], current: &str, delta: i32) -> Option<String> {
    if ids.is_empty() {
        return None;
    }
    let index = ids
        .iter()
        .position(|candidate| candidate == current)
        .map(|i| i as i32)
        .unwrap_or(if delta >= 0 { -1 } else { ids.len() as i32 });
    let len = ids.len() as i32;
    let next = ((index + delta) % len + len) % len;
    ids.get(next as usize).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(slice: &[&str]) -> Vec<String> {
        slice.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn command_item_new_keeps_three_arg_signature() {
        let item = CommandItem::new("open", "Open", "Open a file");
        assert_eq!(item.id, "open");
        assert_eq!(item.label, "Open");
        assert_eq!(item.description, "Open a file");
        // New fields default to empty so existing call sites are unaffected.
        assert!(item.icon_path.is_empty());
        assert!(item.shortcut.is_empty());
    }

    #[test]
    fn builders_attach_icon_and_shortcut() {
        let item = CommandItem::new("save", "Save", "Save the file")
            .with_icon("M5 12l5 5L20 7")
            .with_shortcut("⌘S");
        assert_eq!(item.icon_path, "M5 12l5 5L20 7");
        assert_eq!(item.shortcut, "⌘S");
    }

    #[test]
    fn count_items_sums_across_groups() {
        let groups = vec![
            CommandGroup::new(
                "File",
                vec![
                    CommandItem::new("a", "A", ""),
                    CommandItem::new("b", "B", ""),
                ],
            ),
            CommandGroup::new("Edit", vec![CommandItem::new("c", "C", "")]),
        ];
        assert_eq!(count_items(&groups), 3);
    }

    #[test]
    fn result_count_label_singular_plural_and_loading() {
        assert_eq!(result_count_label(0, false), "No results");
        assert_eq!(result_count_label(1, false), "1 result");
        assert_eq!(result_count_label(7, false), "7 results");
        // Loading takes precedence over the count.
        assert_eq!(result_count_label(0, true), "Loading commands");
        assert_eq!(result_count_label(3, true), "Loading commands");
    }

    #[test]
    fn scroll_script_targets_selected_row() {
        let script = build_scroll_into_view_script("save");
        assert!(script.contains("getElementById('ui-command-item-save')"));
        assert!(script.contains("block: 'nearest'"));
    }

    #[test]
    fn step_selection_wraps_forward_and_back() {
        let list = ids(&["a", "b", "c"]);
        assert_eq!(step_selection(&list, "a", 1).as_deref(), Some("b"));
        assert_eq!(step_selection(&list, "c", 1).as_deref(), Some("a"));
        assert_eq!(step_selection(&list, "a", -1).as_deref(), Some("c"));
    }

    #[test]
    fn step_selection_from_empty_current_picks_first_or_last() {
        let list = ids(&["a", "b", "c"]);
        assert_eq!(step_selection(&list, "", 1).as_deref(), Some("a"));
        assert_eq!(step_selection(&list, "", -1).as_deref(), Some("c"));
    }

    #[test]
    fn step_selection_empty_list_is_none() {
        assert_eq!(step_selection(&[], "x", 1), None);
    }
}
