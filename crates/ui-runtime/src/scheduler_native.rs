//! Tokio-based frame scheduler. Used on non-wasm targets.

#![cfg(not(target_arch = "wasm32"))]

use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::{interval, Instant, MissedTickBehavior};

use super::scheduler::ControlFlow;

const FRAME_PERIOD_MS: u64 = 16;

pub struct FrameHandle {
    join: Option<JoinHandle<()>>,
}

impl Drop for FrameHandle {
    fn drop(&mut self) {
        if let Some(join) = self.join.take() {
            join.abort();
        }
    }
}

pub fn spawn_frame_loop<F>(mut callback: F) -> FrameHandle
where
    F: FnMut(f64) -> ControlFlow + 'static,
{
    let join = tokio::task::spawn_local(async move {
        let mut ticker = interval(Duration::from_millis(FRAME_PERIOD_MS));
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
        // Use `tokio::time::Instant` (rather than `std::time::Instant`)
        // so `dt_ms` reflects virtualised time when tests opt in via
        // `tokio::time::pause()` / `advance()`. In production both clocks
        // are wall-clock backed.
        let mut last = Instant::now();
        loop {
            ticker.tick().await;
            let now = Instant::now();
            let dt_ms = now.duration_since(last).as_secs_f64() * 1000.0;
            last = now;
            if matches!(callback(dt_ms), ControlFlow::Stop) {
                break;
            }
        }
    });
    FrameHandle { join: Some(join) }
}
