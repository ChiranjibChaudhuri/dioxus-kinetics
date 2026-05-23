//! Keyboard-driven command palette with grouped items and arrow navigation.

use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandItem {
    pub id: String,
    pub label: String,
    pub description: String,
}

impl CommandItem {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: description.into(),
        }
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
    #[props(default)] open: bool,
    #[props(default)] query: String,
    #[props(default)] selected_id: String,
    #[props(default = "No commands found".to_string())] empty_text: String,
    #[props(default)] groups: Vec<CommandGroup>,
    on_query: Option<EventHandler<String>>,
    on_select: Option<EventHandler<String>>,
    on_selection_change: Option<EventHandler<String>>,
    on_dismiss: Option<EventHandler<()>>,
) -> Element {
    if !open {
        return rsx! {};
    }

    let has_items = groups.iter().any(|group| !group.items.is_empty());
    let active_descendant = if selected_id.is_empty() {
        String::new()
    } else {
        format!("ui-command-item-{selected_id}")
    };

    // Flatten visible item ids so arrow keys can step through them.
    let flat_ids: Vec<String> = groups
        .iter()
        .flat_map(|group| group.items.iter().map(|item| item.id.clone()))
        .collect();
    let flat_ids_for_key = flat_ids.clone();
    let selected_for_key = selected_id.clone();

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
            div { class: "ui-command-menu-panel",
                input {
                    class: "ui-command-menu-input",
                    value: "{query}",
                    placeholder: "Search commands",
                    "aria-label": "Search commands",
                    "aria-autocomplete": "list",
                    "aria-controls": "ui-command-menu-list",
                    "aria-activedescendant": "{active_descendant}",
                    autocomplete: "off",
                    oninput: move |evt| {
                        if let Some(handler) = &on_query {
                            handler.call(evt.value());
                        }
                    },
                }
                if has_items {
                    div {
                        id: "ui-command-menu-list",
                        class: "ui-command-menu-list",
                        role: "listbox",
                        for group in groups {
                            div { class: "ui-command-menu-group",
                                p { class: "ui-command-menu-group-label", "{group.label}" }
                                for item in group.items {
                                    {
                                        let item_id = item.id.clone();
                                        let is_selected = item.id == selected_id;
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
                                                onclick: move |_evt| {
                                                    if let Some(handler) = &on_select {
                                                        handler.call(item_id.clone());
                                                    }
                                                },
                                                strong { "{item.label}" }
                                                span { "{item.description}" }
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
