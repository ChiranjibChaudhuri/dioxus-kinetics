use ui_motion::{Ease, Transition};
use ui_timeline::{MotionCue, MotionCueSample, PathPoint};

fn linear() -> Transition {
    Transition::Tween {
        duration_ms: 1000,
        ease: Ease::Linear,
    }
}

fn straight_horizontal() -> Vec<PathPoint> {
    vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 0.0) },
    ]
}

#[test]
fn path_cue_samples_translate_at_progress_zero() {
    let cue = MotionCue::Path {
        points: straight_horizontal(),
        from_progress: 0.0,
        to_progress: 1.0,
        rotate_along_path: false,
        transition: linear(),
    };
    let s: MotionCueSample = cue.sample(0.0);
    assert_eq!(s.translate_x, Some(0.0));
    assert_eq!(s.translate_y, Some(0.0));
    assert_eq!(s.rotate_deg, None);
}

#[test]
fn path_cue_samples_translate_at_progress_one() {
    let cue = MotionCue::Path {
        points: straight_horizontal(),
        from_progress: 0.0,
        to_progress: 1.0,
        rotate_along_path: false,
        transition: linear(),
    };
    let s = cue.sample(1.0);
    let x = s.translate_x.unwrap();
    let y = s.translate_y.unwrap();
    assert!((x - 100.0).abs() < 1.0, "x: {x}");
    assert!(y.abs() < 1.0, "y: {y}");
}

#[test]
fn path_cue_with_rotate_along_path_emits_angle() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 100.0) },
    ];
    let cue = MotionCue::Path {
        points: pts,
        from_progress: 0.0,
        to_progress: 1.0,
        rotate_along_path: true,
        transition: linear(),
    };
    let s = cue.sample(0.5);
    let angle = s.rotate_deg.unwrap();
    assert!((angle - 45.0).abs() < 2.0, "angle: {angle}");
}

#[test]
fn path_cue_sub_segment_traversal() {
    // from_progress=0.5, to_progress=1.0 means at cue progress=0.0
    // the position is the midpoint of the path; at cue progress=1.0
    // the position is the endpoint.
    let cue = MotionCue::Path {
        points: straight_horizontal(),
        from_progress: 0.5,
        to_progress: 1.0,
        rotate_along_path: false,
        transition: linear(),
    };
    let s_start = cue.clone().sample(0.0);
    assert!((s_start.translate_x.unwrap() - 50.0).abs() < 1.0);
    let s_end = cue.sample(1.0);
    assert!((s_end.translate_x.unwrap() - 100.0).abs() < 1.0);
}

#[test]
fn path_cue_reduced_motion_collapses_to_endpoint() {
    let cue = MotionCue::Path {
        points: straight_horizontal(),
        from_progress: 0.0,
        to_progress: 1.0,
        rotate_along_path: false,
        transition: linear(),
    };
    // The reduced_motion() variant — accessible via Timeline::reduced_motion
    // — collapses duration_ms to 0 in the surrounding MotionSegment, so any
    // progress sample emits the cue at progress=1.0. Verify here by sampling
    // the reduced cue at progress=0.0 and expecting the endpoint.
    let reduced = cue.reduced_motion();
    let s = reduced.sample(0.0);
    assert!((s.translate_x.unwrap() - 100.0).abs() < 1.0);
}
