mod common;
use common::{golden_check, render_with_material};
use ui_glass::LiquidMaterial;

#[test]
fn disperse_isolated_matches_golden() {
    let pixels = render_with_material(
        128,
        128,
        LiquidMaterial::new()
            .blur(8.0)
            .disperse(4.0)
            .radius(20.0)
            .tint(ui_tokens::Color::rgba(255, 255, 255, 1.0), 0.1),
    );
    golden_check("tests/assets/disperse_128.png", &pixels, 128, 128);
}
