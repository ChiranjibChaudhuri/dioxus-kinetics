use dioxus::prelude::*;

/// One crumb in a `Breadcrumb` trail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreadcrumbItem {
    pub label: String,
    /// Optional anchor target. When empty, the crumb renders as a
    /// non-link span (typically the current page).
    pub href: String,
}

impl BreadcrumbItem {
    pub fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
        }
    }

    pub fn current(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: String::new(),
        }
    }
}

/// Hierarchical wayfinding trail. The last item is rendered as the
/// current location (no link, `aria-current="page"`); earlier items are
/// anchor links separated by a `›` divider.
#[component]
pub fn Breadcrumb(
    items: Vec<BreadcrumbItem>,
    #[props(default = "Breadcrumb".to_string())] aria_label: String,
) -> Element {
    if items.is_empty() {
        return rsx! {};
    }
    let last_idx = items.len() - 1;
    rsx! {
        nav { class: "ui-breadcrumb", "aria-label": "{aria_label}",
            ol { class: "ui-breadcrumb-list",
                for (idx, item) in items.iter().cloned().enumerate() {
                    li { class: "ui-breadcrumb-item",
                        if idx == last_idx || item.href.is_empty() {
                            span { class: "ui-breadcrumb-current", "aria-current": "page", "{item.label}" }
                        } else {
                            a { class: "ui-breadcrumb-link", href: "{item.href}", "{item.label}" }
                            span { class: "ui-breadcrumb-sep", "aria-hidden": "true", "›" }
                        }
                    }
                }
            }
        }
    }
}

/// One step in a `Stepper`. `id` is returned via `on_select`; `label` is
/// the visible text; `description` (optional) adds a small subtitle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StepperStep {
    pub id: String,
    pub label: String,
    pub description: String,
}

