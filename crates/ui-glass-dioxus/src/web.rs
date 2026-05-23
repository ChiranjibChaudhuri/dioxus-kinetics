#![cfg(target_arch = "wasm32")]
//! Web-only helpers for canvas + window access.

use dioxus::prelude::MountedEvent;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCanvasElement};

/// Downcast a Dioxus `MountedEvent` to an `HtmlCanvasElement`. Returns `None`
/// if the underlying element isn't a canvas (shouldn't happen in practice
/// since we attach onmounted to a `canvas` element, but defensive).
pub fn canvas_from_mounted(evt: &MountedEvent) -> Option<HtmlCanvasElement> {
    let element: Element = evt.downcast::<Element>()?.clone();
    element.dyn_into::<HtmlCanvasElement>().ok()
}

/// Read the device pixel ratio for hi-DPI canvas sizing.
pub fn device_pixel_ratio() -> f32 {
    web_sys::window()
        .map(|w| w.device_pixel_ratio() as f32)
        .unwrap_or(1.0)
}

/// Resize the underlying canvas drawing-buffer to match its CSS size scaled
/// by the device pixel ratio. Returns the new (width, height) in physical px.
pub fn resize_canvas_to_css_size(canvas: &HtmlCanvasElement) -> (u32, u32) {
    let css_w = canvas.client_width().max(1) as f32;
    let css_h = canvas.client_height().max(1) as f32;
    let dpr = device_pixel_ratio();
    let w = (css_w * dpr).round().max(1.0) as u32;
    let h = (css_h * dpr).round().max(1.0) as u32;
    canvas.set_width(w);
    canvas.set_height(h);
    (w, h)
}
