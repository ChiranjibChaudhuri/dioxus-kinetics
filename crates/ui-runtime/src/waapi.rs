//! Web Animations API binding for the motion runtime.
//!
//! `WaapiAnimation` wraps `web_sys::Animation` with cancel-on-drop
//! semantics so a re-render that targets the same element can replace
//! its predecessor cleanly. `play_keyframes` is the lowest-level
//! constructor; higher-level hooks build on it.

#![cfg(target_arch = "wasm32")]

use std::cell::RefCell;
use std::rc::Rc;

use ui_motion::{Keyframe, Keyframes};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Animation, Element};

/// Which CSS property the keyframe `value` maps to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimatedProperty {
    Opacity,
    TranslateX,
    TranslateY,
    Scale,
    /// Degrees; mapped to `rotate(<v>deg)`.
    Rotate,
    /// Custom property `--ui-presence-t`; raw number.
    PresenceT,
}

/// Construct a JS keyframe array from `(property, Keyframes)`. Each entry
/// is a `{ <css-prop>: "<value>", offset: <0..1> }` object.
pub fn keyframes_to_js(prop: AnimatedProperty, keyframes: &Keyframes) -> JsValue {
    let array = js_sys::Array::new_with_length(keyframes.frames.len() as u32);
    for (i, frame) in keyframes.frames.iter().enumerate() {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("offset"),
            &JsValue::from_f64(frame.offset as f64),
        )
        .ok();
        let (key, val) = property_kv(prop, frame);
        js_sys::Reflect::set(&obj, &JsValue::from_str(&key), &JsValue::from_str(&val)).ok();
        array.set(i as u32, obj.into());
    }
    array.into()
}

fn property_kv(prop: AnimatedProperty, frame: &Keyframe) -> (String, String) {
    match prop {
        AnimatedProperty::Opacity => ("opacity".into(), format!("{}", frame.value)),
        AnimatedProperty::TranslateX => {
            ("transform".into(), format!("translateX({}px)", frame.value))
        }
        AnimatedProperty::TranslateY => {
            ("transform".into(), format!("translateY({}px)", frame.value))
        }
        AnimatedProperty::Scale => ("transform".into(), format!("scale({})", frame.value)),
        AnimatedProperty::Rotate => ("transform".into(), format!("rotate({}deg)", frame.value)),
        AnimatedProperty::PresenceT => ("--ui-presence-t".into(), format!("{}", frame.value)),
    }
}

/// Construct the `options` object: `{ duration, easing: "linear", fill: "forwards" }`.
/// If `delay_ms > 0`, a `delay` key is also set.
pub fn options_object(duration_ms: f32, delay_ms: f32) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("duration"),
        &JsValue::from_f64(duration_ms as f64),
    )
    .ok();
    if delay_ms > 0.0 {
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("delay"),
            &JsValue::from_f64(delay_ms as f64),
        )
        .ok();
    }
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("easing"),
        &JsValue::from_str("linear"),
    )
    .ok();
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("fill"),
        &JsValue::from_str("forwards"),
    )
    .ok();
    obj.into()
}

/// Active WAAPI animation handle. Cancels its underlying `Animation`
/// when dropped.
pub struct WaapiAnimation {
    inner: Animation,
    cancelled: Rc<RefCell<bool>>,
}

impl WaapiAnimation {
    pub fn play(element: &Element, keyframes_js: &JsValue, options_js: &JsValue) -> Option<Self> {
        let animate_fn = js_sys::Reflect::get(element, &JsValue::from_str("animate")).ok()?;
        let func: &js_sys::Function = animate_fn.dyn_ref::<js_sys::Function>()?;
        let args = js_sys::Array::new_with_length(2);
        args.set(0, keyframes_js.clone());
        args.set(1, options_js.clone());
        let result = func.apply(element.as_ref(), &args).ok()?;
        let animation: Animation = result.dyn_into().ok()?;
        Some(Self {
            inner: animation,
            cancelled: Rc::new(RefCell::new(false)),
        })
    }

    pub fn pause(&self) {
        if !*self.cancelled.borrow() {
            let _ = self.inner.pause();
        }
    }

    pub fn cancel(&self) {
        if !*self.cancelled.borrow() {
            self.inner.cancel();
            *self.cancelled.borrow_mut() = true;
        }
    }

    pub fn set_current_time(&self, ms: f32) {
        if !*self.cancelled.borrow() {
            self.inner.set_current_time(Some(ms as f64));
        }
    }

    pub fn on_finish<F: FnMut() + 'static>(&self, mut callback: F) {
        let closure =
            Closure::wrap(Box::new(move |_evt: JsValue| callback()) as Box<dyn FnMut(JsValue)>);
        self.inner
            .set_onfinish(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }
}

impl Drop for WaapiAnimation {
    fn drop(&mut self) {
        self.cancel();
    }
}

/// Feature detection: returns true iff `Element.prototype.animate` is a
/// function. Cached after first call.
pub fn is_supported() -> bool {
    thread_local! {
        static SUPPORTED: std::cell::OnceCell<bool> = const { std::cell::OnceCell::new() };
    }
    SUPPORTED.with(|cell| {
        *cell.get_or_init(|| {
            let Some(window) = web_sys::window() else {
                return false;
            };
            let Some(document) = window.document() else {
                return false;
            };
            let Some(body) = document.body() else {
                return false;
            };
            let elt: &Element = body.as_ref();
            js_sys::Reflect::get(elt, &JsValue::from_str("animate"))
                .ok()
                .and_then(|v| v.dyn_into::<js_sys::Function>().ok())
                .is_some()
        })
    })
}
