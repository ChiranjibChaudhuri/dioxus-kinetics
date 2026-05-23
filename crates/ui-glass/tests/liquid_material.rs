use ui_glass::GlassFeatures;

#[test]
fn glass_features_empty_has_no_bits_set() {
    let f = GlassFeatures::empty();
    assert!(!f.contains(GlassFeatures::BLUR));
    assert!(!f.contains(GlassFeatures::REFRACT));
    assert!(!f.contains(GlassFeatures::DISPERSE));
    assert!(!f.contains(GlassFeatures::SPECULAR));
    assert!(!f.contains(GlassFeatures::INNER_SHADOW));
    assert!(!f.contains(GlassFeatures::POINTER));
    assert!(!f.contains(GlassFeatures::SCROLL));
    assert!(!f.contains(GlassFeatures::AMBIENT_MESH));
    assert!(!f.contains(GlassFeatures::TINT_ADAPT));
}

#[test]
fn glass_features_compose_with_bitwise_or() {
    let f = GlassFeatures::BLUR | GlassFeatures::SPECULAR;
    assert!(f.contains(GlassFeatures::BLUR));
    assert!(f.contains(GlassFeatures::SPECULAR));
    assert!(!f.contains(GlassFeatures::REFRACT));
}

use ui_glass::LiquidMaterial;
use ui_tokens::Color;

#[test]
fn liquid_material_new_has_neutral_defaults() {
    let m = LiquidMaterial::new();
    assert_eq!(m.tint, Color::rgba(255, 255, 255, 1.0));
    assert_eq!(m.tint_alpha, 0.0);
    assert_eq!(m.blur_radius_px, 0.0);
    assert_eq!(m.saturation, 1.0);
    assert_eq!(m.refraction_strength, 0.0);
    assert_eq!(m.dispersion_px, 0.0);
    assert_eq!(m.light_intensity, 0.0);
    assert_eq!(m.inner_shadow_alpha, 0.0);
    assert_eq!(m.adapt_to_background, 0.0);
    assert_eq!(m.radius_px, 0.0);
    assert_eq!(m.thickness_px, 1.0);
    assert!(!m.pointer_reactive);
    assert!(!m.scroll_reactive);
    assert!(m.ambient_mesh.is_none());
    assert_eq!(m.features, ui_glass::GlassFeatures::empty());
}
