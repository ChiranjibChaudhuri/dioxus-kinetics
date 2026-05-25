//! Signal-backed transport state for a Scene. Owns no rendering — its
//! purpose is to hold `elapsed_ms`, `state`, and `reduced` as Dioxus
//! Signals so any subscriber re-renders on change.
//!
//! Autoplay is wired via `play()` / `pause()`, which spawn a platform
//! frame loop through `crate::scheduler::spawn_frame_loop` and store
//! the returned `FrameHandle` in a Signal-backed slot so the loop is
//! aborted on pause (or drop).

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use ui_composition::FrameClock;

use crate::drivers::{install_scroll_driver, ScrollDriverHandle};
use crate::scene_driver::SceneDriver;
use crate::scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};

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
    handle_slot: Signal<HandleSlot>,
    scroll_slot: Signal<ScrollHandleSlot>,
}

/// Holds the active `FrameHandle` (if any) for an autoplaying clock.
/// Wrapped in `Rc<RefCell<…>>` so the value is `'static` (a requirement
/// for `Signal<T>`) and so the autoplay closure can take a clone of the
/// slot and stash its own handle without moving non-`'static` state.
#[derive(Clone, Default)]
pub(crate) struct HandleSlot(pub(crate) Rc<RefCell<Option<FrameHandle>>>);

/// Holds the active `ScrollDriverHandle` (if any) for a scroll-driven
/// clock. Same Rc<RefCell<...>> wrapping rationale as `HandleSlot`.
#[derive(Clone, Default)]
pub(crate) struct ScrollHandleSlot(pub(crate) Rc<RefCell<Option<ScrollDriverHandle>>>);

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
            handle_slot: Signal::new(HandleSlot::default()),
            scroll_slot: Signal::new(ScrollHandleSlot::default()),
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

    pub fn is_playing(&self) -> bool {
        matches!(*self.state.peek(), SceneState::Playing)
    }

    pub fn pause(&self) {
        if matches!(*self.state.peek(), SceneState::Playing) {
            let mut s = self.state;
            s.set(SceneState::Paused);
        }
        // Drop any active frame loop (the native `FrameHandle::Drop`
        // aborts the underlying tokio task; on wasm it cancels rAF).
        // tokio::task::JoinHandle::abort() is asynchronous. A tick already
        // in flight will complete before the task observes cancellation,
        // so elapsed_ms may advance by up to one FRAME_PERIOD_MS after
        // pause() returns. The scrubber's subsequent seek_ms in response
        // to user input absorbs this drift.
        self.handle_slot.peek().0.borrow_mut().take();
    }

    /// Starts the autoplay loop. Idempotent: calling `play()` on an
    /// already-Playing clock is a no-op. On a `Settled` clock,
    /// `elapsed_ms` rewinds to 0 before the loop starts (replay).
    /// On a `reduced` clock, `play()` synonyms `settle()`.
    pub fn play(&self) {
        if matches!(*self.state.peek(), SceneState::Playing) {
            return;
        }
        if *self.reduced.peek() {
            // Reduced-motion clocks settle immediately and never spawn a
            // frame loop.
            self.settle();
            return;
        }
        if matches!(*self.state.peek(), SceneState::Settled) {
            // Replay from start.
            let mut s = self.elapsed_ms;
            s.set(0.0);
        }
        let mut s = self.state;
        s.set(SceneState::Playing);

        let duration_signal = self.duration_ms;
        let mut elapsed_signal = self.elapsed_ms;
        let mut state_signal = self.state;
        let slot = self.handle_slot.peek().0.clone();

        let handle = spawn_frame_loop(move |dt_ms: f64| {
            let duration = *duration_signal.peek();
            let next = (*elapsed_signal.peek() + dt_ms as f32).min(duration);
            elapsed_signal.set(next);
            if next >= duration {
                state_signal.set(SceneState::Settled);
                ControlFlow::Stop
            } else {
                ControlFlow::Continue
            }
        });
        *slot.borrow_mut() = Some(handle);
    }

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
