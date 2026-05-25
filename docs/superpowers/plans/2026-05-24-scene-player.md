# Scene Player Implementation Plan (SP-1)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Land the keystone of the "HyperFrames for Dioxus" track — a paused-seekable `Scene` Dioxus component plus a `FrameAdapter` contract that lets `Sequence` / WAAPI / CSS-keyframe animations share one clock, proven with one 10-second cinematic showcase in a new gallery category.

**Architecture:** A `SceneClock` (signal-backed transport state in `ui-runtime`) advances `elapsed_ms` either via autoplay (reusing `spawn_frame_loop`) or via user scrubbing. A `Scene` Dioxus component provides the clock + `SceneContext` via Dioxus context; children either subscribe to `elapsed_ms` directly (`Clip`, `KineticBox` via `TimelineClock::Manual`) or register a `FrameAdapter` whose `seek(ms, reduced)` is fanned out from a single `use_effect` in `Scene`. Backwards-compat: `FrameStage`/`FrameClip`/`FrameLayer` stay as `#[deprecated]` shims.

**Tech Stack:** Rust workspace (Cargo), Dioxus 0.6 with `Signal<T>`, `dioxus-ssr` for SSR tests, Tokio (`task::spawn_local`) for native scheduler, `web-sys` + `wasm-bindgen` for WAAPI bridge, Playwright (chromium + webkit projects) for E2E.

**Spec:** `docs/superpowers/specs/2026-05-24-scene-player-design.md`

---

## File Structure

```
crates/ui-composition/src/lib.rs            # +FrameClip::active_at_ms
crates/ui-composition/tests/composition.rs  # +active_at_ms tests

crates/ui-runtime/src/
  frame_adapter.rs            # NEW — FrameAdapter trait + registry
  scene_clock.rs              # NEW — SceneClock + SceneState (signals + autoplay)
  adapters/
    mod.rs                    # NEW — pub use of three adapters
    sequence.rs               # NEW — SequenceAdapter (cross-platform)
    waapi.rs                  # NEW — WaapiAdapter (web-only)
    waapi_stub.rs             # NEW — WaapiAdapter stub (non-web)
    css_keyframes.rs          # NEW — CssKeyframesAdapter (web-only)
    css_keyframes_stub.rs     # NEW — CssKeyframesAdapter stub (non-web)
  lib.rs                      # +mod + re-exports
crates/ui-runtime/tests/
  frame_adapter.rs            # NEW — registry tests
  scene_clock.rs              # NEW — clock unit tests
  sequence_adapter.rs         # NEW — SequenceAdapter tests

crates/ui-dioxus/src/
  scene_player.rs             # NEW — Scene + Clip + transport UI
  composition.rs              # mark FrameStage/FrameClip/FrameLayer #[deprecated]
  lib.rs                      # +re-exports
crates/ui-dioxus/tests/
  scene_player_ssr.rs         # NEW — Scene + Clip + transport SSR tests

crates/ui-styles/src/
  scene_player.css            # NEW — transport + reduced tag styling
  lib.rs                      # +const SCENE_PLAYER_CSS, included by library_css()

crates/kinetics/src/lib.rs    # +prelude + public_api_names entries

examples/component-gallery/src/
  docs.rs                     # +ComponentCategory::Scene + doc entry
  previews/mod.rs             # +pub mod scene;
  previews/scene.rs           # NEW — preview function
  previews/scenes/mod.rs      # NEW
  previews/scenes/product_intro.rs   # NEW — 10s showcase
  previews/scenes/flip_card_deck.rs  # NEW — scene fragment
  previews/scenes/metric_counter.rs  # NEW — scene fragment
  previews/scenes/cta_pulse.rs       # NEW — scene fragment

examples/component-gallery/e2e/tests/
  scene-player.spec.ts        # NEW — Playwright spec
```

---

## Conventions

All Rust tests live in `tests/<name>.rs` (integration-style), matching existing crate convention. All SSR tests use `dioxus_ssr::render_element(rsx!{ ... })`. All commits use Conventional Commits with the `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>` trailer (HEREDOC via `git commit -m`).

Run a single test: `cargo test -p <crate> --test <file> -- <test_name>`. Run all tests in a crate: `cargo test -p <crate>`. Run workspace: `cargo test`.

---

### Task 1: `FrameClip::active_at_ms` in `ui-composition`

The `Clip` Dioxus component (Task 10) needs a ms-based activity check; existing `active_at(frame: u32)` only works in frame space. Add a parallel helper.

**Files:**
- Modify: `crates/ui-composition/src/lib.rs` (append helper to `impl FrameClip`)
- Test: `crates/ui-composition/tests/composition.rs` (append four new tests)

- [ ] **Step 1: Write the failing tests**

Append to `crates/ui-composition/tests/composition.rs`:

```rust
#[test]
fn frame_clip_active_at_ms_none_fill() {
    let clip = FrameClip::new(1_000, 500, ClipFill::None);
    assert!(!clip.active_at_ms(999.9));
    assert!(clip.active_at_ms(1_000.0));
    assert!(clip.active_at_ms(1_499.9));
    assert!(!clip.active_at_ms(1_500.0));
}

#[test]
fn frame_clip_active_at_ms_hold_start() {
    let clip = FrameClip::new(1_000, 500, ClipFill::HoldStart);
    assert!(clip.active_at_ms(0.0));
    assert!(clip.active_at_ms(999.9));
    assert!(clip.active_at_ms(1_200.0));
    assert!(!clip.active_at_ms(1_600.0));
}

#[test]
fn frame_clip_active_at_ms_hold_end() {
    let clip = FrameClip::new(1_000, 500, ClipFill::HoldEnd);
    assert!(!clip.active_at_ms(999.9));
    assert!(clip.active_at_ms(1_000.0));
    assert!(clip.active_at_ms(1_499.9));
    assert!(clip.active_at_ms(10_000.0));
}

#[test]
fn frame_clip_active_at_ms_hold_both() {
    let clip = FrameClip::new(1_000, 500, ClipFill::HoldBoth);
    assert!(clip.active_at_ms(0.0));
    assert!(clip.active_at_ms(1_200.0));
    assert!(clip.active_at_ms(10_000.0));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-composition --test composition frame_clip_active_at_ms`
Expected: 4 failures — `no method named active_at_ms`.

- [ ] **Step 3: Implement `FrameClip::active_at_ms`**

In `crates/ui-composition/src/lib.rs`, append inside `impl FrameClip`:

```rust
    /// Millisecond-space equivalent of [`Self::active_at`]. `ms` is the
    /// elapsed time since the parent scene started. Compares against
    /// `self.start` and `self.duration` interpreted as milliseconds, so
    /// callers must construct the clip with ms values (not frames).
    pub fn active_at_ms(&self, ms: f32) -> bool {
        let ms = if ms.is_finite() { ms } else { 0.0 };
        let start = self.start as f32;
        let end = start + self.duration as f32;
        let within = ms >= start && ms < end;
        match self.fill {
            ClipFill::None => within,
            ClipFill::HoldStart => self.duration > 0 && (ms < start || within),
            ClipFill::HoldEnd => self.duration > 0 && ms >= start,
            ClipFill::HoldBoth => true,
        }
    }
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-composition --test composition frame_clip_active_at_ms`
Expected: 4 PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-composition/src/lib.rs crates/ui-composition/tests/composition.rs
git commit -m "$(cat <<'EOF'
feat(ui-composition): add FrameClip::active_at_ms helper

Mirrors active_at(frame) but in ms-space so Scene's Clip Dioxus
component can drive visibility off the parent SceneClock without
needing to convert ms -> frame on every read.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 2: `FrameAdapter` trait + `FrameAdapterRegistry`

**Files:**
- Create: `crates/ui-runtime/src/frame_adapter.rs`
- Modify: `crates/ui-runtime/src/lib.rs` (add `pub mod frame_adapter;`)
- Test: `crates/ui-runtime/tests/frame_adapter.rs`

- [ ] **Step 1: Write the failing tests**

Create `crates/ui-runtime/tests/frame_adapter.rs`:

```rust
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
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-runtime --test frame_adapter`
Expected: compile error — `frame_adapter` module not found.

- [ ] **Step 3: Implement `FrameAdapter` + registry**

Create `crates/ui-runtime/src/frame_adapter.rs`:

```rust
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
        self.inner.borrow_mut().remove_if_epoch_matches(&self.id, self.epoch);
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
```

Modify `crates/ui-runtime/src/lib.rs`. After the existing `pub mod state;` line, add:

```rust
pub mod frame_adapter;
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-runtime --test frame_adapter`
Expected: 4 PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-runtime/src/frame_adapter.rs crates/ui-runtime/src/lib.rs crates/ui-runtime/tests/frame_adapter.rs
git commit -m "$(cat <<'EOF'
feat(ui-runtime): FrameAdapter trait + per-scene registry

Adapters are infallible, deterministic in elapsed_ms, and idempotent
by id. Registry returns a drop-guard handle that uses an epoch counter
so a stale handle for an overwritten id no-ops on drop.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 3: `SceneClock` + `SceneState` (signals, seek, settle — no autoplay yet)

**Files:**
- Create: `crates/ui-runtime/src/scene_clock.rs`
- Modify: `crates/ui-runtime/src/lib.rs` (add `pub mod scene_clock;`)
- Test: `crates/ui-runtime/tests/scene_clock.rs`

