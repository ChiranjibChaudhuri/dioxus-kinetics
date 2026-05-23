mod common;
use common::{golden_check, render_with_material};
use ui_glass::LiquidMaterial;

#[test]
fn inner_shadow_isolated_matches_golden() {
    let pixels = render_with_material(
        128,
        128,
        LiquidMaterial::new()
            .blur(8.0)
            .inner_shadow(8.0, 0.45)
            .radius(20.0)
            .tint(ui_tokens::Color::rgba(255, 255, 255, 1.0), 0.2),
    );
    golden_check("tests/assets/inner_shadow_128.png", &pixels, 128, 128);
}
