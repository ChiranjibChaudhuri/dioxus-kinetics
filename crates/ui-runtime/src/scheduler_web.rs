//! `web_sys::Window::request_animation_frame` frame scheduler. wasm-only.

#![cfg(target_arch = "wasm32")]

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::scheduler::ControlFlow;

pub struct FrameHandle {
    cancelled: Rc<RefCell<bool>>,
}

impl Drop for FrameHandle {
    fn drop(&mut self) {
        *self.cancelled.borrow_mut() = true;
    }
}

pub fn spawn_frame_loop<F>(callback: F) -> FrameHandle
where
    F: FnMut(f64) -> ControlFlow + 'static,
{
    let cancelled = Rc::new(RefCell::new(false));
    let handle = FrameHandle {
        cancelled: cancelled.clone(),
    };

    let window = match web_sys::window() {
        Some(w) => w,
        None => return handle,
    };

    let callback = Rc::new(RefCell::new(callback));
    let last_timestamp = Rc::new(RefCell::new(None::<f64>));

    let raf_closure: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> =
        Rc::new(RefCell::new(None));
    let raf_closure_outer = raf_closure.clone();

    let window_clone = window.clone();
    let cancelled_clone = cancelled.clone();

    let request_next = move |timestamp: f64| {
        if *cancelled_clone.borrow() {
            *raf_closure.borrow_mut() = None;
            return;
        }
        let dt_ms = match last_timestamp.borrow_mut().replace(timestamp) {
            Some(prev) => timestamp - prev,
            None => 0.0,
        };
        let mut cb = callback.borrow_mut();
        if matches!(cb(dt_ms), ControlFlow::Stop) {
            *raf_closure.borrow_mut() = None;
            return;
        }
        drop(cb);
        if let Some(closure) = raf_closure.borrow().as_ref() {
            let _ = window_clone
                .request_animation_frame(closure.as_ref().unchecked_ref());
        }
    };

    let closure = Closure::wrap(Box::new(request_next) as Box<dyn FnMut(f64)>);
    let _ = window
        .request_animation_frame(closure.as_ref().unchecked_ref());
    *raf_closure_outer.borrow_mut() = Some(closure);

    handle
}
