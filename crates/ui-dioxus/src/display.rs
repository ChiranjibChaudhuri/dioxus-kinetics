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
    #[props(default)] sparkline_points: Vec<f32>,
) -> Element {
    let sparkline = build_sparkline_path(&sparkline_points);

    rsx! {
        article { class: "{tone.class_name()}",
            p { class: "ui-metric-card-label", "{label}" }
            strong { class: "ui-metric-card-value", "{value}" }
            if !delta.is_empty() {
                span { class: "ui-metric-card-delta", "{delta}" }
            }
            div { class: "ui-metric-card-sparkline", "aria-hidden": "true",
                if let Some(path) = sparkline {
                    svg {
                        view_box: "0 0 100 32",
                        preserve_aspect_ratio: "none",
                        path { d: "{path}", fill: "none", stroke: "currentColor", stroke_width: "1.5" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn EmptyState(
    title: String,
    description: String,
    #[props(default)] action_label: String,
    on_action: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        section { class: "ui-empty-state",
            div { class: "ui-empty-state-visual", "aria-hidden": "true" }
            h3 { class: "ui-empty-state-title", "{title}" }
            p { class: "ui-empty-state-description", "{description}" }
            if !action_label.is_empty() {
                button {
                    class: "ui-button ui-button--primary",
                    r#type: "button",
                    onclick: move |_evt| {
                        if let Some(handler) = &on_action {
                            handler.call(());
                        }
                    },
                    "{action_label}"
                }
            }
        }
    }
}

fn build_sparkline_path(points: &[f32]) -> Option<String> {
    if points.len() < 2 {
        return None;
    }
    let finite: Vec<f32> = points
        .iter()
        .copied()
        .map(|v| if v.is_finite() { v } else { 0.0 })
        .collect();
    let (mut min, mut max) = (f32::INFINITY, f32::NEG_INFINITY);
    for value in &finite {
        if *value < min {
            min = *value;
        }
        if *value > max {
            max = *value;
        }
    }
    if !min.is_finite() || !max.is_finite() {
        return None;
    }
    let span = (max - min).max(1e-3);
    let step = 100.0 / ((finite.len() - 1) as f32).max(1.0);

    let mut d = String::new();
    for (index, value) in finite.iter().enumerate() {
        let x = (index as f32) * step;
        // y axis is inverted in SVG: 0 at top, 32 at bottom; map highest value to top.
        let y = 30.0 - ((value - min) / span) * 28.0;
        if index == 0 {
            d.push_str(&format!("M{x:.2} {y:.2}"));
        } else {
            d.push_str(&format!(" L{x:.2} {y:.2}"));
        }
    }
    Some(d)
}