- [ ] **Step 1: Write the failing tests**

Create `crates/ui-runtime/tests/scene_clock.rs`:

```rust
use ui_runtime::scene_clock::{SceneClock, SceneState};

#[test]
fn new_clock_starts_paused_at_zero() {
    let clock = SceneClock::new(1_000.0, 60, false);
    assert_eq!(clock.peek_elapsed_ms(), 0.0);
    assert_eq!(clock.peek_state(), SceneState::Paused);
    assert!(!clock.peek_reduced());
}

#[test]
fn reduced_constructor_settles_immediately() {
    let clock = SceneClock::new(1_000.0, 60, true);
    assert!(clock.peek_reduced());
    assert_eq!(clock.peek_state(), SceneState::Settled);
    assert!((clock.peek_elapsed_ms() - 1_000.0).abs() < f32::EPSILON);
}

#[test]
fn seek_ms_clamps_low() {
    let clock = SceneClock::new(1_000.0, 60, false);
    clock.seek_ms(-50.0);
    assert_eq!(clock.peek_elapsed_ms(), 0.0);
    assert_eq!(clock.peek_state(), SceneState::Paused);
}

#[test]
fn seek_ms_clamps_high_and_settles() {
    let clock = SceneClock::new(1_000.0, 60, false);
    clock.seek_ms(2_000.0);
    assert!((clock.peek_elapsed_ms() - 1_000.0).abs() < f32::EPSILON);
    assert_eq!(clock.peek_state(), SceneState::Settled);
}

#[test]
fn seek_progress_maps_to_duration() {
    let clock = SceneClock::new(2_000.0, 60, false);
    clock.seek_progress(0.5);
    assert!((clock.peek_elapsed_ms() - 1_000.0).abs() < 1e-3);
}

#[test]
fn settle_jumps_to_duration_ms() {
    let clock = SceneClock::new(750.0, 60, false);
    clock.settle();
    assert!((clock.peek_elapsed_ms() - 750.0).abs() < f32::EPSILON);
    assert_eq!(clock.peek_state(), SceneState::Settled);
}

#[test]
fn frame_clock_derives_from_elapsed_and_fps() {
    let clock = SceneClock::new(1_000.0, 30, false);
    clock.seek_ms(500.0);
    let fc = clock.frame_clock();
    assert_eq!(fc.frame, 15);
    assert_eq!(fc.fps, 30);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-runtime --test scene_clock`
Expected: compile error — `scene_clock` module not found.

- [ ] **Step 3: Implement `SceneClock` (no autoplay yet)**

Create `crates/ui-runtime/src/scene_clock.rs`:

```rust
//! Signal-backed transport state for a Scene. Owns no rendering — its
//! purpose is to hold `elapsed_ms`, `state`, and `reduced` as Dioxus
//! Signals so any subscriber re-renders on change.
//!
//! Autoplay (rAF loop) is wired in a follow-up task; this module
//! intentionally ships seek/settle first so unit tests of clamping and
//! state transitions are independent of the scheduler.

use dioxus::prelude::*;
use ui_composition::FrameClock;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SceneState {
    Paused,
    Playing,
    Settled,
}

/// Cheap-to-clone (`Copy`-able) clock handle: every field is a Dioxus
/// `Signal<T>`, which is `Copy`.
#[derive(Clone, Copy)]
pub struct SceneClock {
    pub duration_ms: Signal<f32>,
    pub elapsed_ms: Signal<f32>,
    pub state: Signal<SceneState>,
    pub fps: Signal<u32>,
    pub reduced: Signal<bool>,
}

impl SceneClock {
    pub fn new(duration_ms: f32, fps: u32, reduced: bool) -> Self {
        let duration_ms = finite_non_negative(duration_ms);
        let (elapsed, state) = if reduced {
            (duration_ms, SceneState::Settled)
        } else {
            (0.0, SceneState::Paused)
        };
        Self {
            duration_ms: Signal::new(duration_ms),
            elapsed_ms: Signal::new(elapsed),
            state: Signal::new(state),
            fps: Signal::new(fps.max(1)),
            reduced: Signal::new(reduced),
        }
    }

    pub fn seek_ms(&self, ms: f32) {
        let ms = if ms.is_finite() { ms } else { 0.0 };
        let duration = *self.duration_ms.peek();
        let clamped = ms.clamp(0.0, duration);
        let mut s = self.elapsed_ms;
        s.set(clamped);
        if clamped >= duration {
            let mut s = self.state;
            s.set(SceneState::Settled);
        } else if *self.state.peek() == SceneState::Settled {
            // Scrubbing back from settled returns to paused.
            let mut s = self.state;
            s.set(SceneState::Paused);
        }
    }

    pub fn seek_progress(&self, fraction: f32) {
        let fraction = if fraction.is_finite() {
            fraction.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let duration = *self.duration_ms.peek();
        self.seek_ms(duration * fraction);
    }

    pub fn settle(&self) {
        let duration = *self.duration_ms.peek();
        let mut s = self.elapsed_ms;
        s.set(duration);
        let mut s = self.state;
        s.set(SceneState::Settled);
    }

    pub fn frame_clock(&self) -> FrameClock {
        let fps = *self.fps.peek();
        debug_assert!(fps > 0, "SceneClock::new clamps fps to >= 1");
        let elapsed = *self.elapsed_ms.peek();
        let frame = (elapsed / 1000.0 * fps as f32).round() as u32;
        FrameClock { frame, fps }
    }

    pub fn peek_elapsed_ms(&self) -> f32 {
        *self.elapsed_ms.peek()
    }
    pub fn peek_state(&self) -> SceneState {
        *self.state.peek()
    }
    pub fn peek_reduced(&self) -> bool {
        *self.reduced.peek()
    }
}

fn finite_non_negative(value: f32) -> f32 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        0.0
    }
}
```

Modify `crates/ui-runtime/src/lib.rs`. Below the `frame_adapter` line, add:

```rust
pub mod scene_clock;
```

Also add `ui-composition` as a dependency to `ui-runtime/Cargo.toml` if missing. Check first:

```bash
grep -n "ui-composition" crates/ui-runtime/Cargo.toml || echo "MISSING"
```

If MISSING, append to `[dependencies]` block in `crates/ui-runtime/Cargo.toml`:

```toml
ui-composition = { path = "../ui-composition" }
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-runtime --test scene_clock`
Expected: 7 PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-runtime/src/scene_clock.rs crates/ui-runtime/src/lib.rs crates/ui-runtime/Cargo.toml crates/ui-runtime/tests/scene_clock.rs
git commit -m "$(cat <<'EOF'
feat(ui-runtime): SceneClock signals + seek/settle (no autoplay yet)

Constructs as Paused at 0ms (or Settled at duration_ms if reduced).
seek_ms/seek_progress clamp into [0, duration_ms] and transition to
Settled when they hit the cap. Scrubbing back from Settled returns
to Paused so the autoplay path (next task) can resume from there.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 4: `SceneClock` autoplay loop via `spawn_frame_loop`

**Files:**
- Modify: `crates/ui-runtime/src/scene_clock.rs` (add `play` / `pause` / `is_playing`)
- Test: `crates/ui-runtime/tests/scene_clock.rs` (append autoplay tests)

- [ ] **Step 1: Write the failing tests**

Append to `crates/ui-runtime/tests/scene_clock.rs`:

```rust
use std::time::Duration;
use ui_runtime::scene_clock::SceneClock as SC;

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn play_advances_elapsed_until_settled() {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let clock = SC::new(80.0, 60, false);
            clock.play();
            // Each native scheduler tick is ~16ms; advance enough virtual
            // time to cross duration_ms = 80ms.
            tokio::time::advance(Duration::from_millis(200)).await;
            tokio::task::yield_now().await;
            assert!(clock.peek_elapsed_ms() >= 80.0 - 1e-3);
            assert_eq!(clock.peek_state(), ui_runtime::scene_clock::SceneState::Settled);
        })
        .await;
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn pause_stops_advance() {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let clock = SC::new(10_000.0, 60, false);
            clock.play();
            tokio::time::advance(Duration::from_millis(40)).await;
            tokio::task::yield_now().await;
            let mid = clock.peek_elapsed_ms();
            clock.pause();
            tokio::time::advance(Duration::from_millis(200)).await;
            tokio::task::yield_now().await;
            assert!((clock.peek_elapsed_ms() - mid).abs() < 5.0,
                "pause should freeze elapsed_ms; was {mid} now {}", clock.peek_elapsed_ms());
        })
        .await;
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn reduced_clock_play_is_noop() {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let clock = SC::new(500.0, 60, true);
            clock.play();
            tokio::time::advance(Duration::from_millis(200)).await;
            tokio::task::yield_now().await;
            assert!((clock.peek_elapsed_ms() - 500.0).abs() < f32::EPSILON);
            assert_eq!(clock.peek_state(), ui_runtime::scene_clock::SceneState::Settled);
        })
        .await;
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-runtime --test scene_clock play pause reduced_clock`
Expected: compile error — `play` method not found.

- [ ] **Step 3: Implement `play` / `pause`**

In `crates/ui-runtime/src/scene_clock.rs`, add the imports at the top:

```rust
use std::cell::RefCell;
use std::rc::Rc;

use crate::scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};
```

Add a private handle slot to the struct (keep `Copy` derive — Signals are Copy, so a `Signal<Option<...>>` wrapper is Copy too):

