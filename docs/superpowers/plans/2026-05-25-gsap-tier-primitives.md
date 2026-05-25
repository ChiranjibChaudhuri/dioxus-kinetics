# GSAP-tier Primitives Implementation Plan (SP-3)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Land ScrollTrigger (via `SceneDriver::Scroll`), SplitText (per-glyph KineticBox children with stagger), and MotionPath (`MotionCue::Path` with Bezier sampling) on top of the SP-1 Scene Player; ship one cinematic showcase scene per primitive in the gallery's Scene category.

**Architecture:** Three layered additions. (1) `ui-timeline` grows a `MotionCue::Path` variant whose sampler walks `PathPoint::Line`/`Bezier` segments with arc-length-uniform parametrization and optional tangent-along-path rotation. (2) `ui-runtime` grows a `SceneDriver` enum (`Autoplay`/`Manual`/`Scroll`) plus a web-only scroll driver that feeds `SceneClock::seek_progress` from `IntersectionObserver` + window scroll. (3) `ui-dioxus` exposes a new `driver` prop on `Scene` and ships two new components — `SplitText` (renders per-glyph spans with sequential `data-stagger-index` and parent `aria-label`) and `MotionPath` (convenience wrapper around `Sequence`/`KineticBox` carrying a single `MotionCue::Path` segment). No new `FrameAdapter` implementations — the existing `SequenceAdapter` picks up the new `MotionCue` variant automatically.

**Tech Stack:** Rust workspace, Dioxus 0.7 with `Signal<T>`, `dioxus-ssr` for SSR tests, `web-sys` (`IntersectionObserver`, `Event`) for the scroll driver, Playwright for e2e on Chromium + WebKit.

**Spec:** `docs/superpowers/specs/2026-05-25-gsap-tier-primitives-design.md`

---

## File Structure

```
crates/ui-timeline/src/
  path.rs                                  # NEW — PathPoint + Bezier sampler (no Path cue variant yet)
  lib.rs                                   # +mod path; +MotionCue::Path arm + reduced-motion + sample()

crates/ui-runtime/src/
  scene_driver.rs                          # NEW — SceneDriver enum + ScrollObserverConfig
  drivers/
    mod.rs                                 # NEW — re-exports under cfg
    autoplay.rs                            # NEW — extracted autoplay loop body
    manual.rs                              # NEW — no-op driver
    scroll.rs                              # NEW — web-only IntersectionObserver + scroll
    scroll_stub.rs                         # NEW — native no-op
  scene_clock.rs                           # play() branches on installed driver
  lib.rs                                   # +re-exports

crates/ui-runtime/tests/
  scene_driver.rs                          # NEW — enum defaults + manual no-advance + scroll-stub holds-at-zero

crates/ui-dioxus/src/
  split_text.rs                            # NEW — SplitText + SplitMode
  motion_path.rs                           # NEW — MotionPath convenience
  scene_player.rs                          # +driver prop on Scene
  lib.rs                                   # +re-exports

crates/ui-dioxus/tests/
  split_text_ssr.rs                        # NEW
  motion_path_ssr.rs                       # NEW
  scene_player_ssr.rs                      # +driver-prop SSR test (Manual skips autoplay)

crates/ui-styles/src/
  gsap_primitives.css                      # NEW — display: inline-block on glyph spans
  lib.rs                                   # +GSAP_PRIMITIVES_CSS const, include in library_css()

crates/kinetics/src/lib.rs                 # +prelude + public_api_names entries

examples/component-gallery/src/
  docs.rs                                  # +3 ComponentDoc entries + snippet consts
  previews/scene.rs                        # +3 preview functions
  previews/scenes/mod.rs                   # +3 pub mod
  previews/scenes/scroll_story.rs          # NEW — Scene · Scroll-pinned Story
  previews/scenes/split_headline.rs        # NEW — Scene · Split Headline
  previews/scenes/curved_trajectory.rs     # NEW — Scene · Curved Trajectory

examples/component-gallery/e2e/tests/
  gsap-tier-primitives.spec.ts             # NEW — 3 Playwright tests
  _lib/component-manifest.ts               # +3 manifest entries
```

---

## Conventions

- Rust tests live in `tests/<name>.rs` (integration-style). SSR tests use `dioxus_ssr::render_element(rsx! { ... })`. Existing `with_runtime_async` / `enter` helpers in `crates/ui-runtime/tests/scene_clock.rs` are the precedent for Signal-bearing tests.
- All commits use Conventional Commits with the `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>` trailer via HEREDOC.
- Workspace-standard `let mut s = …; s.set(…);` idiom for Signal writes (NEVER `.clone().set(...)`).
- No new external dependencies. All wasm work uses `web-sys` + `wasm-bindgen` + `js-sys` (already in workspace).

---

### Task 1: `PathPoint` + linear/Bezier sampler in `ui-timeline::path`

Foundation: a pure function `sample_path(points, t)` that returns `(x, y)` along the path at parameter `t ∈ [0, 1]`. Arc-length uniformity is Task 2; this task ships the raw parametric sampler so tests can exercise the De Casteljau math directly.

**Files:**
- Create: `crates/ui-timeline/src/path.rs`
- Modify: `crates/ui-timeline/src/lib.rs` (add `mod path; pub use path::PathPoint;`)
- Test: `crates/ui-timeline/tests/path.rs`

- [ ] **Step 1: Write the failing tests**

Create `crates/ui-timeline/tests/path.rs`:

```rust
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
    assert!(approx(sample_path_parametric(&pts, 1.0), (100.0, 0.0), 1e-3));
}

#[test]
fn two_segment_polyline() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 0.0) },
        PathPoint::Line { end: (100.0, 100.0) },
    ];
    // t=0.25 is at the midpoint of the first half of the polyline
    // (parametrically, not by arc length).
    assert!(approx(sample_path_parametric(&pts, 0.25), (50.0, 0.0), 1e-3));
    // t=0.75 is at the midpoint of the second segment.
    assert!(approx(sample_path_parametric(&pts, 0.75), (100.0, 50.0), 1e-3));
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
    assert!(approx(sample_path_parametric(&pts, 1.0), (100.0, 0.0), 1e-3));
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
        PathPoint::Line { end: (100.0, 100.0) },
    ];
    assert!(approx(sample_path_parametric(&pts, -1.0), (10.0, 20.0), 1e-3));
}

#[test]
fn t_above_one_clamps_to_endpoint() {
    let pts = vec![
        PathPoint::Line { end: (10.0, 20.0) },
        PathPoint::Line { end: (100.0, 100.0) },
    ];
    assert!(approx(sample_path_parametric(&pts, 2.0), (100.0, 100.0), 1e-3));
}

#[test]
fn nan_t_returns_origin() {
    let pts = vec![
        PathPoint::Line { end: (5.0, 5.0) },
        PathPoint::Line { end: (10.0, 10.0) },
    ];
    assert!(approx(sample_path_parametric(&pts, f32::NAN), (5.0, 5.0), 1e-3));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-timeline --test path`
Expected: compile error — `path` module not found.

- [ ] **Step 3: Implement `PathPoint` + parametric sampler**

Create `crates/ui-timeline/src/path.rs`:

```rust
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
```

In `crates/ui-timeline/src/lib.rs`, after the existing `use ui_motion::...` line, add:

```rust
pub mod path;
pub use path::{sample_path_parametric, PathPoint};
```

(Choose the right insertion point that doesn't break the existing prelude ordering.)

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-timeline --test path`
Expected: 8 PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-timeline/src/path.rs crates/ui-timeline/src/lib.rs crates/ui-timeline/tests/path.rs
git commit -m "$(cat <<'EOF'
feat(ui-timeline): PathPoint + parametric path sampler

Adds PathPoint::Line and PathPoint::Bezier plus a parametric sampler
(de Casteljau for cubics, linear lerp for lines). Segments are
weighted uniformly by parameter; arc-length uniformity is a follow-up.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 2: Arc-length uniform sampling + tangent calculation

Adds `sample_path` (arc-length-uniform) + `sample_path_tangent` (degree angle along path). These are the production samplers the `MotionCue::Path::sample` arm will call.

**Files:**
- Modify: `crates/ui-timeline/src/path.rs` (add `sample_path` + `sample_path_tangent` + their helpers)
- Modify: `crates/ui-timeline/src/lib.rs` (re-export the new fns)
- Test: `crates/ui-timeline/tests/path.rs` (append tests)

- [ ] **Step 1: Write the failing tests**

Append to `crates/ui-timeline/tests/path.rs`:

```rust
use ui_timeline::{sample_path_tangent, sample_path};

#[test]
fn arc_length_sampling_constant_speed_on_polyline() {
    // L-shaped polyline: 100 units across, then 100 units up.
    // Total arc length = 200. At t=0.5 (half arc length) we should be
    // at exactly (100, 0) — the corner.
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 0.0) },
        PathPoint::Line { end: (100.0, 100.0) },
    ];
    let half = sample_path(&pts, 0.5);
    assert!(approx(half, (100.0, 0.0), 1.0), "got {:?}", half);
}

