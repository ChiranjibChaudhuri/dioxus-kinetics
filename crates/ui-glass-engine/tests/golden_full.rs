mod common;
use common::{golden_check, prepare_pointer_scene};
use ui_glass::{AmbientMesh, LiquidMaterial};

#[test]
fn all_features_enabled_matches_golden() {
    let mat = LiquidMaterial::new()
        .blur(10.0)
        .refract(0.4)
        .noise(2.5, 0.0)
        .surface_curvature(0.5)
        .thickness(2.0)
        .disperse(3.0)
        .specular(45.0_f32.to_radians(), 0.8)
        .edge_falloff(2.0)
        .inner_shadow(6.0, 0.35)
        .ambient_mesh(AmbientMesh::Aurora)
        .pointer_reactive()
        .scroll_reactive()
        .adapt_to_background(0.4)
        .radius(22.0)
        .tint(ui_tokens::Color::rgba(255, 255, 255, 1.0), 0.18);
    let pixels = prepare_pointer_scene(128, 128, mat, [0.4, -0.2]);
    golden_check("tests/assets/full_128.png", &pixels, 128, 128);
}
