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

use ui_glass::GlassFeatures as F;

#[test]
fn preset_floating_has_blur_specular_inner_shadow() {
    let m = LiquidMaterial::floating();
    assert!(m.features.contains(F::BLUR));
    assert!(m.features.contains(F::SPECULAR));
    assert!(m.features.contains(F::INNER_SHADOW));
    assert!(m.blur_radius_px > 0.0);
    assert!(m.radius_px > 0.0);
}

#[test]
fn preset_chrome_has_heavy_blur_low_refract() {
    let m = LiquidMaterial::chrome();
    assert!(m.features.contains(F::BLUR));
    assert!(m.blur_radius_px >= 28.0);
    if m.features.contains(F::REFRACT) {
        assert!(m.refraction_strength <= 0.2);
    }
}

#[test]
fn preset_overlay_has_strong_refract_and_disperse() {
    let m = LiquidMaterial::overlay();
    assert!(m.features.contains(F::REFRACT));
    assert!(m.features.contains(F::DISPERSE));
    assert!(m.refraction_strength >= 0.3);
}

#[test]
fn preset_sheet_has_ambient_mesh() {
    let m = LiquidMaterial::sheet();
    assert!(m.features.contains(F::AMBIENT_MESH));
}

#[test]
fn preset_tooltip_has_no_reactivity() {
    let m = LiquidMaterial::tooltip();
    assert!(!m.features.contains(F::POINTER));
    assert!(!m.features.contains(F::SCROLL));
}

#[test]
fn preset_button_is_pointer_reactive() {
    let m = LiquidMaterial::button();
    assert!(m.features.contains(F::POINTER));
}

use ui_glass::{
    GlassDepth, MaterialDensity, MaterialEdge, MaterialPolicy, MaterialRequest, MaterialTone,
    MaterialVibrancy,
};

#[test]
fn material_request_floating_neutral_maps_to_floating_preset_baseline() {
    let req = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral);
    let m: LiquidMaterial = req.into();
    assert!(m.features.contains(F::BLUR));
    assert!(m.blur_radius_px >= 16.0 && m.blur_radius_px <= 20.0);
}

#[test]
fn material_request_modal_maps_to_overlay_preset_strength() {
    let req = MaterialRequest::new(GlassDepth::Modal, MaterialTone::Primary);
    let m: LiquidMaterial = req.into();
    assert!(m.features.contains(F::REFRACT));
    assert!(m.refraction_strength >= 0.3);
}

#[test]
fn material_request_high_contrast_clears_reactive_and_visual_features() {
    let req = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_policy(MaterialPolicy::HighContrast);
    let m: LiquidMaterial = req.into();
    assert!(!m.features.contains(F::REFRACT));
    assert!(!m.features.contains(F::DISPERSE));
    assert!(!m.features.contains(F::SPECULAR));
    assert!(!m.features.contains(F::POINTER));
    assert!(!m.features.contains(F::SCROLL));
    assert!(!m.features.contains(F::AMBIENT_MESH));
}

#[test]
fn material_request_vivid_vibrancy_increases_saturation_and_dispersion() {
    let std = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_vibrancy(MaterialVibrancy::Standard);
    let vivid = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_vibrancy(MaterialVibrancy::Vivid);
    let ms: LiquidMaterial = std.into();
    let mv: LiquidMaterial = vivid.into();
    assert!(mv.saturation > ms.saturation);
    assert!(mv.dispersion_px >= ms.dispersion_px);
}

#[test]
fn material_request_emphasized_edge_increases_falloff_and_thickness() {
    let hair = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_edge(MaterialEdge::Hairline);
    let emph = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_edge(MaterialEdge::Emphasized);
    let mh: LiquidMaterial = hair.into();
    let me: LiquidMaterial = emph.into();
    assert!(me.edge_falloff_px > mh.edge_falloff_px);
    assert!(me.thickness_px > mh.thickness_px);
}

#[test]
fn material_request_compact_density_reduces_radius() {
    let comp = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_density(MaterialDensity::Compact);
    let spac = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_density(MaterialDensity::Spacious);
    let mc: LiquidMaterial = comp.into();
    let ms: LiquidMaterial = spac.into();
    assert!(mc.radius_px < ms.radius_px);
}