```rust
#[derive(Clone, Copy)]
pub struct SceneClock {
    pub duration_ms: Signal<f32>,
    pub elapsed_ms: Signal<f32>,
    pub state: Signal<SceneState>,
    pub fps: Signal<u32>,
    pub reduced: Signal<bool>,
    handle_slot: Signal<HandleSlot>,
}
```

Add helper newtype below the struct (Signals require `'static`, so we wrap the non-`'static` `FrameHandle` in `Rc<RefCell<Option<FrameHandle>>>`):

```rust
#[derive(Clone, Default)]
pub(crate) struct HandleSlot(pub(crate) Rc<RefCell<Option<FrameHandle>>>);
```

Update `SceneClock::new` to initialise `handle_slot: Signal::new(HandleSlot::default())`.

Add methods inside `impl SceneClock`:

```rust
    pub fn is_playing(&self) -> bool {
        matches!(*self.state.peek(), SceneState::Playing)
    }

    pub fn pause(&self) {
        if matches!(*self.state.peek(), SceneState::Playing) {
            let mut s = self.state;
            s.set(SceneState::Paused);
        }
        // Drop any active frame loop.
        self.handle_slot.peek().0.borrow_mut().take();
    }

    pub fn play(&self) {
        if *self.reduced.peek() {
            // Reduced-motion: settle once, no loop.
            self.settle();
            return;
        }
        if matches!(*self.state.peek(), SceneState::Settled) {
            // Replay from start.
            let mut s = self.elapsed_ms;
            s.set(0.0);
        }
        let mut s = self.state;
        s.set(SceneState::Playing);

        let duration_signal = self.duration_ms;
        let mut elapsed_signal = self.elapsed_ms;
        let mut state_signal = self.state;
        let slot = self.handle_slot.peek().0.clone();

        let handle = spawn_frame_loop(move |dt_ms: f64| {
            let duration = *duration_signal.peek();
            let next = (*elapsed_signal.peek() + dt_ms as f32).min(duration);
            elapsed_signal.set(next);
            if next >= duration {
                state_signal.set(SceneState::Settled);
                ControlFlow::Stop
            } else {
                ControlFlow::Continue
            }
        });
        *slot.borrow_mut() = Some(handle);
    }
```

Also: `Signal<T>: Copy` requires `T: 'static`. `HandleSlot` is `'static` because it's an `Rc<RefCell<Option<FrameHandle>>>` of owned types. Verify by running cargo check before tests.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-runtime --test scene_clock`
Expected: 10 PASS (7 from Task 3 + 3 new).

- [ ] **Step 5: Commit**

```bash
git add crates/ui-runtime/src/scene_clock.rs crates/ui-runtime/tests/scene_clock.rs
git commit -m "$(cat <<'EOF'
feat(ui-runtime): SceneClock autoplay via spawn_frame_loop

play() spawns the platform frame loop and advances elapsed_ms by
dt_ms per tick until duration_ms is reached, then auto-settles.
pause() drops the FrameHandle (Tokio task aborts on drop in the
native scheduler). Reduced-motion clocks ignore play and stay
settled. Replays from Settled rewind to 0 before starting.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 5: `SequenceAdapter` (cross-platform)

Wraps a `ui_timeline::Timeline` so a `Sequence` / `TimelineScope` participates in the adapter contract. The adapter samples the timeline on every `seek` and stores the resolved states in a shared slot; the corresponding Dioxus component can read the slot to apply inline styles. For this task we only implement the adapter and its sampling test; the Dioxus wiring is covered in Task 12.

**Files:**
- Create: `crates/ui-runtime/src/adapters/mod.rs`
- Create: `crates/ui-runtime/src/adapters/sequence.rs`
- Modify: `crates/ui-runtime/src/lib.rs` (add `pub mod adapters;`)
- Test: `crates/ui-runtime/tests/sequence_adapter.rs`

- [ ] **Step 1: Write the failing test**

Create `crates/ui-runtime/tests/sequence_adapter.rs`:

```rust
use ui_motion::{Ease, Transition};
use ui_runtime::adapters::SequenceAdapter;
use ui_runtime::frame_adapter::FrameAdapter;
use ui_timeline::{MotionCue, MotionSegment, MotionTarget, Timeline, TimelineTrack};

fn linear_tween(ms: u32) -> Transition {
    Transition::Tween {
        duration_ms: ms,
        ease: Ease::Linear,
    }
}

fn opacity_timeline() -> Timeline {
    Timeline::new("intro", 200.0).with_track(TimelineTrack::new(
        MotionTarget::node("title"),
        vec![MotionSegment::new(
            0.0,
            200.0,
            MotionCue::Opacity {
                from: 0.0,
                to: 1.0,
                transition: linear_tween(200),
            },
        )],
    ))
}

#[test]
fn sequence_adapter_id_and_duration_track_timeline() {
    let adapter = SequenceAdapter::new(opacity_timeline());
    assert_eq!(adapter.id(), "intro");
    assert!((adapter.duration_ms() - 200.0).abs() < f32::EPSILON);
}

#[test]
fn seek_writes_resolved_state_into_slot() {
    let adapter = SequenceAdapter::new(opacity_timeline());
    adapter.seek(0.0, false);
    {
        let snap = adapter.snapshot();
        assert_eq!(snap.len(), 1);
        let opacity = snap[0].opacity.expect("opacity at t=0");
        assert!(opacity.abs() < 1e-3, "got {opacity}");
    }
    adapter.seek(100.0, false);
    {
        let snap = adapter.snapshot();
        let opacity = snap[0].opacity.expect("opacity at t=100");
        assert!((opacity - 0.5).abs() < 1e-2, "got {opacity}");
    }
    adapter.seek(200.0, false);
    {
        let snap = adapter.snapshot();
        let opacity = snap[0].opacity.expect("opacity at t=200");
        assert!((opacity - 1.0).abs() < 1e-3, "got {opacity}");
    }
}

