#![cfg(target_arch = "wasm32")]

use web_sys::HtmlElement;

use crate::frame_adapter::FrameAdapter;

pub struct CssKeyframesAdapter {
    id: String,
    duration_ms: f32,
    target: HtmlElement,
    keyframes_name: String,
}

impl CssKeyframesAdapter {
    pub fn new(
        id: impl Into<String>,
        duration_ms: f32,
        target: HtmlElement,
        keyframes_name: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            duration_ms,
            target,
            keyframes_name: keyframes_name.into(),
        }
    }

    fn write_style(&self, elapsed_ms: f32) {
        if !self.target.is_connected() {
            return;
        }
        let style = self.target.style();
        let _ = style.set_property("animation-name", &self.keyframes_name);
        let _ = style.set_property("animation-duration", &format!("{}ms", self.duration_ms));
        let _ = style.set_property("animation-fill-mode", "forwards");
        let _ = style.set_property("animation-play-state", "paused");
        let _ = style.set_property("animation-delay", &format!("-{}ms", elapsed_ms));
    }
}

impl FrameAdapter for CssKeyframesAdapter {
    fn id(&self) -> &str {
        &self.id
    }
    fn duration_ms(&self) -> f32 {
        self.duration_ms
    }
    fn seek(&self, elapsed_ms: f32, reduced: bool) {
        let value = if reduced {
            self.duration_ms
        } else {
            elapsed_ms.clamp(0.0, self.duration_ms)
        };
        self.write_style(value);
    }
}