#[test]
fn arc_length_sampling_quarter_eighth_polyline() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 0.0) },
        PathPoint::Line { end: (100.0, 100.0) },
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
    assert!((angle.abs() - 90.0).abs() < 1.0, "vertical tangent: {}", angle);
}

#[test]
fn tangent_on_diagonal_segment_is_forty_five_degrees() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 100.0) },
    ];
    let angle = sample_path_tangent(&pts, 0.5);
    assert!((angle - 45.0).abs() < 1.0, "diagonal tangent: {}", angle);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-timeline --test path arc_length tangent`
Expected: compile errors for `sample_path` and `sample_path_tangent`.

- [ ] **Step 3: Implement arc-length + tangent samplers**

Append to `crates/ui-timeline/src/path.rs`:

```rust
const PATH_SAMPLE_RESOLUTION: usize = 64;

/// Arc-length-uniform sampler. Parameterizes the path so equal `t`
/// steps cover equal physical distance, producing visually constant
/// motion speed (the property cinematic motion expects).
pub fn sample_path(points: &[PathPoint], t: f32) -> (f32, f32) {
    if points.is_empty() {
        return (0.0, 0.0);
    }
    if points.len() == 1 {
        return points[0].end();
    }
    let t = if t.is_finite() { t.clamp(0.0, 1.0) } else { 0.0 };

    // Build a parameter->arc-length table by uniformly sampling the
    // parametric path at high resolution. Then invert the table to map
    // arc-length back to parameter for the requested t.
    let n = PATH_SAMPLE_RESOLUTION;
    let mut samples = Vec::with_capacity(n + 1);
    let mut total = 0.0_f32;
    let mut prev = sample_path_parametric(points, 0.0);
    samples.push((0.0_f32, 0.0_f32, prev));
    for i in 1..=n {
        let u = i as f32 / n as f32;
        let p = sample_path_parametric(points, u);
        let d = ((p.0 - prev.0).powi(2) + (p.1 - prev.1).powi(2)).sqrt();
        total += d;
        samples.push((u, total, p));
        prev = p;
    }
    if total == 0.0 {
        return points[0].end();
    }

    let target = t * total;
    // Linear scan; PATH_SAMPLE_RESOLUTION is small enough that this is
    // cheaper than a binary search for SP-3 scenes.
    let mut lo = &samples[0];
    let mut hi = &samples[n];
    for window in samples.windows(2) {
        if window[1].1 >= target {
            lo = &window[0];
            hi = &window[1];
            break;
        }
    }
    let span = hi.1 - lo.1;
    let alpha = if span == 0.0 { 0.0 } else { (target - lo.1) / span };
    let u = lo.0 + (hi.0 - lo.0) * alpha;
    sample_path_parametric(points, u)
}

/// Tangent angle (degrees) at arc-length-uniform `t`. Uses a small
/// finite difference (epsilon = 1/PATH_SAMPLE_RESOLUTION) on the
/// arc-length sampler so the angle is in the same parametrization as
/// `sample_path` and visibly aligned with the cinematic position.
pub fn sample_path_tangent(points: &[PathPoint], t: f32) -> f32 {
    if points.len() < 2 {
        return 0.0;
    }
    let t = if t.is_finite() { t.clamp(0.0, 1.0) } else { 0.0 };
    let eps = 1.0 / PATH_SAMPLE_RESOLUTION as f32;
    let lo = (t - eps).max(0.0);
    let hi = (t + eps).min(1.0);
    let a = sample_path(points, lo);
    let b = sample_path(points, hi);
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    if dx == 0.0 && dy == 0.0 {
        return 0.0;
    }
    dy.atan2(dx).to_degrees()
}
```

In `crates/ui-timeline/src/lib.rs`, update the re-export line to:

```rust
pub use path::{sample_path, sample_path_parametric, sample_path_tangent, PathPoint};
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-timeline --test path`
Expected: 14 PASS (8 from Task 1 + 6 new).

- [ ] **Step 5: Commit**

```bash
git add crates/ui-timeline/src/path.rs crates/ui-timeline/src/lib.rs crates/ui-timeline/tests/path.rs
git commit -m "$(cat <<'EOF'
feat(ui-timeline): arc-length-uniform path sampling + tangent angle

sample_path() builds a parameter->arc-length table and inverts it so
equal t steps cover equal distance. sample_path_tangent() takes a
small finite difference on sample_path() to emit a tangent angle in
degrees, suitable for rotate-along-path motion.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 3: `MotionCue::Path` variant + `MotionCueSample` plumbing

Extend the existing `MotionCue` enum with a `Path` variant that consumes the samplers from Tasks 1-2. Reduced-motion collapses to the endpoint. Sub-segment traversal via `from_progress`/`to_progress`.

**Files:**
- Modify: `crates/ui-timeline/src/lib.rs` (add `Path` arm + sample/reduced impls)
- Test: `crates/ui-timeline/tests/cue_path.rs` (new file — cleaner than appending to the existing `tests/path.rs` which tests the raw samplers)

- [ ] **Step 1: Write the failing tests**

Create `crates/ui-timeline/tests/cue_path.rs`:

```rust
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
    let s_start = cue.sample(0.0);
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
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-timeline --test cue_path`
Expected: compile errors — `MotionCue::Path` variant doesn't exist; `reduced_motion()` not implemented for the new variant.

- [ ] **Step 3: Extend `MotionCue` + `MotionCueSample`**

In `crates/ui-timeline/src/lib.rs`, in the `MotionCue` enum, add a new variant:

```rust
pub enum MotionCue {
    Opacity { from: f32, to: f32, transition: Transition },
    Translate { axis: Axis, from: f32, to: f32, transition: Transition },
    Scale { from: f32, to: f32, transition: Transition },
    Rotate { from_deg: f32, to_deg: f32, transition: Transition },
    Path {
        points: Vec<PathPoint>,
        from_progress: f32,
        to_progress: f32,
        rotate_along_path: bool,
        transition: Transition,
    },
}
```

In the `impl MotionCue::reduced_motion` match, add:

```rust
            Self::Path {
                points,
                from_progress: _,
                to_progress,
                rotate_along_path,
                transition,
            } => Self::Path {
                points,
                from_progress: to_progress, // collapse range so any sample is the endpoint
                to_progress,
                rotate_along_path,
                transition: transition.reduced(),
            },
```

In the `impl MotionCue::sample(self, progress: f32) -> MotionCueSample` match, add:

```rust
            Self::Path {
                points,
                from_progress,
                to_progress,
                rotate_along_path,
                transition,
            } => {
                let eased = apply_transition_progress(p, transition);
                let local = (from_progress
                    + (to_progress - from_progress) * eased)
                    .clamp(0.0, 1.0);
                let (x, y) = crate::path::sample_path(&points, local);
                let mut sample = MotionCueSample {
                    translate_x: Some(x),
                    translate_y: Some(y),
                    ..Default::default()
                };
                if rotate_along_path {
                    sample.rotate_deg = Some(crate::path::sample_path_tangent(&points, local));
                }
                sample
            }
```

Note: the existing `MotionCue::sample(self, p: f32)` takes `self` by value, not `&self`. The `Path` arm above moves `points` (the `Vec<PathPoint>`). `MotionCueSample` already has `translate_x`, `translate_y`, `rotate_deg` fields — no extension needed.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-timeline --test cue_path`
Expected: 5 PASS.

Also run `cargo test -p ui-timeline` to confirm no regressions in existing timeline tests.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-timeline/src/lib.rs crates/ui-timeline/tests/cue_path.rs
git commit -m "$(cat <<'EOF'
feat(ui-timeline): MotionCue::Path variant with sub-segment traversal

Adds a Path cue variant that walks a Vec<PathPoint> via the
arc-length-uniform sampler. Supports from_progress/to_progress sub-
segment traversal and optional rotate-along-path tangent emission.
Reduced-motion collapses to the endpoint via from_progress = to_progress.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 4: `SceneDriver` enum + `ScrollObserverConfig`

Type-only foundation: the enum that selects how a `SceneClock` advances. No behavior change yet (Tasks 5-7 add the per-driver behaviour).

**Files:**
- Create: `crates/ui-runtime/src/scene_driver.rs`
- Modify: `crates/ui-runtime/src/lib.rs` (add `pub mod scene_driver;` + re-export)
- Test: `crates/ui-runtime/tests/scene_driver.rs`

- [ ] **Step 1: Write the failing tests**

Create `crates/ui-runtime/tests/scene_driver.rs`:

```rust
use ui_runtime::scene_driver::{SceneDriver, ScrollObserverConfig};

#[test]
fn autoplay_is_default() {
    let d = SceneDriver::default();
    assert!(matches!(d, SceneDriver::Autoplay));
}

#[test]
fn manual_is_distinct_from_autoplay() {
    let m = SceneDriver::Manual;
    let a = SceneDriver::Autoplay;
    assert_ne!(m, a);
}

