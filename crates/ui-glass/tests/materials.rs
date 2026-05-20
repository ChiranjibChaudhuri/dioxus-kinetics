use ui_glass::{resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRequest, GlassTone};
use ui_tokens::{Theme, TransparencyPreference};

#[test]
fn floating_glass_uses_backdrop_blur_by_default() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Floating,
            GlassTone::Neutral,
            GlassDensity::Comfortable,
        ),
    );

    assert_eq!(recipe.backdrop_blur_px, 18.0);
    assert_eq!(recipe.saturate_percent, 160);
    assert!(!recipe.force_solid);
}

#[test]
fn reduced_transparency_forces_solid_recipe() {
    let theme = Theme {
        transparency: TransparencyPreference::Reduce,
        ..Default::default()
    };

    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Overlay,
            GlassTone::Primary,
            GlassDensity::Comfortable,
        ),
    );

    assert_eq!(recipe.backdrop_blur_px, 0.0);
    assert!(recipe.force_solid);
    assert_eq!(
        recipe.background.css_rgba(),
        theme.semantic.surface_solid.css_rgba()
    );
}

#[test]
fn explicit_solid_policy_overrides_blur() {
    let theme = Theme::default();
    let request = GlassRequest::new(
        GlassLevel::Chrome,
        GlassTone::Neutral,
        GlassDensity::Compact,
    )
    .with_policy(GlassPolicy::SolidFallback);

    let recipe = resolve_glass(&theme, request);

    assert!(recipe.force_solid);
    assert_eq!(recipe.backdrop_blur_px, 0.0);
}

#[test]
fn danger_tone_and_spacious_density_resolve_theme_tokens() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Subtle,
            GlassTone::Danger,
            GlassDensity::Spacious,
        ),
    );

    assert_eq!(recipe.background, theme.semantic.danger.with_alpha(0.64));
    assert_eq!(recipe.radius_px, theme.radius.large_px);
}

#[test]
fn contrast_and_reduced_transparency_policies_force_solid_saturation() {
    let theme = Theme::default();

    for policy in [GlassPolicy::HighContrast, GlassPolicy::ReducedTransparency] {
        let recipe = resolve_glass(
            &theme,
            GlassRequest::new(
                GlassLevel::Floating,
                GlassTone::Info,
                GlassDensity::Comfortable,
            )
            .with_policy(policy),
        );

        assert!(recipe.force_solid);
        assert_eq!(recipe.saturate_percent, 100);
    }
}

#[test]
fn fallback_background_uses_solid_surface() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Overlay,
            GlassTone::Success,
            GlassDensity::Compact,
        ),
    );

    assert_eq!(recipe.fallback_background, theme.semantic.surface_solid);
}
