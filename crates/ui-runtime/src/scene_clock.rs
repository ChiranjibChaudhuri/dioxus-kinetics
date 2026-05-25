//! Signal-backed transport state for a Scene. Owns no rendering — its
//! purpose is to hold `elapsed_ms`, `state`, and `reduced` as Dioxus
//! Signals so any subscriber re-renders on change.
//!
//! Autoplay (frame loop) is wired in a follow-up task; this module
//! intentionally ships seek/settle first so unit tests of clamping and
//! state transitions are independent of the scheduler.

use dioxus::prelude::*;
use ui_composition::FrameClock;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SceneState {
    Paused,
    Playing,
    Settled,
}

/// Cheap-to-clone (`Copy`-able) clock handle: every field is a Dioxus
/// `Signal<T>`, which is `Copy`.
#[derive(Clone, Copy)]
pub struct SceneClock {
    pub duration_ms: Signal<f32>,
    pub elapsed_ms: Signal<f32>,
    pub state: Signal<SceneState>,
    pub fps: Signal<u32>,
    pub reduced: Signal<bool>,
}

impl SceneClock {
    pub fn new(duration_ms: f32, fps: u32, reduced: bool) -> Self {
        let duration_ms = finite_non_negative(duration_ms);
        let (elapsed, state) = if reduced {
            (duration_ms, SceneState::Settled)
        } else {
            (0.0, SceneState::Paused)
        };
        Self {
            duration_ms: Signal::new(duration_ms),
            elapsed_ms: Signal::new(elapsed),
            state: Signal::new(state),
            fps: Signal::new(fps.max(1)),
            reduced: Signal::new(reduced),
        }
    }

    pub fn seek_ms(&self, ms: f32) {
        let ms = if ms.is_finite() { ms } else { 0.0 };
        let duration = *self.duration_ms.peek();
        let clamped = ms.clamp(0.0, duration);
        let mut s = self.elapsed_ms;
        s.set(clamped);
        if clamped >= duration {
            let mut s = self.state;
            s.set(SceneState::Settled);
        } else if *self.state.peek() == SceneState::Settled {
            // Scrubbing back from settled returns to paused.
            let mut s = self.state;
            s.set(SceneState::Paused);
        }
    }

    pub fn seek_progress(&self, fraction: f32) {
        let fraction = if fraction.is_finite() {
            fraction.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let duration = *self.duration_ms.peek();
        self.seek_ms(duration * fraction);
    }

    pub fn settle(&self) {
        let duration = *self.duration_ms.peek();
        let mut s = self.elapsed_ms;
        s.set(duration);
        let mut s = self.state;
        s.set(SceneState::Settled);
    }

    pub fn frame_clock(&self) -> FrameClock {
        let fps = *self.fps.peek();
        debug_assert!(fps > 0, "SceneClock::new clamps fps to >= 1");
        let elapsed = *self.elapsed_ms.peek();
        let frame = (elapsed / 1000.0 * fps as f32).round() as u32;
        FrameClock { frame, fps }
    }

    pub fn peek_elapsed_ms(&self) -> f32 {
        *self.elapsed_ms.peek()
    }
    pub fn peek_state(&self) -> SceneState {
        *self.state.peek()
    }
    pub fn peek_reduced(&self) -> bool {
        *self.reduced.peek()
    }
}

fn finite_non_negative(value: f32) -> f32 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        0.0
    }
}