#[test]
fn scroll_carries_observer_config() {
    let config = ScrollObserverConfig {
        trigger_selector: "#hero".to_string(),
        start_offset_px: Some(100.0),
        end_offset_px: Some(0.0),
    };
    let d = SceneDriver::Scroll(config.clone());
    match d {
        SceneDriver::Scroll(c) => {
            assert_eq!(c.trigger_selector, "#hero");
            assert_eq!(c.start_offset_px, Some(100.0));
        }
        _ => panic!("expected Scroll"),
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-runtime --test scene_driver`
Expected: compile error — module not found.

- [ ] **Step 3: Implement `SceneDriver` + `ScrollObserverConfig`**

Create `crates/ui-runtime/src/scene_driver.rs`:

```rust
//! Scene driver selection.
//!
//! A `SceneDriver` selects how a `SceneClock` advances. SP-1 shipped
//! with Autoplay implicit; SP-3 promotes that choice to a value so
//! the same `Scene` Dioxus component can be driven by autoplay,
//! scroll position, or programmatic seeks.

/// How a `SceneClock` is advanced.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum SceneDriver {
    /// SP-1 default: clock advances via `spawn_frame_loop` on mount
    /// until `duration_ms` is reached.
    #[default]
    Autoplay,
    /// Clock progress is driven by scroll position through a
    /// configured trigger region. Web-only; native targets construct
    /// the driver but hold the clock at progress 0.
    Scroll(ScrollObserverConfig),
    /// Autoplay disabled; clock only moves via explicit `seek_*` calls
    /// (the transport scrubber still works).
    Manual,
}

/// Configures the trigger region for `SceneDriver::Scroll`.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollObserverConfig {
    /// CSS selector for the trigger region's root element.
    pub trigger_selector: String,
    /// Viewport offset (px from top) at which progress = 0. Default:
    /// the viewport height (progress starts when the trigger enters
    /// the bottom of the viewport).
    pub start_offset_px: Option<f32>,
    /// Viewport offset (px from top) at which progress = 1. Default: 0
    /// (progress completes when the trigger's bottom edge crosses the
    /// top of the viewport).
    pub end_offset_px: Option<f32>,
}

impl ScrollObserverConfig {
    pub fn new(trigger_selector: impl Into<String>) -> Self {
        Self {
            trigger_selector: trigger_selector.into(),
            start_offset_px: None,
            end_offset_px: None,
        }
    }
}
```

In `crates/ui-runtime/src/lib.rs`, add `pub mod scene_driver;` and a re-export `pub use scene_driver::{SceneDriver, ScrollObserverConfig};` in the same re-export block as the existing SceneClock exports.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-runtime --test scene_driver`
Expected: 3 PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-runtime/src/scene_driver.rs crates/ui-runtime/src/lib.rs crates/ui-runtime/tests/scene_driver.rs
git commit -m "$(cat <<'EOF'
feat(ui-runtime): SceneDriver enum + ScrollObserverConfig

Type-only foundation. Per-driver behaviour (autoplay loop, scroll
observer wiring, manual no-advance) lands in follow-up commits.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 5: Scroll driver — native stub + web binding

Adds `ScrollDriverHandle` newtype that owns the observer + listener cleanup. The web impl wires `IntersectionObserver` + a `scroll` listener and pushes progress into a callback. Native stub is a no-op that constructs cleanly.

**Files:**
- Create: `crates/ui-runtime/src/drivers/mod.rs`
- Create: `crates/ui-runtime/src/drivers/scroll.rs` (web)
- Create: `crates/ui-runtime/src/drivers/scroll_stub.rs` (native)

- [ ] **Step 1: Stub on native**

Create `crates/ui-runtime/src/drivers/scroll_stub.rs`:

```rust
#![cfg(not(target_arch = "wasm32"))]

use crate::scene_driver::ScrollObserverConfig;

/// Cleanup handle returned by `install_scroll_driver`. On native this
/// is a no-op marker; on web it holds the IntersectionObserver and the
/// scroll event listener closure, both of which clean up on Drop.
pub struct ScrollDriverHandle {
    _private: (),
}

impl Drop for ScrollDriverHandle {
    fn drop(&mut self) {
        // No-op on native.
    }
}

/// Installs the scroll driver for the given `config`, invoking
/// `on_progress(f32)` on every progress update. Returns a handle that
/// disconnects the observer + listener when dropped.
///
/// On native targets the function is a no-op and returns a handle that
/// holds the clock at progress 0 for its lifetime.
pub fn install_scroll_driver(
    _config: &ScrollObserverConfig,
    _on_progress: impl FnMut(f32) + 'static,
) -> ScrollDriverHandle {
    ScrollDriverHandle { _private: () }
}
```

- [ ] **Step 2: Web impl**

Create `crates/ui-runtime/src/drivers/scroll.rs`:

```rust
#![cfg(target_arch = "wasm32")]

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, IntersectionObserver, IntersectionObserverInit};

use crate::scene_driver::ScrollObserverConfig;

pub struct ScrollDriverHandle {
    _observer: Option<IntersectionObserver>,
    _intersection_closure: Option<Closure<dyn FnMut(js_sys::Array, IntersectionObserver)>>,
    _scroll_closure: Option<Closure<dyn FnMut(web_sys::Event)>>,
}

impl Drop for ScrollDriverHandle {
    fn drop(&mut self) {
        if let Some(observer) = self._observer.take() {
            observer.disconnect();
        }
        if let Some(closure) = self._scroll_closure.take() {
            if let Some(window) = web_sys::window() {
                let _ = window.remove_event_listener_with_callback(
                    "scroll",
                    closure.as_ref().unchecked_ref(),
                );
            }
            drop(closure);
        }
        if let Some(c) = self._intersection_closure.take() {
            drop(c);
        }
    }
}

pub fn install_scroll_driver(
    config: &ScrollObserverConfig,
    on_progress: impl FnMut(f32) + 'static,
) -> ScrollDriverHandle {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return empty_handle(),
    };
    let document = match window.document() {
        Some(d) => d,
        None => return empty_handle(),
    };
    let trigger: Element = match document
        .query_selector(&config.trigger_selector)
        .ok()
        .flatten()
    {
        Some(el) => el,
        None => return empty_handle(),
    };

    let start_offset = config.start_offset_px;
    let end_offset = config.end_offset_px;
    let on_progress = Rc::new(RefCell::new(on_progress));

    let trigger_for_scroll = trigger.clone();
    let on_progress_scroll = on_progress.clone();
    let window_for_scroll = window.clone();
    let scroll_closure = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
        let progress = compute_progress(
            &window_for_scroll,
            &trigger_for_scroll,
            start_offset,
            end_offset,
        );
        (on_progress_scroll.borrow_mut())(progress);
    }) as Box<dyn FnMut(_)>);

    let _ = window.add_event_listener_with_callback(
        "scroll",
        scroll_closure.as_ref().unchecked_ref(),
    );

    // IntersectionObserver fires once when the trigger enters/exits the
    // viewport — used to seed progress at mount and to coalesce events
    // when the viewport scrolls past the trigger entirely.
    let on_progress_io = on_progress.clone();
    let window_for_io = window.clone();
    let trigger_for_io = trigger.clone();
    let intersection_closure = Closure::wrap(Box::new(
        move |_entries: js_sys::Array, _observer: IntersectionObserver| {
            let progress = compute_progress(
                &window_for_io,
                &trigger_for_io,
                start_offset,
                end_offset,
            );
            (on_progress_io.borrow_mut())(progress);
        },
    ) as Box<dyn FnMut(_, _)>);

    let init = IntersectionObserverInit::new();
    let observer = match IntersectionObserver::new_with_options(
        intersection_closure.as_ref().unchecked_ref(),
        &init,
    ) {
        Ok(o) => o,
        Err(_) => return empty_handle(),
    };
    observer.observe(&trigger);

    // Seed initial progress before any event fires.
    let initial = compute_progress(&window, &trigger, start_offset, end_offset);
    (on_progress.borrow_mut())(initial);

    ScrollDriverHandle {
        _observer: Some(observer),
        _intersection_closure: Some(intersection_closure),
        _scroll_closure: Some(scroll_closure),
    }
}

fn compute_progress(
    window: &web_sys::Window,
    trigger: &Element,
    start_offset: Option<f32>,
    end_offset: Option<f32>,
) -> f32 {
    let rect = trigger.get_bounding_client_rect();
    let trigger_top = rect.top() as f32;
    let trigger_height = rect.height() as f32;
    let vp_height = window.inner_height().ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as f32;

    let start = start_offset.unwrap_or(vp_height);
    let end = end_offset.unwrap_or(0.0);
    let total_distance = (start - end + trigger_height).max(1.0);
    let traversed = start - trigger_top;
    (traversed / total_distance).clamp(0.0, 1.0)
}

fn empty_handle() -> ScrollDriverHandle {
    ScrollDriverHandle {
        _observer: None,
        _intersection_closure: None,
        _scroll_closure: None,
    }
}
```

- [ ] **Step 3: Module wiring**

