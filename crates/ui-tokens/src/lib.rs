#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a: sanitize_alpha(a),
        }
    }

    pub fn css_rgba(self) -> String {
        format!(
            "rgba({}, {}, {}, {:.3})",
            self.r,
            self.g,
            self.b,
            sanitize_alpha(self.a)
        )
    }

    pub fn with_alpha(self, a: f32) -> Self {
        Self {
            a: sanitize_alpha(a),
            ..self
        }
    }
}

const fn sanitize_alpha(a: f32) -> f32 {
    if !a.is_finite() {
        1.0
    } else if a < 0.0 {
        0.0
    } else if a > 1.0 {
        1.0
    } else {
        a
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Density {
    Compact,
    Comfortable,
    Spacious,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionPreference {
    Allow,
    Reduce,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransparencyPreference {
    Allow,
    Reduce,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SemanticColors {
    pub background: Color,
    pub surface: Color,
    pub surface_solid: Color,
    pub foreground: Color,
    pub muted_foreground: Color,
    pub border: Color,
    pub primary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub info: Color,
    pub focus: Color,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RadiusScale {
    pub small_px: f32,
    pub medium_px: f32,
    pub large_px: f32,
    pub floating_px: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpacingScale {
    pub xs_px: f32,
    pub sm_px: f32,
    pub md_px: f32,
    pub lg_px: f32,
    pub xl_px: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionScale {
    pub fast_ms: u32,
    pub normal_ms: u32,
    pub slow_ms: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub mode: ThemeMode,
    pub density: Density,
    pub semantic: SemanticColors,
    pub radius: RadiusScale,
    pub spacing: SpacingScale,
    pub motion: MotionScale,
    pub transparency: TransparencyPreference,
    pub motion_preference: MotionPreference,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Light,
            density: Density::Comfortable,
            semantic: SemanticColors {
                background: Color::rgba(246, 247, 249, 1.0),
                surface: Color::rgba(255, 255, 255, 0.78),
                surface_solid: Color::rgba(255, 255, 255, 1.0),
                foreground: Color::rgba(20, 23, 28, 1.0),
                muted_foreground: Color::rgba(86, 94, 108, 1.0),
                border: Color::rgba(120, 132, 150, 0.24),
                primary: Color::rgba(0, 102, 204, 1.0),
                success: Color::rgba(36, 138, 61, 1.0),
                warning: Color::rgba(176, 105, 0, 1.0),
                danger: Color::rgba(196, 43, 43, 1.0),
                info: Color::rgba(20, 118, 191, 1.0),
                focus: Color::rgba(0, 122, 255, 1.0),
            },
            radius: RadiusScale {
                small_px: 6.0,
                medium_px: 10.0,
                large_px: 14.0,
                floating_px: 18.0,
            },
            spacing: SpacingScale {
                xs_px: 4.0,
                sm_px: 8.0,
                md_px: 12.0,
                lg_px: 16.0,
                xl_px: 24.0,
            },
            motion: MotionScale {
                fast_ms: 120,
                normal_ms: 180,
                slow_ms: 260,
            },
            transparency: TransparencyPreference::Allow,
            motion_preference: MotionPreference::Allow,
        }
    }
}
