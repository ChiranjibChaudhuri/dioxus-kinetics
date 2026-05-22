#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemePref {
    Light,
    Dark,
}

impl ThemePref {
    pub const fn attr_value(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    pub fn from_attr(value: &str) -> Option<Self> {
        match value {
            "light" => Some(Self::Light),
            "dark" => Some(Self::Dark),
            _ => None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DensityPref {
    Compact,
    Comfortable,
    Spacious,
}

impl DensityPref {
    pub const fn attr_value(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Comfortable => "comfortable",
            Self::Spacious => "spacious",
        }
    }

    pub fn from_attr(value: &str) -> Option<Self> {
        match value {
            "compact" => Some(Self::Compact),
            "comfortable" => Some(Self::Comfortable),
            "spacious" => Some(Self::Spacious),
            _ => None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Compact => "Compact",
            Self::Comfortable => "Comfortable",
            Self::Spacious => "Spacious",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionPref {
    Normal,
    Reduced,
}

impl MotionPref {
    pub const fn attr_value(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Reduced => "reduced",
        }
    }

    pub fn from_attr(value: &str) -> Option<Self> {
        match value {
            "normal" => Some(Self::Normal),
            "reduced" => Some(Self::Reduced),
            _ => None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Reduced => "Reduced",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassPolicyUi {
    Translucent,
    Solid,
}

impl GlassPolicyUi {
    pub const fn attr_value(self) -> &'static str {
        match self {
            Self::Translucent => "translucent",
            Self::Solid => "solid",
        }
    }

    pub fn from_attr(value: &str) -> Option<Self> {
        match value {
            "translucent" => Some(Self::Translucent),
            "solid" => Some(Self::Solid),
            _ => None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Translucent => "Translucent",
            Self::Solid => "Solid",
        }
    }
}

use dioxus::prelude::*;

pub const DEFAULT_THEME: ThemePref = ThemePref::Light;
pub const DEFAULT_DENSITY: DensityPref = DensityPref::Comfortable;
pub const DEFAULT_MOTION: MotionPref = MotionPref::Normal;
pub const DEFAULT_GLASS: GlassPolicyUi = GlassPolicyUi::Translucent;

#[derive(Clone, Copy)]
pub struct GalleryPrefs {
    pub theme: Signal<ThemePref>,
    pub density: Signal<DensityPref>,
    pub motion: Signal<MotionPref>,
    pub glass: Signal<GlassPolicyUi>,
}

impl GalleryPrefs {
    pub fn use_provided() -> Self {
        use crate::persistence::{self, KEY_DENSITY, KEY_GLASS, KEY_MOTION, KEY_THEME};

        let initial_theme = persistence::load(KEY_THEME)
            .and_then(|v| ThemePref::from_attr(&v))
            .unwrap_or(DEFAULT_THEME);
        let initial_density = persistence::load(KEY_DENSITY)
            .and_then(|v| DensityPref::from_attr(&v))
            .unwrap_or(DEFAULT_DENSITY);
        let initial_motion = persistence::load(KEY_MOTION)
            .and_then(|v| MotionPref::from_attr(&v))
            .unwrap_or_else(|| {
                if persistence::prefers_reduced_motion() {
                    MotionPref::Reduced
                } else {
                    DEFAULT_MOTION
                }
            });
        let initial_glass = persistence::load(KEY_GLASS)
            .and_then(|v| GlassPolicyUi::from_attr(&v))
            .unwrap_or(DEFAULT_GLASS);

        let theme = use_signal(|| initial_theme);
        let density = use_signal(|| initial_density);
        let motion = use_signal(|| initial_motion);
        let glass = use_signal(|| initial_glass);

        use_effect(move || {
            persistence::save(KEY_THEME, theme.read().attr_value());
        });
        use_effect(move || {
            persistence::save(KEY_DENSITY, density.read().attr_value());
        });
        use_effect(move || {
            persistence::save(KEY_MOTION, motion.read().attr_value());
        });
        use_effect(move || {
            persistence::save(KEY_GLASS, glass.read().attr_value());
        });

        Self { theme, density, motion, glass }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ToggleGroupProps {
    pub label: &'static str,
    pub options: Vec<(&'static str, &'static str, bool)>,
    pub on_select: EventHandler<&'static str>,
}

#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    rsx! {
        div { class: "gallery-toggle-group", role: "radiogroup", "aria-label": "{props.label}",
            span { class: "gallery-control-label", "{props.label}" }
            for (value, label, selected) in props.options.iter().copied() {
                button {
                    class: if selected { "ui-button ui-button--primary" } else { "ui-button ui-button--secondary" },
                    role: "radio",
                    "aria-checked": "{selected}",
                    r#type: "button",
                    onclick: move |_| props.on_select.call(value),
                    "{label}"
                }
            }
        }
    }
}

#[component]
pub fn PreferenceBar() -> Element {
    let prefs = use_context::<GalleryPrefs>();
    let mut theme_sig = prefs.theme;
    let mut density_sig = prefs.density;
    let mut motion_sig = prefs.motion;
    let mut glass_sig = prefs.glass;

    let theme_now = *theme_sig.read();
    let density_now = *density_sig.read();
    let motion_now = *motion_sig.read();
    let glass_now = *glass_sig.read();

    rsx! {
        section { class: "gallery-controls", "aria-label": "Preview settings",
            ToggleGroup {
                label: "Theme",
                options: vec![
                    ("light", ThemePref::Light.label(), theme_now == ThemePref::Light),
                    ("dark", ThemePref::Dark.label(), theme_now == ThemePref::Dark),
                ],
                on_select: move |v: &str| {
                    if let Some(next) = ThemePref::from_attr(v) {
                        theme_sig.set(next);
                    }
                },
            }
            ToggleGroup {
                label: "Density",
                options: vec![
                    ("compact", DensityPref::Compact.label(), density_now == DensityPref::Compact),
                    ("comfortable", DensityPref::Comfortable.label(), density_now == DensityPref::Comfortable),
                    ("spacious", DensityPref::Spacious.label(), density_now == DensityPref::Spacious),
                ],
                on_select: move |v: &str| {
                    if let Some(next) = DensityPref::from_attr(v) {
                        density_sig.set(next);
                    }
                },
            }
            ToggleGroup {
                label: "Motion",
                options: vec![
                    ("normal", MotionPref::Normal.label(), motion_now == MotionPref::Normal),
                    ("reduced", MotionPref::Reduced.label(), motion_now == MotionPref::Reduced),
                ],
                on_select: move |v: &str| {
                    if let Some(next) = MotionPref::from_attr(v) {
                        motion_sig.set(next);
                    }
                },
            }
            ToggleGroup {
                label: "Glass",
                options: vec![
                    ("translucent", GlassPolicyUi::Translucent.label(), glass_now == GlassPolicyUi::Translucent),
                    ("solid", GlassPolicyUi::Solid.label(), glass_now == GlassPolicyUi::Solid),
                ],
                on_select: move |v: &str| {
                    if let Some(next) = GlassPolicyUi::from_attr(v) {
                        glass_sig.set(next);
                    }
                },
            }
        }
    }
}
