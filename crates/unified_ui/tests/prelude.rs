use unified_ui::prelude::*;

#[test]
fn prelude_exposes_semantic_components_and_tokens() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Floating,
            GlassTone::Neutral,
            GlassDensity::Comfortable,
        ),
    );

    assert_eq!(
        ButtonVariant::Primary.class_name(),
        "ui-button ui-button--primary"
    );
    assert_eq!(recipe.backdrop_blur_px, 18.0);
}

#[test]
fn default_features_do_not_expose_gsap_or_hyperframes_names() {
    let public_names = unified_ui::public_api_names();

    assert!(!public_names.iter().any(|name| name.contains("Gsap")));
    assert!(!public_names.iter().any(|name| name.contains("HyperFrames")));
}
