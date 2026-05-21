use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MetricTone {
    #[default]
    Neutral,
    Success,
    Warning,
    Danger,
    Info,
}

impl MetricTone {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Neutral => "ui-metric-card ui-metric-card--neutral",
            Self::Success => "ui-metric-card ui-metric-card--success",
            Self::Warning => "ui-metric-card ui-metric-card--warning",
            Self::Danger => "ui-metric-card ui-metric-card--danger",
            Self::Info => "ui-metric-card ui-metric-card--info",
        }
    }
}

#[component]
pub fn MetricCard(
    label: String,
    value: String,
    #[props(default)] delta: String,
    #[props(default)] tone: MetricTone,
) -> Element {
    rsx! {
        article { class: "{tone.class_name()}",
            p { class: "ui-metric-card-label", "{label}" }
            strong { class: "ui-metric-card-value", "{value}" }
            if !delta.is_empty() {
                span { class: "ui-metric-card-delta", "{delta}" }
            }
            div { class: "ui-metric-card-sparkline", "aria-hidden": "true" }
        }
    }
}

#[component]
pub fn EmptyState(
    title: String,
    description: String,
    #[props(default)] action_label: String,
) -> Element {
    rsx! {
        section { class: "ui-empty-state",
            div { class: "ui-empty-state-visual", "aria-hidden": "true" }
            h3 { class: "ui-empty-state-title", "{title}" }
            p { class: "ui-empty-state-description", "{description}" }
            if !action_label.is_empty() {
                button { class: "ui-button ui-button--primary", r#type: "button", "{action_label}" }
            }
        }
    }
}
