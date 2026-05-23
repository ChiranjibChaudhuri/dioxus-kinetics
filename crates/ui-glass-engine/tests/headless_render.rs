use ui_glass_engine::headless::TestHarness;

#[test]
fn harness_initializes_and_returns_device() {
    let h = pollster::block_on(TestHarness::new()).expect("device init");
    assert!(h.canvas_size().0 > 0);
}

#[test]
fn harness_clear_to_color_returns_solid_pixels() {
    let mut h = pollster::block_on(TestHarness::new()).expect("device init");
    let pixels = h.clear_and_read(64, 64, [0.0, 0.0, 1.0, 1.0]);
    // First pixel should be blue
    assert_eq!(pixels[0], 0);
    assert_eq!(pixels[1], 0);
    assert_eq!(pixels[2], 255);
    assert_eq!(pixels[3], 255);
}
