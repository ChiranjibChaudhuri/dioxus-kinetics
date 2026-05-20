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
        let displacement = value - target;
        let spring_force = -self.stiffness * displacement;
        let damping_force = -self.damping * velocity;
        let acceleration = (spring_force + damping_force) / self.mass;
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
