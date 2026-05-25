//! Scene driver selection.
//!
//! A `SceneDriver` selects how a `SceneClock` advances. SP-1 shipped
//! with Autoplay implicit; SP-3 promotes that choice to a value so
//! the same `Scene` Dioxus component can be driven by autoplay,
//! scroll position, or programmatic seeks.

/// How a `SceneClock` is advanced.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum SceneDriver {
    /// SP-1 default: clock advances via `spawn_frame_loop` on mount
    /// until `duration_ms` is reached.
    #[default]
    Autoplay,
    /// Clock progress is driven by scroll position through a
    /// configured trigger region. Web-only; native targets construct
    /// the driver but hold the clock at progress 0.
    Scroll(ScrollObserverConfig),
    /// Autoplay disabled; clock only moves via explicit `seek_*` calls
    /// (the transport scrubber still works).
    Manual,
}

/// Configures the trigger region for `SceneDriver::Scroll`.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollObserverConfig {
    /// CSS selector for the trigger region's root element.
    pub trigger_selector: String,
    /// Viewport offset (px from top) at which progress = 0. Default:
    /// the viewport height (progress starts when the trigger enters
    /// the bottom of the viewport).
    pub start_offset_px: Option<f32>,
    /// Viewport offset (px from top) at which progress = 1. Default: 0
    /// (progress completes when the trigger's bottom edge crosses the
    /// top of the viewport).
    pub end_offset_px: Option<f32>,
}

impl ScrollObserverConfig {
    pub fn new(trigger_selector: impl Into<String>) -> Self {
        Self {
            trigger_selector: trigger_selector.into(),
            start_offset_px: None,
            end_offset_px: None,
        }
    }
}
