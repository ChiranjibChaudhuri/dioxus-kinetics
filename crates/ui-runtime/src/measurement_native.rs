#![cfg(not(target_arch = "wasm32"))]

use std::collections::HashMap;

use dioxus::prelude::*;
use ui_layout::Rect;

use super::measurement::MountedRectCallback;

pub fn use_element_rect_impl() -> (MountedRectCallback, ReadSignal<Option<Rect>>) {
    let signal = use_signal(|| None);
    let callback = MountedRectCallback(EventHandler::new(move |_evt: MountedEvent| {}));
    (callback, ReadSignal::from(signal))
}

pub fn use_element_computed_style_impl(
    _properties: &'static [&'static str],
) -> (
    MountedRectCallback,
    ReadSignal<HashMap<&'static str, String>>,
) {
    let signal = use_signal(HashMap::new);
    let callback = MountedRectCallback(EventHandler::new(move |_evt: MountedEvent| {}));
    (callback, ReadSignal::from(signal))
}