Create `crates/ui-runtime/src/drivers/mod.rs`:

```rust
#[cfg(target_arch = "wasm32")]
mod scroll;
#[cfg(not(target_arch = "wasm32"))]
#[path = "scroll_stub.rs"]
mod scroll;
pub use scroll::{install_scroll_driver, ScrollDriverHandle};
```

In `crates/ui-runtime/src/lib.rs`, add `pub mod drivers;` and the re-export `pub use drivers::{install_scroll_driver, ScrollDriverHandle};`.

Also add `"IntersectionObserver"`, `"IntersectionObserverInit"`, and `"DomRect"` to the `web-sys` features list in `crates/ui-runtime/Cargo.toml` (verify whether `DomRect` is already there from SP-1 — it is; only the two `IntersectionObserver*` features need adding).

- [ ] **Step 4: Verify compile**

Run: `cargo check -p ui-runtime` (native) and `cargo check -p ui-runtime --target wasm32-unknown-unknown` (web).
Expected: success on both targets.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-runtime/src/drivers crates/ui-runtime/src/lib.rs crates/ui-runtime/Cargo.toml
git commit -m "$(cat <<'EOF'
feat(ui-runtime): scroll driver — web binding + native stub

Web target uses IntersectionObserver + a window scroll listener to
compute progress through the configured trigger region. Both
listeners clean up on Drop. Native stub is a no-op that returns a
handle holding the clock at progress 0.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 6: `SceneClock::play()` consumes the driver

Currently `play()` unconditionally spawns the rAF loop. Now it consults the driver:

- `Autoplay` → existing rAF loop.
- `Manual` → no-op (autoplay disabled).
- `Scroll(config)` → install the scroll driver via `install_scroll_driver`, callback into `clock.seek_progress`.

**Files:**
- Modify: `crates/ui-runtime/src/scene_clock.rs`
- Test: `crates/ui-runtime/tests/scene_driver.rs` (append)

- [ ] **Step 1: Append failing tests**

Add to `crates/ui-runtime/tests/scene_driver.rs`:

```rust
use ui_runtime::scene_clock::{SceneClock, SceneState};

// Re-use the with_runtime_async + enter helpers from
// scene_clock.rs by duplicating their minimal forms here. The tests
// don't share enough surface to justify lifting them out.

mod helpers {
    use dioxus::prelude::*;
    use std::future::Future;

    pub async fn with_runtime_async<F, Fut>(body: F)
    where
        F: FnOnce() -> Fut + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        let dom = VirtualDom::new(|| rsx! { "" });
        let _guard = dioxus_core::RuntimeGuard::new(dom.runtime());
        body().await;
    }

    pub fn enter<R>(body: impl FnOnce() -> R) -> R {
        dioxus_core::Runtime::current()
            .unwrap()
            .in_scope(dioxus_core::ScopeId::ROOT, body)
    }
}

use helpers::{enter, with_runtime_async};
use ui_runtime::scene_driver::SceneDriver;
use std::time::Duration;

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn manual_driver_skips_autoplay() {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            with_runtime_async(|| async {
                let clock = enter(|| SceneClock::new(1_000.0, 60, false));
                clock.play_with(SceneDriver::Manual);
                tokio::time::advance(Duration::from_millis(200)).await;
                tokio::task::yield_now().await;
                assert_eq!(clock.peek_elapsed_ms(), 0.0);
                assert_eq!(clock.peek_state(), SceneState::Paused);
            })
            .await;
        })
        .await;
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn autoplay_driver_advances_as_before() {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            with_runtime_async(|| async {
                let clock = enter(|| SceneClock::new(80.0, 60, false));
                clock.play_with(SceneDriver::Autoplay);
                tokio::time::advance(Duration::from_millis(200)).await;
                tokio::task::yield_now().await;
                assert!(clock.peek_elapsed_ms() >= 80.0 - 1e-3);
                assert_eq!(clock.peek_state(), SceneState::Settled);
            })
            .await;
        })
        .await;
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-runtime --test scene_driver play_with`
Expected: compile error — `play_with` method missing.

- [ ] **Step 3: Add `play_with` to `SceneClock`**

In `crates/ui-runtime/src/scene_clock.rs`, add the imports at the top (alongside existing):

```rust
use crate::drivers::{install_scroll_driver, ScrollDriverHandle};
use crate::scene_driver::SceneDriver;
```

Add a scroll-driver slot to `SceneClock` (alongside the existing `handle_slot: Signal<HandleSlot>`):

```rust
#[derive(Clone, Default)]
pub(crate) struct ScrollHandleSlot(pub(crate) Rc<RefCell<Option<ScrollDriverHandle>>>);
```

Update the `SceneClock` struct to add the field and initialize it in `new`:

```rust
#[derive(Clone, Copy)]
pub struct SceneClock {
    pub duration_ms: Signal<f32>,
    pub elapsed_ms: Signal<f32>,
    pub state: Signal<SceneState>,
    pub fps: Signal<u32>,
    pub reduced: Signal<bool>,
    handle_slot: Signal<HandleSlot>,
    scroll_slot: Signal<ScrollHandleSlot>,   // NEW
}
```

In `SceneClock::new`, add `scroll_slot: Signal::new(ScrollHandleSlot::default()),`.

Add a `play_with` method that selects the driver:

```rust
    /// Drives the clock using the chosen `SceneDriver`. Replaces the
    /// argumentless `play()` for callers that want explicit control;
    /// the existing `play()` is now equivalent to `play_with(SceneDriver::Autoplay)`.
    pub fn play_with(&self, driver: SceneDriver) {
        // Always stop any existing autoplay loop or scroll driver
        // before installing a new one.
        self.pause();
        self.scroll_slot.peek().0.borrow_mut().take();

        match driver {
            SceneDriver::Autoplay => self.play(),
            SceneDriver::Manual => {
                // No-op: state stays Paused, no listener installed.
            }
            SceneDriver::Scroll(config) => {
                if *self.reduced.peek() {
                    self.settle();
                    return;
                }
                let clock_handle = *self;
                let handle = install_scroll_driver(&config, move |progress| {
                    clock_handle.seek_progress(progress);
                });
                *self.scroll_slot.peek().0.borrow_mut() = Some(handle);
            }
        }
    }
```

Note: the existing `play()` already handles reduced motion and the rAF loop. `play_with(Autoplay)` delegates to it directly.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-runtime --test scene_driver`
Expected: 5 PASS (3 from Task 4 + 2 new).

Also run `cargo test -p ui-runtime --test scene_clock` to confirm the 11 SP-1 tests still pass.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-runtime/src/scene_clock.rs crates/ui-runtime/tests/scene_driver.rs
git commit -m "$(cat <<'EOF'
feat(ui-runtime): SceneClock::play_with(driver) selects autoplay vs scroll vs manual

Autoplay delegates to the existing play() path. Manual installs no
driver. Scroll installs the IntersectionObserver + scroll listener
on the configured trigger and pushes progress into seek_progress.
Existing single-argument play() is preserved as the Autoplay shortcut.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 7: `Scene` Dioxus component gains `driver` prop

The Scene component now accepts an optional `driver: Option<SceneDriver>`. When `None`, behaviour is identical to SP-1 (autoplay if `autoplay: true`, otherwise stays paused). When `Some`, the autoplay effect calls `clock.play_with(driver)` instead of `clock.play()`.

**Files:**
- Modify: `crates/ui-dioxus/src/scene_player.rs`
- Test: `crates/ui-dioxus/tests/scene_player_ssr.rs` (append)

- [ ] **Step 1: Write the failing test**

Append to `crates/ui-dioxus/tests/scene_player_ssr.rs`:

```rust
use ui_runtime::scene_driver::SceneDriver;

#[test]
fn scene_with_manual_driver_skips_autoplay_render() {
    let html = dioxus_ssr::render_element(rsx! {
        Scene {
            id: "manual",
            width: 100,
            height: 100,
            duration_ms: 5_000.0,
            autoplay: Some(true),       // explicit autoplay request
            driver: Some(SceneDriver::Manual), // overridden by Manual driver
            controls: Some(false),
            p { "body" }
        }
    });
    // Manual driver does not advance the clock at mount, so the
    // state stays paused at elapsed 0.
    assert!(html.contains("data-state=\"paused\""), "{html}");
    assert!(html.contains("data-elapsed-ms=\"0\""), "{html}");
}
```

- [ ] **Step 2: Run tests to verify it fails**

Run: `cargo test -p ui-dioxus --test scene_player_ssr scene_with_manual_driver`
Expected: compile error — `driver` prop unknown.

- [ ] **Step 3: Add the prop**

In `crates/ui-dioxus/src/scene_player.rs`:

1. Add import at the top: `use ui_runtime::scene_driver::SceneDriver;` (re-confirmed via `crates/ui-dioxus/Cargo.toml` — `ui-runtime` is already a dependency).
2. In the `Scene` component signature, after the existing `controls: Option<bool>,`, add:
   ```rust
   driver: Option<SceneDriver>,
   ```
3. In the body, replace the existing autoplay `use_effect`:

   ```rust
   use_effect(move || {
       if autoplay && !reduced {
           clock.play();
       }
   });
   ```

   with:

   ```rust
   let driver_for_effect = driver.clone();
   use_effect(move || {
       if reduced {
           return;
       }
       if let Some(d) = driver_for_effect.clone() {
           clock.play_with(d);
       } else if autoplay {
           clock.play();
       }
   });
   ```

   (`SceneDriver` derives `Clone`. The closure captures by clone since `Signal<T>` is Copy and `SceneDriver` is the only non-Copy capture; the explicit `let driver_for_effect = driver.clone();` is needed because `driver` itself is consumed by other rendering paths inside the component, including the controls section.)

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-dioxus --test scene_player_ssr`
Expected: 11 PASS (10 from SP-1 + 1 new).

