#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spring {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
}

impl Spring {
    pub const fn snappy() -> Self {
        Self {
            stiffness: 420.0,
            damping: 34.0,
            mass: 1.0,
        }
    }

    /// Critically damped preset: stiffness 300, mass 1, damping `2*sqrt(300)`
    /// (≈ 34.641) so the damping ratio `zeta` is exactly 1.0 — the spring
    /// settles as fast as possible without overshoot. Not `const` because the
    /// critical-damping coefficient requires a runtime `sqrt`.
    pub fn smooth() -> Self {
        Self {
            stiffness: 300.0,
            damping: 2.0 * (300.0_f32).sqrt(),
            mass: 1.0,
        }
    }

    /// Gently underdamped preset (stiffness 170, damping 26, mass 1) — a calm
    /// ease with a barely perceptible settle.
    pub const fn gentle() -> Self {
        Self {
            stiffness: 170.0,
            damping: 26.0,
            mass: 1.0,
        }
    }

    /// Lively, springy preset (stiffness 280, damping 18, mass 1) with a
    /// visible overshoot before settling.
    pub const fn bouncy() -> Self {
        Self {
            stiffness: 280.0,
            damping: 18.0,
            mass: 1.0,
        }
    }

    /// Construct a spring from a SwiftUI-style `(response, dampingRatio)` pair
    /// with unit mass. `response_s` is the perceptual duration of one
    /// oscillation in seconds; `damping_ratio` (zeta) is 1.0 for critical
    /// damping, < 1.0 for overshoot, > 1.0 for an overdamped approach.
    ///
    /// Mirrors `SwiftUI.Animation.spring(response:dampingFraction:)`:
    ///   omega = 2*pi / response;  stiffness = omega^2 * mass;
    ///   damping = 2 * dampingRatio * omega * mass.
    ///
    /// Non-finite or non-positive `response_s` collapses to a stiff,
    /// critically damped fallback so callers never observe NaN springs.
    pub fn from_response_and_damping(response_s: f32, damping_ratio: f32) -> Spring {
        let mass = 1.0_f32;
        let response_s = if response_s.is_finite() && response_s > 0.0 {
            response_s
        } else {
            // Degenerate response → near-instant, critically damped spring.
            let stiffness = 1000.0_f32;
            return Spring {
                stiffness,
                damping: 2.0 * (stiffness * mass).sqrt(),
                mass,
            };
        };
        let damping_ratio = if damping_ratio.is_finite() && damping_ratio >= 0.0 {
            damping_ratio
        } else {
            1.0
        };
        let omega = 2.0 * std::f32::consts::PI / response_s;
        Spring {
            stiffness: omega * omega * mass,
            damping: 2.0 * damping_ratio * omega * mass,
            mass,
        }
    }

    /// Estimates the time (in milliseconds) until the spring settles to within
    /// `tolerance` of its rest position. Uses the underdamped decay envelope
    /// `e^{-zeta * omega_n * t}`; for critically- or overdamped springs the
    /// same closed form holds as a conservative upper bound.
    ///
    /// Returns `f32::INFINITY` if the spring has no restoring force or no
    /// damping (it would never settle deterministically). Callers that need a
    /// finite value should clamp.
    pub fn settling_duration_ms(self, tolerance: f32) -> f32 {
        let stiffness = if self.stiffness.is_finite() {
            self.stiffness.max(0.0)
        } else {
            0.0
        };
        let damping = if self.damping.is_finite() {
            self.damping.max(0.0)
        } else {
            0.0
        };
        let mass = if self.mass.is_finite() && self.mass > 0.0 {
            self.mass
        } else {
            1.0
        };
        let tolerance = if tolerance.is_finite() && tolerance > 0.0 {
            tolerance.min(1.0)
        } else {
            1e-3
        };
        if stiffness <= 0.0 || damping <= 0.0 {
            return f32::INFINITY;
        }
        let omega_n = (stiffness / mass).sqrt();
        let c_crit = 2.0 * (mass * stiffness).sqrt();
        let zeta = damping / c_crit;
        let sigma = zeta * omega_n;
        if sigma <= 0.0 {
            return f32::INFINITY;
        }
        (1.0 / tolerance).ln() / sigma * 1000.0
    }

