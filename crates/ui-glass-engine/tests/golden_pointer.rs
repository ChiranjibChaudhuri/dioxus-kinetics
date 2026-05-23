mod common;
use common::{golden_check, prepare_pointer_scene};
use ui_glass::LiquidMaterial;

#[test]
fn pointer_isolated_matches_golden() {
    let mat = LiquidMaterial::new()
        .blur(8.0)
        .refract(0.4)
        .noise(2.0, 0.0)
        .surface_curvature(0.5)
        .thickness(2.0)
        .pointer_reactive()
        .radius(20.0)
        .tint(ui_tokens::Color::rgba(255, 255, 255, 1.0), 0.2);
    let pixels = prepare_pointer_scene(128, 128, mat, [0.6, -0.3]);
    golden_check("tests/assets/pointer_128.png", &pixels, 128, 128);
}
