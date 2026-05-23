mod common;
use common::{render_with_material, golden_check};
use ui_glass::LiquidMaterial;

#[test]
fn specular_isolated_matches_golden() {
    let pixels = render_with_material(
        128, 128,
        LiquidMaterial::new()
            .blur(8.0)
            .specular(45.0_f32.to_radians(), 0.8)
            .edge_falloff(2.0)
            .radius(20.0)
            .tint(ui_tokens::Color::rgba(255, 255, 255, 1.0), 0.2),
    );
    golden_check("tests/assets/specular_128.png", &pixels, 128, 128);
}
