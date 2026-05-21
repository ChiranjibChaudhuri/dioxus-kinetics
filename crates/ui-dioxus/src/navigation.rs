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
pub fn Tabs(selected: String, items: Vec<TabItem>, panels: Vec<TabPanel>) -> Element {
    rsx! {
        div { class: "ui-tabs",
            div { class: "ui-tabs-list", role: "tablist",
                for item in items.iter() {
                    button {
                        class: "ui-tab",
                        r#type: "button",
                        role: "tab",
                        id: "tab-{item.value}",
                        "aria-controls": "panel-{item.value}",
                        "aria-selected": if item.value == selected { "true" } else { "false" },
                        "{item.label}"
                    }
                }
            }
            for panel in panels.iter().filter(|panel| panel.value == selected) {
                div {
                    class: "ui-tab-panel",
                    role: "tabpanel",
                    id: "panel-{panel.value}",
                    "aria-labelledby": "tab-{panel.value}",
                    "{panel.content}"
                }
            }
        }
    }
}

#[component]
pub fn Toolbar(primary: Vec<String>, #[props(default)] secondary: String) -> Element {
    rsx! {
        div { class: "ui-toolbar", role: "toolbar",
            div { class: "ui-toolbar-group ui-toolbar-group--primary",
                for command in primary {
                    button { class: "ui-button ui-button--secondary", r#type: "button", "{command}" }
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
                        a {
                            class: "ui-sidebar-link",
                            href: "{item.href}",
                            "aria-current": if item.id == selected { "page" } else { "false" },
                            "{item.label}"
                        }
                    }
                }
            }
        }
    }
}
