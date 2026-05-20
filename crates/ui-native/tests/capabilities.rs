use ui_glass::{resolve_glass, GlassDensity, GlassLevel, GlassRequest, GlassTone};
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
