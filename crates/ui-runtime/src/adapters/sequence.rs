//! Cross-platform adapter that wraps a `ui_timeline::Timeline`. On
//! `seek`, samples the timeline at `elapsed_ms` (or the reduced-motion
//! variant when `reduced=true`) and stores the resolved states in an
//! interior slot. A Dioxus component (or other consumer) reads
//! `snapshot()` to apply inline styles.

use std::cell::RefCell;
use std::rc::Rc;

use ui_timeline::{ResolvedMotionState, Timeline, TimelineClock};

use crate::frame_adapter::FrameAdapter;

pub struct SequenceAdapter {
    id: String,
    duration_ms: f32,
    timeline: Timeline,
    reduced_timeline: Timeline,
    slot: Rc<RefCell<Vec<ResolvedMotionState>>>,
}

impl SequenceAdapter {
    pub fn new(timeline: Timeline) -> Self {
        let id = timeline.id.0.clone();
        let duration_ms = timeline.duration_ms;
        let reduced_timeline = timeline.reduced_motion();
        Self {
            id,
            duration_ms,
            timeline,
            reduced_timeline,
            slot: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn snapshot(&self) -> Vec<ResolvedMotionState> {
        self.slot.borrow().clone()
    }
}

impl FrameAdapter for SequenceAdapter {
    fn id(&self) -> &str {
        &self.id
    }
    fn duration_ms(&self) -> f32 {
        self.duration_ms
    }
    fn seek(&self, elapsed_ms: f32, reduced: bool) {
        let source = if reduced {
            &self.reduced_timeline
        } else {
            &self.timeline
        };
        let sample = source.sample(TimelineClock::Manual { elapsed_ms });
        *self.slot.borrow_mut() = sample.states;
    }
}
