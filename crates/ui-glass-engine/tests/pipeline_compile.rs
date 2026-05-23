use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::pipeline::{build_blur_pipeline, BlurDirection};

#[test]
fn blur_horizontal_pipeline_compiles() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let _p = build_blur_pipeline(h.device(), BlurDirection::Horizontal, 13);
}

#[test]
fn blur_vertical_pipeline_compiles() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let _p = build_blur_pipeline(h.device(), BlurDirection::Vertical, 13);
}

#[test]
fn blur_pipeline_compiles_for_smaller_tap_count() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let _p = build_blur_pipeline(h.device(), BlurDirection::Horizontal, 5);
}

use ui_glass::GlassFeatures;
use ui_glass_engine::pipeline::{build_compose_pipeline, ComposeKey};

#[test]
fn compose_pipeline_compiles_with_blur_only() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let key = ComposeKey { features: GlassFeatures::BLUR };
    let _p = build_compose_pipeline(h.device(), key);
}

#[test]
fn compose_pipeline_compiles_with_all_features_off() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let key = ComposeKey { features: GlassFeatures::empty() };
    let _p = build_compose_pipeline(h.device(), key);
}

#[test]
fn compose_key_is_hashable_for_cache() {
    let a = ComposeKey { features: GlassFeatures::BLUR };
    let b = ComposeKey { features: GlassFeatures::BLUR };
    use std::hash::{Hash, Hasher};
    let mut h1 = std::collections::hash_map::DefaultHasher::new();
    let mut h2 = std::collections::hash_map::DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}
