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
    let effective_clock = if reduced {
        TimelineClock::Manual {
            elapsed_ms: timeline.duration_ms,
        }
    } else {
        clock
    };

    // Seed the signal with the first sample so SSR and initial render emit the
    // correct inline styles before any effect runs.
    let initial_sample = timeline.sample(effective_clock);
    let mut sample = use_signal(|| initial_sample.clone());

    let runtime = use_hook(|| TimelineRuntime {
        handle: Rc::new(RefCell::new(None)),
        elapsed_ms: Rc::new(RefCell::new(0.0)),
    });

    match effective_clock {
        TimelineClock::Playback { elapsed_ms: start } => {
            // For Playback, drive elapsed_ms via a frame loop. Spawn the loop
            // once on first render; re-renders reuse the existing handle.
            if runtime.handle.borrow().is_none() {
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
                    sample_signal
                        .set(timeline_clone.sample(TimelineClock::Playback { elapsed_ms: now }));
                    if now >= total {
                        return ControlFlow::Stop;
                    }
                    ControlFlow::Continue
                });
                *runtime.handle.borrow_mut() = Some(handle);
            }
        }
        _ => {
            // For Manual / Frame / Scroll clocks, sample synchronously based
            // on the current prop value. Dioxus hooks do not track non-signal
            // arguments, so the only reliable way to reflect a changed clock
            // is to push the new sample through the signal on every render
            // (peek/set is idempotent when the value is unchanged).
            //
            // Also cancel any in-flight frame loop from a prior Playback clock
            // so we do not keep firing samples behind a switched mode.
            if runtime.handle.borrow().is_some() {
                *runtime.handle.borrow_mut() = None;
            }
            if *sample.peek() != initial_sample {
                sample.set(initial_sample);
            }
        }
    }

    ReadSignal::from(sample)
}
