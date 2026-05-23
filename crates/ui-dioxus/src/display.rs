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

/// Tone of an `Alert` banner, mapped to a CSS modifier class. Danger and
/// Warning tones render as `role="alert"` (assertive announcement);
/// Neutral / Success / Info render as `role="status"` (polite live region).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertTone {
    #[default]
    Neutral,
    Success,
    Warning,
    Danger,
    Info,
}

impl AlertTone {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Neutral => "ui-alert ui-alert--neutral",
            Self::Success => "ui-alert ui-alert--success",
            Self::Warning => "ui-alert ui-alert--warning",
            Self::Danger => "ui-alert ui-alert--danger",
            Self::Info => "ui-alert ui-alert--info",
        }
    }

    pub const fn role(self) -> &'static str {
        match self {
            Self::Danger | Self::Warning => "alert",
            _ => "status",
        }
    }
}

/// A page-level message banner. Unlike `Toast`, Alert is non-dismissible
/// by default and persists in the layout (no auto-timeout). Set
/// `dismissible: true` and provide `on_dismiss` to opt into a close
/// button.
#[component]
pub fn Alert(
    title: String,
    #[props(default)] tone: AlertTone,
    #[props(default)] description: String,
    #[props(default)] dismissible: bool,
    #[props(default = "Dismiss".to_string())] dismiss_label: String,
    on_dismiss: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        div { class: "{tone.class_name()}", role: "{tone.role()}",
            div { class: "ui-alert-content",
                strong { class: "ui-alert-title", "{title}" }
                if !description.is_empty() {
                    p { class: "ui-alert-description", "{description}" }
                }
            }
            if dismissible {
                button {
                    class: "ui-button ui-button--ghost ui-alert-dismiss",
                    r#type: "button",
                    "aria-label": "{dismiss_label}",
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

/// A determinate progress bar with an optional `value` in [0.0, 1.0].
/// When `value` is `None`, renders an indeterminate spinner-style bar
/// (CSS-animated; respects `prefers-reduced-motion` via the host stylesheet).
#[component]
pub fn Progress(
    #[props(default)] label: String,
    #[props(default)] value: Option<f32>,
    #[props(default)] description: String,
) -> Element {
    let pct = value.map(|v| (v.clamp(0.0, 1.0) * 100.0).round() as u32);
    let (class, value_attr, value_text) = match pct {
        Some(p) => (
            "ui-progress ui-progress--determinate",
            format!("{p}"),
            format!("{p}%"),
        ),
        None => (
            "ui-progress ui-progress--indeterminate",
            String::new(),
            "Loading…".to_string(),
        ),
    };

    rsx! {
        div { class: "{class}",
            if !label.is_empty() {
                div { class: "ui-progress-label", "{label}" }
            }
            div {
                class: "ui-progress-track",
                role: "progressbar",
                "aria-valuemin": "0",
                "aria-valuemax": "100",
                "aria-valuenow": "{value_attr}",
                "aria-valuetext": "{value_text}",
                "aria-label": if label.is_empty() { "Progress" } else { "" },
                div {
                    class: "ui-progress-fill",
                    style: if let Some(p) = pct { format!("width:{p}%") } else { String::new() },
                }
            }
            if !description.is_empty() {
                p { class: "ui-progress-description", "{description}" }
            }
        }
    }
}

/// A loading placeholder rendered as a neutral pulsing block. Pair with
/// `Progress` for explicit progress indicators; `Skeleton` is for content
/// shape preservation while data loads.
#[component]
pub fn Skeleton(
    #[props(default = "1em".to_string())] height: String,
    #[props(default = "100%".to_string())] width: String,
    #[props(default = "4px".to_string())] radius: String,
) -> Element {
    let style = format!("height:{height};width:{width};border-radius:{radius};");
    rsx! {
        div {
            class: "ui-skeleton",
            style: "{style}",
            "aria-hidden": "true",
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
