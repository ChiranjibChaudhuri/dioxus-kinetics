#![forbid(unsafe_code)]

use ui_tokens::{Color, Theme, TransparencyPreference};

bitflags::bitflags! {
    /// Per-trait toggles for the glass uber-shader. Each bit corresponds to a
    /// WGSL `override` specialization constant in `compose.wgsl`. Pipelines are
    /// cached keyed by the feature set, so a surface with only `BLUR | TINT_ADAPT`
    /// runs a pipeline where every other branch is eliminated at compile time.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct GlassFeatures: u32 {
        const BLUR         = 1 << 0;
        const REFRACT      = 1 << 1;
        const DISPERSE     = 1 << 2;
        const SPECULAR     = 1 << 3;
        const INNER_SHADOW = 1 << 4;
        const AMBIENT_MESH = 1 << 5;
        const POINTER      = 1 << 6;
        const SCROLL       = 1 << 7;
        const TINT_ADAPT   = 1 << 8;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GlassLevel {
    #[default]
    Subtle,
    Floating,
    Overlay,
    Chrome,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GlassDepth {
    Inline,
    Raised,
    #[default]
    Floating,
    Chrome,
    Overlay,
    Modal,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GlassTone {
    #[default]
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GlassDensity {
    Compact,
    #[default]
    Comfortable,
    Spacious,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialTone {
    #[default]
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialDensity {
    Compact,
    #[default]
    Comfortable,
    Spacious,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialEdge {
    None,
    #[default]
    Hairline,
    Standard,
    Emphasized,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialVibrancy {
    Muted,
    #[default]
    Standard,
    Vivid,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GlassPolicy {
    #[default]
    Auto,
    SolidFallback,
    HighContrast,
    ReducedTransparency,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialPolicy {
    #[default]
    Auto,
    SolidFallback,
    ReducedTransparency,
    HighContrast,
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
pub struct MaterialRequest {
    pub depth: GlassDepth,
    pub tone: MaterialTone,
    pub density: MaterialDensity,
    pub edge: MaterialEdge,
    pub vibrancy: MaterialVibrancy,
    pub policy: MaterialPolicy,
}

impl MaterialRequest {
    pub const fn new(depth: GlassDepth, tone: MaterialTone) -> Self {
        Self {
            depth,
            tone,
            density: MaterialDensity::Comfortable,
            edge: MaterialEdge::Hairline,
            vibrancy: MaterialVibrancy::Standard,
            policy: MaterialPolicy::Auto,
        }
    }

    pub const fn with_density(mut self, density: MaterialDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_edge(mut self, edge: MaterialEdge) -> Self {
        self.edge = edge;
        self
    }

    pub const fn with_vibrancy(mut self, vibrancy: MaterialVibrancy) -> Self {
        self.vibrancy = vibrancy;
        self
    }

    pub const fn with_policy(mut self, policy: MaterialPolicy) -> Self {
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

pub fn resolve_material(theme: &Theme, request: MaterialRequest) -> GlassRecipe {
    let level = match request.depth {
        GlassDepth::Inline | GlassDepth::Raised => GlassLevel::Subtle,
        GlassDepth::Floating => GlassLevel::Floating,
        GlassDepth::Chrome => GlassLevel::Chrome,
        GlassDepth::Overlay | GlassDepth::Modal => GlassLevel::Overlay,
    };
    let tone = match request.tone {
        MaterialTone::Neutral => GlassTone::Neutral,
        MaterialTone::Primary => GlassTone::Primary,
        MaterialTone::Success => GlassTone::Success,
        MaterialTone::Warning => GlassTone::Warning,
        MaterialTone::Danger => GlassTone::Danger,
        MaterialTone::Info => GlassTone::Info,
    };
    let density = match request.density {
        MaterialDensity::Compact => GlassDensity::Compact,
        MaterialDensity::Comfortable => GlassDensity::Comfortable,
        MaterialDensity::Spacious => GlassDensity::Spacious,
    };
    let policy = match request.policy {
        MaterialPolicy::Auto => GlassPolicy::Auto,
        MaterialPolicy::SolidFallback => GlassPolicy::SolidFallback,
        MaterialPolicy::ReducedTransparency => GlassPolicy::ReducedTransparency,
        MaterialPolicy::HighContrast => GlassPolicy::HighContrast,
    };

    let mut recipe = resolve_glass(
        theme,
        GlassRequest::new(level, tone, density).with_policy(policy),
    );

    if !recipe.force_solid {
        let (blur, alpha, shadow_alpha) = match request.depth {
            GlassDepth::Inline => (8.0, 0.58, 0.06),
            GlassDepth::Raised => (12.0, 0.64, 0.10),
            GlassDepth::Floating => (18.0, 0.72, 0.16),
            GlassDepth::Chrome => (28.0, 0.68, 0.18),
            GlassDepth::Overlay => (24.0, 0.80, 0.22),
            GlassDepth::Modal => (32.0, 0.84, 0.28),
        };

        recipe.background = tone_color(theme, tone).with_alpha(alpha);
        recipe.backdrop_blur_px = blur;
        recipe.shadow_alpha = shadow_alpha;
    }

    recipe.saturate_percent = match (recipe.force_solid, request.vibrancy) {
        (true, _) => 100,
        (false, MaterialVibrancy::Muted) => 130,
        (false, MaterialVibrancy::Standard) => recipe.saturate_percent,
        (false, MaterialVibrancy::Vivid) => 180,
    };
    recipe
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
