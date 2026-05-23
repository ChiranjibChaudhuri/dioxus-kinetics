use ui_glass_engine::background::{BackgroundSource, Gradient, GradientStop, MeshKind};
use ui_tokens::Color;

#[test]
fn color_source_constructs() {
    let src = BackgroundSource::Color(Color::rgba(10, 20, 30, 1.0));
    match src {
        BackgroundSource::Color(c) => assert_eq!(c.r, 10),
        _ => panic!("expected Color variant"),
    }
}

#[test]
fn gradient_linear_with_stops() {
    let g = Gradient::linear(
        0.5,
        vec![
            GradientStop { offset: 0.0, color: Color::rgba(0, 0, 0, 1.0) },
            GradientStop { offset: 1.0, color: Color::rgba(255, 255, 255, 1.0) },
        ],
    );
    assert_eq!(g.stops().len(), 2);
    assert!(g.is_linear());
}

#[test]
fn gradient_radial_with_center_and_stops() {
    let g = Gradient::radial(
        [0.5, 0.5],
        0.7,
        vec![GradientStop { offset: 0.0, color: Color::rgba(255, 0, 0, 1.0) }],
    );
    assert!(g.is_radial());
}

#[test]
fn gradient_conic_with_angle() {
    let g = Gradient::conic(
        [0.5, 0.5],
        0.0,
        vec![GradientStop { offset: 0.0, color: Color::rgba(0, 255, 0, 1.0) }],
    );
    assert!(g.is_conic());
}

#[test]
fn mesh_variants_exist() {
    let _a = BackgroundSource::Mesh(MeshKind::Aurora);
    let _o = BackgroundSource::Mesh(MeshKind::Orbs);
    let _g = BackgroundSource::Mesh(MeshKind::Grain);
}
