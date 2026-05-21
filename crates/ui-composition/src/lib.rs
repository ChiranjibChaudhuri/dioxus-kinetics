#![forbid(unsafe_code)]

use std::cmp::Ordering;

use ui_motion::{interpolate, Clamp};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Composition {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub frame_count: u32,
}

impl Composition {
    pub fn new(id: impl Into<String>, width: u32, height: u32, fps: u32, frame_count: u32) -> Self {
        Self {
            id: id.into(),
            width,
            height,
            fps,
            frame_count,
        }
    }

    pub fn validate(&self) -> Result<(), CompositionError> {
        if self.width == 0 || self.height == 0 {
            return Err(CompositionError::InvalidDimensions);
        }
        if self.fps == 0 {
            return Err(CompositionError::InvalidFps);
        }
        if self.frame_count == 0 {
            return Err(CompositionError::InvalidFrameCount);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompositionError {
    InvalidDimensions,
    InvalidFps,
    InvalidFrameCount,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameClock {
    pub frame: u32,
    pub fps: u32,
}

impl FrameClock {
    pub fn seconds(&self) -> f32 {
        if self.fps == 0 {
            0.0
        } else {
            self.frame as f32 / self.fps as f32
        }
    }

    pub fn progress(&self, start: u32, duration: u32) -> f32 {
        if duration == 0 {
            return 1.0;
        }
        let elapsed = self.frame.saturating_sub(start) as f32;
        (elapsed / duration as f32).clamp(0.0, 1.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClipFill {
    None,
    HoldStart,
    HoldEnd,
    HoldBoth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameClip {
    pub start: u32,
    pub duration: u32,
    pub fill: ClipFill,
}

impl FrameClip {
    pub fn new(start: u32, duration: u32, fill: ClipFill) -> Self {
        Self {
            start,
            duration,
            fill,
        }
    }

    pub fn active_at(&self, frame: u32) -> bool {
        let within_range = frame >= self.start && frame.saturating_sub(self.start) < self.duration;
        match self.fill {
            ClipFill::None => within_range,
            ClipFill::HoldStart => self.duration > 0 && (frame < self.start || within_range),
            ClipFill::HoldEnd => self.duration > 0 && frame >= self.start,
            ClipFill::HoldBoth => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameEase {
    Linear,
    Standard,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameCue {
    pub start: u32,
    pub duration: u32,
    pub opacity_from: f32,
    pub opacity_to: f32,
    pub ease: FrameEase,
}

impl FrameCue {
    pub fn opacity(start: u32, duration: u32, from: f32, to: f32, ease: FrameEase) -> Self {
        Self {
            start,
            duration,
            opacity_from: from,
            opacity_to: to,
            ease,
        }
    }

    pub fn fade_in(start: u32, duration: u32) -> Self {
        Self::opacity(start, duration, 0.0, 1.0, FrameEase::Standard)
    }

    pub fn sample_opacity(&self, clock: FrameClock) -> f32 {
        let progress = clock.progress(self.start, self.duration);
        let progress = match self.ease {
            FrameEase::Linear => progress,
            FrameEase::Standard => progress * progress * (3.0 - 2.0 * progress),
        };
        interpolate(self.opacity_from, self.opacity_to, progress, Clamp::Yes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameLayer {
    pub id: String,
    pub depth: i32,
}

impl FrameLayer {
    pub fn new(id: impl Into<String>, depth: i32) -> Self {
        Self {
            id: id.into(),
            depth,
        }
    }
}

impl Ord for FrameLayer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.depth
            .cmp(&other.depth)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for FrameLayer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
