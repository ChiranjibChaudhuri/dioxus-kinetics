use ui_glass_engine::background::{BackgroundSource, Gradient, GradientStop};
use ui_glass_engine::background::render::BackgroundRenderer;
use ui_glass_engine::headless::TestHarness;
use ui_tokens::Color;

#[test]
fn renderer_creates_pipeline() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let _r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
}

#[test]
fn linear_gradient_produces_color_transition() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    let g = Gradient::linear(
        0.0,  // horizontal
        vec![
            GradientStop { offset: 0.0, color: Color::rgba(255,   0,   0, 1.0) },
            GradientStop { offset: 1.0, color: Color::rgba(  0,   0, 255, 1.0) },
        ],
    );
    let pixels = r.render_to_pixels(&[BackgroundSource::Gradient(g)], 64, 64);
    let left = (pixels[0], pixels[1], pixels[2]);
    let right_idx = (63 * 4) as usize;
    let right = (pixels[right_idx], pixels[right_idx + 1], pixels[right_idx + 2]);
    assert!(left.0 > 100, "left should be red, got {left:?}");
    assert!(right.2 > 100, "right should be blue, got {right:?}");
}

#[test]
fn solid_color_fills_whole_texture() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    let pixels = r.render_to_pixels(
        &[BackgroundSource::Color(Color::rgba(0, 128, 0, 1.0))],
        32, 32,
    );
    let center_idx = ((16 * 32 + 16) * 4) as usize;
    assert!(pixels[center_idx + 1] > 50, "expected greenish center");
}