#[test]
fn reduced_seek_uses_reduced_motion_timeline() {
    let adapter = SequenceAdapter::new(opacity_timeline());
    adapter.seek(0.0, true);
    let snap = adapter.snapshot();
    // Reduced motion collapses duration to 0, so any elapsed_ms emits the
    // final settled state (opacity = 1.0).
    assert!((snap[0].opacity.unwrap_or(0.0) - 1.0).abs() < 1e-3);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-runtime --test sequence_adapter`
Expected: compile error — `adapters` module not found.

- [ ] **Step 3: Implement `SequenceAdapter`**

Create `crates/ui-runtime/src/adapters/sequence.rs`:

```rust
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
```

Create `crates/ui-runtime/src/adapters/mod.rs`:

```rust
mod sequence;
pub use sequence::SequenceAdapter;

#[cfg(target_arch = "wasm32")]
mod waapi;
#[cfg(not(target_arch = "wasm32"))]
#[path = "waapi_stub.rs"]
mod waapi;
pub use waapi::WaapiAdapter;

#[cfg(target_arch = "wasm32")]
mod css_keyframes;
#[cfg(not(target_arch = "wasm32"))]
#[path = "css_keyframes_stub.rs"]
mod css_keyframes;
pub use css_keyframes::CssKeyframesAdapter;
```

Note: `waapi`, `waapi_stub`, `css_keyframes`, and `css_keyframes_stub` files are introduced in Tasks 6 and 7. For this task, comment out the `waapi` and `css_keyframes` blocks so the module compiles:

For this task only, `crates/ui-runtime/src/adapters/mod.rs`:

```rust
mod sequence;
pub use sequence::SequenceAdapter;
```

Tasks 6 and 7 will replace this file with the cfg-gated version above.

Modify `crates/ui-runtime/src/lib.rs`. Below the `scene_clock` line, add:

```rust
pub mod adapters;
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-runtime --test sequence_adapter`
Expected: 3 PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-runtime/src/adapters crates/ui-runtime/src/lib.rs crates/ui-runtime/tests/sequence_adapter.rs
git commit -m "$(cat <<'EOF'
feat(ui-runtime): SequenceAdapter wraps a Timeline as a FrameAdapter

On seek, samples either the regular or reduced-motion timeline at
elapsed_ms and stores the resolved states. snapshot() returns the
current sample so a Dioxus consumer can write inline styles.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 6: `WaapiAdapter` + native stub

The adapter wraps a paused `web_sys::Animation` and retimes it via `set_current_time` on each seek. On native (non-wasm) targets, the type still exists but no-ops, so cross-platform code can construct it unconditionally.

**Files:**
- Create: `crates/ui-runtime/src/adapters/waapi.rs`
- Create: `crates/ui-runtime/src/adapters/waapi_stub.rs`
- Modify: `crates/ui-runtime/src/adapters/mod.rs` (add cfg-gated module re-exports)

- [ ] **Step 1: Write the stub adapter (native)**

Create `crates/ui-runtime/src/adapters/waapi_stub.rs`:

```rust
//! Non-wasm stub for `WaapiAdapter`. Stores the metadata but the seek
//! is a no-op. Allows cross-platform code to construct the adapter
//! unconditionally without `cfg` noise.

#![cfg(not(target_arch = "wasm32"))]

use crate::frame_adapter::FrameAdapter;

pub struct WaapiAdapter {
    id: String,
    duration_ms: f32,
}

impl WaapiAdapter {
    /// Builds a stub adapter. On native targets `_animation_handle` is
    /// ignored; on web targets the constructor takes a
    /// `web_sys::Animation` instead — see the web module.
    pub fn new(id: impl Into<String>, duration_ms: f32) -> Self {
        Self {
            id: id.into(),
            duration_ms,
        }
    }
}

impl FrameAdapter for WaapiAdapter {
    fn id(&self) -> &str {
        &self.id
    }
    fn duration_ms(&self) -> f32 {
        self.duration_ms
    }
    fn seek(&self, _elapsed_ms: f32, _reduced: bool) {
        // No-op on non-web targets.
    }
}
```

- [ ] **Step 2: Write the web adapter**

Create `crates/ui-runtime/src/adapters/waapi.rs`:

```rust
//! Web Animations API bridge as a FrameAdapter. Wraps a paused
//! `web_sys::Animation`; each seek forces `play_state = "paused"` and
//! sets `current_time = elapsed_ms`. Reduced-motion freezes at
//! `duration_ms` after one seek.

#![cfg(target_arch = "wasm32")]

use std::cell::Cell;

use wasm_bindgen::JsValue;
use web_sys::{Animation, Element};

use crate::frame_adapter::FrameAdapter;

pub struct WaapiAdapter {
    id: String,
    duration_ms: f32,
    animation: Animation,
    target: Element,
    reduced_locked: Cell<bool>,
}

impl WaapiAdapter {
    /// Caller owns the `web_sys::Animation`. The adapter assumes the
    /// animation was already constructed paused (e.g. via
    /// `target.animate(keyframes, options)` followed by `animation.pause()`).
    pub fn new(
        id: impl Into<String>,
        duration_ms: f32,
        animation: Animation,
        target: Element,
    ) -> Self {
        Self {
            id: id.into(),
            duration_ms,
            animation,
            target,
            reduced_locked: Cell::new(false),
        }
    }
}

impl FrameAdapter for WaapiAdapter {
    fn id(&self) -> &str {
        &self.id
    }
    fn duration_ms(&self) -> f32 {
        self.duration_ms
    }
    fn seek(&self, elapsed_ms: f32, reduced: bool) {
        if !self.target.is_connected() {
            return;
        }
        if reduced {
            if self.reduced_locked.get() {
                return;
            }
            self.reduced_locked.set(true);
            let _ = self.animation.pause();
            self.animation
                .set_current_time(Some(self.duration_ms as f64));
            return;
        }
        self.reduced_locked.set(false);
        // Force paused before retiming; WebKit otherwise jumps a frame.
        let _ = self.animation.pause();
        self.animation
            .set_current_time(Some(elapsed_ms.clamp(0.0, self.duration_ms) as f64));
        let _ = JsValue::from_f64(elapsed_ms as f64); // silence dead_code on JsValue when reduced
    }
}
```

- [ ] **Step 3: Restore the cfg-gated `mod.rs`**

Overwrite `crates/ui-runtime/src/adapters/mod.rs`:

```rust
mod sequence;
pub use sequence::SequenceAdapter;

#[cfg(target_arch = "wasm32")]
mod waapi;
#[cfg(not(target_arch = "wasm32"))]
#[path = "waapi_stub.rs"]
mod waapi;
pub use waapi::WaapiAdapter;
```

- [ ] **Step 4: Verify the workspace compiles on native**

Run: `cargo check -p ui-runtime`
Expected: success. (No new unit tests in this task — WAAPI testing requires `wasm-bindgen-test`, deferred to SP-4's render harness.)

- [ ] **Step 5: Commit**

```bash
git add crates/ui-runtime/src/adapters
git commit -m "$(cat <<'EOF'
feat(ui-runtime): WaapiAdapter web binding + native stub

Web target wraps a paused web_sys::Animation and retimes via
set_current_time, forcing pause first to dodge WebKit jank. Native
stub stores metadata and no-ops on seek so cross-platform code
constructs uniformly.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 7: `CssKeyframesAdapter` + native stub

Same shape as Task 6: web target mutates `animation-play-state` + `animation-delay` on a target element; native target no-ops.

**Files:**
- Create: `crates/ui-runtime/src/adapters/css_keyframes.rs`
- Create: `crates/ui-runtime/src/adapters/css_keyframes_stub.rs`
- Modify: `crates/ui-runtime/src/adapters/mod.rs`

- [ ] **Step 1: Native stub**

Create `crates/ui-runtime/src/adapters/css_keyframes_stub.rs`:

```rust
#![cfg(not(target_arch = "wasm32"))]

use crate::frame_adapter::FrameAdapter;

pub struct CssKeyframesAdapter {
    id: String,
    duration_ms: f32,
}

impl CssKeyframesAdapter {
    pub fn new(id: impl Into<String>, duration_ms: f32) -> Self {
        Self {
            id: id.into(),
            duration_ms,
        }
    }
}

impl FrameAdapter for CssKeyframesAdapter {
    fn id(&self) -> &str {
        &self.id
    }
    fn duration_ms(&self) -> f32 {
        self.duration_ms
    }
    fn seek(&self, _elapsed_ms: f32, _reduced: bool) {}
}
```

- [ ] **Step 2: Web adapter**

Create `crates/ui-runtime/src/adapters/css_keyframes.rs`:

```rust
#![cfg(target_arch = "wasm32")]

use web_sys::HtmlElement;

use crate::frame_adapter::FrameAdapter;

pub struct CssKeyframesAdapter {
    id: String,
    duration_ms: f32,
    target: HtmlElement,
    keyframes_name: String,
}

impl CssKeyframesAdapter {
    pub fn new(
        id: impl Into<String>,
        duration_ms: f32,
        target: HtmlElement,
        keyframes_name: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            duration_ms,
            target,
            keyframes_name: keyframes_name.into(),
        }
    }

    fn write_style(&self, elapsed_ms: f32) {
        if !self.target.is_connected() {
            return;
        }
        let style = self.target.style();
        let _ = style.set_property("animation-name", &self.keyframes_name);
        let _ = style.set_property("animation-duration", &format!("{}ms", self.duration_ms));
        let _ = style.set_property("animation-fill-mode", "forwards");
        let _ = style.set_property("animation-play-state", "paused");
        let _ = style.set_property("animation-delay", &format!("-{}ms", elapsed_ms));
    }
}

impl FrameAdapter for CssKeyframesAdapter {
    fn id(&self) -> &str {
        &self.id
    }
    fn duration_ms(&self) -> f32 {
        self.duration_ms
    }
    fn seek(&self, elapsed_ms: f32, reduced: bool) {
        let value = if reduced {
            self.duration_ms
        } else {
            elapsed_ms.clamp(0.0, self.duration_ms)
        };
        self.write_style(value);
    }
}
```

- [ ] **Step 3: Update `adapters/mod.rs`**

Overwrite `crates/ui-runtime/src/adapters/mod.rs`:

```rust
mod sequence;
pub use sequence::SequenceAdapter;

#[cfg(target_arch = "wasm32")]
mod waapi;
#[cfg(not(target_arch = "wasm32"))]
#[path = "waapi_stub.rs"]
mod waapi;
pub use waapi::WaapiAdapter;

#[cfg(target_arch = "wasm32")]
mod css_keyframes;
#[cfg(not(target_arch = "wasm32"))]
#[path = "css_keyframes_stub.rs"]
mod css_keyframes;
pub use css_keyframes::CssKeyframesAdapter;
```

- [ ] **Step 4: Verify compile**

Run: `cargo check -p ui-runtime`
Expected: success.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-runtime/src/adapters
git commit -m "$(cat <<'EOF'
feat(ui-runtime): CssKeyframesAdapter via animation-delay + native stub

On web, the adapter writes animation-name/duration/fill-mode/play-state
and a negative animation-delay equal to -elapsed_ms so the keyframe
position equals elapsed_ms while the animation stays paused. Native
stub mirrors the WaapiAdapter pattern.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 8: Re-export the new runtime types from `ui-runtime::lib`

Make `FrameAdapter`, `FrameAdapterRegistry`, `FrameAdapterHandle`, `SceneClock`, `SceneState`, and the three adapters available without the module path.

**Files:**
- Modify: `crates/ui-runtime/src/lib.rs`

- [ ] **Step 1: Add re-exports**

In `crates/ui-runtime/src/lib.rs`, append below the existing `pub use` block:

```rust
pub use adapters::{CssKeyframesAdapter, SequenceAdapter, WaapiAdapter};
pub use frame_adapter::{FrameAdapter, FrameAdapterHandle, FrameAdapterRegistry};
pub use scene_clock::{SceneClock, SceneState};
```

- [ ] **Step 2: Verify**

Run: `cargo check -p ui-runtime && cargo test -p ui-runtime`
Expected: all existing tests still pass.

- [ ] **Step 3: Commit**

```bash
git add crates/ui-runtime/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(ui-runtime): re-export Scene/Adapter types at crate root

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 9: `Scene` Dioxus component + `SceneContext` provider

This task ships the bare Scene component (data attributes, context provider, autoplay-on-mount) but does not yet ship the transport UI or adapter fan-out — those come in Tasks 11 and 12. SSR tests assert the initial DOM and the reduced-motion DOM.

**Files:**
- Create: `crates/ui-dioxus/src/scene_player.rs`
- Modify: `crates/ui-dioxus/src/lib.rs` (`pub mod scene_player;` + re-exports)
- Test: `crates/ui-dioxus/tests/scene_player_ssr.rs`

- [ ] **Step 1: Write the failing SSR test**

Create `crates/ui-dioxus/tests/scene_player_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::{Scene, SceneState};
use ui_runtime::reduced_motion::ReducedMotionProvider;

#[test]
fn scene_renders_root_data_attributes() {
    let html = dioxus_ssr::render_element(rsx! {
        Scene {
            id: "intro",
            width: 1920,
            height: 1080,
            duration_ms: 5_000.0,
            fps: Some(60),
            autoplay: Some(false),
            controls: Some(false),
            p { "hello" }
        }
    });
    assert!(html.contains("data-composition-id=\"intro\""), "{html}");
    assert!(html.contains("data-width=\"1920\""), "{html}");
    assert!(html.contains("data-height=\"1080\""), "{html}");
    assert!(html.contains("data-fps=\"60\""), "{html}");
    assert!(html.contains("data-duration-ms=\"5000\""), "{html}");
    assert!(html.contains("data-state=\"paused\""), "{html}");
    assert!(html.contains("data-reduced=\"false\""), "{html}");
    assert!(html.contains("aspect-ratio: 1920 / 1080"), "{html}");
}

#[test]
fn scene_with_reduced_motion_renders_settled() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: true,
            Scene {
                id: "intro",
                width: 100,
                height: 100,
                duration_ms: 1_000.0,
                p { "hi" }
            }
        }
    });
    assert!(html.contains("data-state=\"settled\""), "{html}");
    assert!(html.contains("data-reduced=\"true\""), "{html}");
    assert!(html.contains("data-elapsed-ms=\"1000\""), "{html}");
}

