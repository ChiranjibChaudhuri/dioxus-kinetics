use ui_glass::{
    resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRequest, GlassTone,
};
use ui_tokens::Theme;

#[test]
fn solid_fallback_policy_returns_zero_blur_and_force_solid() {
    let theme = Theme::default();
    let req = GlassRequest::new(GlassLevel::Floating, GlassTone::Neutral, GlassDensity::Comfortable)
        .with_policy(GlassPolicy::SolidFallback);
    let recipe = resolve_glass(&theme, req);
    assert!(recipe.force_solid);
    assert_eq!(recipe.backdrop_blur_px, 0.0);
    assert_eq!(recipe.saturate_percent, 100);
    assert_eq!(recipe.background, theme.semantic.surface_solid);
}

#[test]
fn auto_policy_returns_blurred_recipe() {
    let theme = Theme::default();
    let req = GlassRequest::new(GlassLevel::Floating, GlassTone::Neutral, GlassDensity::Comfortable);
    let recipe = resolve_glass(&theme, req);
    assert!(!recipe.force_solid);
    assert!(recipe.backdrop_blur_px > 0.0);
    assert!(recipe.saturate_percent > 100);
}
