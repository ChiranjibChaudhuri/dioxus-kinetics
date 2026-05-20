#![forbid(unsafe_code)]

use dioxus::prelude::*;
use ui_glass::{GlassDensity, GlassLevel, GlassTone};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
}

impl ButtonVariant {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Primary => "ui-button ui-button--primary",
            Self::Secondary => "ui-button ui-button--secondary",
            Self::Ghost => "ui-button ui-button--ghost",
            Self::Danger => "ui-button ui-button--danger",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] disabled: bool,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: "{variant.class_name()}",
            disabled,
            r#type: "button",
            {children}
        }
    }
}

#[component]
pub fn Surface(children: Element) -> Element {
    rsx! {
        section {
            class: "ui-surface",
            {children}
        }
    }
}

#[component]
pub fn GlassSurface(
    #[props(default)] level: GlassLevel,
    #[props(default)] tone: GlassTone,
    #[props(default)] density: GlassDensity,
    children: Element,
) -> Element {
    rsx! {
        section {
            class: "ui-glass-surface",
            "data-glass-level": glass_level_name(level),
            "data-glass-tone": glass_tone_name(tone),
            "data-glass-density": glass_density_name(density),
            {children}
        }
    }
}

#[component]
pub fn Stack(#[props(default = "md".to_string())] gap: String, children: Element) -> Element {
    rsx! {
        div {
            class: "ui-stack ui-stack--gap-{gap}",
            {children}
        }
    }
}

pub const fn glass_level_name(level: GlassLevel) -> &'static str {
    match level {
        GlassLevel::Subtle => "subtle",
        GlassLevel::Floating => "floating",
        GlassLevel::Overlay => "overlay",
        GlassLevel::Chrome => "chrome",
    }
}

pub const fn glass_tone_name(tone: GlassTone) -> &'static str {
    match tone {
        GlassTone::Neutral => "neutral",
        GlassTone::Primary => "primary",
        GlassTone::Success => "success",
        GlassTone::Warning => "warning",
        GlassTone::Danger => "danger",
        GlassTone::Info => "info",
    }
}

pub const fn glass_density_name(density: GlassDensity) -> &'static str {
    match density {
        GlassDensity::Compact => "compact",
        GlassDensity::Comfortable => "comfortable",
        GlassDensity::Spacious => "spacious",
    }
}
