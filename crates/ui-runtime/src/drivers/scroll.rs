#![cfg(target_arch = "wasm32")]

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, IntersectionObserver, IntersectionObserverInit};

use crate::scene_driver::ScrollObserverConfig;

pub struct ScrollDriverHandle {
    _observer: Option<IntersectionObserver>,
    _intersection_closure: Option<Closure<dyn FnMut(js_sys::Array, IntersectionObserver)>>,
    _scroll_closure: Option<Closure<dyn FnMut(web_sys::Event)>>,
}

impl Drop for ScrollDriverHandle {
    fn drop(&mut self) {
        if let Some(observer) = self._observer.take() {
            observer.disconnect();
        }
        if let Some(closure) = self._scroll_closure.take() {
            if let Some(window) = web_sys::window() {
                let _ = window.remove_event_listener_with_callback(
                    "scroll",
                    closure.as_ref().unchecked_ref(),
                );
            }
            drop(closure);
        }
        if let Some(c) = self._intersection_closure.take() {
            drop(c);
        }
    }
}

pub fn install_scroll_driver(
    config: &ScrollObserverConfig,
    on_progress: impl FnMut(f32) + 'static,
) -> ScrollDriverHandle {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return empty_handle(),
    };
    let document = match window.document() {
        Some(d) => d,
        None => return empty_handle(),
    };
    let trigger: Element = match document
        .query_selector(&config.trigger_selector)
        .ok()
        .flatten()
    {
        Some(el) => el,
        None => return empty_handle(),
    };

    let start_offset = config.start_offset_px;
    let end_offset = config.end_offset_px;
    let on_progress = Rc::new(RefCell::new(on_progress));

    let trigger_for_scroll = trigger.clone();
    let on_progress_scroll = on_progress.clone();
    let window_for_scroll = window.clone();
    let scroll_closure = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
        let progress = compute_progress(
            &window_for_scroll,
            &trigger_for_scroll,
            start_offset,
            end_offset,
        );
        (on_progress_scroll.borrow_mut())(progress);
    }) as Box<dyn FnMut(_)>);

    let _ = window.add_event_listener_with_callback(
        "scroll",
        scroll_closure.as_ref().unchecked_ref(),
    );

    // IntersectionObserver fires once when the trigger enters/exits the
    // viewport — used to seed progress at mount and to coalesce events
    // when the viewport scrolls past the trigger entirely.
    let on_progress_io = on_progress.clone();
    let window_for_io = window.clone();
    let trigger_for_io = trigger.clone();
    let intersection_closure = Closure::wrap(Box::new(
        move |_entries: js_sys::Array, _observer: IntersectionObserver| {
            let progress = compute_progress(
                &window_for_io,
                &trigger_for_io,
                start_offset,
                end_offset,
            );
            (on_progress_io.borrow_mut())(progress);
        },
    ) as Box<dyn FnMut(_, _)>);

    let init = IntersectionObserverInit::new();
    let observer = match IntersectionObserver::new_with_options(
        intersection_closure.as_ref().unchecked_ref(),
        &init,
    ) {
        Ok(o) => o,
        Err(_) => return empty_handle(),
    };
    observer.observe(&trigger);

    // Seed initial progress before any event fires.
    let initial = compute_progress(&window, &trigger, start_offset, end_offset);
    (on_progress.borrow_mut())(initial);

    ScrollDriverHandle {
        _observer: Some(observer),
        _intersection_closure: Some(intersection_closure),
        _scroll_closure: Some(scroll_closure),
    }
}

fn compute_progress(
    window: &web_sys::Window,
    trigger: &Element,
    start_offset: Option<f32>,
    end_offset: Option<f32>,
) -> f32 {
    let rect = trigger.get_bounding_client_rect();
    let trigger_top = rect.top() as f32;
    let trigger_height = rect.height() as f32;
    let vp_height = window.inner_height().ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as f32;

    let start = start_offset.unwrap_or(vp_height);
    let end = end_offset.unwrap_or(0.0);
    let total_distance = (start - end + trigger_height).max(1.0);
    let traversed = start - trigger_top;
    (traversed / total_distance).clamp(0.0, 1.0)
}

fn empty_handle() -> ScrollDriverHandle {
    ScrollDriverHandle {
        _observer: None,
        _intersection_closure: None,
        _scroll_closure: None,
    }
}
