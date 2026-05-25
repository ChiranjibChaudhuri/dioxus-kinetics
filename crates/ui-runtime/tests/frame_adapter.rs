use std::cell::Cell;
use std::rc::Rc;

use ui_runtime::frame_adapter::{FrameAdapter, FrameAdapterRegistry};

struct CountingAdapter {
    id: &'static str,
    duration_ms: f32,
    calls: Rc<Cell<u32>>,
    last_ms: Rc<Cell<f32>>,
    last_reduced: Rc<Cell<bool>>,
}

impl CountingAdapter {
    fn new(id: &'static str, duration_ms: f32) -> Self {
        Self {
            id,
            duration_ms,
            calls: Rc::new(Cell::new(0)),
            last_ms: Rc::new(Cell::new(-1.0)),
            last_reduced: Rc::new(Cell::new(false)),
        }
    }
}

impl FrameAdapter for CountingAdapter {
    fn id(&self) -> &str {
        self.id
    }
    fn duration_ms(&self) -> f32 {
        self.duration_ms
    }
    fn seek(&self, elapsed_ms: f32, reduced: bool) {
        self.calls.set(self.calls.get() + 1);
        self.last_ms.set(elapsed_ms);
        self.last_reduced.set(reduced);
    }
}

#[test]
fn registry_starts_empty() {
    let registry = FrameAdapterRegistry::default();
    assert_eq!(registry.len(), 0);
}

#[test]
fn register_inserts_and_drop_handle_removes() {
    let registry = FrameAdapterRegistry::default();
    let adapter = CountingAdapter::new("a", 1000.0);
    let handle = registry.register(adapter);
    assert_eq!(registry.len(), 1);
    drop(handle);
    assert_eq!(registry.len(), 0);
}

#[test]
fn register_same_id_replaces_first() {
    let registry = FrameAdapterRegistry::default();
    let h1 = registry.register(CountingAdapter::new("a", 1.0));
    let h2 = registry.register(CountingAdapter::new("a", 2.0));
    assert_eq!(registry.len(), 1, "second register should overwrite by id");
    drop(h1); // dropping the *first* handle must not remove the *second* entry
    assert_eq!(registry.len(), 1);
    drop(h2);
    assert_eq!(registry.len(), 0);
}

#[test]
fn broadcast_seek_fans_out_to_every_adapter() {
    let registry = FrameAdapterRegistry::default();
    let a = CountingAdapter::new("a", 1.0);
    let b = CountingAdapter::new("b", 1.0);
    let a_calls = a.calls.clone();
    let a_ms = a.last_ms.clone();
    let b_calls = b.calls.clone();
    let b_reduced = b.last_reduced.clone();
    let _ha = registry.register(a);
    let _hb = registry.register(b);

    registry.broadcast_seek(123.5, true);
    assert_eq!(a_calls.get(), 1);
    assert_eq!(b_calls.get(), 1);
    assert!((a_ms.get() - 123.5).abs() < f32::EPSILON);
    assert!(b_reduced.get());
}
