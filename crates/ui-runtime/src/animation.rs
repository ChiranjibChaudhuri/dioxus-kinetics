//! Animation value hook with WAAPI compositor offload.

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use ui_motion::keyframes_for_transition;
use ui_motion::{apply_ease, Transition};

use crate::reduced_motion::use_reduced_motion;
use crate::scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};
#[cfg(target_arch = "wasm32")]
use crate::waapi::{
    is_supported, keyframes_to_js, options_object, AnimatedProperty, WaapiAnimation,
};

/// Convenience wrapper: animate from `target` → `target` (no motion) — kept
/// for API parity with the pre-WAAPI runtime.
pub fn use_animation_value(target: f32, transition: Transition) -> ReadSignal<f32> {
    use_animation_value_from(target, target, transition)
}

/// Animates a signal from `initial` toward `target`. Under reduced motion
/// the signal jumps directly to the target. Otherwise it ticks per-frame
/// (RAF path) so SSR/test consumers see the in-flight value. The WAAPI
/// compositor offload happens at consumer sites via `use_animation_target`
/// which attaches a mounted-element handle.
pub fn use_animation_value_from(
    initial: f32,
    target: f32,
    transition: Transition,
) -> ReadSignal<f32> {
    let reduced = use_reduced_motion();
    let mut value = use_signal(|| initial);

    let context = use_hook(|| AnimationContext {
        last_target: Rc::new(RefCell::new(initial)),
        handle: Rc::new(RefCell::new(None::<FrameHandle>)),
        velocity: Rc::new(RefCell::new(0.0)),
        elapsed_ms: Rc::new(RefCell::new(0.0)),
        start_value: Rc::new(RefCell::new(initial)),
    });

    use_effect(move || {
        let current_target = target;
        {
            let mut last = context.last_target.borrow_mut();
            if *last == current_target && context.handle.borrow().is_some() {
                return;
            }
            *last = current_target;
        }

        if reduced {
            *context.handle.borrow_mut() = None;
            value.set(current_target);
            return;
        }

        #[cfg(target_arch = "wasm32")]
        if crate::waapi::is_supported() {
            // WAAPI owns in-flight interpolation; the Rust-side signal
            // jumps to the target value synchronously. The visible motion
            // is driven by the consumer's `UseAnimationTarget::play_on(element)`
            // call from its `onmounted` handler. We do NOT spawn a RAF
            // loop here; doing so would race against the compositor's
            // keyframe interpolation and produce the pointer-events
            // flakiness Spec 2's audit surfaced on Dialog/Toast/Tooltip.
            *context.handle.borrow_mut() = None;
            value.set(current_target);
            return;
        }

        let start = value();
        *context.elapsed_ms.borrow_mut() = 0.0;
        *context.start_value.borrow_mut() = start;

        let velocity_cell = context.velocity.clone();
        let elapsed_cell = context.elapsed_ms.clone();
        let start_cell = context.start_value.clone();
        let mut signal = value;
        let handle = spawn_frame_loop(move |dt_ms| {
            let current = signal();
            match transition {
                Transition::Tween { duration_ms, ease } => {
                    if duration_ms == 0 {
                        signal.set(current_target);
                        return ControlFlow::Stop;
                    }
                    let mut elapsed = elapsed_cell.borrow_mut();
                    *elapsed += dt_ms as f32;
                    let progress = (*elapsed / duration_ms as f32).clamp(0.0, 1.0);
                    let eased = apply_ease(progress, ease);
                    let start_value = *start_cell.borrow();
                    let next = start_value + (current_target - start_value) * eased;
                    if progress >= 1.0 {
                        signal.set(current_target);
                        return ControlFlow::Stop;
                    }
                    signal.set(next);
                }
                Transition::Spring(spring) => {
                    let v = *velocity_cell.borrow();
                    let step = spring.step(current, current_target, v, (dt_ms as f32) / 1000.0);
                    *velocity_cell.borrow_mut() = step.velocity;
                    signal.set(step.value);
                    if (step.value - current_target).abs() < 0.001 && step.velocity.abs() < 0.01 {
                        signal.set(current_target);
                        *velocity_cell.borrow_mut() = 0.0;
                        return ControlFlow::Stop;
                    }
                }
            }
            ControlFlow::Continue
        });
        *context.handle.borrow_mut() = Some(handle);
    });

    ReadSignal::from(value)
}

#[derive(Clone)]
struct AnimationContext {
    last_target: Rc<RefCell<f32>>,
    handle: Rc<RefCell<Option<FrameHandle>>>,
    velocity: Rc<RefCell<f32>>,
    elapsed_ms: Rc<RefCell<f32>>,
    start_value: Rc<RefCell<f32>>,
}

