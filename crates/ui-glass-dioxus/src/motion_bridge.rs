//! Reads pointer/scroll/reduced-motion. Task 6 fills in event listeners; this
//! file currently defines just the data shape.

use ui_glass_engine::motion::MotionInputs;

#[derive(Clone, Copy, Debug, Default)]
pub struct MotionState {
    pub pointer_px: [f32; 2],
    pub scroll_velocity_px: [f32; 2],
    pub reduced_motion: bool,
}

impl MotionState {
    /// Build a `MotionInputs` snapshot. `start_time_ms` is the loop start in
    /// performance.now() units; the resulting `time_seconds` is relative.
    pub fn to_motion_inputs(self, start_time_ms: f64) -> MotionInputs {
        let now = web_sys_performance_now();
        let elapsed_s = ((now - start_time_ms) / 1000.0) as f32;
        MotionInputs::new()
            .with_pointer(self.pointer_px)
            .with_scroll_velocity(self.scroll_velocity_px)
            .with_time(elapsed_s)
            .with_reduced_motion(self.reduced_motion)
    }
}

#[cfg(target_arch = "wasm32")]
fn web_sys_performance_now() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
fn web_sys_performance_now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

/// Attach pointer + scroll + matchMedia listeners that update the given
/// motion_state signal. Returns a `MotionListenersGuard` whose drop removes
/// the listeners. Web-only.
#[cfg(target_arch = "wasm32")]
pub fn attach_listeners(
    canvas: &web_sys::HtmlCanvasElement,
    mut motion_state: dioxus::prelude::Signal<MotionState>,
) -> MotionListenersGuard {
    use dioxus::prelude::WritableExt;
    use gloo_events::EventListener;
    use wasm_bindgen::JsCast;

    let canvas_for_pointer = canvas.clone();
    let pointer = EventListener::new(canvas, "pointermove", move |evt| {
        if let Some(e) = evt.dyn_ref::<web_sys::PointerEvent>() {
            let rect = canvas_for_pointer.get_bounding_client_rect();
            let x = (e.client_x() as f64 - rect.left()) as f32;
            let y = (e.client_y() as f64 - rect.top()) as f32;
            motion_state.with_mut(|s| s.pointer_px = [x, y]);
        }
    });

    let window = web_sys::window().expect("window");
    let scroll_state = std::rc::Rc::new(std::cell::RefCell::new(0.0f64));
    let scroll_state_clone = scroll_state.clone();
    let scroll = EventListener::new(&window, "scroll", move |_| {
        if let Some(w) = web_sys::window() {
            let y = w.scroll_y().unwrap_or(0.0);
            let mut prev = scroll_state_clone.borrow_mut();
            let dy = (y - *prev) as f32;
            *prev = y;
            motion_state.with_mut(|s| s.scroll_velocity_px = [0.0, dy]);
        }
    });

    // Reduced-motion preference
    let media = window
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten();
    if let Some(mql) = &media {
        let initial = mql.matches();
        motion_state.with_mut(|s| s.reduced_motion = initial);
    }
    let media_listener = media.as_ref().map(|mql| {
        let mql_target: &web_sys::EventTarget = mql.as_ref();
        EventListener::new(mql_target, "change", move |evt| {
            if let Some(e) = evt.dyn_ref::<web_sys::MediaQueryListEvent>() {
                let on = e.matches();
                motion_state.with_mut(|s| s.reduced_motion = on);
            }
        })
    });

    MotionListenersGuard {
        _pointer: pointer,
        _scroll: scroll,
        _media: media_listener,
    }
}

#[cfg(target_arch = "wasm32")]
pub struct MotionListenersGuard {
    _pointer: gloo_events::EventListener,
    _scroll: gloo_events::EventListener,
    _media: Option<gloo_events::EventListener>,
}
