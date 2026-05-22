#![cfg(target_arch = "wasm32")]

use std::collections::HashMap;

use dioxus::prelude::*;
use ui_layout::Rect;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

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

fn mounted_event_rect(evt: &MountedEvent) -> Option<Rect> {
    let element = evt.downcast::<Element>()?;
    let dom_rect = element.get_bounding_client_rect();
    Some(Rect {
        x: dom_rect.left() as f32,
        y: dom_rect.top() as f32,
        width: dom_rect.width() as f32,
        height: dom_rect.height() as f32,
    })
}

fn mounted_event_computed_style(
    evt: &MountedEvent,
    properties: &'static [&'static str],
) -> Option<HashMap<&'static str, String>> {
    let element = evt.downcast::<Element>()?;
    // `get_computed_style` lives on `Window`; the element itself does not expose
    // it. Fall back to inline style for non-HTML elements (SVG, MathML) where
    // window().get_computed_style() may still work but returns CSSOM defaults.
    let window = web_sys::window()?;
    let declaration = window
        .get_computed_style(element)
        .ok()
        .flatten()
        .or_else(|| element.dyn_ref::<HtmlElement>().map(|html| html.style()))?;

    let mut map = HashMap::with_capacity(properties.len());
    for property in properties {
        if let Ok(value) = declaration.get_property_value(property) {
            if !value.is_empty() {
                map.insert(*property, value);
            }
        }
    }
    Some(map)
}