// ----- New WAAPI-compositor-offload hook -----

#[cfg(target_arch = "wasm32")]
pub fn use_animation_target(
    property: AnimatedProperty,
    initial: f32,
    target: f32,
    transition: Transition,
) -> (UseAnimationTarget, ReadSignal<f32>) {
    let reduced = use_reduced_motion();
    let value = use_animation_value_from(initial, target, transition);

    let handle_cell: Rc<RefCell<Option<WaapiAnimation>>> = use_hook(|| Rc::new(RefCell::new(None)));
    let last_target: Rc<RefCell<f32>> = use_hook(|| Rc::new(RefCell::new(initial)));

    let attach = UseAnimationTarget {
        handle: handle_cell,
        last_target,
        target,
        transition,
        reduced,
        property,
        delay_ms: 0.0,
    };

    (attach, value)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn use_animation_target(
    _property: crate::waapi::AnimatedProperty,
    _initial: f32,
    target: f32,
    transition: Transition,
) -> (UseAnimationTarget, ReadSignal<f32>) {
    let v = use_animation_value(target, transition);
    (UseAnimationTarget, v)
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct UseAnimationTarget {
    handle: Rc<RefCell<Option<WaapiAnimation>>>,
    last_target: Rc<RefCell<f32>>,
    target: f32,
    transition: Transition,
    reduced: bool,
    property: AnimatedProperty,
    delay_ms: f32,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct UseAnimationTarget;

#[cfg(target_arch = "wasm32")]
impl UseAnimationTarget {
    /// Call from a Dioxus `onmounted` handler with the underlying element.
    /// Plays (or replaces) a WAAPI animation on it. `current_value` is the
    /// starting point for the keyframe array (typically the Rust-side signal's
    /// current value, NOT necessarily `initial` — a re-animation from a
    /// partial value should start from where it actually is).
    pub fn play_on(&self, element: &web_sys::Element, current_value: f32) {
        if self.reduced || !is_supported() {
            return;
        }
        if (*self.last_target.borrow() - self.target).abs() < 1e-6 && self.handle.borrow().is_some()
        {
            return;
        }
        *self.last_target.borrow_mut() = self.target;
        let keyframes = keyframes_for_transition(current_value, self.target, self.transition);
        let js_keyframes = keyframes_to_js(self.property, &keyframes);
        let js_options = options_object(keyframes.duration_ms, self.delay_ms);
        // keyframes_to_js returns JsValue directly per T6 — no .into() needed.
        if let Some(animation) = WaapiAnimation::play(element, &js_keyframes, &js_options) {
            *self.handle.borrow_mut() = Some(animation);
        }
    }

    pub fn cancel(&self) {
        if let Some(handle) = self.handle.borrow_mut().take() {
            handle.cancel();
        }
    }

    /// Sets the WAAPI `delay` option (ms) for the next `play_on` call.
    /// Used by stagger consumers (e.g. `Sequence` cues with non-zero
    /// `start_ms`). Negative values are clamped to 0.
    pub fn with_delay(mut self, delay_ms: f32) -> Self {
        self.delay_ms = delay_ms.max(0.0);
        self
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl UseAnimationTarget {
    pub fn with_delay(self, _delay_ms: f32) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_motion::Ease;

    fn tween_at(start: f32, target: f32, elapsed_ms: f32, duration_ms: f32, ease: Ease) -> f32 {
        let progress = (elapsed_ms / duration_ms).clamp(0.0, 1.0);
        let eased = apply_ease(progress, ease);
        start + (target - start) * eased
    }

    #[test]
    fn cumulative_tween_reaches_target_with_standard_ease() {
        let value = tween_at(0.0, 100.0, 220.0, 220.0, Ease::Standard);
        assert!((value - 100.0).abs() < 0.001);
    }

    #[test]
    fn cumulative_tween_midpoint_is_eased_not_linear() {
        let value = tween_at(0.0, 100.0, 110.0, 220.0, Ease::Standard);
        assert!((value - 50.0).abs() < 0.001);
    }

    #[test]
    fn cumulative_tween_quarter_progress_is_below_linear() {
        let value = tween_at(0.0, 100.0, 55.0, 220.0, Ease::Standard);
        assert!(
            value < 25.0,
            "expected eased value below linear; got {value}"
        );
        assert!(
            value > 10.0,
            "expected eased value above zero region; got {value}"
        );
    }
}
