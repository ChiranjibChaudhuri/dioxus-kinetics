mod common;
use common::{render_with_material, golden_check};
use ui_glass::LiquidMaterial;

#[test]
fn tint_adapt_isolated_matches_golden() {
    let pixels = render_with_material(
        128, 128,
        LiquidMaterial::new()
            .blur(8.0)
            .adapt_to_background(0.6)
            .radius(20.0)
            .tint(ui_tokens::Color::rgba(255, 255, 255, 1.0), 0.05),
    );
    golden_check("tests/assets/tint_adapt_128.png", &pixels, 128, 128);
}
