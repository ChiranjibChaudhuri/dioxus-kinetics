//! Parametric path support for `MotionCue::Path`. Points are emitted
//! sequentially — the first point's `end` is the starting position;
//! every subsequent segment connects the previous endpoint to the
//! next point's `end` either as a straight line or a cubic Bezier.
//!
//! `sample_path_parametric` walks the segments uniformly by parameter
//! (not by arc length). Arc-length-uniform sampling is layered on top
//! in a follow-up task so the cinematic showcase doesn't accelerate
//! visibly through high-curvature regions.

/// A single point on a parametric path.
#[derive(Clone, Debug, PartialEq)]
pub enum PathPoint {
    /// Straight line from the previous point's endpoint to `end`. When
    /// `PathPoint::Line` is the first point in a path, its `end` is the
    /// path's starting position (no segment is drawn into it).
    Line { end: (f32, f32) },
    /// Cubic Bezier from the previous point's endpoint through
    /// `control_1` and `control_2` to `end`.
    Bezier {
        control_1: (f32, f32),
        control_2: (f32, f32),
        end: (f32, f32),
    },
}

impl PathPoint {
    pub fn end(&self) -> (f32, f32) {
        match self {
            PathPoint::Line { end } => *end,
            PathPoint::Bezier { end, .. } => *end,
        }
    }
}

/// Sample the path at parameter `t ∈ [0, 1]`. `t` outside the range
/// clamps to the nearest endpoint. NaN clamps to the start.
///
/// An empty path returns the origin. A single-point path returns
/// that point's `end` for all `t`.
///
/// Segments are weighted uniformly by parameter, so a 2-segment
/// polyline has each segment span `t ∈ [0, 0.5]` and `t ∈ [0.5, 1]`.
pub fn sample_path_parametric(points: &[PathPoint], t: f32) -> (f32, f32) {
    if points.is_empty() {
        return (0.0, 0.0);
    }
    if points.len() == 1 {
        return points[0].end();
    }
    let t = if t.is_finite() { t.clamp(0.0, 1.0) } else { 0.0 };

    let segment_count = (points.len() - 1) as f32;
    let scaled = t * segment_count;
    let mut idx = scaled.floor() as usize;
    let mut local = scaled - idx as f32;
    if idx >= points.len() - 1 {
        idx = points.len() - 2;
        local = 1.0;
    }

    let start = points[idx].end();
    match &points[idx + 1] {
        PathPoint::Line { end } => lerp(start, *end, local),
        PathPoint::Bezier {
            control_1,
            control_2,
            end,
        } => de_casteljau(start, *control_1, *control_2, *end, local),
    }
}

fn lerp(a: (f32, f32), b: (f32, f32), t: f32) -> (f32, f32) {
    (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
}

fn de_casteljau(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
    t: f32,
) -> (f32, f32) {
    let a = lerp(p0, p1, t);
    let b = lerp(p1, p2, t);
    let c = lerp(p2, p3, t);
    let d = lerp(a, b, t);
    let e = lerp(b, c, t);
    lerp(d, e, t)
}
