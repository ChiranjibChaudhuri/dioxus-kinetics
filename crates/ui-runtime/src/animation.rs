//! Animation value hook with per-frame ticking.

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use ui_motion::{apply_ease, Ease, Transition};

use crate::reduced_motion::use_reduced_motion;
use crate::scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};

#[derive(Clone)]
struct AnimationContext {
    handle: Rc<RefCell<Option<FrameHandle>>>,
    velocity: Rc<RefCell<f32>>,
    last_target: Rc<RefCell<f32>>,
}

pub fn use_animation_value(target: f32, transition: Transition) -> ReadSignal<f32> {
    let reduced = use_reduced_motion();
    let mut value = use_signal(|| target);

    let ctx = use_hook(|| AnimationContext {
        handle: Rc::new(RefCell::new(None)),
        velocity: Rc::new(RefCell::new(0.0)),
        last_target: Rc::new(RefCell::new(target)),
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

        let velocity_cell = ctx.velocity.clone();
        let mut signal = value;

        let handle = spawn_frame_loop(move |dt_ms| {
            let current = signal();
            match transition {
                Transition::Tween { duration_ms, ease } => {
                    let progress_step = if duration_ms == 0 {
                        1.0
                    } else {
                        (dt_ms as f32) / (duration_ms as f32)
                    };
                    let next = step_tween(current, current_target, progress_step, ease);
                    if (next - current_target).abs() < 0.001 {
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

fn step_tween(current: f32, target: f32, progress_step: f32, ease: Ease) -> f32 {
    let p = progress_step.clamp(0.0, 1.0);
    let eased = apply_ease(p, ease);
    current + (target - current) * eased
}