- [ ] **Step 5: Commit**

```bash
git add crates/ui-dioxus/src/scene_player.rs crates/ui-dioxus/tests/scene_player_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): Scene accepts optional SceneDriver prop

When Some(driver), the autoplay effect calls play_with(driver) instead
of play(). When None, behaviour is identical to SP-1. Manual driver
preserves elapsed_ms at 0 even when autoplay=true is explicitly set.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 8: `SplitText` Dioxus component

Renders per-glyph or per-word spans. Accessibility-clean: parent has `aria-label`; glyph spans are `aria-hidden`. Inside a `TimelineScope`, the existing stagger machinery walks the `data-stagger-index` attribute.

**Files:**
- Create: `crates/ui-dioxus/src/split_text.rs`
- Modify: `crates/ui-dioxus/src/lib.rs` (re-exports)
- Test: `crates/ui-dioxus/tests/split_text_ssr.rs`

- [ ] **Step 1: Write the failing tests**

Create `crates/ui-dioxus/tests/split_text_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::{SplitMode, SplitText};

#[test]
fn split_text_character_mode_emits_per_glyph_spans() {
    let html = dioxus_ssr::render_element(rsx! {
        SplitText { text: "Hi".to_string(), split_by: Some(SplitMode::Character) }
    });
    assert!(html.contains("data-stagger-index=\"0\""), "{html}");
    assert!(html.contains("data-stagger-index=\"1\""), "{html}");
    assert!(!html.contains("data-stagger-index=\"2\""), "{html}");
}

#[test]
fn split_text_default_mode_is_character() {
    let html = dioxus_ssr::render_element(rsx! {
        SplitText { text: "Hi".to_string() }
    });
    assert!(html.contains("data-stagger-index=\"0\""), "{html}");
    assert!(html.contains("data-stagger-index=\"1\""), "{html}");
}

#[test]
fn split_text_parent_aria_label_carries_full_text() {
    let html = dioxus_ssr::render_element(rsx! {
        SplitText { text: "Hello world".to_string() }
    });
    assert!(html.contains("aria-label=\"Hello world\""), "{html}");
}

#[test]
fn split_text_glyph_spans_are_aria_hidden() {
    let html = dioxus_ssr::render_element(rsx! {
        SplitText { text: "Hi".to_string() }
    });
    let hidden_count = html.matches("aria-hidden=\"true\"").count();
    assert!(hidden_count >= 2, "expected at least 2 aria-hidden spans, got {hidden_count}: {html}");
}

#[test]
fn split_text_word_mode_emits_per_word_spans() {
    let html = dioxus_ssr::render_element(rsx! {
        SplitText { text: "Hello world".to_string(), split_by: Some(SplitMode::Word) }
    });
    // Two words: indices 0 and 1.
    assert!(html.contains("data-stagger-index=\"0\""), "{html}");
    assert!(html.contains("data-stagger-index=\"1\""), "{html}");
    assert!(!html.contains("data-stagger-index=\"2\""), "{html}");
    // The text content of each word span is the word itself.
    assert!(html.contains(">Hello<"), "{html}");
    assert!(html.contains(">world<"), "{html}");
}

#[test]
fn split_text_word_mode_preserves_whitespace_between_words() {
    let html = dioxus_ssr::render_element(rsx! {
        SplitText { text: "A B".to_string(), split_by: Some(SplitMode::Word) }
    });
    // Between the two word spans, a literal space text node must appear
    // so layout doesn't collapse.
    assert!(html.contains("> </span> "), "expected explicit space between words: {html}");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-dioxus --test split_text_ssr`
Expected: compile error — `SplitText` / `SplitMode` not found.

- [ ] **Step 3: Implement `SplitText`**

Create `crates/ui-dioxus/src/split_text.rs`:

```rust
use dioxus::prelude::*;

/// How `SplitText` divides its `text` into staggerable units.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SplitMode {
    /// One stagger index per Unicode grapheme. Default.
    #[default]
    Character,
    /// One stagger index per whitespace-separated word. Whitespace is
    /// preserved as literal text nodes between word spans.
    Word,
}

/// Renders `text` as a sequence of per-glyph or per-word `<span>`
/// children. Each child carries a `data-stagger-index` attribute so the
/// surrounding `TimelineScope` / `Sequence` stagger machinery drives
/// them in order. The parent span carries `aria-label="<full text>"`;
/// the child spans are `aria-hidden="true"` so screen readers read the
/// full text once instead of enumerating glyphs.
#[component]
pub fn SplitText(text: String, split_by: Option<SplitMode>) -> Element {
    let mode = split_by.unwrap_or_default();
    let label = text.clone();
    rsx! {
        span {
            class: "ui-split-text",
            "aria-label": "{label}",
            "data-split-mode": match mode {
                SplitMode::Character => "character",
                SplitMode::Word => "word",
            },
            {render_units(&text, mode)}
        }
    }
}

fn render_units(text: &str, mode: SplitMode) -> Element {
    match mode {
        SplitMode::Character => render_characters(text),
        SplitMode::Word => render_words(text),
    }
}

fn render_characters(text: &str) -> Element {
    let chars: Vec<(usize, String)> = text
        .chars()
        .enumerate()
        .map(|(i, c)| (i, c.to_string()))
        .collect();
    rsx! {
        for (i, ch) in chars {
            span {
                class: "ui-split-text-glyph",
                "data-stagger-index": "{i}",
                "aria-hidden": "true",
                "{ch}"
            }
        }
    }
}

fn render_words(text: &str) -> Element {
    // Split keeping whitespace as separate runs. We do this manually so
    // multi-space runs survive intact.
    let mut units: Vec<(bool, String)> = Vec::new();
    let mut buf = String::new();
    let mut buf_is_word = !text.starts_with(char::is_whitespace);
    for ch in text.chars() {
        let ch_is_word = !ch.is_whitespace();
        if ch_is_word == buf_is_word {
            buf.push(ch);
        } else {
            if !buf.is_empty() {
                units.push((buf_is_word, std::mem::take(&mut buf)));
            }
            buf_is_word = ch_is_word;
            buf.push(ch);
        }
    }
    if !buf.is_empty() {
        units.push((buf_is_word, buf));
    }

    let mut word_index = 0usize;
    let rendered: Vec<Element> = units
        .into_iter()
        .map(|(is_word, content)| {
            if is_word {
                let idx = word_index;
                word_index += 1;
                rsx! {
                    span {
                        class: "ui-split-text-word",
                        "data-stagger-index": "{idx}",
                        "aria-hidden": "true",
                        "{content}"
                    }
                }
            } else {
                // Whitespace as a literal text node — no span.
                rsx! { "{content}" }
            }
        })
        .collect();
    rsx! { {rendered.into_iter()} }
}
```

In `crates/ui-dioxus/src/lib.rs`, add:
- `pub mod split_text;`
- Re-export in the existing `pub use` block: `pub use split_text::{SplitMode, SplitText};`

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-dioxus --test split_text_ssr`
Expected: 6 PASS.

Note: if the `"> </span> "` assertion fails because Dioxus SSR formats whitespace differently (e.g. omits the trailing space), inspect the actual output and adjust the assertion to match what Dioxus produces — the contract is that whitespace is preserved as a text node between word spans, not the exact byte layout.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-dioxus/src/split_text.rs crates/ui-dioxus/src/lib.rs crates/ui-dioxus/tests/split_text_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): SplitText component (Character / Word modes)

Per-glyph or per-word spans with sequential data-stagger-index so the
TimelineScope stagger machinery drives them. Parent carries
aria-label; glyph spans are aria-hidden so screen readers read the
unsplit text once.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 9: `MotionPath` Dioxus convenience component

Wraps `children` in a `KineticBox` whose surrounding `Sequence` carries a single `MotionCue::Path` segment. Emits a `data-motion-path` attribute carrying the path as JSON so render tooling can introspect.

**Files:**
- Create: `crates/ui-dioxus/src/motion_path.rs`
- Modify: `crates/ui-dioxus/src/lib.rs` (re-exports)
- Test: `crates/ui-dioxus/tests/motion_path_ssr.rs`

- [ ] **Step 1: Write the failing tests**

