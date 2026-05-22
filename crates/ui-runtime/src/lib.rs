#![forbid(unsafe_code)]

//! Animation runtime for the kinetics UI library.

#[cfg(not(target_arch = "wasm32"))]
mod scheduler_native;

#[cfg(target_arch = "wasm32")]
mod scheduler_web;

pub mod animation;
pub mod measurement;
pub mod presence;
pub mod reduced_motion;
pub mod scheduler;
pub mod shared;
pub mod state;
pub mod timeline;

#[cfg(not(target_arch = "wasm32"))]
mod measurement_native;

#[cfg(target_arch = "wasm32")]
mod measurement_web;

pub use animation::use_animation_value;
pub use measurement::{use_element_computed_style, use_element_rect, MountedRectCallback};
pub use presence::use_presence_state;
pub use reduced_motion::{use_reduced_motion, ReducedMotion};
pub use scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};
pub use shared::{
    use_shared_element_registry, ElementSnapshot, SharedElementRegistry, SharedTransition,
};
pub use state::{advance_presence, PresenceInputs, PresenceState, PresenceTransition};
pub use timeline::use_timeline_sample;
