use dioxus::prelude::*;

#[component]
pub fn Dialog(
    title: String,
    #[props(default)] open: bool,
    #[props(default)] description: String,
    #[props(default)] body: String,
    #[props(default)] actions: Vec<String>,
    #[props(default = true)] dismissible: bool,
    on_dismiss: Option<EventHandler<()>>,
    on_action: Option<EventHandler<String>>,
) -> Element {
    if !open {
        return rsx! {};
    }

    let has_description = !description.is_empty();
    let described_by = if has_description {
        "ui-dialog-description"
    } else {
        ""
    };

    rsx! {
        div {
            class: "ui-dialog",
            role: "dialog",
            "aria-modal": "true",
            "aria-labelledby": "ui-dialog-title",
            "aria-describedby": "{described_by}",
            onkeydown: move |evt| {
                if dismissible && evt.key() == Key::Escape {
                    evt.stop_propagation();
                    if let Some(handler) = &on_dismiss {
                        handler.call(());
                    }
                }
            },
            div {
                class: "ui-dialog-backdrop",
                onclick: move |_evt| {
                    if dismissible {
                        if let Some(handler) = &on_dismiss {
                            handler.call(());
                        }
                    }
                },
            }
            div {
                class: "ui-dialog-panel",
                tabindex: "-1",
                onmounted: move |evt| {
                    spawn(async move {
                        let _ = evt.set_focus(true).await;
                    });
                },
                h2 { id: "ui-dialog-title", class: "ui-dialog-title", "{title}" }
                if has_description {
                    p { id: "ui-dialog-description", class: "ui-dialog-description", "{description}" }
                }
                if !body.is_empty() {
                    div { class: "ui-dialog-body", "{body}" }
                }
                if !actions.is_empty() {
                    div { class: "ui-dialog-actions",
                        for action in actions {
                            {
                                let action_id = action.clone();
                                rsx! {
                                    button {
                                        class: "ui-button ui-button--secondary",
                                        r#type: "button",
                                        onclick: move |_evt| {
                                            if let Some(handler) = &on_action {
                                                handler.call(action_id.clone());
                                            }
                                        },
                                        "{action}"
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastTone {
    #[default]
    Neutral,
    Success,
    Warning,
    Danger,
    Info,
}

impl ToastTone {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Neutral => "ui-toast ui-toast--neutral",
            Self::Success => "ui-toast ui-toast--success",
            Self::Warning => "ui-toast ui-toast--warning",
            Self::Danger => "ui-toast ui-toast--danger",
            Self::Info => "ui-toast ui-toast--info",
        }
    }

    pub const fn role(self) -> &'static str {
        match self {
            Self::Danger | Self::Warning => "alert",
            _ => "status",
        }
    }
}

#[component]
pub fn Toast(
    title: String,
    #[props(default)] tone: ToastTone,
    #[props(default)] description: String,
    #[props(default)] action_label: String,
    #[props(default)] dismiss_label: String,
    on_action: Option<EventHandler<()>>,
    on_dismiss: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        div { class: "{tone.class_name()}", role: "{tone.role()}",
            div { class: "ui-toast-content",
                strong { class: "ui-toast-title", "{title}" }
                if !description.is_empty() {
                    p { class: "ui-toast-description", "{description}" }
                }
            }
            if !action_label.is_empty() || !dismiss_label.is_empty() {
                div { class: "ui-toast-actions",
                    if !action_label.is_empty() {
                        button {
                            class: "ui-button ui-button--secondary",
                            r#type: "button",
                            onclick: move |_evt| {
                                if let Some(handler) = &on_action {
                                    handler.call(());
                                }
                            },
                            "{action_label}"
                        }
                    }
                    if !dismiss_label.is_empty() {
                        button {
                            class: "ui-button ui-button--ghost",
                            r#type: "button",
                            onclick: move |_evt| {
                                if let Some(handler) = &on_dismiss {
                                    handler.call(());
                                }
                            },
                            "{dismiss_label}"
                        }
                    }
                }
            }
        }
    }
}

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

#[component]
pub fn Tooltip(id: String, visible: bool, trigger_label: String, content: String) -> Element {
    let described_by = if visible { id.clone() } else { String::new() };

    rsx! {
        span { class: "ui-tooltip",
            span {
                class: "ui-tooltip-trigger",
                "aria-describedby": "{described_by}",
                "{trigger_label}"
            }
            if visible {
                span { id: "{id}", class: "ui-tooltip-content", role: "tooltip", "{content}" }
            }
        }
    }
}
