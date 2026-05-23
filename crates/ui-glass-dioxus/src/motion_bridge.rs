//! Reads pointer/scroll/reduced-motion. Task 6 fills in event listeners; this
//! file currently defines just the data shape.

use ui_glass_engine::motion::MotionInputs;

#[derive(Clone, Copy, Debug, Default)]
pub struct MotionState {
    pub pointer_px: [f32; 2],
    pub scroll_velocity_px: [f32; 2],
    pub reduced_motion: bool,
}

impl MotionState {
    /// Build a `MotionInputs` snapshot. `start_time_ms` is the loop start in
    /// performance.now() units; the resulting `time_seconds` is relative.
    pub fn to_motion_inputs(self, start_time_ms: f64) -> MotionInputs {
        let now = web_sys_performance_now();
        let elapsed_s = ((now - start_time_ms) / 1000.0) as f32;
        MotionInputs::new()
            .with_pointer(self.pointer_px)
            .with_scroll_velocity(self.scroll_velocity_px)
            .with_time(elapsed_s)
            .with_reduced_motion(self.reduced_motion)
    }
}

#[cfg(target_arch = "wasm32")]
fn web_sys_performance_now() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
fn web_sys_performance_now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}