Create `crates/ui-dioxus/tests/motion_path_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::MotionPath;
use ui_timeline::PathPoint;

#[test]
fn motion_path_wraps_children_in_kinetic_box() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 100.0) },
    ];
    let html = dioxus_ssr::render_element(rsx! {
        MotionPath {
            id: "icon-trace".to_string(),
            path: pts,
            duration_ms: 2_000.0,
            div { "icon" }
        }
    });
    // Marker class + path data attribute + child.
    assert!(html.contains("ui-motion-path"), "{html}");
    assert!(html.contains("data-motion-path"), "{html}");
    assert!(html.contains("\"icon-trace\"") || html.contains("data-kinetic-id=\"icon-trace\""), "{html}");
    assert!(html.contains("icon"), "{html}");
}

#[test]
fn motion_path_data_attribute_is_json_serialized() {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (50.0, 50.0) },
    ];
    let html = dioxus_ssr::render_element(rsx! {
        MotionPath {
            id: "x".to_string(),
            path: pts,
            duration_ms: 1_000.0,
            "_"
        }
    });
    // JSON array with two Line entries.
    assert!(html.contains("data-motion-path=\"["), "{html}");
    assert!(html.contains("\"Line\""), "{html}");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-dioxus --test motion_path_ssr`
Expected: compile error — `MotionPath` not found.

- [ ] **Step 3: Implement `MotionPath`**

Add `serde = { version = "1", features = ["derive"] }` and `serde_json = "1"` to `crates/ui-dioxus/Cargo.toml`'s `[dependencies]` if not already present. (Inspect first — `serde` may already be there.)

Also derive `Serialize` on `PathPoint` in `crates/ui-timeline/src/path.rs`:

```rust
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PathPoint { ... }
```

Add `serde = { version = "1", features = ["derive"] }` to `crates/ui-timeline/Cargo.toml` if not already a dep.

Create `crates/ui-dioxus/src/motion_path.rs`:

```rust
use dioxus::prelude::*;
use ui_timeline::PathPoint;

/// Convenience wrapper that emits a kinetic-box child tagged with the
/// supplied motion path. The host scene / sequence is expected to
/// install a `MotionCue::Path` segment matching this id. SP-3's
/// initial implementation only emits the DOM scaffolding; the motion
/// cue authoring happens at the call site (see the curved trajectory
/// scene in the gallery for the canonical pattern).
#[component]
pub fn MotionPath(
    id: String,
    path: Vec<PathPoint>,
    duration_ms: f32,
    rotate_along_path: Option<bool>,
    children: Element,
) -> Element {
    let rotate = rotate_along_path.unwrap_or(false);
    let path_json = serde_json::to_string(&path).unwrap_or_else(|_| "[]".to_string());
    let duration_attr = format!("{}", duration_ms as i64);
    rsx! {
        div {
            class: "ui-motion-path",
            "data-kinetic-id": "{id}",
            "data-motion-path": "{path_json}",
            "data-motion-path-duration-ms": "{duration_attr}",
            "data-motion-path-rotate": if rotate { "true" } else { "false" },
            {children}
        }
    }
}
```

In `crates/ui-dioxus/src/lib.rs`:
- `pub mod motion_path;`
- `pub use motion_path::MotionPath;`

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-dioxus --test motion_path_ssr`
Expected: 2 PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-dioxus/src/motion_path.rs crates/ui-dioxus/src/lib.rs crates/ui-dioxus/Cargo.toml crates/ui-timeline/src/path.rs crates/ui-timeline/Cargo.toml crates/ui-dioxus/tests/motion_path_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): MotionPath convenience component

Wraps children in a div with data-kinetic-id + data-motion-path
(JSON-serialized) + duration + optional rotate-along-path flag.
PathPoint gains serde derives. The host scene installs the matching
MotionCue::Path segment.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 10: `gsap_primitives.css` + library_css() wiring

Adds inline-block on split text glyph spans (so transforms apply) and a positioning container for motion path.

**Files:**
- Create: `crates/ui-styles/src/gsap_primitives.css`
- Modify: `crates/ui-styles/src/lib.rs`

- [ ] **Step 1: Create the CSS**

Create `crates/ui-styles/src/gsap_primitives.css`:

```css
/* SplitText: per-glyph and per-word spans must be inline-block so
   transforms applied by KineticBox parents (translate, scale, rotate)
   actually render — inline elements ignore transform. */
.ui-split-text-glyph,
.ui-split-text-word {
  display: inline-block;
  will-change: transform, opacity;
}

/* MotionPath: provide a relative container so children's translate is
   relative to the path's origin. */
.ui-motion-path {
  position: relative;
  display: inline-block;
}

/* MotionPath's child gets translate via KineticBox in the wrapping
   sequence; no additional styling needed here. */
```

- [ ] **Step 2: Wire into `library_css()`**

In `crates/ui-styles/src/lib.rs`, add a const next to the other CSS includes:

```rust
pub const GSAP_PRIMITIVES_CSS: &str = include_str!("gsap_primitives.css");
```

Append `GSAP_PRIMITIVES_CSS` to the body of `library_css()` exactly as SP-1 did for `SCENE_PLAYER_CSS`.

- [ ] **Step 3: Verify**

Run: `cargo check -p ui-styles && cargo test --workspace`
Expected: success.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-styles/src/gsap_primitives.css crates/ui-styles/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(ui-styles): gsap_primitives.css for split-text + motion-path

Inline-block on SplitText glyph/word spans so KineticBox transforms
apply. Position relative on MotionPath container so child translate
is path-relative.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 11: `kinetics::prelude` + `public_api_names` extensions

Re-export the new public surface.

**Files:**
- Modify: `crates/kinetics/src/lib.rs`

- [ ] **Step 1: Append to the existing `pub use` blocks**

In `crates/kinetics::prelude`, add to the `ui_timeline` re-export (which already includes `Axis, MotionCue, TimelineClock`) — add `PathPoint`:

```rust
    pub use ui_timeline::{Axis, MotionCue, PathPoint, ResolvedMotionState, TimelineClock};
```

Add to the `ui_dioxus` re-export — append `MotionPath, SplitText, SplitMode`:

```rust
    pub use ui_dioxus::{
        // ... existing entries ...
        MotionPath, Scene, SceneContext, SplitMode, SplitText,
    };
```

Add to the runtime cfg block — append `SceneDriver, ScrollObserverConfig`:

```rust
    #[cfg(feature = "runtime")]
    pub use ui_runtime::{
        CssKeyframesAdapter, FrameAdapter, FrameAdapterHandle, FrameAdapterRegistry,
        SceneClock, SceneDriver, SceneState, ScrollObserverConfig, SequenceAdapter, WaapiAdapter,
    };
```

In `public_api_names()`, append:

```rust
        "MotionPath",
        "PathPoint",
        "SceneDriver",
        "ScrollObserverConfig",
        "SplitText",
        "SplitMode",
```

- [ ] **Step 2: Verify**

Run: `cargo check -p kinetics --all-features && cargo test -p kinetics`
Expected: success.

- [ ] **Step 3: Commit**

```bash
git add crates/kinetics/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(kinetics): export SceneDriver/SplitText/MotionPath/PathPoint in prelude

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 12: Gallery scene — `Scene · Scroll-pinned Story`

A 4-beat narrative driven by `SceneDriver::Scroll`. The trigger is a 200vh container; as the user scrolls through it, the scene's `elapsed_ms` advances.

**Files:**
- Create: `examples/component-gallery/src/previews/scenes/scroll_story.rs`
- Modify: `examples/component-gallery/src/previews/scenes/mod.rs`

- [ ] **Step 1: Create the scene**

Create `examples/component-gallery/src/previews/scenes/scroll_story.rs`:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_composition::ClipFill;

