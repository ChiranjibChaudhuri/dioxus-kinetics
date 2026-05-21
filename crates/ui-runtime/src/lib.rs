#![forbid(unsafe_code)]

//! Animation runtime for the kinetics UI library.

#[cfg(not(target_arch = "wasm32"))]
mod scheduler_native;

#[cfg(target_arch = "wasm32")]
mod scheduler_web;

pub mod animation;
pub mod reduced_motion;
pub mod scheduler;
pub mod state;

pub use animation::use_animation_value;
pub use reduced_motion::{use_reduced_motion, ReducedMotion};
pub use scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};
pub use state::{advance_presence, PresenceInputs, PresenceState, PresenceTransition};