#[test]
fn scene_state_enum_is_re_exported_via_kinetics_prelude() {
    // Sanity: SceneState surfaces correctly.
    let s: SceneState = SceneState::Settled;
    let _ = s;
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-dioxus --test scene_player_ssr`
Expected: compile error — `Scene` not found.

- [ ] **Step 3: Implement `Scene` component**

Create `crates/ui-dioxus/src/scene_player.rs`:

```rust
//! Scene Dioxus component: hosts a SceneClock, provides a
//! SceneContext, emits hyperframes-compatible data attributes. The
//! transport UI (scrubber, play/pause) and adapter fan-out arrive in
//! later tasks; this module ships the shell.

use std::rc::Rc;

use dioxus::prelude::*;
use ui_runtime::frame_adapter::FrameAdapterRegistry;
use ui_runtime::reduced_motion::use_reduced_motion;
use ui_runtime::scene_clock::{SceneClock, SceneState};

#[derive(Clone, Copy)]
pub struct SceneContext {
    pub clock: SceneClock,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub duration_ms: f32,
    pub adapters: Signal<FrameAdapterRegistry>,
    pub id_signal: Signal<Rc<str>>,
}

#[component]
pub fn Scene(
    id: String,
    width: u32,
    height: u32,
    duration_ms: f32,
    fps: Option<u32>,
    autoplay: Option<bool>,
    controls: Option<bool>,
    children: Element,
) -> Element {
    let _ = controls; // wired in Task 11
    let fps = fps.unwrap_or(60).max(1);
    let autoplay = autoplay.unwrap_or(true);
    let reduced = use_reduced_motion();

    let clock = use_hook(|| SceneClock::new(duration_ms, fps, reduced));
    let registry = use_hook(FrameAdapterRegistry::default);
    let id_rc: Rc<str> = Rc::from(id.as_str());
    let id_signal = use_hook(|| Signal::new(id_rc.clone()));
    let adapters_signal = use_hook(|| Signal::new(registry.clone()));

    use_context_provider(|| SceneContext {
        clock,
        width,
        height,
        fps,
        duration_ms,
        adapters: adapters_signal,
        id_signal,
    });

    use_effect(move || {
        if autoplay && !reduced {
            clock.play();
        }
    });

    let elapsed = clock.elapsed_ms;
    let state = clock.state;
    let reduced_signal = clock.reduced;

    let state_attr = match *state.read() {
        SceneState::Paused => "paused",
        SceneState::Playing => "playing",
        SceneState::Settled => "settled",
    };
    let elapsed_attr = format!("{}", *elapsed.read() as i64);
    let duration_attr = format!("{}", duration_ms as i64);
    let reduced_attr = if *reduced_signal.read() { "true" } else { "false" };
    let aspect = format!("aspect-ratio: {} / {}", width, height);

    rsx! {
        section {
            class: "ui-scene-stage",
            style: "{aspect}",
            "data-composition-id": "{id}",
            "data-width": "{width}",
            "data-height": "{height}",
            "data-fps": "{fps}",
            "data-duration-ms": "{duration_attr}",
            "data-elapsed-ms": "{elapsed_attr}",
            "data-state": "{state_attr}",
            "data-reduced": "{reduced_attr}",
            {children}
        }
    }
}
```

Modify `crates/ui-dioxus/src/lib.rs`. Find the existing `pub mod composition;` line and add below it:

```rust
pub mod scene_player;
```

In the same file, find the existing `pub use composition::{FrameClip, FrameLayer, FrameStage};` line and add below it:

```rust
pub use scene_player::{Scene, SceneContext};
pub use ui_runtime::SceneState;
```

If `ui-runtime` is not yet a dependency of `ui-dioxus`, check:

```bash
grep -n "ui-runtime" crates/ui-dioxus/Cargo.toml
```

It almost certainly is (existing code uses `use_reduced_motion`, `SharedElementRegistry`, etc.).

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-dioxus --test scene_player_ssr`
Expected: 3 PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-dioxus/src/scene_player.rs crates/ui-dioxus/src/lib.rs crates/ui-dioxus/tests/scene_player_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): Scene component shell with SceneContext

Provides the SceneClock + FrameAdapterRegistry via context, emits
hyperframes-compatible data-* attributes and an aspect-ratio inline
style. Reduced-motion settles immediately. Transport UI and adapter
fan-out are wired in follow-up tasks.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 10: `Clip` Dioxus component (drives visibility off `SceneContext`)

**Files:**
- Modify: `crates/ui-dioxus/src/scene_player.rs` (append `Clip` component)
- Modify: `crates/ui-dioxus/src/lib.rs` (re-export `Clip`)
- Test: `crates/ui-dioxus/tests/scene_player_ssr.rs` (append tests)

- [ ] **Step 1: Write the failing tests**

Append to `crates/ui-dioxus/tests/scene_player_ssr.rs`:

```rust
use ui_composition::ClipFill;

#[test]
fn clip_inside_scene_renders_active_at_settled() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: true,
            Scene {
                id: "intro", width: 100, height: 100, duration_ms: 2_000.0,
                Clip { start_ms: 0.0, duration_ms: 1_000.0, p { "first" } }
                Clip { start_ms: 1_000.0, duration_ms: 1_000.0, p { "second" } }
            }
        }
    });
    // At elapsed = duration = 2000ms, second clip is in range; first is past
    // its end and (default ClipFill::None) inactive.
    assert!(html.contains("data-clip-active=\"false\"") && html.contains("first"), "{html}");
    assert!(html.contains("data-clip-active=\"true\"") && html.contains("second"), "{html}");
}

#[test]
fn clip_with_hold_end_remains_active_after_range() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: true,
            Scene {
                id: "intro", width: 100, height: 100, duration_ms: 5_000.0,
                Clip {
                    start_ms: 0.0,
                    duration_ms: 1_000.0,
                    fill: Some(ClipFill::HoldEnd),
                    p { "held" }
                }
            }
        }
    });
    assert!(html.contains("data-clip-active=\"true\""), "{html}");
}

#[test]
fn clip_outside_scene_panics_in_debug_renders_warning_in_release() {
    // We choose: a Clip without a SceneContext renders the children as-is
    // with data-clip-active="true" and a `data-clip-orphan="true"` flag.
    let html = dioxus_ssr::render_element(rsx! {
        Clip { start_ms: 0.0, duration_ms: 1.0, p { "orphan" } }
    });
    assert!(html.contains("data-clip-orphan=\"true\""), "{html}");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-dioxus --test scene_player_ssr clip_`
Expected: compile error — `Clip` not found.

- [ ] **Step 3: Implement `Clip`**

Append to `crates/ui-dioxus/src/scene_player.rs`:

```rust
use ui_composition::{ClipFill, FrameClip};

#[component]
pub fn Clip(
    start_ms: f32,
    duration_ms: f32,
    fill: Option<ClipFill>,
    children: Element,
) -> Element {
    let fill = fill.unwrap_or(ClipFill::None);
    let ctx = try_consume_context::<SceneContext>();
    let Some(ctx) = ctx else {
        // Orphan clip (no Scene ancestor): render children, flag for diagnostics.
        return rsx! {
            div {
                class: "ui-scene-clip ui-scene-clip--orphan",
                "data-clip-orphan": "true",
                "data-clip-active": "true",
                {children}
            }
        };
    };

    let frame_clip = FrameClip::new(start_ms.max(0.0) as u32, duration_ms.max(0.0) as u32, fill);
    let elapsed = ctx.clock.elapsed_ms;
    let active = frame_clip.active_at_ms(*elapsed.read());

    let style = if active {
        "opacity: 1"
    } else {
        match fill {
            ClipFill::None => "opacity: 0; visibility: hidden; pointer-events: none",
            ClipFill::HoldStart | ClipFill::HoldEnd | ClipFill::HoldBoth => "opacity: 1",
        }
    };
    let fill_attr = match fill {
        ClipFill::None => "none",
        ClipFill::HoldStart => "hold-start",
        ClipFill::HoldEnd => "hold-end",
        ClipFill::HoldBoth => "hold-both",
    };

    rsx! {
        div {
            class: "ui-scene-clip",
            style: "{style}",
            "data-start-ms": "{start_ms}",
            "data-duration-ms": "{duration_ms}",
            "data-fill": "{fill_attr}",
            "data-clip-active": "{active}",
            {children}
        }
    }
}
```