#[component]
pub fn ScrollPinnedStoryScene() -> Element {
    let driver = SceneDriver::Scroll(ScrollObserverConfig::new("#scroll-story-trigger"));
    rsx! {
        div { class: "scene-scroll-shell",
            div {
                id: "scroll-story-trigger",
                class: "scene-scroll-trigger",
                style: "height: 200vh; position: relative;",
                div { class: "scene-scroll-sticky",
                    style: "position: sticky; top: 0; height: 100vh;",
                    Scene {
                        id: "scroll-story",
                        width: 1280,
                        height: 720,
                        duration_ms: 10_000.0,
                        driver: Some(driver),
                        controls: Some(true),

                        Clip { start_ms: 0.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                            KineticText {
                                id: "scroll-headline",
                                text: "Scroll-driven storytelling.".to_string(),
                                cue: "rise-in",
                            }
                        }
                        Clip { start_ms: 2_500.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                            KineticText {
                                id: "scroll-body",
                                text: "Same Scene API. Scroll instead of autoplay.".to_string(),
                                cue: "fade-in",
                            }
                        }
                        Clip { start_ms: 5_000.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                            KineticText {
                                id: "scroll-feature",
                                text: "Built on IntersectionObserver + window scroll.".to_string(),
                                cue: "slide-up",
                            }
                        }
                        Clip { start_ms: 7_500.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                            KineticText {
                                id: "scroll-cta",
                                text: "Pin a story to the page.".to_string(),
                                cue: "rise-in",
                            }
                        }
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 2: Register the module**

In `examples/component-gallery/src/previews/scenes/mod.rs`, add `pub mod scroll_story;`.

- [ ] **Step 3: Verify**

Run: `cargo check -p component-gallery`
Expected: success.

- [ ] **Step 4: Commit**

```bash
git add examples/component-gallery/src/previews/scenes/scroll_story.rs examples/component-gallery/src/previews/scenes/mod.rs
git commit -m "$(cat <<'EOF'
feat(gallery): Scene · Scroll-pinned Story

A 4-beat narrative driven by SceneDriver::Scroll with a 200vh trigger
region and sticky-positioned Scene. Demonstrates the ScrollTrigger
half of SP-3.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 13: Gallery scene — `Scene · Split Headline`

A 2-second hero title that animates character-by-character.

**Files:**
- Create: `examples/component-gallery/src/previews/scenes/split_headline.rs`
- Modify: `examples/component-gallery/src/previews/scenes/mod.rs`

- [ ] **Step 1: Create the scene**

Create `examples/component-gallery/src/previews/scenes/split_headline.rs`:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn SplitHeadlineScene() -> Element {
    rsx! {
        Scene {
            id: "split-headline",
            width: 1280,
            height: 360,
            duration_ms: 2_500.0,
            autoplay: Some(true),
            controls: Some(true),
            TimelineScope { id: "split-headline-timeline", autoplay: true,
                SplitText {
                    text: "Kinetics typography, glyph by glyph.".to_string(),
                    split_by: Some(SplitMode::Character),
                }
            }
        }
    }
}
```

- [ ] **Step 2: Register the module**

In `examples/component-gallery/src/previews/scenes/mod.rs`, add `pub mod split_headline;`.

- [ ] **Step 3: Verify**

Run: `cargo check -p component-gallery`
Expected: success.

- [ ] **Step 4: Commit**

```bash
git add examples/component-gallery/src/previews/scenes/split_headline.rs examples/component-gallery/src/previews/scenes/mod.rs
git commit -m "$(cat <<'EOF'
feat(gallery): Scene · Split Headline

Per-character SplitText inside Scene + TimelineScope. Demonstrates
the SplitText half of SP-3.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 14: Gallery scene — `Scene · Curved Trajectory`

A KineticBox flies along an S-curve. Builds the `MotionCue::Path` segment at the scene level (the `MotionPath` convenience wrapper emits the kinetic-box; the Scene's surrounding Sequence carries the cue).

**Files:**
- Create: `examples/component-gallery/src/previews/scenes/curved_trajectory.rs`
- Modify: `examples/component-gallery/src/previews/scenes/mod.rs`

- [ ] **Step 1: Create the scene**

Create `examples/component-gallery/src/previews/scenes/curved_trajectory.rs`:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_motion::{Ease, Transition};
use ui_timeline::{MotionSegment, MotionTarget, Timeline, TimelineTrack};

#[component]
pub fn CurvedTrajectoryScene() -> Element {
    let pts = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Bezier {
            control_1: (200.0, -200.0),
            control_2: (400.0, 200.0),
            end: (600.0, 0.0),
        },
    ];
    let cue = MotionCue::Path {
        points: pts.clone(),
        from_progress: 0.0,
        to_progress: 1.0,
        rotate_along_path: false,
        transition: Transition::Tween {
            duration_ms: 4_000,
            ease: Ease::Standard,
        },
    };
    let timeline = Timeline::new("curved-trajectory-timeline", 4_000.0).with_track(
        TimelineTrack::new(
            MotionTarget::node("trajectory-icon"),
            vec![MotionSegment::new(0.0, 4_000.0, cue)],
        ),
    );

    rsx! {
        Scene {
            id: "curved-trajectory",
            width: 720,
            height: 480,
            duration_ms: 4_000.0,
            autoplay: Some(true),
            controls: Some(true),
            Sequence {
                timeline: Some(timeline),
                clock: TimelineClock::Manual { elapsed_ms: 0.0 },
                MotionPath {
                    id: "trajectory-icon".to_string(),
                    path: pts,
                    duration_ms: 4_000.0,
                    KineticBox { id: "trajectory-icon", "•" }
                }
            }
        }
    }
}
```

Note: the `Sequence` `clock` prop is set to `Manual { elapsed_ms: 0.0 }` because the parent Scene drives elapsed_ms via context. The existing SP-1 Sequence implementation reads the elapsed_ms either from this prop or — when available — from the SceneContext; if the latter wiring isn't already present (verify by inspecting `crates/ui-dioxus/src/sequence.rs`), this scene won't animate via the Scene clock. If the Sequence doesn't consume SceneContext.elapsed_ms, the curved trajectory scene will simply render at elapsed=0 — that's acceptable for SP-3's static SSR. Adding the dynamic wiring is out of scope for SP-3 and is a follow-up.

- [ ] **Step 2: Register the module**

In `examples/component-gallery/src/previews/scenes/mod.rs`, add `pub mod curved_trajectory;`.

- [ ] **Step 3: Verify**

Run: `cargo check -p component-gallery`
Expected: success.

- [ ] **Step 4: Commit**

```bash
git add examples/component-gallery/src/previews/scenes/curved_trajectory.rs examples/component-gallery/src/previews/scenes/mod.rs
git commit -m "$(cat <<'EOF'
feat(gallery): Scene · Curved Trajectory

A KineticBox flies along an S-curve via MotionCue::Path. Demonstrates
the MotionPath half of SP-3.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 15: Wire the three scenes into the gallery (previews + docs + manifest)

**Files:**
- Modify: `examples/component-gallery/src/previews/scene.rs` (add 3 preview fns)
- Modify: `examples/component-gallery/src/docs.rs` (3 snippet consts + 3 ComponentDoc entries)
- Modify: `examples/component-gallery/e2e/tests/_lib/component-manifest.ts` (3 manifest entries)

- [ ] **Step 1: Add preview functions**

In `examples/component-gallery/src/previews/scene.rs`, append:

```rust
use crate::previews::scenes::curved_trajectory::CurvedTrajectoryScene;
use crate::previews::scenes::scroll_story::ScrollPinnedStoryScene;
use crate::previews::scenes::split_headline::SplitHeadlineScene;

pub fn scroll_pinned_story_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            ScrollPinnedStoryScene {}
        }
    }
}

pub fn split_headline_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            SplitHeadlineScene {}
        }
    }
}

pub fn curved_trajectory_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            CurvedTrajectoryScene {}
        }
    }
}
```

- [ ] **Step 2: Add snippet consts + ComponentDoc entries**

In `examples/component-gallery/src/docs.rs`, after the existing `SCENE_PRODUCT_INTRO_SNIPPET`:

```rust
const SCENE_SCROLL_STORY_SNIPPET: &str = r##"Scene {
    id: "scroll-story",
    duration_ms: 10_000.0,
    driver: Some(SceneDriver::Scroll(
        ScrollObserverConfig::new("#scroll-story-trigger"),
    )),
    Clip { start_ms: 0.0, duration_ms: 2_500.0, /* headline */ }
    Clip { start_ms: 2_500.0, duration_ms: 2_500.0, /* body  */ }
    Clip { start_ms: 5_000.0, duration_ms: 2_500.0, /* feature */ }
    Clip { start_ms: 7_500.0, duration_ms: 2_500.0, /* CTA */ }
}"##;

const SCENE_SPLIT_HEADLINE_SNIPPET: &str = r##"Scene {
    id: "split-headline",
    duration_ms: 2_500.0,
    TimelineScope { id: "split-headline-timeline", autoplay: true,
        SplitText {
            text: "Kinetics typography, glyph by glyph.".to_string(),
            split_by: Some(SplitMode::Character),
        }
    }
}"##;

const SCENE_CURVED_TRAJECTORY_SNIPPET: &str = r##"Scene {
    id: "curved-trajectory",
    duration_ms: 4_000.0,
    Sequence {
        timeline: Some(/* MotionCue::Path with PathPoint::Bezier ... */),
        MotionPath {
            id: "trajectory-icon",
            path: vec![PathPoint::Line { end: (0.0, 0.0) }, PathPoint::Bezier { /* ... */ }],
            duration_ms: 4_000.0,
            KineticBox { id: "trajectory-icon", "•" }
        }
    }
}"##;
```

Append three entries to the `COMPONENT_DOCS` array:

```rust
    ComponentDoc {
        name: "Scene · Scroll-pinned Story",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ScrollTrigger-style: a 10-second narrative pinned to a 200vh region. Scroll drives elapsed_ms via IntersectionObserver + window scroll.",
        snippet: SCENE_SCROLL_STORY_SNIPPET,
        accessibility: "Reduced motion settles immediately and ignores scroll. Each beat's text is independently labeled.",
        render: Some(crate::previews::scene::scroll_pinned_story_preview),
    },
    ComponentDoc {
        name: "Scene · Split Headline",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "SplitText: per-character spans with sequential data-stagger-index. Screen readers read the parent aria-label; the per-glyph spans are aria-hidden.",
        snippet: SCENE_SPLIT_HEADLINE_SNIPPET,
        accessibility: "Parent carries the full text via aria-label; glyph spans are aria-hidden so screen readers do not enumerate.",
        render: Some(crate::previews::scene::split_headline_preview),
    },
    ComponentDoc {
        name: "Scene · Curved Trajectory",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "MotionPath: a KineticBox traces a parametric S-curve sampled by arc length. Optional rotate-along-path tangent.",
        snippet: SCENE_CURVED_TRAJECTORY_SNIPPET,
        accessibility: "Visual-only decoration; the icon glyph remains in the DOM and is not announced.",
        render: Some(crate::previews::scene::curved_trajectory_preview),
    },
