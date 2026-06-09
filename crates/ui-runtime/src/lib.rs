#![forbid(unsafe_code)]

//! Animation runtime for the kinetics UI library.

#[cfg(not(target_arch = "wasm32"))]
mod scheduler_native;

#[cfg(target_arch = "wasm32")]
mod scheduler_web;

pub mod adapters;
pub mod animation;
pub mod drivers;
pub mod frame_adapter;
pub mod measurement;
pub mod presence;
pub mod reduced_motion;
pub mod scene_clock;
pub mod scene_driver;
pub mod scheduler;
pub mod shared;
pub mod state;
pub mod theme;
pub mod timeline;

#[cfg(not(target_arch = "wasm32"))]
mod measurement_native;

#[cfg(target_arch = "wasm32")]
mod measurement_web;

#[cfg(target_arch = "wasm32")]
pub mod waapi;
#[cfg(not(target_arch = "wasm32"))]
#[path = "waapi_stub.rs"]
pub mod waapi;

pub use animation::{
    use_animation_target, use_animation_value, use_animation_value_from, UseAnimationTarget,
};
pub use measurement::{use_element_computed_style, use_element_rect, MountedRectCallback};
pub use presence::{use_presence_animation, use_presence_state};
pub use reduced_motion::{
    detect_reduced_motion_at_root, use_reduced_motion, ReducedMotion, ReducedMotionProvider,
};
pub use scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};
pub use shared::{
    now_ms, use_shared_element_registry, ElementSnapshot, SharedElementRegistry, SharedTransition,
    SHARED_SNAPSHOT_STALENESS_MS,
};
pub use state::{advance_presence, PresenceInputs, PresenceState, PresenceTransition};
pub use theme::{
    detect_density_at_root, detect_theme_mode_at_root, use_density, use_theme_mode, ThemeProvider,
};
pub use timeline::use_timeline_sample;
pub use waapi::{is_supported as is_waapi_supported, AnimatedProperty, WaapiAnimation};

pub use adapters::{CssKeyframesAdapter, SequenceAdapter, WaapiAdapter};
pub use drivers::{install_scroll_driver, ScrollDriverHandle};
pub use frame_adapter::{FrameAdapter, FrameAdapterHandle, FrameAdapterRegistry};
pub use scene_clock::{SceneClock, SceneState};
pub use scene_driver::{SceneDriver, ScrollObserverConfig};
