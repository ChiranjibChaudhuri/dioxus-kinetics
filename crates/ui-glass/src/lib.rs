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

/// Pre-baked quality preset. Maps to feature mask + blur tap count + scroll/
/// pointer reactivity gates. Used by the runtime to scale visual cost
/// against device class and user preferences.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QualityProfile {
    /// All 9 traits, 13-tap blur, full mip chain.
    #[default]
    High,
    /// Tier 1 minus AMBIENT_MESH, 9-tap blur.
    Balanced,
    /// Tier 2 forced (battery-saving): drops REFRACT, DISPERSE, AMBIENT_MESH, TINT_ADAPT.
    Power,
    /// Engine off — Solid CSS surface only.
    Off,
}

impl QualityProfile {
    /// Mask out features that this profile suppresses. Combine with the
    /// material's existing features via `material.features &= profile.feature_mask()`.
    pub fn feature_mask(self) -> GlassFeatures {
        use GlassFeatures as F;
        match self {
            QualityProfile::High => F::all(),
            QualityProfile::Balanced => F::all().difference(F::AMBIENT_MESH),
            QualityProfile::Power => F::BLUR | F::SPECULAR | F::INNER_SHADOW,
            QualityProfile::Off => F::empty(),
        }
    }

    /// Blur taps to use under this profile.
    pub fn blur_taps(self) -> u32 {
        match self {
            QualityProfile::High => 13,
            QualityProfile::Balanced => 9,
            QualityProfile::Power => 5,
            QualityProfile::Off => 1,
        }
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

/// Ambient mesh contribution variants. Plan 1 carries the descriptor; the
/// shader binding lands in Plan 2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AmbientMesh {
    Aurora,
    Orbs,
    Grain,
}

/// Full shader-parameter descriptor for a Liquid Glass surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LiquidMaterial {
    pub tint: Color,
    pub tint_alpha: f32,
    pub blur_radius_px: f32,
    pub saturation: f32,
    pub refraction_strength: f32,
    pub surface_curvature: f32,
    pub noise_frequency: f32,
    pub noise_seed: f32,
    pub dispersion_px: f32,
    pub light_angle_rad: f32,
    pub light_intensity: f32,
    pub edge_falloff_px: f32,
    pub inner_shadow_px: f32,
    pub inner_shadow_alpha: f32,
    pub pointer_reactive: bool,
    pub scroll_reactive: bool,
    pub ambient_mesh: Option<AmbientMesh>,
    pub adapt_to_background: f32,
    pub radius_px: f32,
    pub thickness_px: f32,
    pub features: GlassFeatures,
}

impl LiquidMaterial {
    pub const fn new() -> Self {
        Self {
            tint: Color::rgba(255, 255, 255, 1.0),
            tint_alpha: 0.0,
            blur_radius_px: 0.0,
            saturation: 1.0,
            refraction_strength: 0.0,
            surface_curvature: 0.0,
            noise_frequency: 1.0,
            noise_seed: 0.0,
            dispersion_px: 0.0,
            light_angle_rad: 0.0,
            light_intensity: 0.0,
            edge_falloff_px: 0.0,
            inner_shadow_px: 0.0,
            inner_shadow_alpha: 0.0,
            pointer_reactive: false,
            scroll_reactive: false,
            ambient_mesh: None,
            adapt_to_background: 0.0,
            radius_px: 0.0,
            thickness_px: 1.0,
            features: GlassFeatures::empty(),
        }
    }

    pub fn blur(mut self, radius_px: f32) -> Self {
        self.blur_radius_px = radius_px;
        self.features |= GlassFeatures::BLUR;
        self
    }

    pub fn tint(mut self, color: Color, alpha: f32) -> Self {
        self.tint = color;
        self.tint_alpha = alpha;
        self
    }

    pub fn saturation(mut self, value: f32) -> Self {
        self.saturation = value;
        self
    }

    pub fn refract(mut self, strength: f32) -> Self {
        self.refraction_strength = strength;
        self.features |= GlassFeatures::REFRACT;
        self
    }

    pub fn surface_curvature(mut self, value: f32) -> Self {
        self.surface_curvature = value;
        self
    }

    pub fn noise(mut self, frequency: f32, seed: f32) -> Self {
        self.noise_frequency = frequency;
        self.noise_seed = seed;
        self
    }

    pub fn disperse(mut self, px: f32) -> Self {
        self.dispersion_px = px;
        self.features |= GlassFeatures::DISPERSE;
        self
    }

    pub fn specular(mut self, angle_rad: f32, intensity: f32) -> Self {
        self.light_angle_rad = angle_rad;
        self.light_intensity = intensity;
        self.features |= GlassFeatures::SPECULAR;
        self
    }

    pub fn edge_falloff(mut self, px: f32) -> Self {
        self.edge_falloff_px = px;
        self
    }

    pub fn inner_shadow(mut self, px: f32, alpha: f32) -> Self {
        self.inner_shadow_px = px;
        self.inner_shadow_alpha = alpha;
        self.features |= GlassFeatures::INNER_SHADOW;
        self
    }

    pub fn pointer_reactive(mut self) -> Self {
        self.pointer_reactive = true;
        self.features |= GlassFeatures::POINTER;
        self
    }

    pub fn scroll_reactive(mut self) -> Self {
        self.scroll_reactive = true;
        self.features |= GlassFeatures::SCROLL;
        self
    }

