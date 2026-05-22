#![cfg(target_arch = "wasm32")]

use std::collections::HashMap;

use dioxus::prelude::*;
use ui_layout::Rect;

use super::measurement::MountedRectCallback;

pub fn use_element_rect_impl() -> (MountedRectCallback, ReadSignal<Option<Rect>>) {
    let mut signal = use_signal(|| None);

    let callback = MountedRectCallback(EventHandler::new(move |evt: MountedEvent| {
        if let Some(rect) = mounted_event_rect(&evt) {
            signal.set(Some(rect));
        }
    }));

    (callback, ReadSignal::from(signal))
}

pub fn use_element_computed_style_impl(
    properties: &'static [&'static str],
) -> (
    MountedRectCallback,
    ReadSignal<HashMap<&'static str, String>>,
) {
    let mut signal = use_signal(HashMap::new);

    let callback = MountedRectCallback(EventHandler::new(move |evt: MountedEvent| {
        if let Some(map) = mounted_event_computed_style(&evt, properties) {
            signal.set(map);
        }
    }));

    (callback, ReadSignal::from(signal))
}

fn mounted_event_rect(_evt: &MountedEvent) -> Option<Rect> {
    // Dioxus 0.7's MountedEvent does not directly expose the underlying
    // web_sys::Element through a stable public API. The mounted data is
    // accessible via `evt.data`, but the concrete type is renderer-specific
    // (`dioxus-web`'s internal `WebEventData`). Until that API is stabilised
    // or a helper crate is added, we return None and accept that wasm builds
    // compile but do not actually measure.
    //
    // Known limitation: real rect measurement on wasm is not yet implemented.
    None
}

fn mounted_event_computed_style(
    _evt: &MountedEvent,
    _properties: &'static [&'static str],
) -> Option<HashMap<&'static str, String>> {
    // Same limitation as mounted_event_rect above.
    None
}
