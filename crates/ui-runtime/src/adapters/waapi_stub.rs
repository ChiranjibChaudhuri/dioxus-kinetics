//! Non-wasm stub for `WaapiAdapter`. Stores the metadata but the seek
//! is a no-op. Allows cross-platform code to construct the adapter
//! unconditionally without `cfg` noise.

#![cfg(not(target_arch = "wasm32"))]

use crate::frame_adapter::FrameAdapter;

pub struct WaapiAdapter {
    id: String,
    duration_ms: f32,
}

impl WaapiAdapter {
    pub fn new(id: impl Into<String>, duration_ms: f32) -> Self {
        Self {
            id: id.into(),
            duration_ms,
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
    fn seek(&self, _elapsed_ms: f32, _reduced: bool) {
        // No-op on non-web targets.
    }
}
