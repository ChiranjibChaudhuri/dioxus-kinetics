use ui_glass_engine::background::{BackgroundScene, BackgroundSource, MeshKind};
use ui_glass_engine::background::render::BackgroundRenderer;
use ui_glass_engine::headless::TestHarness;
use ui_tokens::Color;

#[test]
fn scene_with_multiple_layers_composites() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    let scene = BackgroundScene::new()
        .layer(BackgroundSource::Color(Color::rgba(20, 20, 80, 1.0)))
        .layer(BackgroundSource::Mesh(MeshKind::Aurora));

    let pixels = r.render_to_pixels(&scene.layers, 32, 32);
    let mut max_blue = 0u8;
    for chunk in pixels.chunks(4) {
        if chunk[2] > max_blue { max_blue = chunk[2]; }
    }
    assert!(max_blue > 40, "expected the dark blue base to show through");
}

#[test]
fn empty_scene_returns_black_texture() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    let scene = BackgroundScene::new();
    assert!(scene.is_empty());
    let pixels = r.render_to_pixels(&scene.layers, 32, 32);
    let center = ((16 * 32 + 16) * 4) as usize;
    assert!(pixels[center] < 16, "expected black center for empty scene");
}