    pub fn ambient_mesh(mut self, mesh: AmbientMesh) -> Self {
        self.ambient_mesh = Some(mesh);
        self.features |= GlassFeatures::AMBIENT_MESH;
        self
    }

    pub fn adapt_to_background(mut self, strength: f32) -> Self {
        self.adapt_to_background = strength;
        self.features |= GlassFeatures::TINT_ADAPT;
        self
    }

    pub fn radius(mut self, px: f32) -> Self {
        self.radius_px = px;
        self
    }

    pub fn thickness(mut self, px: f32) -> Self {
        self.thickness_px = px;
        self
    }

    pub fn chrome() -> Self {
        Self::new()
            .blur(32.0)
            .saturation(1.6)
            .refract(0.15)
            .specular(0.78, 0.5)
            .inner_shadow(6.0, 0.18)
            .edge_falloff(2.0)
            .radius(0.0)
            .thickness(2.0)
    }

    pub fn floating() -> Self {
        Self::new()
            .blur(18.0)
            .saturation(1.6)
            .refract(0.25)
            .disperse(1.0)
            .specular(0.78, 0.6)
            .inner_shadow(4.0, 0.14)
            .edge_falloff(1.5)
            .radius(14.0)
            .thickness(1.5)
    }

    pub fn overlay() -> Self {
        Self::new()
            .blur(24.0)
            .saturation(1.8)
            .refract(0.35)
            .disperse(2.0)
            .specular(0.78, 0.7)
            .inner_shadow(6.0, 0.22)
            .edge_falloff(2.0)
            .radius(18.0)
            .thickness(2.0)
    }

    pub fn sheet() -> Self {
        Self::floating()
            .ambient_mesh(AmbientMesh::Aurora)
            .radius(20.0)
    }

    pub fn tooltip() -> Self {
        Self::new()
            .blur(10.0)
            .saturation(1.3)
            .inner_shadow(2.0, 0.10)
            .radius(8.0)
            .thickness(1.0)
    }

    pub fn button() -> Self {
        Self::new()
            .blur(12.0)
            .saturation(1.4)
            .specular(0.78, 0.5)
            .inner_shadow(2.0, 0.12)
            .pointer_reactive()
            .radius(10.0)
            .thickness(1.0)
    }
}

impl Default for LiquidMaterial {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MaterialRequest> for LiquidMaterial {
    fn from(req: MaterialRequest) -> Self {
        let mut m = match req.depth {
            GlassDepth::Inline | GlassDepth::Raised => LiquidMaterial::floating().blur(12.0),
            GlassDepth::Floating => LiquidMaterial::floating(),
            GlassDepth::Chrome => LiquidMaterial::chrome(),
            GlassDepth::Overlay | GlassDepth::Modal => LiquidMaterial::overlay(),
        };

        // Tone → tint (alpha applied below from depth)
        m.tint = match req.tone {
            MaterialTone::Neutral => Color::rgba(255, 255, 255, 1.0),
            MaterialTone::Primary => Color::rgba(0, 102, 204, 1.0),
            MaterialTone::Success => Color::rgba(36, 138, 61, 1.0),
            MaterialTone::Warning => Color::rgba(176, 105, 0, 1.0),
            MaterialTone::Danger => Color::rgba(196, 43, 43, 1.0),
            MaterialTone::Info => Color::rgba(20, 118, 191, 1.0),
        };
        m.tint_alpha = match req.depth {
            GlassDepth::Inline => 0.58,
            GlassDepth::Raised => 0.64,
            GlassDepth::Floating => 0.72,
            GlassDepth::Chrome => 0.68,
            GlassDepth::Overlay => 0.80,
            GlassDepth::Modal => 0.84,
        };

        // Vibrancy → saturation + dispersion
        let (sat, disp) = match req.vibrancy {
            MaterialVibrancy::Muted => (1.3, 0.0),
            MaterialVibrancy::Standard => (1.6, 1.0),
            MaterialVibrancy::Vivid => (1.8, 2.0),
        };
        m.saturation = sat;
        if m.features.contains(GlassFeatures::DISPERSE) || disp > 0.0 {
            m = m.disperse(disp);
        }

        // Edge → falloff + thickness
        let (fall, thick) = match req.edge {
            MaterialEdge::None => (0.0, 1.0),
            MaterialEdge::Hairline => (1.0, 1.0),
            MaterialEdge::Standard => (1.5, 1.5),
            MaterialEdge::Emphasized => (2.5, 2.5),
        };
        m.edge_falloff_px = fall;
        m.thickness_px = thick;

        // Density → radius scaling against current radius_px
        let scale = match req.density {
            MaterialDensity::Compact => 0.75,
            MaterialDensity::Comfortable => 1.0,
            MaterialDensity::Spacious => 1.4,
        };
        m.radius_px *= scale;

        // Policy → feature masking
        if matches!(
            req.policy,
            MaterialPolicy::HighContrast
                | MaterialPolicy::ReducedTransparency
                | MaterialPolicy::SolidFallback
        ) {
            m.features.remove(
                GlassFeatures::REFRACT
                    | GlassFeatures::DISPERSE
                    | GlassFeatures::SPECULAR
                    | GlassFeatures::POINTER
                    | GlassFeatures::SCROLL
                    | GlassFeatures::AMBIENT_MESH,
            );
            m.refraction_strength = 0.0;
            m.dispersion_px = 0.0;
            m.light_intensity = 0.0;
            m.pointer_reactive = false;
            m.scroll_reactive = false;
            m.ambient_mesh = None;
        }

        m
    }
}
