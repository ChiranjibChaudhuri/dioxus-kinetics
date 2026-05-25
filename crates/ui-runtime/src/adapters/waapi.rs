//! Web Animations API bridge as a FrameAdapter. Wraps a paused
//! `web_sys::Animation`; each seek forces `play_state = "paused"` and
//! sets `current_time = elapsed_ms`. Reduced-motion freezes at
//! `duration_ms` after one seek.

#![cfg(target_arch = "wasm32")]

use std::cell::Cell;

use web_sys::{Animation, Element};

use crate::frame_adapter::FrameAdapter;

pub struct WaapiAdapter {
    id: String,
    duration_ms: f32,
    animation: Animation,
    target: Element,
    reduced_locked: Cell<bool>,
}

impl WaapiAdapter {
    pub fn new(
        id: impl Into<String>,
        duration_ms: f32,
        animation: Animation,
        target: Element,
    ) -> Self {
        Self {
            id: id.into(),
            duration_ms,
            animation,
            target,
            reduced_locked: Cell::new(false),
        }
    }
}

impl FrameAdapter for WaapiAdapter {
    fn id(&self) -> &str {
        &self.id
    }
    fn duration_ms(&self) -> f32 {
        self.duration_ms
    }
    fn seek(&self, elapsed_ms: f32, reduced: bool) {
        if !self.target.is_connected() {
            return;
        }
        if reduced {
            if self.reduced_locked.get() {
                return;
            }
            self.reduced_locked.set(true);
            let _ = self.animation.pause();
            self.animation
                .set_current_time(Some(self.duration_ms as f64));
            return;
        }
        self.reduced_locked.set(false);
        let _ = self.animation.pause();
        self.animation
            .set_current_time(Some(elapsed_ms.clamp(0.0, self.duration_ms) as f64));
    }
}
