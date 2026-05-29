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

/// Tone of a `Badge`, mapped to a CSS modifier class. `Neutral` (the
/// default) carries no modifier.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeTone {
    #[default]
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

impl BadgeTone {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Neutral => "ui-badge",
            Self::Primary => "ui-badge ui-badge--primary",
            Self::Success => "ui-badge ui-badge--success",
            Self::Warning => "ui-badge ui-badge--warning",
            Self::Danger => "ui-badge ui-badge--danger",
            Self::Info => "ui-badge ui-badge--info",
        }
    }
}

/// A small inline status pill. Neutral by default; pick a `tone` to
/// signal semantics (e.g. `Success` for "Active", `Danger` for "Down").
#[component]
pub fn Badge(#[props(default)] tone: BadgeTone, children: Element) -> Element {
    rsx! {
        span { class: "{tone.class_name()}", {children} }
    }
}

/// Diameter preset for an `Avatar`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl AvatarSize {
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

/// Derives up to two uppercase initials from a display name. Takes the
/// first letter of the first and last whitespace-separated words;
/// degrades to a single initial for one-word names and an empty string
/// for blank input.
fn initials(name: &str) -> String {
    let words: Vec<&str> = name.split_whitespace().collect();
    let first = words.first().and_then(|w| w.chars().next());
    let last = if words.len() > 1 {
        words.last().and_then(|w| w.chars().next())
    } else {
        None
    };

    let mut out = String::new();
    if let Some(c) = first {
        out.extend(c.to_uppercase());
    }
    if let Some(c) = last {
        out.extend(c.to_uppercase());
    }
    out
}

/// A circular user/entity avatar. Renders the image at `src` (with
/// `alt = name`) when one is provided, otherwise falls back to derived
/// initials with `aria-label = name` so the identity is still
/// announced.
#[component]
pub fn Avatar(
    name: String,
    #[props(default)] src: String,
    #[props(default)] size: AvatarSize,
) -> Element {
    let class = format!("ui-avatar ui-avatar--{}", size.class_suffix());

    rsx! {
        span { class: "{class}",
            if !src.is_empty() {
                img { class: "ui-avatar-image", src: "{src}", alt: "{name}" }
            } else {
                span {
                    class: "ui-avatar-initials",
                    "aria-label": "{name}",
                    "{initials(&name)}"
                }
            }
        }
    }
}

/// An indeterminate loading spinner. Exposes `role="status"` with an
/// `aria-label` so screen readers announce the loading state; the spin
/// animation is CSS-driven and gated by `prefers-reduced-motion` in the
/// host stylesheet.
#[component]
pub fn Spinner(#[props(default = "Loading…".to_string())] label: String) -> Element {
    rsx! {
        span {
            class: "ui-spinner",
            role: "status",
            "aria-label": "{label}",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn badge_tone_neutral_has_no_modifier() {
        assert_eq!(BadgeTone::Neutral.class_name(), "ui-badge");
        assert_eq!(
            BadgeTone::Primary.class_name(),
            "ui-badge ui-badge--primary"
        );
        assert_eq!(BadgeTone::Danger.class_name(), "ui-badge ui-badge--danger");
    }

    #[test]
    fn avatar_size_maps_to_suffix() {
        assert_eq!(AvatarSize::Sm.class_suffix(), "sm");
        assert_eq!(AvatarSize::Md.class_suffix(), "md");
        assert_eq!(AvatarSize::Lg.class_suffix(), "lg");
        assert_eq!(AvatarSize::default(), AvatarSize::Md);
    }

    #[test]
    fn initials_takes_first_and_last() {
        assert_eq!(initials("Ada Lovelace"), "AL");
        assert_eq!(initials("Grace Brewster Hopper"), "GH");
    }

    #[test]
    fn initials_single_word_is_one_letter() {
        assert_eq!(initials("Plato"), "P");
    }

    #[test]
    fn initials_blank_is_empty() {
        assert_eq!(initials(""), "");
        assert_eq!(initials("   "), "");
    }

    #[test]
    fn initials_uppercases() {
        assert_eq!(initials("ada lovelace"), "AL");
    }
}
