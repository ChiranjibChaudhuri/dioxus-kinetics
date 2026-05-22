//! Animation value hook with per-frame ticking.

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use ui_motion::{apply_ease, Transition};

use crate::reduced_motion::use_reduced_motion;
use crate::scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};

#[derive(Clone)]
struct AnimationContext {
    handle: Rc<RefCell<Option<FrameHandle>>>,
    velocity: Rc<RefCell<f32>>,
    last_target: Rc<RefCell<f32>>,
    elapsed_ms: Rc<RefCell<f32>>,
    start_value: Rc<RefCell<f32>>,
}

pub fn use_animation_value(target: f32, transition: Transition) -> ReadSignal<f32> {
    let reduced = use_reduced_motion();
    let mut value = use_signal(|| target);

    let ctx = use_hook(|| AnimationContext {
        handle: Rc::new(RefCell::new(None)),
        velocity: Rc::new(RefCell::new(0.0)),
        last_target: Rc::new(RefCell::new(target)),
        elapsed_ms: Rc::new(RefCell::new(0.0)),
        start_value: Rc::new(RefCell::new(target)),
    });

    use_effect(move || {
        let current_target = target;
        {
            let mut last = ctx.last_target.borrow_mut();
            if *last == current_target && ctx.handle.borrow().is_some() {
                return;
            }
            *last = current_target;
        }

        if reduced {
            *ctx.handle.borrow_mut() = None;
            value.set(current_target);
            return;
        }

        // Reset tween bookkeeping. Tween easing applies to cumulative progress
        // from start_value to current_target, so we capture the value at the
        // moment the target changed.
        let start = value();
        *ctx.elapsed_ms.borrow_mut() = 0.0;
        *ctx.start_value.borrow_mut() = start;

        let velocity_cell = ctx.velocity.clone();
        let elapsed_cell = ctx.elapsed_ms.clone();
        let start_cell = ctx.start_value.clone();
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

        *ctx.handle.borrow_mut() = Some(handle);
    });

    ReadSignal::from(value)
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
        // Standard ease at p=0.5 is 0.5 (smoothstep symmetric at midpoint).
        assert!((value - 50.0).abs() < 0.001);
    }

    #[test]
    fn cumulative_tween_quarter_progress_is_below_linear() {
        let value = tween_at(0.0, 100.0, 55.0, 220.0, Ease::Standard);
        // Standard ease at p=0.25 = 0.25^2 * (3 - 2*0.25) = 0.0625 * 2.5 = 0.15625
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
