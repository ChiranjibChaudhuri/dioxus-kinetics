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
