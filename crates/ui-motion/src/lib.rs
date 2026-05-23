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
    }
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
    match transition {
        Transition::Tween { duration_ms, ease } => tween_keyframes(from, to, duration_ms, ease),
        Transition::Spring(spring) => spring_keyframes(from, to, spring),
    }
}

fn tween_keyframes(from: f32, to: f32, duration_ms: u32, ease: Ease) -> Keyframes {
    let duration = duration_ms as f32;
    if duration == 0.0 {
        return Keyframes {
            frames: vec![
                Keyframe { offset: 0.0, value: to },
                Keyframe { offset: 1.0, value: to },
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
        frames.push(Keyframe { offset: progress, value });
    }
    Keyframes { frames, duration_ms: duration }
}

fn spring_keyframes(from: f32, to: f32, spring: Spring) -> Keyframes {
    let settle = spring.settling_duration_ms(SPRING_TOLERANCE).clamp(50.0, 4_000.0);
    let count = ((settle * SPRING_FPS / 1000.0).ceil() as usize).max(2);
    let dt = 1.0 / SPRING_FPS;
    let mut value = from;
    let mut velocity = 0.0_f32;
    let mut frames = Vec::with_capacity(count + 2);
    frames.push(Keyframe { offset: 0.0, value: from });
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
    Keyframes { frames, duration_ms: settle }
}

#[cfg(test)]
mod keyframe_tests {
    use super::*;

    #[test]
    fn tween_first_frame_is_from() {
        let kf = keyframes_for_transition(
            0.0,
            1.0,
            Transition::Tween { duration_ms: 220, ease: Ease::Standard },
        );
        assert_eq!(kf.frames.first().unwrap().offset, 0.0);
        assert!((kf.frames.first().unwrap().value - 0.0).abs() < 1e-4);
    }

    #[test]
    fn tween_last_frame_is_to() {
        let kf = keyframes_for_transition(
            0.0,
            1.0,
            Transition::Tween { duration_ms: 220, ease: Ease::Standard },
        );
        assert_eq!(kf.frames.last().unwrap().offset, 1.0);
        assert!((kf.frames.last().unwrap().value - 1.0).abs() < 1e-4);
    }

    #[test]
    fn tween_midpoint_matches_apply_ease() {
        let kf = keyframes_for_transition(
            0.0,
            100.0,
            Transition::Tween { duration_ms: 220, ease: Ease::Standard },
        );
        let near_mid = kf
            .frames
            .iter()
            .min_by(|a, b| (a.offset - 0.5).abs().partial_cmp(&(b.offset - 0.5).abs()).unwrap())
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
        let expected = spring.settling_duration_ms(SPRING_TOLERANCE).clamp(50.0, 4_000.0);
        assert!((kf.duration_ms - expected).abs() < 1e-3);
    }

    #[test]
    fn zero_duration_tween_emits_two_frames_pinned_to_target() {
        let kf = keyframes_for_transition(
            0.0,
            5.0,
            Transition::Tween { duration_ms: 0, ease: Ease::Linear },
        );
        assert_eq!(kf.frames.len(), 2);
        assert!((kf.frames[0].value - 5.0).abs() < 1e-6);
        assert!((kf.frames[1].value - 5.0).abs() < 1e-6);
    }
}
