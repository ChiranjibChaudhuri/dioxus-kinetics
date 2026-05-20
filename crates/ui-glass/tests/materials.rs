use ui_glass::{resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRequest, GlassTone};
use ui_tokens::{Theme, TransparencyPreference};

#[test]
fn floating_glass_uses_backdrop_blur_by_default() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(GlassLevel::Floating, GlassTone::Neutral, GlassDensity::Comfortable),
    );

    assert_eq!(recipe.backdrop_blur_px, 18.0);
    assert_eq!(recipe.saturate_percent, 160);
    assert!(!recipe.force_solid);
}

#[test]
fn reduced_transparency_forces_solid_recipe() {
    let mut theme = Theme::default();
    theme.transparency = TransparencyPreference::Reduce;

    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(GlassLevel::Overlay, GlassTone::Primary, GlassDensity::Comfortable),
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
    let request = GlassRequest::new(GlassLevel::Chrome, GlassTone::Neutral, GlassDensity::Compact)
        .with_policy(GlassPolicy::SolidFallback);

    let recipe = resolve_glass(&theme, request);

    assert!(recipe.force_solid);
    assert_eq!(recipe.backdrop_blur_px, 0.0);
}
