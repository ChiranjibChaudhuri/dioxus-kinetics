use ui_dom::{glass_style, CssStyleWriter};
use ui_glass::{
    resolve_glass, resolve_material, GlassDensity, GlassDepth, GlassLevel, GlassPolicy,
    GlassRequest, GlassTone, MaterialRequest, MaterialTone,
};
use ui_tokens::Theme;

#[test]
fn style_writer_serializes_declarations() {
    let style = CssStyleWriter::new()
        .set("color", "red")
        .set("min-height", "44px")
        .to_inline_style();

    assert_eq!(style, "color:red;min-height:44px;");
}

#[test]
fn glass_style_uses_backdrop_filter_when_supported() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Floating,
            GlassTone::Neutral,
            GlassDensity::Comfortable,
        ),
    );
    let style = glass_style(&recipe, true);

    assert!(style.contains("backdrop-filter:blur(18px) saturate(160%);"));
    assert!(style.contains("background:rgba(255, 255, 255, 0.720);"));
}

#[test]
fn glass_style_uses_solid_background_without_backdrop_support() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Floating,
            GlassTone::Neutral,
            GlassDensity::Comfortable,
        ),
    );
    let style = glass_style(&recipe, false);

    assert!(!style.contains("backdrop-filter"));
    assert!(style.contains("background:rgba(255, 255, 255, 1.000);"));
}

#[test]
fn glass_style_uses_solid_background_when_recipe_forces_solid() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Floating,
            GlassTone::Neutral,
            GlassDensity::Comfortable,
        )
        .with_policy(GlassPolicy::SolidFallback),
    );
    let style = glass_style(&recipe, true);

    assert!(!style.contains("backdrop-filter"));
    assert!(style.contains("background:rgba(255, 255, 255, 1.000);"));
}

#[test]
fn glass_style_sanitizes_non_finite_recipe_numbers() {
    let theme = Theme::default();
    let mut recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Floating,
            GlassTone::Neutral,
            GlassDensity::Comfortable,
        ),
    );
    recipe.radius_px = f32::NAN;
    recipe.backdrop_blur_px = f32::INFINITY;
    recipe.shadow_alpha = f32::NEG_INFINITY;

    let style = glass_style(&recipe, true);
    let lower_style = style.to_ascii_lowercase();

    assert!(!lower_style.contains("nan"));
    assert!(!lower_style.contains("inf"));
    assert!(style.contains("border-radius:0px;"));
    assert!(style.contains("backdrop-filter:blur(0px) saturate(160%);"));
    assert!(style.contains("box-shadow:0 18px 42px rgba(20, 23, 28, 0.000);"));
}

#[test]
fn material_style_writes_css_variables_for_native_material_recipe() {
    let theme = ui_tokens::Theme::default();
    let recipe = resolve_material(
        &theme,
        MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral),
    );

    let css = ui_dom::material_style(&recipe);

    assert!(css.contains("--ui-material-blur"));
    assert!(css.contains("--ui-material-saturate"));
    assert!(css.contains("--ui-material-bg"));
    assert!(css.contains("-webkit-backdrop-filter"));
}

#[test]
fn material_style_sanitizes_non_finite_recipe_numbers() {
    let theme = Theme::default();
    let mut recipe = resolve_material(
        &theme,
        MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral),
    );
    recipe.backdrop_blur_px = f32::NAN;

    let style = ui_dom::material_style(&recipe);
    let lower_style = style.to_ascii_lowercase();

    assert!(!lower_style.contains("nan"));
    assert!(!lower_style.contains("inf"));
    assert!(style.contains("--ui-material-blur:0px;"));
}
