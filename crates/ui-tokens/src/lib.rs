#![forbid(unsafe_code)]

pub mod elevation;

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

    /// Emit a CSS Color Module Level 4 `color(display-p3 ...)` string.
    ///
    /// Channels are normalised to the 0..1 range. Values are clamped to sRGB's
    /// gamut by construction (the underlying components are `u8`), so this stays
    /// visually identical to [`Color::css_rgba`] while opting into the wider
    /// working space when the engine supports it.
    pub fn css_p3(self) -> String {
        format!(
            "color(display-p3 {:.4} {:.4} {:.4} / {:.3})",
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            sanitize_alpha(self.a)
        )
    }

    /// Emit an opaque `#rrggbb` hex string. Alpha is intentionally ignored so
    /// this can feed solid-fill code paths and future token generation.
    pub fn css_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
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

impl ThemeMode {
    /// The `data-ui-theme` attribute value for this mode, matching the
    /// selectors in the shared CSS (`light` / `dark`).
    pub const fn data_attr(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    /// Parse a `data-ui-theme` attribute value back into a `ThemeMode`.
    /// Returns `None` for any unrecognized value.
    pub fn from_attr(value: &str) -> Option<Self> {
        match value {
            "light" => Some(Self::Light),
            "dark" => Some(Self::Dark),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Density {
    Compact,
    Comfortable,
    Spacious,
}

impl Density {
    /// The `data-ui-density` attribute value for this density, matching the
    /// selectors in the shared CSS (`compact` / `comfortable` / `spacious`).
    pub const fn data_attr(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Comfortable => "comfortable",
            Self::Spacious => "spacious",
        }
    }

    /// Parse a `data-ui-density` attribute value back into a `Density`.
    /// Returns `None` for any unrecognized value.
    pub fn from_attr(value: &str) -> Option<Self> {
        match value {
            "compact" => Some(Self::Compact),
            "comfortable" => Some(Self::Comfortable),
            "spacious" => Some(Self::Spacious),
            _ => None,
        }
    }
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

impl SemanticColors {
    /// Light semantic palette tuned for WCAG AA on white surfaces. Mirrors the
    /// CSS `:root` accents in the shared design contract.
    pub fn light() -> Self {
        Self {
            background: Color::rgba(246, 247, 249, 1.0),
            surface: Color::rgba(255, 255, 255, 0.78),
            surface_solid: Color::rgba(255, 255, 255, 1.0),
            foreground: Color::rgba(20, 23, 28, 1.0),
            muted_foreground: Color::rgba(86, 94, 108, 1.0),
            border: Color::rgba(120, 132, 150, 0.24),
            primary: Color::rgba(0, 88, 179, 1.0),
            success: Color::rgba(26, 107, 46, 1.0),
            warning: Color::rgba(154, 88, 0, 1.0),
            danger: Color::rgba(196, 43, 43, 1.0),
            info: Color::rgba(15, 99, 163, 1.0),
            focus: Color::rgba(0, 122, 255, 1.0),
        }
    }

    /// Dark semantic palette tuned so every accent clears 4.5:1 on both the
    /// `#151b23` surface and the `#0d1117` background. Accent values match the
    /// CSS `[data-ui-theme="dark"]` block and the wgpu glass engine.
    pub fn dark() -> Self {
        Self {
            background: Color::rgba(13, 17, 23, 1.0),
            surface: Color::rgba(21, 27, 35, 1.0),
            surface_solid: Color::rgba(21, 27, 35, 1.0),
            foreground: Color::rgba(238, 243, 248, 1.0),
            muted_foreground: Color::rgba(170, 180, 194, 1.0),
            border: Color::rgba(205, 215, 228, 0.18),
            primary: Color::rgba(76, 155, 255, 1.0),
            success: Color::rgba(62, 207, 106, 1.0),
            warning: Color::rgba(240, 168, 46, 1.0),
            danger: Color::rgba(255, 107, 107, 1.0),
            info: Color::rgba(92, 182, 255, 1.0),
            focus: Color::rgba(100, 181, 255, 1.0),
        }
    }
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
    /// Hairline spacing (`--ui-space-0`, 2px).
    pub xxs_px: f32,
    pub xs_px: f32,
    pub sm_px: f32,
    pub md_px: f32,
    pub lg_px: f32,
    pub xl_px: f32,
    /// `--ui-space-6`, 32px.
    pub xxl_px: f32,
    /// `--ui-space-7`, 48px.
    pub xxxl_px: f32,
    /// `--ui-space-8`, 64px.
    pub xxxxl_px: f32,
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
            semantic: SemanticColors::light(),
            radius: RadiusScale {
                small_px: 6.0,
                medium_px: 10.0,
                large_px: 14.0,
                floating_px: 18.0,
            },
            spacing: SpacingScale {
                xxs_px: 2.0,
                xs_px: 4.0,
                sm_px: 8.0,
                md_px: 12.0,
                lg_px: 16.0,
                xl_px: 24.0,
                xxl_px: 32.0,
                xxxl_px: 48.0,
                xxxxl_px: 64.0,
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

impl Theme {
    /// Dark-mode theme. Shares the light radius/spacing/motion ramps but swaps
    /// in the [`SemanticColors::dark`] palette and flips [`ThemeMode`].
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            semantic: SemanticColors::dark(),
            ..Self::default()
        }
    }
}

/// Serialize a [`Theme`] to a `--ui-*` CSS custom-property block scoped under
/// the theme's mode selector. This is the "token studio" export: a host can
/// build a custom `Theme` (override any palette/ramp), dump this CSS, and
/// inject it once near the app root to re-skin every kinetics surface.
///
/// Colors use `rgba()` so translucent tokens (borders, glass) round-trip
/// correctly; opaque tokens still render identically.
pub fn export_tokens_css(theme: &Theme) -> String {
    let selector = match theme.mode {
        ThemeMode::Light => ":root, [data-ui-theme=\"light\"]",
        ThemeMode::Dark => "[data-ui-theme=\"dark\"]",
    };
    let c = &theme.semantic;
    let r = &theme.radius;
    let s = &theme.spacing;
    let m = &theme.motion;
    format!(
        r#"{selector} {{
    --ui-bg: {bg};
    --ui-surface: {surface};
    --ui-surface-strong: {surface_strong};
    --ui-fg: {fg};
    --ui-muted-fg: {mfg};
    --ui-border: {border};
    --ui-primary: {primary};
    --ui-success: {success};
    --ui-warning: {warning};
    --ui-danger: {danger};
    --ui-info: {info};
    --ui-focus: {focus};
    --ui-radius-sm: {rsm}px;
    --ui-radius-md: {rmd}px;
    --ui-radius-lg: {rlg}px;
    --ui-radius-floating: {rfl}px;
    --ui-space-0: {s0}px;
    --ui-space-1: {s1}px;
    --ui-space-2: {s2}px;
    --ui-space-3: {s3}px;
    --ui-space-4: {s4}px;
    --ui-space-5: {s5}px;
    --ui-space-6: {s6}px;
    --ui-space-7: {s7}px;
    --ui-space-8: {s8}px;
    --ui-motion-fast: {mf}ms;
    --ui-motion-normal: {mn}ms;
    --ui-motion-slow: {mslow}ms;
}}
"#,
        selector = selector,
        bg = c.background.css_rgba(),
        surface = c.surface.css_rgba(),
        surface_strong = c.surface_solid.css_rgba(),
        fg = c.foreground.css_rgba(),
        mfg = c.muted_foreground.css_rgba(),
        border = c.border.css_rgba(),
        primary = c.primary.css_rgba(),
        success = c.success.css_rgba(),
        warning = c.warning.css_rgba(),
        danger = c.danger.css_rgba(),
        info = c.info.css_rgba(),
        focus = c.focus.css_rgba(),
        rsm = r.small_px,
        rmd = r.medium_px,
        rlg = r.large_px,
        rfl = r.floating_px,
        s0 = s.xxs_px,
        s1 = s.xs_px,
        s2 = s.sm_px,
        s3 = s.md_px,
        s4 = s.lg_px,
        s5 = s.xl_px,
        s6 = s.xxl_px,
        s7 = s.xxxl_px,
        s8 = s.xxxxl_px,
        mf = m.fast_ms,
        mn = m.normal_ms,
        mslow = m.slow_ms,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_targets_mode_selector() {
        assert!(export_tokens_css(&Theme::default()).contains(":root,"));
        assert!(export_tokens_css(&Theme::dark()).contains("[data-ui-theme=\"dark\"]"));
    }

    #[test]
    fn export_includes_every_token_family() {
        let css = export_tokens_css(&Theme::default());
        assert!(css.contains("--ui-primary:"));
        assert!(css.contains("--ui-radius-md:"));
        assert!(css.contains("--ui-space-4:"));
        assert!(css.contains("--ui-motion-normal:"));
    }

    #[test]
    fn export_round_trips_custom_primary() {
        let mut theme = Theme::default();
        theme.semantic.primary = Color::rgba(1, 2, 3, 1.0);
        assert!(export_tokens_css(&theme).contains("rgba(1, 2, 3, 1.000)"));
    }
}
