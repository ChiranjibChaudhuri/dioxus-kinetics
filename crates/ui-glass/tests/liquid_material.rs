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

#[test]
fn builder_blur_sets_radius_and_feature() {
    let m = LiquidMaterial::new().blur(18.0);
    assert_eq!(m.blur_radius_px, 18.0);
    assert!(m.features.contains(ui_glass::GlassFeatures::BLUR));
}

#[test]
fn builder_tint_sets_color_and_alpha() {
    let m = LiquidMaterial::new().tint(Color::rgba(10, 20, 30, 1.0), 0.4);
    assert_eq!(m.tint, Color::rgba(10, 20, 30, 1.0));
    assert_eq!(m.tint_alpha, 0.4);
}

#[test]
fn builder_refract_sets_strength_and_feature() {
    let m = LiquidMaterial::new().refract(0.35);
    assert_eq!(m.refraction_strength, 0.35);
    assert!(m.features.contains(ui_glass::GlassFeatures::REFRACT));
}

#[test]
fn builder_disperse_sets_px_and_feature() {
    let m = LiquidMaterial::new().disperse(2.0);
    assert_eq!(m.dispersion_px, 2.0);
    assert!(m.features.contains(ui_glass::GlassFeatures::DISPERSE));
}

#[test]
fn builder_specular_sets_light_params_and_feature() {
    let m = LiquidMaterial::new().specular(45.0_f32.to_radians(), 0.8);
    assert!((m.light_angle_rad - 45.0_f32.to_radians()).abs() < 1e-5);
    assert_eq!(m.light_intensity, 0.8);
    assert!(m.features.contains(ui_glass::GlassFeatures::SPECULAR));
}

#[test]
fn builder_inner_shadow_sets_px_alpha_and_feature() {
    let m = LiquidMaterial::new().inner_shadow(4.0, 0.18);
    assert_eq!(m.inner_shadow_px, 4.0);
    assert_eq!(m.inner_shadow_alpha, 0.18);
    assert!(m.features.contains(ui_glass::GlassFeatures::INNER_SHADOW));
}

#[test]
fn builder_pointer_reactive_sets_flag_and_feature() {
    let m = LiquidMaterial::new().pointer_reactive();
    assert!(m.pointer_reactive);
    assert!(m.features.contains(ui_glass::GlassFeatures::POINTER));
}

#[test]
fn builder_scroll_reactive_sets_flag_and_feature() {
    let m = LiquidMaterial::new().scroll_reactive();
    assert!(m.scroll_reactive);
    assert!(m.features.contains(ui_glass::GlassFeatures::SCROLL));
}

#[test]
fn builder_ambient_mesh_sets_variant_and_feature() {
    let m = LiquidMaterial::new().ambient_mesh(ui_glass::AmbientMesh::Aurora);
    assert_eq!(m.ambient_mesh, Some(ui_glass::AmbientMesh::Aurora));
    assert!(m.features.contains(ui_glass::GlassFeatures::AMBIENT_MESH));
}

#[test]
fn builder_adapt_to_background_sets_strength_and_feature() {
    let m = LiquidMaterial::new().adapt_to_background(0.4);
    assert_eq!(m.adapt_to_background, 0.4);
    assert!(m.features.contains(ui_glass::GlassFeatures::TINT_ADAPT));
}

#[test]
fn builder_radius_and_saturation_do_not_flip_features() {
    let m = LiquidMaterial::new().radius(20.0).saturation(1.6);
    assert_eq!(m.radius_px, 20.0);
    assert_eq!(m.saturation, 1.6);
    assert_eq!(m.features, ui_glass::GlassFeatures::empty());
}

#[test]
fn builder_chains_compose_features() {
    let m = LiquidMaterial::new()
        .blur(18.0)
        .refract(0.3)
        .specular(0.78, 0.7)
        .pointer_reactive();
    assert!(m.features.contains(ui_glass::GlassFeatures::BLUR));
    assert!(m.features.contains(ui_glass::GlassFeatures::REFRACT));
    assert!(m.features.contains(ui_glass::GlassFeatures::SPECULAR));
    assert!(m.features.contains(ui_glass::GlassFeatures::POINTER));
}
