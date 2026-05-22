//! Timeline sampling hook.

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use ui_timeline::{Timeline, TimelineClock, TimelineSample};

use crate::reduced_motion::use_reduced_motion;
use crate::scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};

#[derive(Clone)]
struct TimelineRuntime {
    handle: Rc<RefCell<Option<FrameHandle>>>,
    elapsed_ms: Rc<RefCell<f32>>,
}

pub fn use_timeline_sample(timeline: Timeline, clock: TimelineClock) -> ReadSignal<TimelineSample> {
    let reduced = use_reduced_motion();
    let initial = if reduced {
        timeline.sample(TimelineClock::Manual {
            elapsed_ms: timeline.duration_ms,
        })
    } else {
        timeline.sample(clock)
    };
    let mut sample = use_signal(|| initial);

    let runtime = use_hook(|| TimelineRuntime {
        handle: Rc::new(RefCell::new(None)),
        elapsed_ms: Rc::new(RefCell::new(0.0)),
    });

    use_effect(move || {
        if reduced {
            *runtime.handle.borrow_mut() = None;
            sample.set(timeline.sample(TimelineClock::Manual {
                elapsed_ms: timeline.duration_ms,
            }));
            return;
        }

        match clock {
            TimelineClock::Playback { elapsed_ms: start } => {
                *runtime.elapsed_ms.borrow_mut() = start;
                let timeline_clone = timeline.clone();
                let elapsed_cell = runtime.elapsed_ms.clone();
                let mut sample_signal = sample;
                let total = timeline.duration_ms;
                let handle = spawn_frame_loop(move |dt_ms| {
                    let now = {
                        let mut elapsed = elapsed_cell.borrow_mut();
                        *elapsed += dt_ms as f32;
                        *elapsed
                    };
                    sample_signal.set(timeline_clone.sample(TimelineClock::Playback {
                        elapsed_ms: now,
                    }));
                    if now >= total {
                        return ControlFlow::Stop;
                    }
                    ControlFlow::Continue
                });
                *runtime.handle.borrow_mut() = Some(handle);
            }
            other => {
                *runtime.handle.borrow_mut() = None;
                sample.set(timeline.sample(other));
            }
        }
    });

    ReadSignal::from(sample)
}
