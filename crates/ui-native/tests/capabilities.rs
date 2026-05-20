use ui_glass::{resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRequest, GlassTone};
use ui_native::{plan_native_glass, NativeCapabilities};
use ui_tokens::Theme;

#[test]
fn native_without_backdrop_sampling_uses_simulated_glass() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Chrome,
            GlassTone::Neutral,
            GlassDensity::Comfortable,
        ),
    );
    let plan = plan_native_glass(&recipe, NativeCapabilities::minimal());

    assert!(plan.uses_simulated_glass);
    assert!(!plan.uses_backdrop_blur);
    assert_eq!(plan.effective_blur_px, 0.0);
}

#[test]
fn native_with_filter_support_can_use_real_blur() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Chrome,
            GlassTone::Neutral,
            GlassDensity::Comfortable,
        ),
    );
    let plan = plan_native_glass(&recipe, NativeCapabilities::with_backdrop_filters());

    assert!(!plan.uses_simulated_glass);
    assert!(plan.uses_backdrop_blur);
    assert_eq!(plan.effective_blur_px, 28.0);
}

#[test]
fn forced_solid_recipe_suppresses_native_blur() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Chrome,
            GlassTone::Neutral,
            GlassDensity::Comfortable,
        )
        .with_policy(GlassPolicy::SolidFallback),
    );
    let plan = plan_native_glass(&recipe, NativeCapabilities::with_backdrop_filters());

    assert!(plan.uses_simulated_glass);
    assert!(!plan.uses_backdrop_blur);
    assert_eq!(plan.effective_blur_px, 0.0);
}

#[test]
fn non_finite_recipe_blur_becomes_zero() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Chrome,
            GlassTone::Neutral,
            GlassDensity::Comfortable,
        ),
    );

    for blur_px in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let mut recipe = recipe;
        recipe.backdrop_blur_px = blur_px;

        let plan = plan_native_glass(&recipe, NativeCapabilities::with_backdrop_filters());

        assert!(plan.uses_backdrop_blur);
        assert!(!plan.uses_simulated_glass);
        assert_eq!(plan.effective_blur_px, 0.0);
    }
}