Modify `crates/ui-dioxus/src/lib.rs` re-export line:

```rust
pub use scene_player::{Clip, Scene, SceneContext};
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-dioxus --test scene_player_ssr`
Expected: 6 PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-dioxus/src/scene_player.rs crates/ui-dioxus/src/lib.rs crates/ui-dioxus/tests/scene_player_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): Clip drives visibility from SceneContext clock

Consumes SceneContext, computes active_at_ms from the underlying
FrameClip, and applies opacity/visibility/pointer-events on the
out-of-range case based on ClipFill. Orphan clips (no Scene ancestor)
render children with data-clip-orphan="true" for diagnostics rather
than panicking in SSR.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 11: Transport UI (play/pause button + scrubber + time readout)

**Files:**
- Modify: `crates/ui-dioxus/src/scene_player.rs` (wire `controls` flag)
- Test: `crates/ui-dioxus/tests/scene_player_ssr.rs` (append tests)

- [ ] **Step 1: Write the failing tests**

Append:

```rust
#[test]
fn scene_with_controls_renders_transport_bar() {
    let html = dioxus_ssr::render_element(rsx! {
        Scene {
            id: "intro", width: 100, height: 100, duration_ms: 5_000.0,
            autoplay: Some(false),
            controls: Some(true),
            p { "body" }
        }
    });
    assert!(html.contains("ui-scene-transport"), "{html}");
    assert!(html.contains("ui-scene-play"), "{html}");
    assert!(html.contains("type=\"range\""), "{html}");
    assert!(html.contains("min=\"0\""), "{html}");
    assert!(html.contains("max=\"5000\""), "{html}");
    assert!(html.contains("ui-scene-time"), "{html}");
}

#[test]
fn scene_transport_marks_scrubber_disabled_under_reduced_motion() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: true,
            Scene {
                id: "intro", width: 100, height: 100, duration_ms: 1_000.0,
                controls: Some(true),
                p { "body" }
            }
        }
    });
    assert!(html.contains("aria-disabled=\"true\""), "{html}");
    assert!(html.contains("ui-scene-reduced-tag"), "{html}");
    assert!(html.contains("Reduced motion · settled state"), "{html}");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p ui-dioxus --test scene_player_ssr transport`
Expected: failures — transport elements absent.

- [ ] **Step 3: Implement transport bar**

In `crates/ui-dioxus/src/scene_player.rs`, replace the existing `Scene` `rsx!` block return so the transport renders when `controls=true`. Update the prefix (already drops `let _ = controls;`) and add at the bottom:

```rust
    let show_transport = controls.unwrap_or(false);
    let duration_attr_for_input = duration_ms.max(0.0);
    let reduced_now = *reduced_signal.read();

    let play_label = if matches!(*state.read(), SceneState::Playing) {
        "Pause"
    } else {
        "Play"
    };

    let scrubber_value = format!("{}", *elapsed.read() as i64);
    let scrubber_max = format!("{}", duration_attr_for_input as i64);
    let time_text = format!(
        "{:.2}s / {:.2}s",
        *elapsed.read() / 1000.0,
        duration_ms / 1000.0
    );

    let scrubber_disabled = reduced_now;

    rsx! {
        section {
            class: "ui-scene-stage",
            style: "{aspect}",
            "data-composition-id": "{id}",
            "data-width": "{width}",
            "data-height": "{height}",
            "data-fps": "{fps}",
            "data-duration-ms": "{duration_attr}",
            "data-elapsed-ms": "{elapsed_attr}",
            "data-state": "{state_attr}",
            "data-reduced": "{reduced_attr}",
            {children}
            if show_transport {
                div { class: "ui-scene-transport",
                    button {
                        class: "ui-scene-play",
                        r#type: "button",
                        disabled: reduced_now,
                        onclick: move |_| {
                            if matches!(*state.read(), SceneState::Playing) {
                                clock.pause();
                            } else {
                                clock.play();
                            }
                        },
                        "{play_label}"
                    }
                    input {
                        class: "ui-scene-scrubber",
                        r#type: "range",
                        min: "0",
                        max: "{scrubber_max}",
                        step: "1",
                        value: "{scrubber_value}",
                        aria_disabled: if scrubber_disabled { "true" } else { "false" },
                        oninput: move |evt| {
                            if reduced_now {
                                return;
                            }
                            if let Ok(ms) = evt.value().parse::<f32>() {
                                clock.seek_ms(ms);
                            }
                        },
                    }
                    span { class: "ui-scene-time", "{time_text}" }
                    if reduced_now {
                        span { class: "ui-scene-reduced-tag",
                            "Reduced motion · settled state"
                        }
                    }
                }
            }
        }
    }
```

Remove the old single `rsx!` block at the end of the component (replace it with the new one above).

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui-dioxus --test scene_player_ssr`
Expected: 8 PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-dioxus/src/scene_player.rs crates/ui-dioxus/tests/scene_player_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): Scene transport UI (play/pause, scrubber, time)

Controls render only when controls=true. Under reduced-motion the
scrubber is aria-disabled and a 'Reduced motion · settled state' tag
appears next to the time readout so the policy is visible.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 12: Adapter fan-out effect inside `Scene`

The Scene already provides the registry through context; this task wires the `use_effect` that, on every `elapsed_ms` (or `state`) change, calls `registry.broadcast_seek`. The effect intentionally subscribes to both signals so a settle that doesn't actually change `elapsed_ms` (e.g. final tick coalesced) still fires one last broadcast.

**Files:**
- Modify: `crates/ui-dioxus/src/scene_player.rs`
- Test: `crates/ui-dioxus/tests/scene_player_ssr.rs` (append regression test for registry presence)

- [ ] **Step 1: Add the fan-out effect**

In `crates/ui-dioxus/src/scene_player.rs`, inside the `Scene` function body, just after the `use_context_provider` call, add:

```rust
    // Fan seek out to every registered adapter whenever elapsed_ms or
    // state changes. We read both signals so settle transitions emit
    // one final broadcast even if elapsed_ms is unchanged.
    use_effect(move || {
        let ms = *clock.elapsed_ms.read();
        let _ = *clock.state.read();
        let reduced = *clock.reduced.read();
        adapters_signal.read().broadcast_seek(ms, reduced);
    });
```

- [ ] **Step 2: Append regression test**

Append to `crates/ui-dioxus/tests/scene_player_ssr.rs`:

```rust
#[test]
fn scene_provides_adapter_registry_via_context() {
    // Smoke test: SceneContext is accessible inside children.
    #[component]
    fn ContextProbe() -> Element {
        let ctx = try_consume_context::<ui_dioxus::SceneContext>();
        let has = ctx.is_some();
        rsx! { span { "data-probe-has-context": "{has}", "ctx?" } }
    }
    let html = dioxus_ssr::render_element(rsx! {
        Scene {
            id: "intro", width: 1, height: 1, duration_ms: 10.0,
            autoplay: Some(false),
            ContextProbe {}
        }
    });
    assert!(html.contains("data-probe-has-context=\"true\""), "{html}");
}
```

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test -p ui-dioxus --test scene_player_ssr`
Expected: 9 PASS.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-dioxus/src/scene_player.rs crates/ui-dioxus/tests/scene_player_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): Scene fans seek out to registered FrameAdapters

The effect subscribes to elapsed_ms AND state so settle transitions
emit one final broadcast even when elapsed_ms didn't change. Adapter
registry is reachable via SceneContext for children that need to
register at mount time.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 13: Stylesheet `scene_player.css` wired into `library_css()`

**Files:**
- Create: `crates/ui-styles/src/scene_player.css`
- Modify: `crates/ui-styles/src/lib.rs` (include the new CSS)

- [ ] **Step 1: Inspect the existing `ui-styles` layout**

Run: `cat crates/ui-styles/src/lib.rs | head -60`

You will see a pattern of `pub const FOO_CSS: &str = include_str!("foo.css");` and a `library_css()` function that concatenates the pieces. Match it.

- [ ] **Step 2: Create the stylesheet**

Create `crates/ui-styles/src/scene_player.css`:

```css
/* Scene player transport, scrubber, and reduced-motion tag styling. */
.ui-scene-stage {
  position: relative;
  display: block;
  width: 100%;
  background: var(--ui-color-surface, #0b0b0f);
  color: var(--ui-color-on-surface, #f5f5f7);
  border-radius: 12px;
  overflow: hidden;
}

.ui-scene-transport {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 12px;
  background: color-mix(in oklab, var(--ui-color-surface, #0b0b0f), transparent 30%);
  border-top: 1px solid color-mix(in oklab, currentColor, transparent 80%);
}

.ui-scene-play {
  appearance: none;
  border: 1px solid currentColor;
  background: transparent;
  color: inherit;
  border-radius: 999px;
  padding: 4px 12px;
  font: inherit;
  cursor: pointer;
}
.ui-scene-play:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.ui-scene-scrubber {
  flex: 1 1 auto;
  accent-color: var(--ui-color-accent, #7aa2ff);
}
.ui-scene-scrubber[aria-disabled="true"] {
  opacity: 0.4;
  pointer-events: none;
}

.ui-scene-time {
  font-variant-numeric: tabular-nums;
  font-size: 12px;
  opacity: 0.8;
}

.ui-scene-reduced-tag {
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 999px;
  background: color-mix(in oklab, var(--ui-color-warn, #f7c948), transparent 60%);
  color: var(--ui-color-on-warn, #1a1a1a);
}

.ui-scene-clip[data-clip-active="false"] {
  /* Inline style on the element already handles opacity/visibility;
     this rule is here so designer overrides have a single selector
     to target. */
}
```

