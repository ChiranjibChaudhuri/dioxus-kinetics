use ui_glass_engine::background::{BackgroundSource, Gradient, GradientStop, ImageSource, MeshKind};
use ui_glass_engine::background::render::BackgroundRenderer;
use ui_glass_engine::background::ImageCache;
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

#[test]
fn aurora_mesh_produces_non_uniform_output() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    let pixels = r.render_to_pixels(&[BackgroundSource::Mesh(MeshKind::Aurora)], 64, 64);
    let mut min = 255u8;
    let mut max = 0u8;
    for chunk in pixels.chunks(4) {
        for c in &chunk[..3] {
            if *c < min { min = *c; }
            if *c > max { max = *c; }
        }
    }
    assert!(max - min > 40, "aurora should vary across the texture; got range {min}..{max}");
}

#[test]
fn orbs_and_grain_produce_distinct_outputs() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    let orbs = r.render_to_pixels(&[BackgroundSource::Mesh(MeshKind::Orbs)], 32, 32);
    let grain = r.render_to_pixels(&[BackgroundSource::Mesh(MeshKind::Grain)], 32, 32);
    assert_ne!(orbs, grain, "orbs and grain should render differently");
}

#[test]
fn dynamic_image_can_be_uploaded_and_sampled() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut cache = ImageCache::new(h.device().clone(), h.queue().clone());
    let pixels = vec![255u8, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255];
    let handle = cache.upload_rgba(&pixels, 2, 2);
    assert!(cache.get(&handle).is_some());

    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    r.set_image_cache(cache);
    let out = r.render_to_pixels(
        &[BackgroundSource::Image(ImageSource::Dynamic(handle))],
        4, 4,
    );
    let nonzero = out.iter().filter(|&&b| b > 16).count();
    assert!(nonzero > 0, "expected non-black pixels from uploaded image");
}
