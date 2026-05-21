//! Platform-abstracted frame scheduler.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlFlow {
    Continue,
    Stop,
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    pub use super::super::scheduler_native::*;
}

#[cfg(target_arch = "wasm32")]
mod imp {
    pub use super::super::scheduler_web::*;
}

pub use imp::{spawn_frame_loop, FrameHandle};
