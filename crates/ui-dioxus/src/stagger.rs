//! Per-render stagger cursor for `TimelineScope` (Task 7).
//!
//! `TimelineScope` provides a `StaggerCursor` via Dioxus context. Each
//! kinetic leaf that renders inside the scope grabs the next index and
//! computes its local elapsed_ms = max(0, parent_elapsed - index *
//! step_ms). The cursor advances as each leaf renders.
//!
//! SSR is single-threaded, so a `Cell<u32>` counter inside an `Rc` is
//! sufficient — no atomics, no mutexes.

use std::cell::Cell;
use std::rc::Rc;

/// SSR-friendly cursor for assigning stagger indices to leaves as they
/// render inside a `TimelineScope`. Single-threaded by design.
#[derive(Clone)]
pub struct StaggerCursor {
    pub cursor: Rc<Cell<u32>>,
    pub step_ms: f32,
}

impl StaggerCursor {
    pub fn new(step_ms: f32) -> Self {
        Self {
            cursor: Rc::new(Cell::new(0)),
            step_ms,
        }
    }

    /// Returns the current index, then advances. Each call returns a
    /// fresh integer 0, 1, 2, …
    pub fn next_index(&self) -> u32 {
        let i = self.cursor.get();
        self.cursor.set(i + 1);
        i
    }

    /// Returns the offset corresponding to the current (un-advanced)
    /// cursor index.
    pub fn current_offset_ms(&self) -> f32 {
        let i = self.cursor.get();
        i as f32 * self.step_ms
    }

    /// Resets the cursor back to index 0. Must be called at the start
    /// of every TimelineScope render — `use_context_provider` only
    /// runs its initializer once, so without this the counter retains
    /// the previous render's terminal value and subsequent renders
    /// emit indices N, N+1, … instead of 0, 1, …
    pub fn reset(&self) {
        self.cursor.set(0);
    }
}
