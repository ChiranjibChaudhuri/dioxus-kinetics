//! DOM measurement hooks. Wasm: real measurement via web-sys. Other targets: no-op.

use std::collections::HashMap;

use dioxus::prelude::*;
use ui_layout::Rect;

#[derive(Clone)]
pub struct MountedRectCallback(pub EventHandler<MountedEvent>);

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    pub use super::super::measurement_native::*;
}

#[cfg(target_arch = "wasm32")]
mod imp {
    pub use super::super::measurement_web::*;
}

pub fn use_element_rect() -> (MountedRectCallback, ReadSignal<Option<Rect>>) {
    imp::use_element_rect_impl()
}

pub fn use_element_computed_style(
    properties: &'static [&'static str],
) -> (
    MountedRectCallback,
    ReadSignal<HashMap<&'static str, String>>,
) {
    imp::use_element_computed_style_impl(properties)
}