- [ ] **Step 3: Wire the const into `library_css()`**

In `crates/ui-styles/src/lib.rs`, add a `pub const SCENE_PLAYER_CSS: &str = include_str!("scene_player.css");` adjacent to the other CSS constants, and append `SCENE_PLAYER_CSS` to the string returned by `library_css()` (the function concatenates the workspace stylesheets).

- [ ] **Step 4: Verify**

Run: `cargo check -p ui-styles && cargo test --workspace`
Expected: success.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-styles/src/scene_player.css crates/ui-styles/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(ui-styles): scene_player.css for transport + reduced tag

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 14: Deprecate `FrameStage` / `FrameClip` / `FrameLayer`

**Files:**
- Modify: `crates/ui-dioxus/src/composition.rs`

- [ ] **Step 1: Add deprecation attributes**

In `crates/ui-dioxus/src/composition.rs`, prefix each of the three `#[component]` functions with `#[deprecated(since = "0.7.0", note = "use kinetics::Scene / kinetics::Clip")]`. Example:

```rust
#[deprecated(since = "0.7.0", note = "use kinetics::Scene / kinetics::Clip")]
#[component]
pub fn FrameStage(composition: Composition, frame: u32, children: Element) -> Element {
    // ... existing body unchanged
}
```

Repeat for `FrameClip` and `FrameLayer`.

Suppress the deprecation warning at the existing call sites in `examples/component-gallery/src/previews/composition.rs` by adding `#![allow(deprecated)]` at the top of that file (until SP-2 migrates the previews).

- [ ] **Step 2: Verify**

Run: `cargo check -p ui-dioxus -p component-gallery`
Expected: success with the new symbols available; the existing previews still compile because of `#![allow(deprecated)]`.

- [ ] **Step 3: Commit**

```bash
git add crates/ui-dioxus/src/composition.rs examples/component-gallery/src/previews/composition.rs
git commit -m "$(cat <<'EOF'
chore(ui-dioxus): deprecate FrameStage/FrameClip/FrameLayer

Suppress the warning at existing gallery preview sites so the legacy
showcase stays green; new code uses Scene + Clip.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 15: `kinetics::prelude` + `public_api_names()` updates

**Files:**
- Modify: `crates/kinetics/src/lib.rs`

- [ ] **Step 1: Append the new symbols to the prelude**

In `crates/kinetics/src/lib.rs`, find the `pub use ui_dioxus::{ ... }` block and add `Clip, Scene, SceneContext` to the list. Just after the `pub use` lines, add:

```rust
    #[cfg(feature = "runtime")]
    pub use ui_runtime::{
        CssKeyframesAdapter, FrameAdapter, FrameAdapterHandle, FrameAdapterRegistry,
        SceneClock, SceneState, SequenceAdapter, WaapiAdapter,
    };
```

(If the prelude already has a `#[cfg(feature = "runtime")] pub use ui_runtime::{...}` block, merge into it instead of duplicating.)

In `public_api_names()`, append entries:

```rust
        "Scene",
        "Clip",
        "SceneContext",
        "SceneState",
        "SceneClock",
        "FrameAdapter",
        "FrameAdapterRegistry",
        "SequenceAdapter",
        "WaapiAdapter",
        "CssKeyframesAdapter",
```

- [ ] **Step 2: Verify**

Run: `cargo check -p kinetics --all-features`
Expected: success.

- [ ] **Step 3: Commit**

```bash
git add crates/kinetics/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(kinetics): export Scene/Clip/FrameAdapter in prelude

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 16: Gallery category `Scene`

**Files:**
- Modify: `examples/component-gallery/src/docs.rs`

- [ ] **Step 1: Add the variant**

In `examples/component-gallery/src/docs.rs`, extend `ComponentCategory`:

```rust
pub enum ComponentCategory {
    // ... existing variants
    Capture,
    Scene,
}
```

Add `label`, `description`, and `slug` arms:

```rust
    Self::Scene => "Scene",
    // description
    Self::Scene => {
        "Seekable cinematic compositions: one paused clock drives every animation runtime."
    },
    // slug
    Self::Scene => "scene",
```

Append `ComponentCategory::Scene` to the `categories()` list, after `ComponentCategory::Capture`.

- [ ] **Step 2: Verify**

Run: `cargo check -p component-gallery`
Expected: success. The new category renders empty until Task 18 lands the doc entry.

- [ ] **Step 3: Commit**

```bash
git add examples/component-gallery/src/docs.rs
git commit -m "$(cat <<'EOF'
feat(gallery): add Scene category enum + metadata

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 17: Scene fragments — `flip_card_deck.rs`, `metric_counter.rs`, `cta_pulse.rs`

These three are small Dioxus components that the showcase composes. Each is purely visual; no new runtime types are introduced.

**Files:**
- Create: `examples/component-gallery/src/previews/scenes/mod.rs`
- Create: `examples/component-gallery/src/previews/scenes/flip_card_deck.rs`
- Create: `examples/component-gallery/src/previews/scenes/metric_counter.rs`
- Create: `examples/component-gallery/src/previews/scenes/cta_pulse.rs`

- [ ] **Step 1: `scenes/mod.rs`**

Create `examples/component-gallery/src/previews/scenes/mod.rs`:

```rust
pub mod cta_pulse;
pub mod flip_card_deck;
pub mod metric_counter;
pub mod product_intro;
```

(The `product_intro` module is created in Task 18.)

- [ ] **Step 2: `flip_card_deck.rs`**

