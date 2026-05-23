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
