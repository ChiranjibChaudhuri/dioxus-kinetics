use ui_glass::LiquidMaterial;
use ui_glass_engine::svg_fallback::{defs_for, filter_element, filter_id};

#[test]
fn filter_id_stable_for_same_material() {
    let m = LiquidMaterial::floating();
    assert_eq!(filter_id(&m), filter_id(&m));
}

#[test]
fn filter_id_differs_for_different_materials() {
    let a = LiquidMaterial::floating();
    let b = LiquidMaterial::chrome();
    assert_ne!(filter_id(&a), filter_id(&b));
}

#[test]
fn filter_element_contains_blur_when_set() {
    let m = LiquidMaterial::new().blur(12.0);
    let el = filter_element(&m);
    assert!(el.contains("feGaussianBlur"), "expected feGaussianBlur, got {el}");
}

#[test]
fn filter_element_contains_displacement_when_refract() {
    let m = LiquidMaterial::new().refract(0.3).noise(2.0, 0.0);
    let el = filter_element(&m);
    assert!(el.contains("feDisplacementMap"));
    assert!(el.contains("feTurbulence"));
}

#[test]
fn filter_element_contains_specular_when_set() {
    let m = LiquidMaterial::new().specular(45.0_f32.to_radians(), 0.8);
    let el = filter_element(&m);
    assert!(el.contains("feSpecularLighting"));
}

#[test]
fn defs_for_wraps_multiple_filters_in_svg_defs() {
    let a = LiquidMaterial::floating();
    let b = LiquidMaterial::chrome();
    let defs = defs_for(&[&a, &b]);
    assert!(defs.starts_with("<svg"));
    assert!(defs.contains("<defs>"));
    assert!(defs.contains(&filter_id(&a)));
    assert!(defs.contains(&filter_id(&b)));
}
