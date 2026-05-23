//! Latest motion-input snapshot consumed by the compositor when building per-
//! frame uniforms. Springs and decays live in `ui-motion`; this struct just
//! holds the most recent values written by the host. Plan 4's Dioxus
//! integration will subscribe to ui-motion signals and call
//! `Compositor::update_inputs` per rAF tick.

#[derive(Clone, Copy, Debug, Default)]
pub struct MotionInputs {
    /// Pointer in canvas-relative coords (px). Compositor normalizes to
    /// surface-local (-1..1) per region.
    pub pointer_px: [f32; 2],
    /// Scroll velocity in px/s.
    pub scroll_velocity_px: [f32; 2],
    /// Seconds since route mount.
    pub time_seconds: f32,
    /// Whether `prefers-reduced-motion` is active. When true the compositor
    /// zeroes pointer/scroll/time before writing uniforms.
    pub reduced_motion: bool,
}

impl MotionInputs {
    pub fn new() -> Self { Self::default() }

    pub fn with_pointer(mut self, px: [f32; 2]) -> Self { self.pointer_px = px; self }
    pub fn with_scroll_velocity(mut self, vel: [f32; 2]) -> Self { self.scroll_velocity_px = vel; self }
    pub fn with_time(mut self, t: f32) -> Self { self.time_seconds = t; self }
    pub fn with_reduced_motion(mut self, on: bool) -> Self { self.reduced_motion = on; self }
}
