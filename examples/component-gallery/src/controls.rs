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
        let theme = use_signal(|| DEFAULT_THEME);
        let density = use_signal(|| DEFAULT_DENSITY);
        let motion = use_signal(|| DEFAULT_MOTION);
        let glass = use_signal(|| DEFAULT_GLASS);
        Self { theme, density, motion, glass }
    }
}
