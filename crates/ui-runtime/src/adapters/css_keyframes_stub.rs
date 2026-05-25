#![cfg(not(target_arch = "wasm32"))]

use crate::frame_adapter::FrameAdapter;

pub struct CssKeyframesAdapter {
    id: String,
    duration_ms: f32,
}

impl CssKeyframesAdapter {
    pub fn new(id: impl Into<String>, duration_ms: f32) -> Self {
        Self {
            id: id.into(),
            duration_ms,
        }
    }
}

impl FrameAdapter for CssKeyframesAdapter {
    fn id(&self) -> &str {
        &self.id
    }
    fn duration_ms(&self) -> f32 {
        self.duration_ms
    }
    fn seek(&self, _elapsed_ms: f32, _reduced: bool) {}
}