impl StepperStep {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: String::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

/// Multi-step workflow indicator. Marks each step `completed`, `active`,
/// or `upcoming` based on its index relative to the `current` id.
#[component]
pub fn Stepper(
    steps: Vec<StepperStep>,
    current: String,
    #[props(default = false)] vertical: bool,
    on_select: Option<EventHandler<String>>,
) -> Element {
    let active_idx = steps.iter().position(|s| s.id == current).unwrap_or(0);
    let direction_class = if vertical {
        "ui-stepper ui-stepper--vertical"
    } else {
        "ui-stepper ui-stepper--horizontal"
    };
    rsx! {
        nav { class: "{direction_class}", "aria-label": "Progress",
            ol { class: "ui-stepper-list",
                for (idx, step) in steps.iter().cloned().enumerate() {
                    {
                        let step_id = step.id.clone();
                        let is_active = idx == active_idx;
                        let is_complete = idx < active_idx;
                        let state_class = if is_active {
                            "ui-stepper-step ui-stepper-step--active"
                        } else if is_complete {
                            "ui-stepper-step ui-stepper-step--complete"
                        } else {
                            "ui-stepper-step ui-stepper-step--upcoming"
                        };
                        let state_text = if is_active {
                            "current step"
                        } else if is_complete {
                            "completed"
                        } else {
                            "upcoming"
                        };
                        rsx! {
                            li { class: "{state_class}",
                                button {
                                    class: "ui-stepper-trigger",
                                    r#type: "button",
                                    "aria-current": if is_active { "step" } else { "" },
                                    onclick: move |_| {
                                        if let Some(handler) = &on_select {
                                            handler.call(step_id.clone());
                                        }
                                    },
                                    span { class: "ui-stepper-marker", "aria-hidden": "true", "{idx + 1}" }
                                    span { class: "ui-stepper-body",
                                        strong { class: "ui-stepper-label", "{step.label}" }
                                        if !step.description.is_empty() {
                                            span { class: "ui-stepper-description", "{step.description}" }
                                        }
                                    }
                                    span { class: "visually-hidden", " — {state_text}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// One page button in a `Pagination` control.
fn pagination_button(page: u32, current: u32, on_select: &Option<EventHandler<u32>>) -> Element {
    let is_current = page == current;
    let class = if is_current {
        "ui-pagination-button ui-pagination-button--current"
    } else {
        "ui-pagination-button"
    };
    let on_select = *on_select;
    rsx! {
        li { class: "ui-pagination-item",
            button {
                class: "{class}",
                r#type: "button",
                "aria-current": if is_current { "page" } else { "" },
                "aria-label": "Page {page}",
                onclick: move |_| {
                    if let Some(handler) = &on_select {
                        handler.call(page);
                    }
                },
                "{page}"
            }
        }
    }
}

/// Offset-style pagination for data-heavy lists. Shows first, current
/// (with one neighbour on each side), and last pages; ellipses fill in
/// any gaps. Prev/Next buttons are always rendered (disabled at
/// boundaries).
#[component]
pub fn Pagination(
    page: u32,
    total_pages: u32,
    on_select: Option<EventHandler<u32>>,
    #[props(default = "Pagination".to_string())] aria_label: String,
) -> Element {
    if total_pages <= 1 {
        return rsx! {};
    }
    let current = page.clamp(1, total_pages);
    let visible = visible_pages(current, total_pages);

    rsx! {
        nav { class: "ui-pagination", "aria-label": "{aria_label}",
            ul { class: "ui-pagination-list",
                li { class: "ui-pagination-item",
                    button {
                        class: "ui-pagination-button ui-pagination-prev",
                        r#type: "button",
                        disabled: current == 1,
                        "aria-label": "Previous page",
                        onclick: {
                            let on_select = on_select;
                            move |_| {
                                if current > 1 {
                                    if let Some(handler) = &on_select {
                                        handler.call(current - 1);
                                    }
                                }
                            }
                        },
                        "‹"
                    }
                }
                {
                    let mut last_emitted: Option<u32> = None;
                    rsx! {
                        for p in visible.iter().copied() {
                            if let Some(prev) = last_emitted {
                                if p > prev + 1 {
                                    li { class: "ui-pagination-item ui-pagination-item--ellipsis",
                                        span { "aria-hidden": "true", "…" }
                                        span { class: "visually-hidden", "More pages" }
                                    }
                                }
                            }
                            {
                                last_emitted = Some(p);
                                pagination_button(p, current, &on_select)
                            }
                        }
                    }
                }
                li { class: "ui-pagination-item",
                    button {
                        class: "ui-pagination-button ui-pagination-next",
                        r#type: "button",
                        disabled: current == total_pages,
                        "aria-label": "Next page",
                        onclick: {
                            let on_select = on_select;
                            move |_| {
                                if current < total_pages {
                                    if let Some(handler) = &on_select {
                                        handler.call(current + 1);
                                    }
                                }
                            }
                        },
                        "›"
                    }
                }
            }
        }
    }
}

/// Pure helper: build the sorted, deduped set of pages the Pagination
/// control should render directly (first, current ± 1, last). Gaps
/// between adjacent entries become ellipses at render time.
fn visible_pages(current: u32, total: u32) -> Vec<u32> {
    if total == 0 {
        return Vec::new();
    }
    let mut visible: Vec<u32> = vec![1, total];
    for delta in -1i32..=1 {
        let p = current as i32 + delta;
        if (1..=total as i32).contains(&p) {
            visible.push(p as u32);
        }
    }
    visible.sort();
    visible.dedup();
    visible
}

/// One option in a `SegmentedControl`. The `value` is returned via
/// `on_select` when clicked; `label` is the visible text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SegmentItem {
    pub value: String,
    pub label: String,
}

impl SegmentItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// Mutually-exclusive choice picker rendered as a button group.
/// Functionally equivalent to a radio group but laid out as a single
/// connected control — useful for short, related options (e.g.,
/// view-mode switchers). Carries `role="radiogroup"` + per-item
/// `role="radio"` so assistive tech announces the pattern correctly.
#[component]
pub fn SegmentedControl(
    options: Vec<SegmentItem>,
    selected: String,
    #[props(default = String::new())] group_label: String,
    on_select: Option<EventHandler<String>>,
) -> Element {
    rsx! {
        div {
            class: "ui-segmented",
            role: "radiogroup",
            "aria-label": "{group_label}",
            for option in options {
                {
                    let is_selected = option.value == selected;
                    let class = if is_selected {
                        "ui-segmented-option ui-segmented-option--selected"
                    } else {
                        "ui-segmented-option"
                    };
                    let value = option.value.clone();
                    rsx! {
                        button {
                            class: "{class}",
                            role: "radio",
                            r#type: "button",
                            "aria-checked": if is_selected { "true" } else { "false" },
                            onclick: move |_| {
                                if let Some(handler) = &on_select {
                                    handler.call(value.clone());
                                }
                            },
                            "{option.label}"
                        }
                    }
                }
            }
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::{step_tab, visible_pages};

    #[test]
    fn visible_pages_short_run_returns_full_range() {
        assert_eq!(visible_pages(1, 1), vec![1]);
        assert_eq!(visible_pages(2, 3), vec![1, 2, 3]);
    }

    #[test]
    fn visible_pages_middle_keeps_first_neighbours_last() {
        assert_eq!(visible_pages(5, 10), vec![1, 4, 5, 6, 10]);
    }

    #[test]
    fn visible_pages_at_start_dedupes_first_with_neighbour() {
        assert_eq!(visible_pages(1, 10), vec![1, 2, 10]);
    }

    #[test]
    fn visible_pages_at_end_dedupes_last_with_neighbour() {
        assert_eq!(visible_pages(10, 10), vec![1, 9, 10]);
    }

    #[test]
    fn visible_pages_zero_total_is_empty() {
        assert!(visible_pages(0, 0).is_empty());
    }

    fn ids() -> Vec<String> {
        vec!["one".into(), "two".into(), "three".into()]
    }

    #[test]
    fn step_tab_moves_forward() {
        assert_eq!(step_tab(&ids(), "one", 1).as_deref(), Some("two"));
    }

    #[test]
    fn step_tab_wraps_to_first_after_last() {
        assert_eq!(step_tab(&ids(), "three", 1).as_deref(), Some("one"));
    }

    #[test]
    fn step_tab_wraps_backwards_from_first() {
        assert_eq!(step_tab(&ids(), "one", -1).as_deref(), Some("three"));
    }

    #[test]
    fn step_tab_unknown_current_starts_at_first_for_forward() {
        assert_eq!(step_tab(&ids(), "missing", 1).as_deref(), Some("one"));
    }

    #[test]
    fn step_tab_unknown_current_starts_at_last_for_backward() {
        assert_eq!(step_tab(&ids(), "missing", -1).as_deref(), Some("three"));
    }

    #[test]
    fn step_tab_empty_returns_none() {
        let empty: Vec<String> = Vec::new();
        assert!(step_tab(&empty, "anything", 1).is_none());
    }
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
