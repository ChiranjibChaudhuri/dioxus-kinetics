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
    let duration_ms = if duration_ms.is_finite() && duration_ms > 0.0 {
        duration_ms
    } else {
        1.0
    };
    let raw = finite_or_zero(elapsed_ms) / duration_ms;
    let progress = apply_ease(raw.clamp(0.0, 1.0), ease);

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
