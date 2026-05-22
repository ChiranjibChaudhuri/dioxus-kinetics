use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabItem {
    pub value: String,
    pub label: String,
}

impl TabItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabPanel {
    pub value: String,
    pub content: String,
}

impl TabPanel {
    pub fn new(value: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            content: content.into(),
        }
    }
}

#[component]
pub fn Tabs(
    selected: String,
    items: Vec<TabItem>,
    panels: Vec<TabPanel>,
    onselect: Option<EventHandler<String>>,
) -> Element {
    let tab_ids: Vec<String> = items.iter().map(|item| item.value.clone()).collect();

    rsx! {
        div { class: "ui-tabs",
            div {
                class: "ui-tabs-list",
                role: "tablist",
                "aria-orientation": "horizontal",
                onkeydown: {
                    let ids = tab_ids.clone();
                    let selected_now = selected.clone();
                    move |evt: KeyboardEvent| {
                        let next = match evt.key() {
                            Key::ArrowRight => step_tab(&ids, &selected_now, 1),
                            Key::ArrowLeft => step_tab(&ids, &selected_now, -1),
                            Key::Home => ids.first().cloned(),
                            Key::End => ids.last().cloned(),
                            _ => None,
                        };
                        if let Some(next_id) = next {
                            evt.prevent_default();
                            evt.stop_propagation();
                            if let Some(handler) = &onselect {
                                handler.call(next_id.clone());
                            }
                            focus_tab(&next_id);
                        }
                    }
                },
                for item in items.iter() {
                    {
                        let value = item.value.clone();
                        let is_selected = item.value == selected;
                        let tabindex = if is_selected { "0" } else { "-1" };
                        rsx! {
                            button {
                                class: if is_selected { "ui-tab ui-tab--selected" } else { "ui-tab" },
                                r#type: "button",
                                role: "tab",
                                id: "tab-{item.value}",
                                "aria-controls": "panel-{item.value}",
                                "aria-selected": if is_selected { "true" } else { "false" },
                                tabindex: "{tabindex}",
                                onclick: move |_evt| {
                                    if let Some(handler) = &onselect {
                                        handler.call(value.clone());
                                    }
                                },
                                "{item.label}"
                            }
                        }
                    }
                }
            }
            for panel in panels.iter().filter(|panel| panel.value == selected) {
                div {
                    class: "ui-tab-panel",
                    role: "tabpanel",
                    id: "panel-{panel.value}",
                    "aria-labelledby": "tab-{panel.value}",
                    tabindex: "0",
                    "{panel.content}"
                }
            }
        }
    }
}

fn step_tab(ids: &[String], current: &str, delta: i32) -> Option<String> {
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

fn focus_tab(value: &str) {
    // Manual focus follows selection for the WAI-ARIA tab pattern. Allow only
    // identifier characters before interpolating into the JS literal.
    if !value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ':' || c == '.')
    {
        return;
    }
    let _ = dioxus::document::eval(&format!(
        "const el = document.getElementById('tab-{value}'); if (el) el.focus();"
    ));
}

#[component]
pub fn Toolbar(
    primary: Vec<String>,
    #[props(default)] secondary: String,
    onaction: Option<EventHandler<String>>,
) -> Element {
    rsx! {
        div { class: "ui-toolbar", role: "toolbar",
            div { class: "ui-toolbar-group ui-toolbar-group--primary",
                for command in primary {
                    {
                        let command_label = command.clone();
                        rsx! {
                            button {
                                class: "ui-button ui-button--secondary",
                                r#type: "button",
                                onclick: move |_evt| {
                                    if let Some(handler) = &onaction {
                                        handler.call(command_label.clone());
                                    }
                                },
                                "{command}"
                            }
                        }
                    }
                }
            }
            if !secondary.is_empty() {
                div { class: "ui-toolbar-secondary", "{secondary}" }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidebarItem {
    pub id: String,
    pub label: String,
    pub href: String,
}

impl SidebarItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            href: href.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidebarSection {
    pub label: String,
    pub items: Vec<SidebarItem>,
}

impl SidebarSection {
    pub fn new(label: impl Into<String>, items: Vec<SidebarItem>) -> Self {
        Self {
            label: label.into(),
            items,
        }
    }
}

#[component]
pub fn Sidebar(
    sections: Vec<SidebarSection>,
    #[props(default)] collapsed: bool,
    #[props(default)] selected: String,
    onnavigate: Option<EventHandler<String>>,
) -> Element {
    let class_name = if collapsed {
        "ui-sidebar ui-sidebar--collapsed"
    } else {
        "ui-sidebar"
    };

    rsx! {
        nav { class: "{class_name}", "aria-label": "Application navigation",
            for section in sections {
                div { class: "ui-sidebar-section",
                    h3 { class: "ui-sidebar-section-label", "{section.label}" }
                    for item in section.items {
                        {
                            let item_id = item.id.clone();
                            let is_selected = item.id == selected;
                            let link_class = if is_selected {
                                "ui-sidebar-link ui-sidebar-link--selected"
                            } else {
                                "ui-sidebar-link"
                            };
                            rsx! {
                                a {
                                    class: "{link_class}",
                                    href: "{item.href}",
                                    "aria-current": if is_selected { "page" } else { "false" },
                                    onclick: move |evt| {
                                        if let Some(handler) = &onnavigate {
                                            evt.prevent_default();
                                            handler.call(item_id.clone());
                                        }
                                    },
                                    "{item.label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
