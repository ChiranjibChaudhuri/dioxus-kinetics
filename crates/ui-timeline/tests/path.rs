use ui_timeline::{sample_path_parametric, PathPoint};

fn approx(a: (f32, f32), b: (f32, f32), tol: f32) -> bool {
    (a.0 - b.0).abs() < tol && (a.1 - b.1).abs() < tol
}

#[test]
fn empty_path_returns_origin() {
    let pts: Vec<PathPoint> = vec![];
    let p = sample_path_parametric(&pts, 0.5);
    assert!(approx(p, (0.0, 0.0), 1e-6));
}

#[test]
fn single_line_lerp() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 0.0) },
    ];
    assert!(approx(sample_path_parametric(&pts, 0.0), (0.0, 0.0), 1e-3));
    assert!(approx(sample_path_parametric(&pts, 0.5), (50.0, 0.0), 1e-3));
    assert!(approx(
        sample_path_parametric(&pts, 1.0),
        (100.0, 0.0),
        1e-3
    ));
}

#[test]
fn two_segment_polyline() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 0.0) },
        PathPoint::Line {
            end: (100.0, 100.0),
        },
    ];
    // t=0.25 is at the midpoint of the first half of the polyline
    // (parametrically, not by arc length).
    assert!(approx(
        sample_path_parametric(&pts, 0.25),
        (50.0, 0.0),
        1e-3
    ));
    // t=0.75 is at the midpoint of the second segment.
    assert!(approx(
        sample_path_parametric(&pts, 0.75),
        (100.0, 50.0),
        1e-3
    ));
}

#[test]
fn cubic_bezier_at_endpoints() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Bezier {
            control_1: (33.0, 100.0),
            control_2: (66.0, -100.0),
            end: (100.0, 0.0),
        },
    ];
    assert!(approx(sample_path_parametric(&pts, 0.0), (0.0, 0.0), 1e-3));
    assert!(approx(
        sample_path_parametric(&pts, 1.0),
        (100.0, 0.0),
        1e-3
    ));
}

#[test]
fn cubic_bezier_at_midpoint() {
    // Standard cubic Bezier at t=0.5 with control points (33,100), (66,-100).
    // De Casteljau midpoint = (1/8)*P0 + (3/8)*C1 + (3/8)*C2 + (1/8)*P3
    //                       = (1/8)*0   + (3/8)*33  + (3/8)*66  + (1/8)*100
    //                       = 49.5 (x)
    // y = (3/8)*100 + (3/8)*(-100) = 0
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Bezier {
            control_1: (33.0, 100.0),
            control_2: (66.0, -100.0),
            end: (100.0, 0.0),
        },
    ];
    let mid = sample_path_parametric(&pts, 0.5);
    assert!((mid.0 - 49.5).abs() < 1.0, "x midpoint: {}", mid.0);
    assert!(mid.1.abs() < 1.0, "y midpoint: {}", mid.1);
}

#[test]
fn t_below_zero_clamps_to_origin() {
    let pts = vec![
        PathPoint::Line { end: (10.0, 20.0) },
        PathPoint::Line {
            end: (100.0, 100.0),
        },
    ];
    assert!(approx(
        sample_path_parametric(&pts, -1.0),
        (10.0, 20.0),
        1e-3
    ));
}

#[test]
fn t_above_one_clamps_to_endpoint() {
    let pts = vec![
        PathPoint::Line { end: (10.0, 20.0) },
        PathPoint::Line {
            end: (100.0, 100.0),
        },
    ];
    assert!(approx(
        sample_path_parametric(&pts, 2.0),
        (100.0, 100.0),
        1e-3
    ));
}

#[test]
fn nan_t_returns_origin() {
    let pts = vec![
        PathPoint::Line { end: (5.0, 5.0) },
        PathPoint::Line { end: (10.0, 10.0) },
    ];
    assert!(approx(
        sample_path_parametric(&pts, f32::NAN),
        (5.0, 5.0),
        1e-3
    ));
}

use ui_timeline::{sample_path, sample_path_tangent};

#[test]
fn arc_length_sampling_constant_speed_on_polyline() {
    // L-shaped polyline: 100 units across, then 100 units up.
    // Total arc length = 200. At t=0.5 (half arc length) we should be
    // at exactly (100, 0) — the corner.
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 0.0) },
        PathPoint::Line {
            end: (100.0, 100.0),
        },
    ];
    let half = sample_path(&pts, 0.5);
    assert!(approx(half, (100.0, 0.0), 1.0), "got {:?}", half);
}

#[test]
fn arc_length_sampling_quarter_eighth_polyline() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 0.0) },
        PathPoint::Line {
            end: (100.0, 100.0),
        },
    ];
    // Arc length = 200; t=0.25 → 50 units along, which is (50, 0).
    assert!(approx(sample_path(&pts, 0.25), (50.0, 0.0), 1.0));
    // t=0.75 → 150 units along: 100 units in segment 1 + 50 in segment 2 → (100, 50).
    assert!(approx(sample_path(&pts, 0.75), (100.0, 50.0), 1.0));
}

#[test]
fn arc_length_clamps_outside_range() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 0.0) },
    ];
    assert!(approx(sample_path(&pts, -0.5), (0.0, 0.0), 1e-3));
    assert!(approx(sample_path(&pts, 1.5), (100.0, 0.0), 1e-3));
}

#[test]
fn tangent_on_horizontal_line_is_zero_degrees() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 0.0) },
    ];
    let angle = sample_path_tangent(&pts, 0.5);
    assert!(angle.abs() < 1.0, "horizontal tangent: {}", angle);
}

#[test]
fn tangent_on_vertical_segment_is_ninety_degrees() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (0.0, 100.0) },
    ];
    let angle = sample_path_tangent(&pts, 0.5);
    assert!(
        (angle.abs() - 90.0).abs() < 1.0,
        "vertical tangent: {}",
        angle
    );
}

#[test]
fn tangent_on_diagonal_segment_is_forty_five_degrees() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line {
            end: (100.0, 100.0),
        },
    ];
    let angle = sample_path_tangent(&pts, 0.5);
    assert!((angle - 45.0).abs() < 1.0, "diagonal tangent: {}", angle);
}