Create `examples/component-gallery/src/previews/scenes/flip_card_deck.rs`:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn FlipCardDeckScene() -> Element {
    rsx! {
        SharedLayout {
            div { class: "scene-card-deck",
                for (i, label) in ["Concept", "Build", "Ship"].iter().enumerate() {
                    SharedElement { id: format!("card-{i}"),
                        div {
                            class: "scene-card",
                            "data-card-index": "{i}",
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 3: `metric_counter.rs`**

Create `examples/component-gallery/src/previews/scenes/metric_counter.rs`:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn MetricCounterScene() -> Element {
    rsx! {
        div { class: "scene-metric",
            KineticText { id: "metric-headline", text: "Active builds".to_string(), cue: "fade-in" }
            KineticText { id: "metric-value", text: "1,287".to_string(), cue: "rise-in" }
            KineticText {
                id: "metric-delta",
                text: "+24% week over week".to_string(),
                cue: "fade-in",
            }
        }
    }
}
```

- [ ] **Step 4: `cta_pulse.rs`**

Create `examples/component-gallery/src/previews/scenes/cta_pulse.rs`:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn CtaPulseScene() -> Element {
    rsx! {
        div { class: "scene-cta",
            Button { variant: ButtonVariant::Primary, "Start building" }
            span { class: "scene-cta-caption", "Free to try. No credit card." }
        }
    }
}
```

- [ ] **Step 5: Verify**

Run: `cargo check -p component-gallery`
Expected: success.

- [ ] **Step 6: Commit**

```bash
git add examples/component-gallery/src/previews/scenes
git commit -m "$(cat <<'EOF'
feat(gallery): scene fragments — flip deck, metric counter, CTA pulse

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 18: `product_intro.rs` — the 10-second cinematic showcase

**Files:**
- Create: `examples/component-gallery/src/previews/scenes/product_intro.rs`

- [ ] **Step 1: Implement the scene**

Create `examples/component-gallery/src/previews/scenes/product_intro.rs`:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;

use super::cta_pulse::CtaPulseScene;
use super::flip_card_deck::FlipCardDeckScene;
use super::metric_counter::MetricCounterScene;

#[component]
pub fn ProductIntroScene() -> Element {
    rsx! {
        Scene {
            id: "product-intro",
            width: 1920,
            height: 1080,
            duration_ms: 10_000.0,
            fps: Some(60),
            autoplay: Some(true),
            controls: Some(true),

            Clip { start_ms: 0.0, duration_ms: 2_400.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "intro-title",
                    text: "Kinetics moves like light.".to_string(),
                    cue: "rise-in",
                }
            }
            Clip { start_ms: 800.0, duration_ms: 2_400.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "intro-body",
                    text: "Composable motion for downstream SaaS.".to_string(),
                    cue: "fade-in",
                }
            }
            Clip { start_ms: 3_000.0, duration_ms: 4_000.0,
                FlipCardDeckScene {}
            }
            Clip { start_ms: 4_800.0, duration_ms: 2_200.0,
                MetricCounterScene {}
            }
            Clip { start_ms: 6_800.0, duration_ms: 3_200.0, fill: ClipFill::HoldEnd,
                CtaPulseScene {}
            }
        }
    }
}
```

- [ ] **Step 2: Verify**

Run: `cargo check -p component-gallery`
Expected: success.

- [ ] **Step 3: Commit**

```bash
git add examples/component-gallery/src/previews/scenes/product_intro.rs
git commit -m "$(cat <<'EOF'
feat(gallery): Product Intro 10s scene composed from Scene + Clip

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 19: Wire `previews/scene.rs` + `docs.rs` entry

**Files:**
- Create: `examples/component-gallery/src/previews/scene.rs`
- Modify: `examples/component-gallery/src/previews/mod.rs`
- Modify: `examples/component-gallery/src/docs.rs` (add `ComponentDoc` entry + snippet const)

- [ ] **Step 1: Create the preview function**

Create `examples/component-gallery/src/previews/scene.rs`:

```rust
use dioxus::prelude::*;

use crate::previews::scenes::product_intro::ProductIntroScene;

pub fn product_intro_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            ProductIntroScene {}
        }
    }
}
```

- [ ] **Step 2: Register the modules**

In `examples/component-gallery/src/previews/mod.rs`, add:

```rust
pub mod scene;
pub mod scenes;
```

- [ ] **Step 3: Add the snippet + doc entry**

In `examples/component-gallery/src/docs.rs`, add a snippet constant near the other `*_SNIPPET` consts:

```rust
const SCENE_PRODUCT_INTRO_SNIPPET: &str = r##"Scene {
    id: "product-intro",
    width: 1920,
    height: 1080,
    duration_ms: 10_000.0,
    autoplay: true,
    controls: true,
    Clip { start_ms: 0.0,    duration_ms: 2_400.0, fill: ClipFill::HoldEnd, /* title */ }
    Clip { start_ms: 800.0,  duration_ms: 2_400.0, fill: ClipFill::HoldEnd, /* body  */ }
    Clip { start_ms: 3_000.0,duration_ms: 4_000.0,                          /* deck  */ }
    Clip { start_ms: 4_800.0,duration_ms: 2_200.0,                          /* count */ }
    Clip { start_ms: 6_800.0,duration_ms: 3_200.0, fill: ClipFill::HoldEnd, /* CTA   */ }
}"##;
```

Append to the `COMPONENT_DOCS` array (the last `ComponentDoc { ... }` literal in the file):

```rust
    ComponentDoc {
        name: "Scene · Product Intro 10s",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "Seekable 10-second cinematic composition: title, FLIP card deck, metric counter, CTA pulse — one paused clock for every runtime.",
        snippet: SCENE_PRODUCT_INTRO_SNIPPET,
        accessibility: "Scrubber is keyboard-operable; reduced-motion renders the settled state and disables the scrubber with an explicit tag.",
        render: Some(crate::previews::scene::product_intro_preview),
    },
```

- [ ] **Step 4: Verify build**

Run: `cargo check -p component-gallery && cargo test -p component-gallery`
Expected: success.

- [ ] **Step 5: Commit**

```bash
git add examples/component-gallery/src
git commit -m "$(cat <<'EOF'
feat(gallery): wire Scene category + Product Intro doc entry

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 20: Playwright E2E spec

**Files:**
- Create: `examples/component-gallery/e2e/tests/scene-player.spec.ts`

- [ ] **Step 1: Inspect an existing spec for shape**

Read `examples/component-gallery/e2e/tests/` for an example spec (e.g. one that uses `data-ui-motion` to set the reduced-motion preference). Note the page-load helper and the preference-toggle helper.

- [ ] **Step 2: Write the spec**

Create `examples/component-gallery/e2e/tests/scene-player.spec.ts`:

```ts
import { expect, test } from "@playwright/test";

const SCENE_SECTION = "#scene";
const CARD_SELECTOR = "article.gallery-entry:has(h4:has-text('Scene · Product Intro 10s'))";

test.describe("Scene player", () => {
  test("renders transport controls and scrubs to settled", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(CARD_SELECTOR);
    await expect(card).toBeVisible();

    const scrubber = card.locator("input.ui-scene-scrubber");
    await expect(scrubber).toBeVisible();
    await expect(scrubber).toHaveAttribute("min", "0");
    await expect(scrubber).toHaveAttribute("max", "10000");

    const stage = card.locator(".ui-scene-stage");
    // Drive the scrubber to the end.
    await scrubber.evaluate((el: HTMLInputElement) => {
      el.value = "10000";
      el.dispatchEvent(new Event("input", { bubbles: true }));
    });
    await expect(stage).toHaveAttribute("data-state", "settled");
    await expect(stage).toHaveAttribute("data-elapsed-ms", "10000");
  });

  test("reduced-motion disables scrubber and shows tag", async ({ page }) => {
    await page.goto("/");
    // The gallery preference bar toggles data-ui-motion="reduced" on the shell;
    // find the Reduced option in the Motion preference group.
    await page.getByRole("radio", { name: /Reduced/i }).click();

    const card = page.locator(CARD_SELECTOR);
    const scrubber = card.locator("input.ui-scene-scrubber");
    await expect(scrubber).toHaveAttribute("aria-disabled", "true");
    await expect(card.locator(".ui-scene-reduced-tag")).toBeVisible();

    const stage = card.locator(".ui-scene-stage");
    await expect(stage).toHaveAttribute("data-state", "settled");
    await expect(stage).toHaveAttribute("data-reduced", "true");
  });
});
```

- [ ] **Step 3: Build the static gallery and run the spec**

Build the gallery in release mode (the spec's `static` project serves from `target/dx/component-gallery/release/web/public`):

```bash
cd examples/component-gallery
dx build --release
```

Then from the `e2e/` directory:

```bash
cd e2e
npx playwright test --project=static tests/scene-player.spec.ts
npx playwright test --project=static-webkit tests/scene-player.spec.ts
```

Expected: both projects pass.

If the Reduced radio's accessible name differs from the regex (`/Reduced/i`), inspect the existing PreferenceBar markup in `examples/component-gallery/src/controls.rs` and adjust the selector accordingly. Match the pattern used by sibling specs — do not invent new selectors.

- [ ] **Step 4: Commit**

```bash
git add examples/component-gallery/e2e/tests/scene-player.spec.ts
git commit -m "$(cat <<'EOF'
test(gallery-e2e): Playwright spec for Scene player

Covers scrub-to-settled on Chromium and WebKit and the reduced-motion
scrubber-disabled + tag-visible policy.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 21: Workspace-wide verification

**Steps:**

- [ ] **Step 1: Lints + format**

Run sequentially:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Expected: zero warnings. Fix any clippy hits inline (the most common will be unused-variable or `needless_borrow` in the new previews — clear them up without changing behavior).

- [ ] **Step 2: Full test suite**

```bash
cargo test --workspace
```

Expected: all tests pass.

- [ ] **Step 3: E2E on both engines**

```bash
cd examples/component-gallery
dx build --release
cd e2e
npx playwright test --project=static
npx playwright test --project=static-webkit
```

Expected: every spec, including the new `scene-player.spec.ts`, passes on both engines. If a WebKit-only failure appears in transport scrubbing, check whether the WebKit `<input type=range>` raises `input` events on programmatic `.value=` assignments — if not, dispatch `change` as well.

- [ ] **Step 4: Smoke the gallery interactively**

```bash
cd examples/component-gallery
dx serve --hot-reload
```

Open the URL the dev server prints, navigate to `#scene`, and visually confirm:
- The Product Intro card autoplays the 10-second composition.
- The scrubber drags from 0 to 10 s and the scene responds.
- Toggling the Motion preference to Reduced disables the scrubber and reveals the "Reduced motion · settled state" tag.

- [ ] **Step 5: Final commit**

If the above steps required any tweaks (formatting, clippy fixes, small ts selector edits), commit them as:

```bash
git add -A
git commit -m "$(cat <<'EOF'
chore(scene-player): fmt + clippy + e2e cleanup

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

If no tweaks were needed, skip the commit.

---

## Self-Review Notes

**Spec coverage:** Every section of `2026-05-24-scene-player-design.md` maps to one or more tasks above — trait/registry → Task 2, clock → Tasks 3-4, three adapters → Tasks 5-7, `Scene`/`Clip` components → Tasks 9-12, CSS → Task 13, deprecations → Task 14, prelude → Task 15, gallery → Tasks 16-19, Playwright → Task 20, verification → Task 21. Acceptance criteria from the spec (cargo + Playwright + scrub-to-settled + reduced tag) are exercised by Task 21.

**Out-of-scope checks:** No task introduces ScrollTrigger / SplitText / MotionPath (SP-3), MP4 render (SP-4), CLI (SP-5), or catalog blocks (SP-6). The plan keeps SP-1 strictly to the keystone.

**Naming consistency:** Types `Scene`, `Clip`, `SceneClock`, `SceneContext`, `SceneState`, `FrameAdapter`, `FrameAdapterRegistry`, `FrameAdapterHandle`, `SequenceAdapter`, `WaapiAdapter`, `CssKeyframesAdapter`. Modules `scene_clock`, `scene_player`, `frame_adapter`, `adapters/`. Files `scene_player.rs`, `scene_player.css`, `scene-player.spec.ts`. CSS classes `ui-scene-stage`, `ui-scene-transport`, `ui-scene-play`, `ui-scene-scrubber`, `ui-scene-time`, `ui-scene-reduced-tag`, `ui-scene-clip`. Gallery category `Scene`, slug `scene`. All consistent across tasks.
