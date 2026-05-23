mod common;
use common::{golden_check, render_with_material};
use ui_glass::{AmbientMesh, LiquidMaterial};

#[test]
fn ambient_mesh_isolated_matches_golden() {
    // time_seconds is 0 in the test pipeline (no rAF clock); the mesh evaluates
    // to a deterministic configuration suitable for golden capture.
    let pixels = render_with_material(
        128,
        128,
        LiquidMaterial::new()
            .blur(8.0)
            .ambient_mesh(AmbientMesh::Aurora)
            .radius(20.0)
            .tint(ui_tokens::Color::rgba(255, 255, 255, 1.0), 0.1),
    );
    golden_check("tests/assets/ambient_mesh_128.png", &pixels, 128, 128);
}