    pub fn step(self, value: f32, target: f32, velocity: f32, delta_seconds: f32) -> SpringStep {
        let stiffness = if self.stiffness.is_finite() {
            self.stiffness.max(0.0)
        } else {
            0.0
        };
        let damping = if self.damping.is_finite() {
            self.damping.max(0.0)
        } else {
            0.0
        };
        let mass = if self.mass.is_finite() && self.mass > 0.0 {
            self.mass
        } else {
            1.0
        };
        let value = if value.is_finite() { value } else { 0.0 };
        let target = if target.is_finite() { target } else { 0.0 };
        let velocity = if velocity.is_finite() { velocity } else { 0.0 };
        let delta_seconds = if delta_seconds.is_finite() && delta_seconds >= 0.0 {
            delta_seconds
        } else {
            0.0
        };

        let displacement = value - target;
        let spring_force = -stiffness * displacement;
        let damping_force = -damping * velocity;
        let acceleration = (spring_force + damping_force) / mass;
        let next_velocity = velocity + acceleration * delta_seconds;
        let next_value = value + next_velocity * delta_seconds;

        SpringStep {
            value: next_value,
            velocity: next_velocity,
        }
    }

    /// Integrates the damped-spring ODE over `delta_seconds` in ONE step using
    /// the closed-form analytic solution. Unlike [`Spring::step`] (a single
    /// semi-implicit Euler step that overshoots or diverges when a frame `dt`
    /// spikes — e.g. when a background tab regains focus and the RAF loop is
    /// handed a multi-second delta), this is numerically stable for any
    /// `delta_seconds`: a large `dt` converges toward the settled target
    /// instead of exploding. For small `dt` it matches `step` to within
    /// integration error.
    ///
    /// Solves `x'' + 2·ζ·ω·x' + ω²·x = 0` for the displacement `x = value -
    /// target`, branching on the damping ratio `ζ` (under-, critically-, and
    /// over-damped). Degenerate springs (no restoring force, zero `dt`) defer
    /// to [`Spring::step`].
    pub fn analytic_step(
        self,
        value: f32,
        target: f32,
        velocity: f32,
        delta_seconds: f32,
    ) -> SpringStep {
        let stiffness = if self.stiffness.is_finite() {
            self.stiffness.max(0.0)
        } else {
            0.0
        };
        let mass = if self.mass.is_finite() && self.mass > 0.0 {
            self.mass
        } else {
            1.0
        };
        let damping = if self.damping.is_finite() {
            self.damping.max(0.0)
        } else {
            0.0
        };
        let value = if value.is_finite() { value } else { 0.0 };
        let target = if target.is_finite() { target } else { 0.0 };
        let velocity = if velocity.is_finite() { velocity } else { 0.0 };
        let dt = if delta_seconds.is_finite() && delta_seconds >= 0.0 {
            delta_seconds
        } else {
            0.0
        };

        // No restoring force, or a zero step: the closed form is undefined /
        // a no-op — defer to the explicit Euler step.
        if stiffness <= 0.0 || dt == 0.0 {
            return self.step(value, target, velocity, dt);
        }

        let omega = (stiffness / mass).sqrt();
        let zeta = damping / (2.0 * (stiffness * mass).sqrt());
        let x0 = value - target;
        let v0 = velocity;

        let (x, v) = if zeta < 1.0 - 1e-4 {
            // Underdamped: oscillatory decay.
            let wd = omega * (1.0 - zeta * zeta).sqrt();
            let sigma = zeta * omega;
            let e = (-sigma * dt).exp();
            let (sin, cos) = (wd * dt).sin_cos();
            let a = x0;
            let b = (v0 + sigma * x0) / wd;
            let x = e * (a * cos + b * sin);
            let v = e * ((b * wd - sigma * a) * cos - (a * wd + sigma * b) * sin);
            (x, v)
        } else if zeta > 1.0 + 1e-4 {
            // Overdamped: sum of two decaying exponentials.
            let disc = omega * (zeta * zeta - 1.0).sqrt();
            let r1 = -zeta * omega + disc;
            let r2 = -zeta * omega - disc;
            let c2 = (v0 - r1 * x0) / (r2 - r1);
            let c1 = x0 - c2;
            let e1 = (r1 * dt).exp();
            let e2 = (r2 * dt).exp();
            let x = c1 * e1 + c2 * e2;
            let v = c1 * r1 * e1 + c2 * r2 * e2;
            (x, v)
        } else {
            // Critically damped.
            let e = (-omega * dt).exp();
            let a = x0;
            let b = v0 + omega * x0;
            let x = (a + b * dt) * e;
            let v = (b - omega * (a + b * dt)) * e;
            (x, v)
        };

        SpringStep {
            value: target + x,
            velocity: v,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringStep {
    pub value: f32,
    pub velocity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ease {
    Linear,
    Standard,
    /// Material/HIG "emphasized" motion. Maps to the CSS cubic-bezier token
    /// `--ui-ease-emphasized: cubic-bezier(0.2, 0, 0, 1)`.
    Emphasized,
    /// Decelerating (front-loaded) motion — fast out of the gate, easing into
    /// rest. CSS token `--ui-ease-decelerate: cubic-bezier(0, 0, 0, 1)`.
    Decelerate,
    /// Accelerating (back-loaded) motion — slow to start, ramping up.
    /// CSS token `--ui-ease-accelerate: cubic-bezier(0.4, 0, 1, 1)`.
    Accelerate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clamp {
    Yes,
    No,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TweenSample {
    pub progress: f32,
    pub value: f32,
}

pub fn interpolate(from: f32, to: f32, progress: f32, clamp: Clamp) -> f32 {
    let from = finite_or_zero(from);
    let to = finite_or_zero(to);
    let progress = finite_or_zero(progress);
    let progress = match clamp {
        Clamp::Yes => progress.clamp(0.0, 1.0),
        Clamp::No => progress,
    };

    from + (to - from) * progress
}

pub fn sample_tween(
    from: f32,
    to: f32,
    elapsed_ms: f32,
    duration_ms: f32,
    ease: Ease,
) -> TweenSample {
    let progress = if duration_ms.is_finite() {
        if duration_ms <= 0.0 {
            1.0
        } else {
            let raw = finite_or_zero(elapsed_ms) / duration_ms;
            apply_ease(raw.clamp(0.0, 1.0), ease)
        }
    } else {
        let raw = finite_or_zero(elapsed_ms);
        apply_ease(raw.clamp(0.0, 1.0), ease)
    };

    TweenSample {
        progress,
        value: interpolate(from, to, progress, Clamp::Yes),
    }
}

pub fn apply_ease(progress: f32, ease: Ease) -> f32 {
    let progress = finite_or_zero(progress).clamp(0.0, 1.0);
    match ease {
        Ease::Linear => progress,
        Ease::Standard => progress * progress * (3.0 - 2.0 * progress),
        // The CSS cubic-bezier tokens are mirrored here so the WAAPI keyframe
        // sampler and the wgpu engine produce identical curves to the browser's
        // native `cubic-bezier(...)` timing function. A cubic-bezier easing
        // curve is parameterised by its two control points (the endpoints are
        // pinned at (0,0) and (1,1)); `cubic_bezier_ease` solves for the curve
        // parameter `t` such that x(t) == progress, then evaluates y(t).
        Ease::Emphasized => cubic_bezier_ease(progress, 0.2, 0.0, 0.0, 1.0),
        Ease::Decelerate => cubic_bezier_ease(progress, 0.0, 0.0, 0.0, 1.0),
        Ease::Accelerate => cubic_bezier_ease(progress, 0.4, 0.0, 1.0, 1.0),
    }
}

/// Evaluate a CSS-style cubic-bezier timing function at `x` (the eased
/// progress for input fraction `x`). Control points are `(x1, y1)` and
/// `(x2, y2)`; the curve passes through `(0,0)` and `(1,1)`.
///
/// CSS timing functions are defined as a parametric Bézier where the curve
/// parameter `s ∈ [0,1]` is *not* the same as the horizontal axis `x`. We
/// first solve `bezier_axis(s, x1, x2) == x` for `s` (a monotonic function on
/// `[0,1]` whenever `x1, x2 ∈ [0,1]`, which holds for all our presets), then
/// evaluate the vertical axis `bezier_axis(s, y1, y2)`.
///
/// The solve uses a few Newton-Raphson iterations seeded by `x` (a good guess
/// because the axis function is close to identity), falling back to bisection
/// when the derivative is too small to step reliably. This mirrors the WebKit
/// / Blink `UnitBezier::solveCurveX` strategy.
fn cubic_bezier_ease(x: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let s = solve_bezier_axis_for_x(x, x1, x2);
    bezier_axis(s, y1, y2).clamp(0.0, 1.0)
}

/// One axis of a cubic Bézier with endpoints pinned at 0 and 1, evaluated at
/// curve parameter `s ∈ [0,1]`. With control coordinates `p1`, `p2` this is the
/// expanded Bernstein form `3(1-s)^2 s·p1 + 3(1-s) s^2·p2 + s^3`.
fn bezier_axis(s: f32, p1: f32, p2: f32) -> f32 {
    let one_minus = 1.0 - s;
    3.0 * one_minus * one_minus * s * p1 + 3.0 * one_minus * s * s * p2 + s * s * s
}

/// Derivative of [`bezier_axis`] with respect to the curve parameter `s`.
fn bezier_axis_derivative(s: f32, p1: f32, p2: f32) -> f32 {
    let one_minus = 1.0 - s;
    3.0 * one_minus * one_minus * p1 + 6.0 * one_minus * s * (p2 - p1) + 3.0 * s * s * (1.0 - p2)
}

/// Solve `bezier_axis(s, x1, x2) == target_x` for `s ∈ [0,1]`.
fn solve_bezier_axis_for_x(target_x: f32, x1: f32, x2: f32) -> f32 {
    // Newton-Raphson, seeded with the (close) guess s = target_x.
    let mut s = target_x;
    for _ in 0..8 {
        let x = bezier_axis(s, x1, x2) - target_x;
        if x.abs() < 1e-6 {
            return s.clamp(0.0, 1.0);
        }
        let dx = bezier_axis_derivative(s, x1, x2);
        if dx.abs() < 1e-6 {
            break;
        }
        s -= x / dx;
    }
    // Fallback: bisection within the valid parameter range.
    let mut lo = 0.0_f32;
    let mut hi = 1.0_f32;
    let mut s = target_x.clamp(lo, hi);
    for _ in 0..32 {
        let x = bezier_axis(s, x1, x2);
        if (x - target_x).abs() < 1e-6 {
            break;
        }
        if x < target_x {
            lo = s;
        } else {
            hi = s;
        }
        s = 0.5 * (lo + hi);
    }
    s.clamp(0.0, 1.0)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Transition {
    Tween { duration_ms: u32, ease: Ease },
    Spring(Spring),
}

impl Transition {
    pub const fn tween(duration_ms: u32) -> Self {
        Self::Tween {
            duration_ms,
            ease: Ease::Standard,
        }
    }

    pub const fn spring(spring: Spring) -> Self {
        Self::Spring(spring)
    }

    pub const fn reduced(self) -> Self {
        match self {
            Self::Tween { ease, .. } => Self::Tween {
                duration_ms: 0,
                ease,
            },
            Self::Spring(_) => Self::Tween {
                duration_ms: 0,
                ease: Ease::Linear,
            },
        }
    }

    pub const fn duration_ms(self) -> u32 {
        match self {
            Self::Tween { duration_ms, .. } => duration_ms,
            Self::Spring(_) => 0,
        }
    }

    pub const fn fixed_duration_ms(self) -> Option<u32> {
        match self {
            Self::Tween { duration_ms, .. } => Some(duration_ms),
            Self::Spring(_) => None,
        }
    }

    /// Returns a finite duration estimate suitable for timeline budgeting. For
    /// tweens this is the configured duration; for springs it is the settling
    /// time derived from the damping envelope, clamped to a reasonable upper
    /// bound to avoid `Infinity` for undamped or undertuned springs.
    pub fn estimated_duration_ms(self) -> f32 {
        match self {
            Self::Tween { duration_ms, .. } => duration_ms as f32,
            Self::Spring(spring) => {
                let estimate = spring.settling_duration_ms(0.005);
                if estimate.is_finite() {
                    estimate.clamp(0.0, 4_000.0)
                } else {
                    2_000.0
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenceState {
    Present,
    Exiting,
    Removed,
}

impl PresenceState {
    pub const fn request_exit(self) -> Self {
        match self {
            Self::Present => Self::Exiting,
            Self::Exiting | Self::Removed => self,
        }
    }

    pub const fn finish_exit(self) -> Self {
        match self {
            Self::Exiting => Self::Removed,
            Self::Present | Self::Removed => self,
        }
    }
}

fn finite_or_zero(value: f32) -> f32 {
    if value.is_finite() {
        value
    } else {
        0.0
    }
}

// ---------------------------------------------------------------------------
// Keyframe compilation for WAAPI consumption.
//
// A `Keyframes` value is a series of per-frame property maps that can be
// handed to `Element.animate(...)` in the browser. Tweens are sampled at
// 30fps × duration_ms so that smoothstep (the Standard ease) round-trips
// exactly through WAAPI's linear interpolation. Springs are sampled at
// 60fps × settling_duration_ms.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Keyframes {
    pub frames: Vec<Keyframe>,
    pub duration_ms: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Keyframe {
    /// Offset within [0.0, 1.0]; passed to WAAPI verbatim.
    pub offset: f32,
    /// Animated value at this offset (the caller decides which CSS
    /// property it maps to — opacity, transform component, etc.).
    pub value: f32,
}

const TWEEN_FPS: f32 = 30.0;
const SPRING_FPS: f32 = 60.0;
const SPRING_TOLERANCE: f32 = 0.005;

/// Compile a transition between `from` and `to` into a `Keyframes` array
/// suitable for `Element.animate(...)`. The number of frames depends on
/// the transition: tweens use 30fps sampling, springs use 60fps sampling.
pub fn keyframes_for_transition(from: f32, to: f32, transition: Transition) -> Keyframes {
    keyframes_for_transition_from_velocity(from, to, transition, 0.0)
}

/// Like [`keyframes_for_transition`] but seeds the spring with a non-zero
/// `initial_velocity` (in value-units per second). Use this to make an
/// *interrupted* spring continue with the velocity it already had instead of
/// restarting from rest — e.g. when a target changes mid-flight and the
/// consumer has sampled the in-flight velocity. Tweens ignore the velocity
/// (they have no momentum); only the `Spring` arm uses it.
pub fn keyframes_for_transition_from_velocity(
    from: f32,
    to: f32,
    transition: Transition,
    initial_velocity: f32,
) -> Keyframes {
    match transition {
        Transition::Tween { duration_ms, ease } => tween_keyframes(from, to, duration_ms, ease),
        Transition::Spring(spring) => spring_keyframes_seeded(from, to, spring, initial_velocity),
    }
}

fn tween_keyframes(from: f32, to: f32, duration_ms: u32, ease: Ease) -> Keyframes {
    let duration = duration_ms as f32;
    if duration == 0.0 {
        return Keyframes {
            frames: vec![
                Keyframe {
                    offset: 0.0,
                    value: to,
                },
                Keyframe {
                    offset: 1.0,
                    value: to,
                },
            ],
            duration_ms: 0.0,
        };
    }
    let count = ((duration * TWEEN_FPS / 1000.0).ceil() as usize).max(2);
    let mut frames = Vec::with_capacity(count + 1);
    for i in 0..=count {
        let progress = i as f32 / count as f32;
        let eased = apply_ease(progress, ease);
        let value = from + (to - from) * eased;
        frames.push(Keyframe {
            offset: progress,
            value,
        });
    }
    Keyframes {
        frames,
        duration_ms: duration,
    }
}

fn spring_keyframes_seeded(from: f32, to: f32, spring: Spring, initial_velocity: f32) -> Keyframes {
    let settle = spring
        .settling_duration_ms(SPRING_TOLERANCE)
        .clamp(50.0, 4_000.0);
    let count = ((settle * SPRING_FPS / 1000.0).ceil() as usize).max(2);
    let dt = 1.0 / SPRING_FPS;
    let mut value = from;
    let mut velocity = if initial_velocity.is_finite() {
        initial_velocity
    } else {
        0.0
    };
    let mut frames = Vec::with_capacity(count + 2);
    frames.push(Keyframe {
        offset: 0.0,
        value: from,
    });
    for i in 1..=count {
        let step = spring.step(value, to, velocity, dt);
        value = step.value;
        velocity = step.velocity;
        frames.push(Keyframe {
            offset: (i as f32) / (count as f32),
            value,
        });
    }
    if let Some(last) = frames.last_mut() {
        last.value = to;
    }
    Keyframes {
        frames,
        duration_ms: settle,
    }
}

#[cfg(test)]
mod keyframe_tests {
    use super::*;

    #[test]
    fn tween_first_frame_is_from() {
        let kf = keyframes_for_transition(
            0.0,
            1.0,
            Transition::Tween {
                duration_ms: 220,
                ease: Ease::Standard,
            },
        );
        assert_eq!(kf.frames.first().unwrap().offset, 0.0);
        assert!((kf.frames.first().unwrap().value - 0.0).abs() < 1e-4);
    }

    #[test]
    fn tween_last_frame_is_to() {
        let kf = keyframes_for_transition(
            0.0,
            1.0,
            Transition::Tween {
                duration_ms: 220,
                ease: Ease::Standard,
            },
        );
        assert_eq!(kf.frames.last().unwrap().offset, 1.0);
        assert!((kf.frames.last().unwrap().value - 1.0).abs() < 1e-4);
    }

    #[test]
    fn tween_midpoint_matches_apply_ease() {
        let kf = keyframes_for_transition(
            0.0,
            100.0,
            Transition::Tween {
                duration_ms: 220,
                ease: Ease::Standard,
            },
        );
        let near_mid = kf
            .frames
            .iter()
            .min_by(|a, b| {
                (a.offset - 0.5)
                    .abs()
                    .partial_cmp(&(b.offset - 0.5).abs())
                    .unwrap()
            })
            .unwrap();
        let expected = 0.0 + (100.0 - 0.0) * apply_ease(near_mid.offset, Ease::Standard);
        assert!((near_mid.value - expected).abs() < 1e-3);
    }

    #[test]
    fn spring_first_frame_is_from() {
        let kf = keyframes_for_transition(0.0, 1.0, Transition::Spring(Spring::snappy()));
        assert_eq!(kf.frames.first().unwrap().offset, 0.0);
        assert!((kf.frames.first().unwrap().value - 0.0).abs() < 1e-4);
    }

    #[test]
    fn spring_last_frame_pins_to_target() {
        let kf = keyframes_for_transition(0.0, 1.0, Transition::Spring(Spring::snappy()));
        let last = kf.frames.last().unwrap();
        assert_eq!(last.offset, 1.0);
        assert!((last.value - 1.0).abs() < 1e-6);
    }

    #[test]
    fn spring_duration_matches_settling() {
        let spring = Spring::snappy();
        let kf = keyframes_for_transition(0.0, 1.0, Transition::Spring(spring));
        let expected = spring
            .settling_duration_ms(SPRING_TOLERANCE)
            .clamp(50.0, 4_000.0);
        assert!((kf.duration_ms - expected).abs() < 1e-3);
    }

    #[test]
    fn zero_duration_tween_emits_two_frames_pinned_to_target() {
        let kf = keyframes_for_transition(
            0.0,
            5.0,
            Transition::Tween {
                duration_ms: 0,
                ease: Ease::Linear,
            },
        );
        assert_eq!(kf.frames.len(), 2);
        assert!((kf.frames[0].value - 5.0).abs() < 1e-6);
        assert!((kf.frames[1].value - 5.0).abs() < 1e-6);
    }
}

#[cfg(test)]
mod ease_tests {
    use super::*;

    const NEW_EASES: [Ease; 3] = [Ease::Emphasized, Ease::Decelerate, Ease::Accelerate];

    #[test]
    fn endpoints_are_pinned() {
        for ease in NEW_EASES {
            assert!(
                apply_ease(0.0, ease).abs() < 1e-4,
                "f(0) should be 0 for {ease:?}, got {}",
                apply_ease(0.0, ease)
            );
            assert!(
                (apply_ease(1.0, ease) - 1.0).abs() < 1e-4,
                "f(1) should be 1 for {ease:?}, got {}",
                apply_ease(1.0, ease)
            );
        }
    }

    #[test]
    fn new_eases_are_monotonic_on_unit_interval() {
        for ease in NEW_EASES {
            let mut prev = apply_ease(0.0, ease);
            for i in 1..=1000 {
                let t = i as f32 / 1000.0;
                let y = apply_ease(t, ease);
                assert!(
                    y >= prev - 1e-4,
                    "{ease:?} not monotonic at t={t}: {y} < {prev}"
                );
                prev = y;
            }
        }
    }

    #[test]
    fn decelerate_is_front_loaded_and_accelerate_is_back_loaded() {
        // Decelerate (0,0,0,1) leaves the origin fast: f(0.2) > 0.2.
        let dec = apply_ease(0.2, Ease::Decelerate);
        assert!(dec > 0.2, "Decelerate f(0.2) should exceed 0.2, got {dec}");
        // Accelerate (0.4,0,1,1) lingers near the origin: f(0.2) < 0.2.
        let acc = apply_ease(0.2, Ease::Accelerate);
        assert!(
            acc < 0.2,
            "Accelerate f(0.2) should be below 0.2, got {acc}"
        );
    }

    #[test]
    fn linear_and_standard_remain_bit_exact() {
        // Linear is the identity; Standard is the smoothstep polynomial.
        for i in 0..=100 {
            let t = i as f32 / 100.0;
            assert_eq!(apply_ease(t, Ease::Linear), t.clamp(0.0, 1.0));
            let expected = {
                let p = t.clamp(0.0, 1.0);
                p * p * (3.0 - 2.0 * p)
            };
            assert_eq!(apply_ease(t, Ease::Standard), expected);
        }
    }

    #[test]
    fn new_eases_round_trip_through_keyframes() {
        // The existing keyframe sampler should accept the new variants and
        // pin the endpoints to `from`/`to`.
        for ease in NEW_EASES {
            let kf = keyframes_for_transition(
                0.0,
                1.0,
                Transition::Tween {
                    duration_ms: 220,
                    ease,
                },
            );
            assert!((kf.frames.first().unwrap().value - 0.0).abs() < 1e-3);
            assert!((kf.frames.last().unwrap().value - 1.0).abs() < 1e-3);
        }
    }
}

#[cfg(test)]
mod spring_preset_tests {
    use super::*;

    fn damping_ratio(spring: Spring) -> f32 {
        let c_crit = 2.0 * (spring.mass * spring.stiffness).sqrt();
        spring.damping / c_crit
    }

    #[test]
    fn smooth_is_critically_damped() {
        let zeta = damping_ratio(Spring::smooth());
        assert!(
            (zeta - 1.0).abs() < 1e-3,
            "smooth() zeta should be ~1.0, got {zeta}"
        );
    }

    #[test]
    fn from_response_and_damping_unit_ratio_is_critically_damped() {
        let spring = Spring::from_response_and_damping(0.5, 1.0);
        let zeta = damping_ratio(spring);
        assert!(
            (zeta - 1.0).abs() < 1e-3,
            "expected critically damped, got zeta {zeta}"
        );
    }

    #[test]
    fn from_response_and_damping_matches_swiftui_formula() {
        let response = 0.55_f32;
        let ratio = 0.8_f32;
        let spring = Spring::from_response_and_damping(response, ratio);
        let omega = 2.0 * std::f32::consts::PI / response;
        assert!((spring.stiffness - omega * omega).abs() < 1e-2);
        assert!((spring.damping - 2.0 * ratio * omega).abs() < 1e-3);
        assert_eq!(spring.mass, 1.0);
    }

    #[test]
    fn presets_have_finite_settling_duration() {
        for spring in [
            Spring::smooth(),
            Spring::gentle(),
            Spring::bouncy(),
            Spring::from_response_and_damping(0.5, 1.0),
            Spring::from_response_and_damping(0.3, 0.7),
        ] {
            let settle = spring.settling_duration_ms(0.005);
            assert!(
                settle.is_finite() && settle > 0.0,
                "expected finite positive settle for {spring:?}, got {settle}"
            );
        }
    }

    #[test]
    fn bouncy_is_underdamped_and_gentle_settles_calmly() {
        assert!(
            damping_ratio(Spring::bouncy()) < 1.0,
            "bouncy() should overshoot (zeta < 1)"
        );
        assert!(
            damping_ratio(Spring::gentle()) < 1.0,
            "gentle() is lightly underdamped (zeta < 1)"
        );
    }

    #[test]
    fn from_response_and_damping_rejects_degenerate_inputs() {
        for spring in [
            Spring::from_response_and_damping(0.0, 1.0),
            Spring::from_response_and_damping(-1.0, 1.0),
            Spring::from_response_and_damping(f32::NAN, 1.0),
        ] {
            assert!(spring.stiffness.is_finite() && spring.stiffness > 0.0);
            assert!(spring.damping.is_finite() && spring.damping > 0.0);
            assert!(spring.settling_duration_ms(0.005).is_finite());
        }
    }
}

#[cfg(test)]
mod analytic_step_tests {
    use super::*;

    /// A huge frame `dt` (e.g. a backgrounded tab regaining focus) must not
    /// make the analytic integrator diverge — it should converge toward the
    /// target, which is exactly the regression `step`'s explicit Euler cannot
    /// survive.
    #[test]
    fn analytic_step_is_stable_under_huge_dt() {
        for spring in [
            Spring::snappy(),
            Spring::smooth(),
            Spring::gentle(),
            Spring::bouncy(),
        ] {
            // Start far from rest with a large initial velocity, then take one
            // 5-second step.
            let s = spring.analytic_step(0.0, 100.0, 800.0, 5.0);
            assert!(s.value.is_finite(), "value diverged: {}", s.value);
            assert!(s.velocity.is_finite(), "velocity diverged: {}", s.velocity);
            assert!(
                (s.value - 100.0).abs() < 1.0,
                "expected convergence to target after 5s, got {}",
                s.value
            );
        }
    }

    /// Euler `step` diverges here; `analytic_step` does not — this pins the
    /// difference that motivates the analytic integrator.
    #[test]
    fn euler_diverges_but_analytic_does_not_for_a_large_dt() {
        let spring = Spring::bouncy();
        let euler = spring.step(0.0, 1.0, 0.0, 1.0);
        let analytic = spring.analytic_step(0.0, 1.0, 0.0, 1.0);
        assert!(analytic.value.is_finite());
        // The Euler single-step with dt=1s overshoots wildly (|value| >> 1).
        assert!(
            euler.value.abs() > 2.0,
            "euler unexpectedly stable: {}",
            euler.value
        );
        assert!(
            (analytic.value - 1.0).abs() < 0.6,
            "analytic should be near target, got {}",
            analytic.value
        );
    }

    /// The closed-form `analytic_step` over a duration must agree with a
    /// fine-grained Euler integration of the same ODE over that duration (a
    /// single Euler step would not — it carries O(dt) error — which is exactly
    /// why the analytic form exists).
    #[test]
    fn analytic_matches_fine_grained_euler() {
        let spring = Spring::smooth();
        let total = 0.05_f32;
        let substeps = 4000;
        let h = total / substeps as f32;
        let mut val = 0.0_f32;
        let mut v = 0.0_f32;
        for _ in 0..substeps {
            let s = spring.step(val, 1.0, v, h);
            val = s.value;
            v = s.velocity;
        }
        let a = spring.analytic_step(0.0, 1.0, 0.0, total);
        assert!(
            (a.value - val).abs() < 2e-3,
            "analytic value {} vs fine-euler {}",
            a.value,
            val
        );
        assert!(
            (a.velocity - v).abs() < 2e-1,
            "analytic velocity {} vs fine-euler {}",
            a.velocity,
            v
        );
    }

    /// Initial conditions are reproduced at t≈0.
    #[test]
    fn analytic_step_preserves_initial_conditions_at_tiny_dt() {
        let spring = Spring::snappy();
        let s = spring.analytic_step(10.0, 0.0, -5.0, 1e-5);
        assert!((s.value - 10.0).abs() < 1e-2);
        assert!((s.velocity - (-5.0)).abs() < 1e-1);
    }

    /// Seeding a non-zero velocity changes the early spring trajectory (proves
    /// interruption continuity is wired through the keyframe builder).
    #[test]
    fn velocity_seeded_keyframes_differ_from_rest() {
        let t = Transition::Spring(Spring::snappy());
        let at_rest = keyframes_for_transition(0.0, 1.0, t);
        let moving = keyframes_for_transition_from_velocity(0.0, 1.0, t, 600.0);
        // Both still pin first frame to `from` and last to `to`.
        assert!((moving.frames.first().unwrap().value - 0.0).abs() < 1e-4);
        assert!((moving.frames.last().unwrap().value - 1.0).abs() < 1e-6);
        // An early frame should be further along when launched with velocity.
        let i = (moving.frames.len() / 8).max(1);
        assert!(
            moving.frames[i].value > at_rest.frames[i].value,
            "velocity-seeded spring should lead the at-rest spring early on"
        );
    }
}
