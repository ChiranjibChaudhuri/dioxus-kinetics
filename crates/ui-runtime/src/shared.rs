//! Shared element registry. Tracks element snapshots by id so SharedElement
//! components can coordinate cross-tree transitions.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use ui_layout::Rect;
use ui_motion::{Ease, Transition};

/// Snapshots older than this are treated as cold and do not seed a FLIP
/// animation. Matches the design spec for SharedElement cross-tree matching.
pub const SHARED_SNAPSHOT_STALENESS_MS: f64 = 500.0;

/// Monotonic-ish millisecond clock. On web this is `performance.now()`; on
/// native it is the wall clock since the unix epoch. Callers should treat
/// values as opaque and only compare deltas.
#[cfg(target_arch = "wasm32")]
pub fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn now_ms() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElementSnapshot {
    pub rect: Rect,
    pub computed: HashMap<&'static str, String>,
    /// `now_ms()` value at the moment the snapshot was recorded. Consumers
    /// compare against `now_ms()` to detect stale snapshots before seeding a
    /// FLIP animation; see `SHARED_SNAPSHOT_STALENESS_MS`.
    pub timestamp_ms: f64,
}

/// Stores element snapshots by id. Wrapped in `Rc` so it can be placed in a
/// Dioxus `Signal` (which requires `Clone + 'static`).
#[derive(Default)]
pub struct SharedElementRegistry {
    snapshots: RefCell<HashMap<String, ElementSnapshot>>,
}

impl SharedElementRegistry {
    pub fn snapshot(&self, id: &str) -> Option<ElementSnapshot> {
        self.snapshots.borrow().get(id).cloned()
    }

    pub fn record(&self, id: String, snapshot: ElementSnapshot) {
        self.snapshots.borrow_mut().insert(id, snapshot);
    }

    pub fn forget(&self, id: &str) {
        self.snapshots.borrow_mut().remove(id);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SharedTransition {
    pub layout: Transition,
    pub fade: Transition,
    pub computed: Transition,
}

impl Default for SharedTransition {
    fn default() -> Self {
        Self {
            layout: Transition::Tween {
                duration_ms: 280,
                ease: Ease::Standard,
            },
            fade: Transition::Tween {
                duration_ms: 200,
                ease: Ease::Standard,
            },
            computed: Transition::Tween {
                duration_ms: 280,
                ease: Ease::Standard,
            },
        }
    }
}

/// Returns (or creates) the shared-element registry as an `Rc<SharedElementRegistry>`
/// stored in a `Signal`. Using `Rc` because `SharedElementRegistry` contains a
/// `RefCell` and is therefore not `Clone`.
pub fn use_shared_element_registry() -> Signal<Rc<SharedElementRegistry>> {
    try_consume_context::<Signal<Rc<SharedElementRegistry>>>().unwrap_or_else(|| {
        use_context_provider(|| Signal::new(Rc::new(SharedElementRegistry::default())))
    })
}
