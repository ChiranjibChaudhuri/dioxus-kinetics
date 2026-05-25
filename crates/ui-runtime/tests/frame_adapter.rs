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

#[test]
fn broadcast_seek_tolerates_adapter_mutating_registry() {
    use std::cell::RefCell;

    // Adapter that, on its first seek, registers a sibling adapter via a
    // captured registry clone. The snapshot-before-iterate strategy in
    // broadcast_seek must keep this from panicking on a re-entrant borrow_mut.
    struct ReentrantAdapter {
        id: &'static str,
        registry: FrameAdapterRegistry,
        seen: Rc<Cell<u32>>,
        spawned: RefCell<Option<ui_runtime::frame_adapter::FrameAdapterHandle>>,
    }

    impl FrameAdapter for ReentrantAdapter {
        fn id(&self) -> &str {
            self.id
        }
        fn duration_ms(&self) -> f32 {
            1.0
        }
        fn seek(&self, _elapsed_ms: f32, _reduced: bool) {
            self.seen.set(self.seen.get() + 1);
            if self.spawned.borrow().is_none() {
                let sibling = CountingAdapter::new("sibling", 1.0);
                let handle = self.registry.register(sibling);
                *self.spawned.borrow_mut() = Some(handle);
            }
        }
    }

    let registry = FrameAdapterRegistry::default();
    let seen = Rc::new(Cell::new(0u32));
    let parent = ReentrantAdapter {
        id: "parent",
        registry: registry.clone(),
        seen: seen.clone(),
        spawned: RefCell::new(None),
    };
    let _h = registry.register(parent);

    // First broadcast: parent runs, mutates registry mid-iteration.
    registry.broadcast_seek(10.0, false);
    assert_eq!(seen.get(), 1);
    assert_eq!(registry.len(), 2, "sibling should be registered by parent's seek");

    // Second broadcast: both parent and sibling get the call.
    registry.broadcast_seek(20.0, false);
    assert_eq!(seen.get(), 2);
}
