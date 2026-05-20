#![forbid(unsafe_code)]

use ui_tokens::{Color, Theme, TransparencyPreference};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassLevel {
    Subtle,
    Floating,
    Overlay,
    Chrome,
}

impl Default for GlassLevel {
    fn default() -> Self {
        Self::Subtle
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassTone {
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

impl Default for GlassTone {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassDensity {
    Compact,
    Comfortable,
    Spacious,
}

impl Default for GlassDensity {
    fn default() -> Self {
        Self::Comfortable
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassPolicy {
    Auto,
    SolidFallback,
    HighContrast,
    ReducedTransparency,
}

impl Default for GlassPolicy {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassRequest {
    pub level: GlassLevel,
    pub tone: GlassTone,
    pub density: GlassDensity,
    pub policy: GlassPolicy,
}

impl GlassRequest {
    pub const fn new(level: GlassLevel, tone: GlassTone, density: GlassDensity) -> Self {
        Self {
            level,
            tone,
            density,
            policy: GlassPolicy::Auto,
        }
    }

    pub const fn with_policy(mut self, policy: GlassPolicy) -> Self {
        self.policy = policy;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassRecipe {
    pub background: Color,
    pub fallback_background: Color,
    pub foreground: Color,
    pub border: Color,
    pub focus_ring: Color,
    pub inner_highlight: Color,
    pub shadow_alpha: f32,
    pub backdrop_blur_px: f32,
    pub saturate_percent: u16,
    pub radius_px: f32,
    pub force_solid: bool,
}

pub fn resolve_glass(theme: &Theme, request: GlassRequest) -> GlassRecipe {
    let force_solid = matches!(
        request.policy,
        GlassPolicy::SolidFallback | GlassPolicy::HighContrast | GlassPolicy::ReducedTransparency
    ) || theme.transparency == TransparencyPreference::Reduce;

    let tone = tone_color(theme, request.tone);
    let (blur, alpha, shadow_alpha) = match request.level {
        GlassLevel::Subtle => (10.0, 0.64, 0.10),
        GlassLevel::Floating => (18.0, 0.72, 0.16),
        GlassLevel::Overlay => (24.0, 0.80, 0.22),
        GlassLevel::Chrome => (28.0, 0.68, 0.18),
    };
    let radius_px = match request.density {
        GlassDensity::Compact => theme.radius.small_px,
        GlassDensity::Comfortable => theme.radius.medium_px,
        GlassDensity::Spacious => theme.radius.large_px,
    };

    GlassRecipe {
        background: if force_solid {
            theme.semantic.surface_solid
        } else {
            tone.with_alpha(alpha)
        },
        fallback_background: theme.semantic.surface_solid,
        foreground: theme.semantic.foreground,
        border: theme.semantic.border,
        focus_ring: theme.semantic.focus,
        inner_highlight: Color::rgba(255, 255, 255, if force_solid { 0.0 } else { 0.38 }),
        shadow_alpha,
        backdrop_blur_px: if force_solid { 0.0 } else { blur },
        saturate_percent: if force_solid { 100 } else { 160 },
        radius_px,
        force_solid,
    }
}

fn tone_color(theme: &Theme, tone: GlassTone) -> Color {
    match tone {
        GlassTone::Neutral => theme.semantic.surface,
        GlassTone::Primary => theme.semantic.primary,
        GlassTone::Success => theme.semantic.success,
        GlassTone::Warning => theme.semantic.warning,
        GlassTone::Danger => theme.semantic.danger,
        GlassTone::Info => theme.semantic.info,
    }
}