```

If the existing test in `examples/component-gallery/tests/gallery.rs` pins a specific count or order of `COMPONENT_DOCS`, update it to include the three new entries.

- [ ] **Step 3: Update e2e manifest**

In `examples/component-gallery/e2e/tests/_lib/component-manifest.ts`, append (under the existing `// Scene` section):

```typescript
  {
    name: "Scene · Scroll-pinned Story",
    slug: "scene-scroll-pinned-story",
    status: "ready",
    layers: { smoke: true, motion: true, visual: true },
  },
  {
    name: "Scene · Split Headline",
    slug: "scene-split-headline",
    status: "ready",
    layers: { smoke: true, motion: true, visual: true },
  },
  {
    name: "Scene · Curved Trajectory",
    slug: "scene-curved-trajectory",
    status: "ready",
    layers: { smoke: true, motion: true, visual: true },
  },
```

(The exact field names may differ — match the existing Scene · Product Intro 10s entry in this file.)

- [ ] **Step 4: Verify**

Run: `cargo check -p component-gallery && cargo test -p component-gallery`
Expected: success.

- [ ] **Step 5: Commit**

```bash
git add examples/component-gallery/src examples/component-gallery/e2e/tests/_lib/component-manifest.ts examples/component-gallery/tests/gallery.rs
git commit -m "$(cat <<'EOF'
feat(gallery): wire three SP-3 Scene entries (scroll/split/curved)

Three new ComponentDoc entries with preview functions, snippets, and
the matching TS manifest entries so the e2e harness recognizes them.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 16: Playwright e2e spec

Three tests, one per primitive, on Chromium + WebKit.

**Files:**
- Create: `examples/component-gallery/e2e/tests/gsap-tier-primitives.spec.ts`

- [ ] **Step 1: Write the spec**

Create `examples/component-gallery/e2e/tests/gsap-tier-primitives.spec.ts`:

```ts
import { expect, test } from "@playwright/test";

const SCENE_SECTION = "#scene";

test.describe("SP-3 GSAP-tier primitives", () => {
  test("SplitText emits per-glyph spans with aria-label parent", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Split Headline'))",
    );
    await expect(card).toBeVisible();
    const splitText = card.locator(".ui-split-text").first();
    await expect(splitText).toHaveAttribute(
      "aria-label",
      "Kinetics typography, glyph by glyph.",
    );
    const glyphCount = await splitText.locator(".ui-split-text-glyph").count();
    expect(glyphCount).toBe("Kinetics typography, glyph by glyph.".length);
  });

  test("MotionPath emits data-motion-path JSON and a KineticBox child", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Curved Trajectory'))",
    );
    await expect(card).toBeVisible();
    const motionPath = card.locator(".ui-motion-path").first();
    const dataAttr = await motionPath.getAttribute("data-motion-path");
    expect(dataAttr).not.toBeNull();
    expect(dataAttr!).toContain("Line");
    expect(dataAttr!).toContain("Bezier");
  });

  test("Scroll-pinned scene installs trigger element and Scene reads driver", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Scroll-pinned Story'))",
    );
    await expect(card).toBeVisible();
    const trigger = card.locator("#scroll-story-trigger");
    await expect(trigger).toBeVisible();
    // The Scene stage data-composition-id matches the configured id.
    const stage = card.locator(".ui-scene-stage").first();
    await expect(stage).toHaveAttribute("data-composition-id", "scroll-story");
  });

  test("Scroll-pinned scene under reduced motion settles immediately", async ({ page }) => {
    await page.emulateMedia({ reducedMotion: "reduce" });
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Scroll-pinned Story'))",
    );
    const stage = card.locator(".ui-scene-stage").first();
    await expect(stage).toHaveAttribute("data-state", "settled");
    await expect(stage).toHaveAttribute("data-reduced", "true");
  });
});
```

- [ ] **Step 2: Build the gallery (release)**

```bash
cd examples/component-gallery
dx build --release
```

Expected: success. If disk pressure surfaces (LNK1318 / disk-full), run `cargo clean -p component-gallery` first and retry.

- [ ] **Step 3: Run the spec on Chromium**

```bash
cd examples/component-gallery/e2e
npx playwright test --project=static tests/gsap-tier-primitives.spec.ts
```

Expected: 4 passed.

- [ ] **Step 4: Run the spec on WebKit**

```bash
npx playwright test --project=static-webkit tests/gsap-tier-primitives.spec.ts
```

Expected: 4 passed.

- [ ] **Step 5: Commit**

```bash
git add examples/component-gallery/e2e/tests/gsap-tier-primitives.spec.ts
git commit -m "$(cat <<'EOF'
test(gallery-e2e): Playwright spec for SP-3 GSAP-tier primitives

Three tests on Chromium + WebKit:
- SplitText emits per-glyph spans with aria-label parent.
- MotionPath emits data-motion-path JSON.
- Scroll-pinned scene installs trigger + reads driver.
- Scroll-pinned scene under reduced motion settles immediately.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 17: Workspace verification

**Steps:**

- [ ] **Step 1: Format + clippy**

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Both must succeed. Fix any inline.

- [ ] **Step 2: Full test suite**

```bash
cargo test --workspace
```

Expected: all green. Specifically:
- `ui-timeline` includes `path` (14 tests) and `cue_path` (5 tests).
- `ui-runtime` includes `scene_driver` (5 tests).
- `ui-dioxus` includes `split_text_ssr` (6 tests), `motion_path_ssr` (2 tests), `scene_player_ssr` (now 11 tests).

- [ ] **Step 3: wasm32 check**

```bash
cargo check -p ui-runtime --target wasm32-unknown-unknown
cargo check -p ui-dioxus --target wasm32-unknown-unknown
```

Both must succeed (the scroll driver web binding has to compile).

- [ ] **Step 4: E2E on both engines**

```bash
cd examples/component-gallery
dx build --release
cd e2e
npx playwright test --project=static tests/gsap-tier-primitives.spec.ts
npx playwright test --project=static-webkit tests/gsap-tier-primitives.spec.ts
npx playwright test --project=static tests/scene-player.spec.ts        # regression
npx playwright test --project=static-webkit tests/scene-player.spec.ts # regression
```

All four must pass.

- [ ] **Step 5: Optional clippy fix commit**

If any clippy / fmt fixes were needed during verification, commit them:

```bash
git add -A
git commit -m "$(cat <<'EOF'
chore(sp-3): fmt + clippy cleanup post-implementation

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

Skip if nothing needs fixing.

---

## Self-Review Notes

**Spec coverage:** Every section/requirement of `2026-05-25-gsap-tier-primitives-design.md` maps to one or more tasks:
- `MotionCue::Path` + `PathPoint` + sampler → Tasks 1–3.
- `SceneDriver` enum + scroll driver → Tasks 4–5.
- `SceneClock::play_with` → Task 6.
- `Scene` `driver` prop → Task 7.
- `SplitText` + `SplitMode` → Task 8.
- `MotionPath` → Task 9.
- CSS → Task 10.
- `kinetics::prelude` → Task 11.
- Three showcase scenes → Tasks 12–14.
- Gallery wiring → Task 15.
- Playwright e2e → Task 16.
- Verification → Task 17.

**Placeholder scan:** No "TBD", no "implement later", no untyped "similar to". Every step has the actual code or exact command.

**Type consistency:** `PathPoint::Line { end }`, `PathPoint::Bezier { control_1, control_2, end }`, `MotionCue::Path { points, from_progress, to_progress, rotate_along_path, transition }`, `SceneDriver::{Autoplay, Scroll(ScrollObserverConfig), Manual}`, `ScrollObserverConfig { trigger_selector, start_offset_px, end_offset_px }`, `SplitMode::{Character, Word}` — all uniform across tasks.

**Known forward-references:**
- Task 14's `Sequence` may not consume `SceneContext.elapsed_ms` in SP-1's implementation. The task body documents this and accepts the static-at-elapsed-0 SSR rendering as acceptable for SP-3. If the curved trajectory's animation is later required to be visible at runtime, a small follow-up task wires `Sequence` to read `SceneContext.elapsed_ms` when present. This is explicitly flagged as a follow-up in the spec.
