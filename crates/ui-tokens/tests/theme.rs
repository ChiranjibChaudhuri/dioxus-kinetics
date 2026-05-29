use ui_tokens::{
    Color, Density, MotionPreference, SemanticColors, SpacingScale, Theme, ThemeMode,
    TransparencyPreference,
};

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
        "rgba(31, 122, 53, 1.000)"
    );
    assert_eq!(theme.semantic.warning.css_rgba(), "rgba(154, 88, 0, 1.000)");
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

/// WCAG relative luminance for an opaque sRGB color in the 0..1 range.
fn relative_luminance(color: Color) -> f32 {
    fn linearize(channel: u8) -> f32 {
        let c = channel as f32 / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    0.2126 * linearize(color.r) + 0.7152 * linearize(color.g) + 0.0722 * linearize(color.b)
}

#[test]
fn dark_theme_uses_dark_mode_and_dark_palette() {
    let dark = Theme::dark();

    assert_eq!(dark.mode, ThemeMode::Dark);
    assert_eq!(dark.semantic, SemanticColors::dark());
    // Non-color ramps stay in lockstep with the light theme.
    assert_eq!(dark.radius, Theme::default().radius);
    assert_eq!(dark.spacing, Theme::default().spacing);
    assert_eq!(dark.motion, Theme::default().motion);

    assert_eq!(
        dark.semantic.background.css_rgba(),
        "rgba(13, 17, 23, 1.000)"
    );
    assert_eq!(dark.semantic.surface.css_rgba(), "rgba(21, 27, 35, 1.000)");
    assert_eq!(
        dark.semantic.primary.css_rgba(),
        "rgba(76, 155, 255, 1.000)"
    );
    assert_eq!(
        dark.semantic.success.css_rgba(),
        "rgba(62, 207, 106, 1.000)"
    );
    assert_eq!(
        dark.semantic.warning.css_rgba(),
        "rgba(240, 168, 46, 1.000)"
    );
    assert_eq!(
        dark.semantic.danger.css_rgba(),
        "rgba(255, 107, 107, 1.000)"
    );
    assert_eq!(dark.semantic.info.css_rgba(), "rgba(92, 182, 255, 1.000)");
    assert_eq!(dark.semantic.focus.css_rgba(), "rgba(100, 181, 255, 1.000)");
}

#[test]
fn dark_accents_differ_from_light_accents() {
    let light = SemanticColors::light();
    let dark = SemanticColors::dark();

    assert_ne!(light.primary, dark.primary);
    assert_ne!(light.success, dark.success);
    assert_ne!(light.warning, dark.warning);
    assert_ne!(light.danger, dark.danger);
    assert_ne!(light.info, dark.info);
    assert_ne!(light.focus, dark.focus);

    // The default light theme is the canonical light palette.
    assert_eq!(Theme::default().semantic, light);
}

#[test]
fn dark_accents_have_distinct_luminance_from_light() {
    let light = SemanticColors::light();
    let dark = SemanticColors::dark();

    for (l, d) in [
        (light.primary, dark.primary),
        (light.success, dark.success),
        (light.info, dark.info),
    ] {
        let delta = (relative_luminance(l) - relative_luminance(d)).abs();
        assert!(
            delta > 0.05,
            "expected a clear luminance gap between light and dark accent, got {delta}"
        );
    }
}

#[test]
fn css_p3_emits_display_p3_color_function() {
    let color = Color::rgba(0, 102, 204, 1.0);
    let p3 = color.css_p3();

    assert!(
        p3.starts_with("color(display-p3"),
        "expected display-p3 prefix, got {p3}"
    );
    assert_eq!(p3, "color(display-p3 0.0000 0.4000 0.8000 / 1.000)");
}

#[test]
fn css_hex_round_trips_a_known_color() {
    assert_eq!(Color::rgba(0, 102, 204, 1.0).css_hex(), "#0066cc");
    assert_eq!(Color::rgba(255, 107, 107, 1.0).css_hex(), "#ff6b6b");
    // Alpha is intentionally ignored by the hex serializer.
    assert_eq!(Color::rgba(31, 122, 53, 0.5).css_hex(), "#1f7a35");
}

#[test]
fn spacing_scale_ramp_is_monotonically_increasing() {
    let spacing = Theme::default().spacing;
    let ramp = [
        spacing.xxs_px,
        spacing.xs_px,
        spacing.sm_px,
        spacing.md_px,
        spacing.lg_px,
        spacing.xl_px,
        spacing.xxl_px,
        spacing.xxxl_px,
        spacing.xxxxl_px,
    ];

    for pair in ramp.windows(2) {
        assert!(
            pair[1] > pair[0],
            "spacing ramp must strictly increase: {} !> {}",
            pair[1],
            pair[0]
        );
    }

    assert_eq!(ramp[0], 2.0);
    assert_eq!(ramp[ramp.len() - 1], 64.0);
}

#[test]
fn spacing_scale_constructs_with_full_4pt_ramp() {
    // Compile-time guard: every SpacingScale field must be supplied. If the
    // integrator adds another construction site, this keeps the field set
    // documented in one place.
    let spacing = SpacingScale {
        xxs_px: 2.0,
        xs_px: 4.0,
        sm_px: 8.0,
        md_px: 12.0,
        lg_px: 16.0,
        xl_px: 24.0,
        xxl_px: 32.0,
        xxxl_px: 48.0,
        xxxxl_px: 64.0,
    };

    assert_eq!(spacing, Theme::default().spacing);
}
