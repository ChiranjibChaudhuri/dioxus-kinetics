use ui_tokens::{Color, Density, MotionPreference, Theme, ThemeMode, TransparencyPreference};

#[test]
fn default_theme_has_apple_like_surface_bias() {
    let theme = Theme::default();

    assert_eq!(theme.mode, ThemeMode::Light);
    assert_eq!(theme.density, Density::Comfortable);
    assert_eq!(theme.radius.medium_px, 10.0);
    assert_eq!(theme.transparency, TransparencyPreference::Allow);
    assert_eq!(
        theme.semantic.background.css_rgba(),
        "rgba(246, 247, 249, 1.000)"
    );
    assert_eq!(
        theme.semantic.surface.css_rgba(),
        "rgba(255, 255, 255, 0.780)"
    );
    assert_eq!(
        theme.semantic.surface_solid.css_rgba(),
        "rgba(255, 255, 255, 1.000)"
    );
    assert_eq!(theme.spacing.xs_px, 4.0);
    assert_eq!(theme.spacing.sm_px, 8.0);
    assert_eq!(theme.spacing.md_px, 12.0);
    assert_eq!(theme.spacing.lg_px, 16.0);
    assert_eq!(theme.spacing.xl_px, 24.0);
    assert_eq!(theme.motion.fast_ms, 120);
    assert_eq!(theme.motion.normal_ms, 180);
    assert_eq!(theme.motion.slow_ms, 260);
    assert_eq!(
        theme.semantic.success.css_rgba(),
        "rgba(36, 138, 61, 1.000)"
    );
    assert_eq!(
        theme.semantic.warning.css_rgba(),
        "rgba(176, 105, 0, 1.000)"
    );
    assert_eq!(theme.semantic.danger.css_rgba(), "rgba(196, 43, 43, 1.000)");
    assert_eq!(theme.semantic.info.css_rgba(), "rgba(20, 118, 191, 1.000)");
    assert_eq!(theme.motion_preference, MotionPreference::Allow);
}

#[test]
fn colors_are_css_serializable() {
    let color = Color::rgba(12, 34, 56, 0.375);

    assert_eq!(color.css_rgba(), "rgba(12, 34, 56, 0.375)");
}

#[test]
fn rgba_sanitizes_alpha_on_construction() {
    assert_eq!(Color::rgba(12, 34, 56, -0.25).a, 0.0);
    assert_eq!(Color::rgba(12, 34, 56, 1.25).a, 1.0);
    assert_eq!(Color::rgba(12, 34, 56, f32::NAN).a, 1.0);
    assert_eq!(Color::rgba(12, 34, 56, f32::INFINITY).a, 1.0);
    assert_eq!(Color::rgba(12, 34, 56, f32::NEG_INFINITY).a, 1.0);
}

#[test]
fn with_alpha_sanitizes_replacement_alpha() {
    let color = Color::rgba(12, 34, 56, 0.375);

    assert_eq!(color.with_alpha(-0.25).a, 0.0);
    assert_eq!(color.with_alpha(1.25).a, 1.0);
    assert_eq!(color.with_alpha(f32::NAN).a, 1.0);
    assert_eq!(color.with_alpha(f32::INFINITY).a, 1.0);
    assert_eq!(color.with_alpha(f32::NEG_INFINITY).a, 1.0);
}

#[test]
fn css_rgba_sanitizes_public_alpha_field() {
    let color = Color {
        r: 12,
        g: 34,
        b: 56,
        a: f32::NAN,
    };

    assert_eq!(color.css_rgba(), "rgba(12, 34, 56, 1.000)");
    assert_eq!(
        Color { a: -0.25, ..color }.css_rgba(),
        "rgba(12, 34, 56, 0.000)"
    );
    assert_eq!(
        Color { a: 1.25, ..color }.css_rgba(),
        "rgba(12, 34, 56, 1.000)"
    );
    assert_eq!(
        Color {
            a: f32::INFINITY,
            ..color
        }
        .css_rgba(),
        "rgba(12, 34, 56, 1.000)"
    );
    assert_eq!(
        Color {
            a: f32::NEG_INFINITY,
            ..color
        }
        .css_rgba(),
        "rgba(12, 34, 56, 1.000)"
    );
}
