use dioxus::prelude::*;

#[component]
pub fn Dialog(
    title: String,
    #[props(default)] open: bool,
    #[props(default)] description: String,
    #[props(default)] body: String,
    #[props(default)] actions: Vec<String>,
    #[props(default)] on_dismiss: Option<EventHandler<()>>,
    #[props(default)] on_action: Option<EventHandler<String>>,
) -> Element {
    if !open {
        return rsx! {};
    }

    rsx! {
        div { class: "ui-dialog", role: "dialog", "aria-modal": "true", "aria-labelledby": "ui-dialog-title",
            div {
                class: "ui-dialog-backdrop",
                onclick: move |_| {
                    if let Some(handler) = on_dismiss.as_ref() {
                        handler.call(());
                    }
                },
            }
            div { class: "ui-dialog-panel",
                h2 { id: "ui-dialog-title", class: "ui-dialog-title", "{title}" }
                if !description.is_empty() {
                    p { class: "ui-dialog-description", "{description}" }
                }
                if !body.is_empty() {
                    div { class: "ui-dialog-body", "{body}" }
                }
                if !actions.is_empty() {
                    div { class: "ui-dialog-actions",
                        for action in actions {
                            button {
                                class: "ui-button ui-button--secondary",
                                r#type: "button",
                                onclick: {
                                    let action = action.clone();
                                    move |_| {
                                        if let Some(handler) = on_action.as_ref() {
                                            handler.call(action.clone());
                                        }
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
                        button { class: "ui-button ui-button--secondary", r#type: "button", "{action_label}" }
                    }
                    if !dismiss_label.is_empty() {
                        button { class: "ui-button ui-button--ghost", r#type: "button", "{dismiss_label}" }
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
) -> Element {
    if !open {
        return rsx! {};
    }

    let has_items = groups.iter().any(|group| !group.items.is_empty());

    rsx! {
        div { class: "ui-command-menu", role: "dialog", "aria-modal": "true",
            div { class: "ui-command-menu-panel",
                input {
                    class: "ui-command-menu-input",
                    value: "{query}",
                    placeholder: "Search commands",
                    "aria-label": "Search commands",
                }
                if has_items {
                    div { class: "ui-command-menu-list", role: "listbox",
                        for group in groups {
                            div { class: "ui-command-menu-group",
                                p { class: "ui-command-menu-group-label", "{group.label}" }
                                for item in group.items {
                                    div {
                                        class: "ui-command-menu-item",
                                        role: "option",
                                        "aria-selected": if item.id == selected_id { "true" } else { "false" },
                                        strong { "{item.label}" }
                                        span { "{item.description}" }
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
