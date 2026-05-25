//! Frame adapter contract: paused, seekable bridge from the SceneClock
//! to any animation runtime (native cues, WAAPI, CSS keyframes, future
//! external libraries). Adapters register through `FrameAdapterRegistry`;
//! the `Scene` Dioxus component owns one registry per instance and
//! broadcasts `seek` on every clock tick.

use std::cell::RefCell;
use std::rc::Rc;

/// Animation-runtime bridge. Adapters MUST be deterministic in
/// `elapsed_ms` and infallible (clamp / no-op internally on bad input).
pub trait FrameAdapter {
    fn id(&self) -> &str;
    fn duration_ms(&self) -> f32;
    fn seek(&self, elapsed_ms: f32, reduced: bool);
}

type AdapterBox = Rc<dyn FrameAdapter>;

/// Returned by `FrameAdapterRegistry::register`. Drops the entry when
/// the handle is dropped. Identity is by adapter id, so re-registering
/// the same id replaces the prior entry and only that entry's handle
/// removes it.
pub struct FrameAdapterHandle {
    id: String,
    epoch: u64,
    inner: Rc<RefCell<RegistryInner>>,
}

impl Drop for FrameAdapterHandle {
    fn drop(&mut self) {
        self.inner
            .borrow_mut()
            .remove_if_epoch_matches(&self.id, self.epoch);
    }
}

struct Entry {
    epoch: u64,
    adapter: AdapterBox,
}

#[derive(Default)]
struct RegistryInner {
    entries: Vec<(String, Entry)>,
    next_epoch: u64,
}

impl RegistryInner {
    fn upsert(&mut self, id: String, adapter: AdapterBox) -> u64 {
        self.next_epoch += 1;
        let epoch = self.next_epoch;
        if let Some(slot) = self.entries.iter_mut().find(|(k, _)| *k == id) {
            slot.1 = Entry { epoch, adapter };
        } else {
            self.entries.push((id, Entry { epoch, adapter }));
        }
        epoch
    }

    fn remove_if_epoch_matches(&mut self, id: &str, epoch: u64) {
        self.entries
            .retain(|(k, e)| !(k == id && e.epoch == epoch));
    }
}

/// Clonable handle to the per-Scene adapter registry. Cheap to clone
/// (`Rc` under the hood); pass through Dioxus context.
#[derive(Clone, Default)]
pub struct FrameAdapterRegistry {
    inner: Rc<RefCell<RegistryInner>>,
}

impl FrameAdapterRegistry {
    pub fn register<A: FrameAdapter + 'static>(&self, adapter: A) -> FrameAdapterHandle {
        let id = adapter.id().to_string();
        let epoch = self
            .inner
            .borrow_mut()
            .upsert(id.clone(), Rc::new(adapter));
        FrameAdapterHandle {
            id,
            epoch,
            inner: self.inner.clone(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.borrow().entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Fan `seek` out to every registered adapter in insertion order.
    /// Cloning the entry list before iterating avoids re-entrancy if an
    /// adapter's `seek` triggers a registry mutation.
    pub fn broadcast_seek(&self, elapsed_ms: f32, reduced: bool) {
        let snapshot: Vec<AdapterBox> = self
            .inner
            .borrow()
            .entries
            .iter()
            .map(|(_, e)| e.adapter.clone())
            .collect();
        for adapter in snapshot {
            adapter.seek(elapsed_ms, reduced);
        }
    }
}
