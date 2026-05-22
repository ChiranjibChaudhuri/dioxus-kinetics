#![cfg(not(target_arch = "wasm32"))]

//! Non-wasm measurement uses Dioxus' cross-platform `MountedData::get_client_rect`
//! so Desktop and Mobile WebView targets get real bounding rectangles. Computed
//! style queries fall back to a no-op map because the WebView's CSSOM is not
//! exposed through MountedData; callers that need computed style values should
//! use `dioxus::document::eval` directly on those targets.

use std::collections::HashMap;

use dioxus::prelude::*;
use ui_layout::Rect;

use super::measurement::MountedRectCallback;

pub fn use_element_rect_impl() -> (MountedRectCallback, ReadSignal<Option<Rect>>) {
    let mut signal = use_signal(|| None);

    let callback = MountedRectCallback(EventHandler::new(move |evt: MountedEvent| {
        let mounted = evt.data();
        spawn(async move {
            if let Ok(rect) = mounted.get_client_rect().await {
                signal.set(Some(Rect {
                    x: rect.origin.x as f32,
                    y: rect.origin.y as f32,
                    width: rect.size.width as f32,
                    height: rect.size.height as f32,
                }));
            }
        });
    }));

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
