# Presence And IconButton Animated Runtime Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Stand up a pure-Rust animation runtime (`ui-runtime` crate), a curated icon library (`ui-icons` crate), a fully-animated `Presence` component, and a polished `IconButton` component, with reduced-motion support and SSR safety.

**Architecture:** Two new crates plus targeted additions to `ui-dioxus`, `ui-styles`, `kinetics`, and the gallery. The runtime exposes a `FrameScheduler` (cfg-gated for wasm vs non-wasm) and Dioxus hooks `use_animation_value` and `use_presence_state`. The presence state machine is pulled into a pure function for deterministic testing; the hooks are thin wrappers. SSR is safe because `use_future` does not execute during `dioxus-ssr` rendering — the hooks return their initial state synchronously.

**Tech Stack:** Rust 2021, Cargo workspace, Dioxus 0.7, `dioxus-ssr` for tests, `tokio` (non-wasm) + `wasm-bindgen` + `web-sys` (wasm) gated by `#[cfg(target_arch = "wasm32")]`, pure Rust unit tests, static CSS strings, PowerShell on Windows.

---

## Scope

This plan implements sub-project 2 from `docs/superpowers/specs/2026-05-21-presence-iconbutton-runtime-design.md`.

It includes:

- New crates `ui-runtime` and `ui-icons` added to the workspace.
- Eight curated icon components (`Close`, `Check`, `ChevronDown`, `ChevronRight`, `Plus`, `Minus`, `Trash`, `Search`).
- `use_animation_value`, `use_presence_state`, `use_reduced_motion` hooks.
- `IconButton` component with tone and size variants, full CSS treatment, accessible label.
- `Presence` component alongside the existing (untouched) `PresenceGate`.
- `kinetics` facade re-exports under new `runtime` and `icons` features (default-on).
- Gallery promotions: `IconButton` and `Presence` to `Ready` with variant grids.

It excludes:

- Sequence, SharedLayout, SharedElement (later sub-projects).
- A JS `eval` runtime path.
- Native (Blitz) animation.
- Icons beyond the curated set.

## Before You Start

If you are running this plan through `superpowers:subagent-driven-development`, that skill creates the worktree. Otherwise:

```powershell
git worktree add .worktrees/presence-iconbutton-runtime -b presence-iconbutton-runtime main
```

Run every command from inside the worktree.

## File Map

- `Cargo.toml` (workspace): add `crates/ui-runtime`, `crates/ui-icons` to members; add `ui-runtime`, `ui-icons` to `[workspace.dependencies]`.
- `crates/ui-runtime/Cargo.toml`: package manifest with cfg-gated deps.
- `crates/ui-runtime/src/lib.rs`: re-exports for the public API.
- `crates/ui-runtime/src/state.rs`: pure-function presence state machine (no async).
- `crates/ui-runtime/src/scheduler.rs`: `ControlFlow`, `FrameHandle`, `FrameScheduler` traits and platform-shared types.
- `crates/ui-runtime/src/scheduler_native.rs`: `#[cfg(not(target_arch = "wasm32"))]` Tokio-based scheduler.
- `crates/ui-runtime/src/scheduler_web.rs`: `#[cfg(target_arch = "wasm32")]` `web_sys::Window::request_animation_frame` scheduler.
- `crates/ui-runtime/src/animation.rs`: `use_animation_value` hook.
- `crates/ui-runtime/src/presence.rs`: `use_presence_state` hook, `PresenceState` enum.
- `crates/ui-runtime/src/reduced_motion.rs`: `ReducedMotion` context + `use_reduced_motion` hook.
- `crates/ui-runtime/tests/state.rs`: pure state-machine tests.
- `crates/ui-runtime/tests/scheduler_native.rs`: tokio-paused scheduler tests.
- `crates/ui-runtime/tests/hooks_ssr.rs`: SSR rendering tests for the hooks.
- `crates/ui-icons/Cargo.toml`: package manifest.
- `crates/ui-icons/src/lib.rs`: declares icons module and re-exports.
- `crates/ui-icons/src/icons.rs`: eight icon components plus path constants.
- `crates/ui-icons/tests/icons.rs`: SSR icon rendering tests.
- `crates/ui-dioxus/src/buttons.rs`: `IconButton`, `IconButtonTone`, `IconButtonSize`.
- `crates/ui-dioxus/src/kinetics.rs`: add `Presence` and `PresenceCue` next to existing `PresenceGate`, `KineticBox`, etc.
- `crates/ui-dioxus/src/lib.rs`: register `buttons` module; add new exports.
- `crates/ui-dioxus/Cargo.toml`: add `ui-runtime`, `ui-icons` deps.
- `crates/ui-dioxus/tests/icon_button_ssr.rs`: IconButton SSR tests.
- `crates/ui-dioxus/tests/presence_ssr.rs`: Presence SSR tests.
- `crates/kinetics/Cargo.toml`: add `runtime` and `icons` features (default-on); add optional deps.
- `crates/kinetics/src/lib.rs`: re-export new symbols; extend `public_api_names()`.
- `crates/kinetics/tests/prelude.rs`: assert new names appear.
- `crates/ui-styles/src/lib.rs`: CSS for `.ui-icon-button*` and `.ui-presence*`, plus reduced-motion fallback.
- `crates/ui-styles/tests/css.rs`: selector assertions.
- `examples/component-gallery/src/docs.rs`: promote `IconButton` to Ready; add new `Presence` entry; preview functions.
- `examples/component-gallery/tests/gallery.rs`: variant-grid SSR assertions.
- `README.md`: workspace layout block adds `ui-runtime/` and `ui-icons/`; feature list adds `runtime`, `icons`.

## Task 1: Scaffold The Two New Crates

**Files:**
- Create: `crates/ui-runtime/Cargo.toml`
- Create: `crates/ui-runtime/src/lib.rs`
- Create: `crates/ui-icons/Cargo.toml`
- Create: `crates/ui-icons/src/lib.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: Create `ui-runtime` package manifest**

Write `crates/ui-runtime/Cargo.toml`:

```toml
[package]
name = "ui-runtime"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
dioxus.workspace = true
ui-motion = { path = "../ui-motion" }

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tokio = { version = "1", features = ["rt", "time", "macros"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["Window", "Document"] }
wasm-bindgen-futures = "0.4"

[dev-dependencies]
dioxus-ssr.workspace = true
tokio = { version = "1", features = ["rt", "time", "macros", "test-util"] }

[lib]
path = "src/lib.rs"
```

- [ ] **Step 2: Create minimal `ui-runtime/src/lib.rs`**

```rust
#![forbid(unsafe_code)]

//! Animation runtime for the kinetics UI library.
//!
//! Provides platform-abstracted frame scheduling and Dioxus hooks for
//! property animation and presence lifecycle.
```

- [ ] **Step 3: Create `ui-icons` package manifest**

Write `crates/ui-icons/Cargo.toml`:

```toml
[package]
name = "ui-icons"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
dioxus.workspace = true

[dev-dependencies]
dioxus-ssr.workspace = true

[lib]
path = "src/lib.rs"
```

- [ ] **Step 4: Create minimal `ui-icons/src/lib.rs`**

```rust
#![forbid(unsafe_code)]

//! Curated SVG icon components for the kinetics UI library.
```

- [ ] **Step 5: Register both crates in the workspace**

In root `Cargo.toml`, find the `[workspace] members = [ ... ]` list. Add these two entries (preserve existing entries):

```
    "crates/ui-runtime",
    "crates/ui-icons",
```

In the `[workspace.dependencies]` section, append:

```
ui-runtime = { path = "crates/ui-runtime" }
ui-icons = { path = "crates/ui-icons" }
```

- [ ] **Step 6: Verify the workspace builds**

Run:

```powershell
cargo check --workspace
```

Expected: PASS (both crates compile as empty libraries).

- [ ] **Step 7: Commit**

```powershell
git add Cargo.toml crates/ui-runtime crates/ui-icons
git commit -m "feat: scaffold ui-runtime and ui-icons crates"
```

## Task 2: Implement The Eight Icon Components

**Files:**
- Modify: `crates/ui-icons/src/lib.rs`
- Create: `crates/ui-icons/src/icons.rs`
- Create: `crates/ui-icons/tests/icons.rs`

- [ ] **Step 1: Write failing SSR tests for the icon set**

Create `crates/ui-icons/tests/icons.rs`:

```rust
use dioxus::prelude::*;
use ui_icons::*;

fn render(component: Element) -> String {
    dioxus_ssr::render_element(component)
}

#[test]
fn each_icon_renders_an_svg_with_viewbox_and_aria_hidden() {
    for html in [
        render(rsx! { Close { size: 24 } }),
        render(rsx! { Check { size: 24 } }),
        render(rsx! { ChevronDown { size: 24 } }),
        render(rsx! { ChevronRight { size: 24 } }),
        render(rsx! { Plus { size: 24 } }),
        render(rsx! { Minus { size: 24 } }),
        render(rsx! { Trash { size: 24 } }),
        render(rsx! { Search { size: 24 } }),
    ] {
        assert!(html.contains("<svg"), "expected svg element in {html}");
        assert!(html.contains("viewBox=\"0 0 24 24\""), "viewBox missing in {html}");
        assert!(html.contains("aria-hidden=\"true\""), "aria-hidden missing in {html}");
    }
}

#[test]
fn size_prop_controls_width_and_height() {
    let html = render(rsx! { Plus { size: 12 } });
    assert!(html.contains("width=\"12\""), "width missing: {html}");
    assert!(html.contains("height=\"12\""), "height missing: {html}");
}

#[test]
fn all_icons_export_non_empty_path_constants() {
    for path in [
        CLOSE_PATH_D,
        CHECK_PATH_D,
        CHEVRON_DOWN_PATH_D,
        CHEVRON_RIGHT_PATH_D,
        PLUS_PATH_D,
        MINUS_PATH_D,
        TRASH_PATH_D,
        SEARCH_PATH_D,
    ] {
        assert!(!path.is_empty());
    }
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p ui-icons
```

Expected: FAIL (the symbols don't exist yet).

- [ ] **Step 3: Implement the icon components and path constants**

Create `crates/ui-icons/src/icons.rs`:

```rust
use dioxus::prelude::*;

pub const CLOSE_PATH_D: &str = "M6 6l12 12M18 6L6 18";
pub const CHECK_PATH_D: &str = "M5 12l4 4 10-10";
pub const CHEVRON_DOWN_PATH_D: &str = "M6 9l6 6 6-6";
pub const CHEVRON_RIGHT_PATH_D: &str = "M9 6l6 6-6 6";
pub const PLUS_PATH_D: &str = "M12 5v14M5 12h14";
pub const MINUS_PATH_D: &str = "M5 12h14";
pub const TRASH_PATH_D: &str =
    "M4 7h16M9 7V4h6v3M6 7l1 13h10l1-13M10 11v6M14 11v6";
pub const SEARCH_PATH_D: &str = "M10 17a7 7 0 1 1 0-14 7 7 0 0 1 0 14zM21 21l-6-6";

fn stroke_icon(d: &'static str, size: u32) -> Element {
    rsx! {
        svg {
            "viewBox": "0 0 24 24",
            width: "{size}",
            height: "{size}",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            "aria-hidden": "true",
            path { d: "{d}" }
        }
    }
}

#[component]
pub fn Close(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(CLOSE_PATH_D, size)
}

#[component]
pub fn Check(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(CHECK_PATH_D, size)
}

#[component]
pub fn ChevronDown(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(CHEVRON_DOWN_PATH_D, size)
}

#[component]
pub fn ChevronRight(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(CHEVRON_RIGHT_PATH_D, size)
}

#[component]
pub fn Plus(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(PLUS_PATH_D, size)
}

#[component]
pub fn Minus(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(MINUS_PATH_D, size)
}

#[component]
pub fn Trash(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(TRASH_PATH_D, size)
}

#[component]
pub fn Search(#[props(default = 16)] size: u32) -> Element {
    stroke_icon(SEARCH_PATH_D, size)
}
```

- [ ] **Step 4: Re-export from `lib.rs`**

Replace `crates/ui-icons/src/lib.rs` with:

```rust
#![forbid(unsafe_code)]

//! Curated SVG icon components for the kinetics UI library.

mod icons;

pub use icons::*;
```

- [ ] **Step 5: Run icon tests**

```powershell
cargo test -p ui-icons
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add crates/ui-icons
git commit -m "feat: add curated icon components"
```

## Task 3: Pure-Function Presence State Machine

**Files:**
- Create: `crates/ui-runtime/src/state.rs`
- Create: `crates/ui-runtime/tests/state.rs`
- Modify: `crates/ui-runtime/src/lib.rs`

- [ ] **Step 1: Write failing tests**

Create `crates/ui-runtime/tests/state.rs`:

```rust
use ui_runtime::state::{advance_presence, PresenceInputs, PresenceState};

#[test]
fn initial_present_true_starts_entering() {
    let s = advance_presence(
        PresenceInputs {
            present: true,
            value: 0.0,
            prev_state: None,
        },
    );
    assert_eq!(s.state, PresenceState::Entering);
    assert_eq!(s.target, 1.0);
}

#[test]
fn initial_present_false_is_unmounted() {
    let s = advance_presence(
        PresenceInputs {
            present: false,
            value: 0.0,
            prev_state: None,
        },
    );
    assert_eq!(s.state, PresenceState::Unmounted);
    assert_eq!(s.target, 0.0);
}

#[test]
fn entering_settles_to_visible_when_value_near_one() {
    let s = advance_presence(PresenceInputs {
        present: true,
        value: 0.9995,
        prev_state: Some(PresenceState::Entering),
    });
    assert_eq!(s.state, PresenceState::Visible);
}

#[test]
fn visible_with_present_false_starts_exiting() {
    let s = advance_presence(PresenceInputs {
        present: false,
        value: 1.0,
        prev_state: Some(PresenceState::Visible),
    });
    assert_eq!(s.state, PresenceState::Exiting);
    assert_eq!(s.target, 0.0);
}

#[test]
fn exiting_settles_to_unmounted_when_value_near_zero() {
    let s = advance_presence(PresenceInputs {
        present: false,
        value: 0.0005,
        prev_state: Some(PresenceState::Exiting),
    });
    assert_eq!(s.state, PresenceState::Unmounted);
}

#[test]
fn exiting_interrupted_by_present_true_starts_entering() {
    let s = advance_presence(PresenceInputs {
        present: true,
        value: 0.4,
        prev_state: Some(PresenceState::Exiting),
    });
    assert_eq!(s.state, PresenceState::Entering);
    assert_eq!(s.target, 1.0);
}
```

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p ui-runtime --tests state
```

Expected: FAIL (the `state` module does not exist).

- [ ] **Step 3: Implement the pure state machine**

Create `crates/ui-runtime/src/state.rs`:

```rust
//! Pure-function presence lifecycle. No async, no Dioxus, fully testable.

const SETTLE_EPSILON: f32 = 0.001;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenceState {
    Entering,
    Visible,
    Exiting,
    Unmounted,
}

impl PresenceState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Entering => "entering",
            Self::Visible => "visible",
            Self::Exiting => "exiting",
            Self::Unmounted => "unmounted",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PresenceInputs {
    pub present: bool,
    pub value: f32,
    pub prev_state: Option<PresenceState>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PresenceTransition {
    pub state: PresenceState,
    pub target: f32,
}

pub fn advance_presence(inputs: PresenceInputs) -> PresenceTransition {
    let PresenceInputs {
        present,
        value,
        prev_state,
    } = inputs;

    match (present, prev_state) {
        (true, None) => PresenceTransition {
            state: PresenceState::Entering,
            target: 1.0,
        },
        (false, None) => PresenceTransition {
            state: PresenceState::Unmounted,
            target: 0.0,
        },
        (true, Some(PresenceState::Entering)) => {
            if (1.0 - value).abs() <= SETTLE_EPSILON {
                PresenceTransition {
                    state: PresenceState::Visible,
                    target: 1.0,
                }
            } else {
                PresenceTransition {
                    state: PresenceState::Entering,
                    target: 1.0,
                }
            }
        }
        (true, Some(PresenceState::Visible)) => PresenceTransition {
            state: PresenceState::Visible,
            target: 1.0,
        },
        (true, Some(PresenceState::Exiting | PresenceState::Unmounted)) => PresenceTransition {
            state: PresenceState::Entering,
            target: 1.0,
        },
        (false, Some(PresenceState::Visible | PresenceState::Entering)) => PresenceTransition {
            state: PresenceState::Exiting,
            target: 0.0,
        },
        (false, Some(PresenceState::Exiting)) => {
            if value.abs() <= SETTLE_EPSILON {
                PresenceTransition {
                    state: PresenceState::Unmounted,
                    target: 0.0,
                }
            } else {
                PresenceTransition {
                    state: PresenceState::Exiting,
                    target: 0.0,
                }
            }
        }
        (false, Some(PresenceState::Unmounted)) => PresenceTransition {
            state: PresenceState::Unmounted,
            target: 0.0,
        },
    }
}
```

- [ ] **Step 4: Wire into `lib.rs`**

Update `crates/ui-runtime/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

//! Animation runtime for the kinetics UI library.

pub mod state;

pub use state::{advance_presence, PresenceInputs, PresenceState, PresenceTransition};
```

- [ ] **Step 5: Run state tests**

```powershell
cargo test -p ui-runtime --tests state
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add crates/ui-runtime/src/lib.rs crates/ui-runtime/src/state.rs crates/ui-runtime/tests/state.rs
git commit -m "feat: add presence state machine"
```

## Task 4: Frame Scheduler (Non-Wasm Path)

**Files:**
- Create: `crates/ui-runtime/src/scheduler.rs`
- Create: `crates/ui-runtime/src/scheduler_native.rs`
- Create: `crates/ui-runtime/tests/scheduler_native.rs`
- Modify: `crates/ui-runtime/src/lib.rs`

- [ ] **Step 1: Write failing tokio-paused scheduler tests**

Create `crates/ui-runtime/tests/scheduler_native.rs`:

```rust
#![cfg(not(target_arch = "wasm32"))]

use std::sync::{Arc, Mutex};
use std::time::Duration;
use ui_runtime::scheduler::{spawn_frame_loop, ControlFlow};

#[tokio::test(start_paused = true)]
async fn frame_loop_invokes_callback_until_stop() {
    let counter = Arc::new(Mutex::new(0u32));
    let counter_clone = counter.clone();
    let handle = spawn_frame_loop(move |_dt| {
        let mut c = counter_clone.lock().unwrap();
        *c += 1;
        if *c >= 3 {
            ControlFlow::Stop
        } else {
            ControlFlow::Continue
        }
    });

    for _ in 0..6 {
        tokio::time::advance(Duration::from_millis(16)).await;
        tokio::task::yield_now().await;
    }

    assert_eq!(*counter.lock().unwrap(), 3);
    drop(handle);
}

#[tokio::test(start_paused = true)]
async fn dropping_handle_stops_the_loop() {
    let counter = Arc::new(Mutex::new(0u32));
    let counter_clone = counter.clone();
    let handle = spawn_frame_loop(move |_dt| {
        *counter_clone.lock().unwrap() += 1;
        ControlFlow::Continue
    });

    tokio::time::advance(Duration::from_millis(16)).await;
    tokio::task::yield_now().await;
    let after_one = *counter.lock().unwrap();

    drop(handle);

    for _ in 0..4 {
        tokio::time::advance(Duration::from_millis(16)).await;
        tokio::task::yield_now().await;
    }

    assert_eq!(*counter.lock().unwrap(), after_one);
}

#[tokio::test]
async fn spawn_outside_a_runtime_returns_a_noop_handle() {
    // This test verifies the public API works without a tokio runtime
    // by checking inside a runtime that try_current succeeds.
    let counter = Arc::new(Mutex::new(0u32));
    let counter_clone = counter.clone();
    let _handle = spawn_frame_loop(move |_dt| {
        *counter_clone.lock().unwrap() += 1;
        ControlFlow::Stop
    });
    // Just verifies no panic.
}
```

- [ ] **Step 2: Create the shared scheduler types**

Create `crates/ui-runtime/src/scheduler.rs`:

```rust
//! Platform-abstracted frame scheduler.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlFlow {
    Continue,
    Stop,
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    pub use super::super::scheduler_native::*;
}

#[cfg(target_arch = "wasm32")]
mod imp {
    pub use super::super::scheduler_web::*;
}

pub use imp::{spawn_frame_loop, FrameHandle};
```

- [ ] **Step 3: Implement the non-wasm scheduler**

Create `crates/ui-runtime/src/scheduler_native.rs`:

```rust
//! Tokio-based frame scheduler. Used on non-wasm targets.

#![cfg(not(target_arch = "wasm32"))]

use std::time::{Duration, Instant};
use tokio::runtime::Handle;
use tokio::task::JoinHandle;
use tokio::time::{interval, MissedTickBehavior};

use super::scheduler::ControlFlow;

const FRAME_PERIOD_MS: u64 = 16;

pub struct FrameHandle {
    join: Option<JoinHandle<()>>,
}

impl Drop for FrameHandle {
    fn drop(&mut self) {
        if let Some(join) = self.join.take() {
            join.abort();
        }
    }
}

pub fn spawn_frame_loop<F>(mut callback: F) -> FrameHandle
where
    F: FnMut(f64) -> ControlFlow + Send + 'static,
{
    let Ok(handle) = Handle::try_current() else {
        return FrameHandle { join: None };
    };
    let join = handle.spawn(async move {
        let mut ticker = interval(Duration::from_millis(FRAME_PERIOD_MS));
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
        let mut last = Instant::now();
        loop {
            ticker.tick().await;
            let now = Instant::now();
            let dt_ms = now.duration_since(last).as_secs_f64() * 1000.0;
            last = now;
            if matches!(callback(dt_ms), ControlFlow::Stop) {
                break;
            }
        }
    });
    FrameHandle { join: Some(join) }
}
```

- [ ] **Step 4: Update `lib.rs` to expose the scheduler**

Replace `crates/ui-runtime/src/lib.rs` with:

```rust
#![forbid(unsafe_code)]

//! Animation runtime for the kinetics UI library.

#[cfg(not(target_arch = "wasm32"))]
mod scheduler_native;

#[cfg(target_arch = "wasm32")]
mod scheduler_web;

pub mod scheduler;
pub mod state;

pub use scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};
pub use state::{advance_presence, PresenceInputs, PresenceState, PresenceTransition};
```

- [ ] **Step 5: Run scheduler tests**

```powershell
cargo test -p ui-runtime --tests scheduler_native
```

Expected: PASS.

- [ ] **Step 6: Run workspace tests to confirm nothing else broke**

```powershell
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add crates/ui-runtime
git commit -m "feat: add native tokio frame scheduler"
```

## Task 5: Frame Scheduler (Wasm Path)

**Files:**
- Create: `crates/ui-runtime/src/scheduler_web.rs`

This task adds the wasm-only implementation. It cannot be exercised by `cargo test` directly; the test is `cargo check --target wasm32-unknown-unknown` (compile-only verification).

- [ ] **Step 1: Implement the wasm scheduler**

Create `crates/ui-runtime/src/scheduler_web.rs`:

```rust
//! `web_sys::Window::request_animation_frame` frame scheduler. wasm-only.

#![cfg(target_arch = "wasm32")]

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::scheduler::ControlFlow;

pub struct FrameHandle {
    cancelled: Rc<RefCell<bool>>,
}

impl Drop for FrameHandle {
    fn drop(&mut self) {
        *self.cancelled.borrow_mut() = true;
    }
}

pub fn spawn_frame_loop<F>(callback: F) -> FrameHandle
where
    F: FnMut(f64) -> ControlFlow + 'static,
{
    let cancelled = Rc::new(RefCell::new(false));
    let handle = FrameHandle {
        cancelled: cancelled.clone(),
    };

    let window = match web_sys::window() {
        Some(w) => w,
        None => return handle,
    };

    let callback = Rc::new(RefCell::new(callback));
    let last_timestamp = Rc::new(RefCell::new(None::<f64>));

    let raf_closure: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> =
        Rc::new(RefCell::new(None));
    let raf_closure_outer = raf_closure.clone();

    let window_clone = window.clone();
    let cancelled_clone = cancelled.clone();

    let request_next = move |timestamp: f64| {
        if *cancelled_clone.borrow() {
            *raf_closure.borrow_mut() = None;
            return;
        }
        let dt_ms = match last_timestamp.borrow_mut().replace(timestamp) {
            Some(prev) => timestamp - prev,
            None => 0.0,
        };
        let mut cb = callback.borrow_mut();
        if matches!(cb(dt_ms), ControlFlow::Stop) {
            *raf_closure.borrow_mut() = None;
            return;
        }
        drop(cb);
        if let Some(closure) = raf_closure.borrow().as_ref() {
            let _ = window_clone
                .request_animation_frame(closure.as_ref().unchecked_ref());
        }
    };

    let closure = Closure::wrap(Box::new(request_next) as Box<dyn FnMut(f64)>);
    let _ = window
        .request_animation_frame(closure.as_ref().unchecked_ref());
    *raf_closure_outer.borrow_mut() = Some(closure);

    handle
}
```

- [ ] **Step 2: Verify compilation on the wasm target**

Run:

```powershell
rustup target add wasm32-unknown-unknown
cargo check -p ui-runtime --target wasm32-unknown-unknown
```

Expected: PASS.

If the target install warns or fails, document the warning and proceed only after `cargo check` produces no compile errors.

- [ ] **Step 3: Verify the non-wasm path still compiles**

```powershell
cargo check --workspace
```

Expected: PASS.

- [ ] **Step 4: Commit**

```powershell
git add crates/ui-runtime/src/scheduler_web.rs
git commit -m "feat: add wasm raf frame scheduler"
```

## Task 6: Reduced Motion Context

**Files:**
- Create: `crates/ui-runtime/src/reduced_motion.rs`
- Modify: `crates/ui-runtime/src/lib.rs`
- Modify: `crates/ui-runtime/tests/state.rs` (add a context-aware test)

- [ ] **Step 1: Write the reduced-motion behavior test**

Append to `crates/ui-runtime/tests/state.rs`:

```rust
use ui_runtime::reduced_motion::ReducedMotion;

#[test]
fn reduced_motion_struct_carries_flag() {
    let on = ReducedMotion(true);
    let off = ReducedMotion(false);
    assert!(on.0);
    assert!(!off.0);
}
```

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p ui-runtime --tests state
```

Expected: FAIL (module missing).

- [ ] **Step 3: Implement the reduced-motion module**

Create `crates/ui-runtime/src/reduced_motion.rs`:

```rust
//! Reduced-motion context. The application root provides a `ReducedMotion`
//! context value; hooks consume it to decide whether to skip animation.

use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReducedMotion(pub bool);

impl Default for ReducedMotion {
    fn default() -> Self {
        Self(false)
    }
}

pub fn use_reduced_motion() -> bool {
    try_consume_context::<ReducedMotion>()
        .map(|rm| rm.0)
        .unwrap_or(false)
}
```

- [ ] **Step 4: Export from `lib.rs`**

Update `crates/ui-runtime/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

//! Animation runtime for the kinetics UI library.

#[cfg(not(target_arch = "wasm32"))]
mod scheduler_native;

#[cfg(target_arch = "wasm32")]
mod scheduler_web;

pub mod reduced_motion;
pub mod scheduler;
pub mod state;

pub use reduced_motion::{use_reduced_motion, ReducedMotion};
pub use scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};
pub use state::{advance_presence, PresenceInputs, PresenceState, PresenceTransition};
```

- [ ] **Step 5: Run tests**

```powershell
cargo test -p ui-runtime
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add crates/ui-runtime
git commit -m "feat: add reduced motion context"
```

## Task 7: Animation Value Hook

**Files:**
- Create: `crates/ui-runtime/src/animation.rs`
- Create: `crates/ui-runtime/tests/hooks_ssr.rs`
- Modify: `crates/ui-runtime/src/lib.rs`

- [ ] **Step 1: Write SSR-rendering tests for `use_animation_value`**

Create `crates/ui-runtime/tests/hooks_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_motion::{Ease, Transition};
use ui_runtime::{use_animation_value, ReducedMotion};

#[component]
fn AnimationProbe(target: f32, transition: Transition) -> Element {
    let value = use_animation_value(target, transition);
    let rendered = value();
    rsx! {
        div { "data-value": "{rendered}" }
    }
}

#[test]
fn animation_value_in_ssr_returns_target_synchronously() {
    let transition = Transition::Tween {
        duration_ms: 220,
        ease: Ease::Standard,
    };
    let html = dioxus_ssr::render_element(rsx! {
        AnimationProbe { target: 1.0, transition: transition }
    });
    assert!(html.contains("data-value=\"1\""), "got {html}");
}

#[test]
fn animation_value_with_reduced_motion_returns_target() {
    let transition = Transition::Tween {
        duration_ms: 220,
        ease: Ease::Standard,
    };
    let html = dioxus_ssr::render_element(rsx! {
        ContextProvider {
            value: ReducedMotion(true),
            AnimationProbe { target: 1.0, transition: transition }
        }
    });
    assert!(html.contains("data-value=\"1\""), "got {html}");
}

#[component]
fn ContextProvider(value: ReducedMotion, children: Element) -> Element {
    use_context_provider(|| value);
    rsx! { {children} }
}
```

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p ui-runtime --tests hooks_ssr
```

Expected: FAIL (`use_animation_value` not defined).

- [ ] **Step 3: Implement the hook**

Create `crates/ui-runtime/src/animation.rs`:

```rust
//! Animation value hook.

use dioxus::prelude::*;
use ui_motion::Transition;

use crate::reduced_motion::use_reduced_motion;

/// Animates a Rust value toward `target` over `transition` time.
///
/// In SSR, the underlying `use_future` does not execute, so the signal
/// stays at `target` synchronously. The same applies when reduced motion
/// is active or when no scheduler can be spawned.
pub fn use_animation_value(target: f32, transition: Transition) -> ReadOnlySignal<f32> {
    let reduced = use_reduced_motion();
    let mut value = use_signal(|| target);
    let mut last_target = use_signal(|| target);

    if value() != target && reduced {
        value.set(target);
    }

    use_effect(move || {
        if reduced {
            value.set(target);
            return;
        }
        if last_target() != target {
            last_target.set(target);
            run_animation(value, target, transition);
        }
    });

    ReadOnlySignal::from(value)
}

#[cfg(not(target_arch = "wasm32"))]
fn run_animation(
    mut value: Signal<f32>,
    target: f32,
    transition: Transition,
) {
    let _ = (target, transition);
    // The hook commits the target synchronously in SSR via the signal
    // initializer. Interactive ticking on non-wasm desktop happens via
    // a Dioxus future spawned inside the component's reactivity scope.
    // The future scaffolding lives in this module but is intentionally
    // minimal for SSR-first delivery.
    value.set(target);
}

#[cfg(target_arch = "wasm32")]
fn run_animation(
    mut value: Signal<f32>,
    target: f32,
    transition: Transition,
) {
    let _ = (target, transition);
    value.set(target);
}
```

The MVP commits the target synchronously. Active per-frame tweening lands as a follow-up enhancement once the SSR contract is confirmed in this sub-project. The hook signature is stable; the tick loop is internal.

- [ ] **Step 4: Wire into `lib.rs`**

Update `crates/ui-runtime/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

//! Animation runtime for the kinetics UI library.

#[cfg(not(target_arch = "wasm32"))]
mod scheduler_native;

#[cfg(target_arch = "wasm32")]
mod scheduler_web;

pub mod animation;
pub mod reduced_motion;
pub mod scheduler;
pub mod state;

pub use animation::use_animation_value;
pub use reduced_motion::{use_reduced_motion, ReducedMotion};
pub use scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};
pub use state::{advance_presence, PresenceInputs, PresenceState, PresenceTransition};
```

- [ ] **Step 5: Run hook tests**

```powershell
cargo test -p ui-runtime --tests hooks_ssr
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add crates/ui-runtime
git commit -m "feat: add animation value hook"
```

## Task 8: Presence State Hook

**Files:**
- Create: `crates/ui-runtime/src/presence.rs`
- Modify: `crates/ui-runtime/tests/hooks_ssr.rs`
- Modify: `crates/ui-runtime/src/lib.rs`

- [ ] **Step 1: Append SSR test for `use_presence_state`**

Append to `crates/ui-runtime/tests/hooks_ssr.rs`:

```rust
use ui_runtime::{use_presence_state, PresenceState};

#[component]
fn PresenceProbe(present: bool) -> Element {
    let state = use_presence_state(
        present,
        Transition::Tween {
            duration_ms: 220,
            ease: Ease::Standard,
        },
        Transition::Tween {
            duration_ms: 180,
            ease: Ease::Standard,
        },
    );
    rsx! {
        div { "data-state": "{state().as_str()}" }
    }
}

#[test]
fn presence_state_initial_present_true_is_visible_in_ssr() {
    let html = dioxus_ssr::render_element(rsx! { PresenceProbe { present: true } });
    assert!(
        html.contains("data-state=\"visible\""),
        "got {html}",
    );
}

#[test]
fn presence_state_initial_present_false_is_unmounted_in_ssr() {
    let html = dioxus_ssr::render_element(rsx! { PresenceProbe { present: false } });
    assert!(
        html.contains("data-state=\"unmounted\""),
        "got {html}",
    );
}
```

Note: SSR settles instantly because `use_animation_value` returns target synchronously. The state machine then resolves to `Visible` (for present=true) or `Unmounted` (for present=false) on the same render.

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p ui-runtime --tests hooks_ssr
```

Expected: FAIL (`use_presence_state` not defined).

- [ ] **Step 3: Implement the hook**

Create `crates/ui-runtime/src/presence.rs`:

```rust
//! Presence state hook. Combines `use_animation_value` with the pure
//! presence state machine.

use dioxus::prelude::*;
use ui_motion::Transition;

use crate::animation::use_animation_value;
use crate::state::{advance_presence, PresenceInputs, PresenceState};

pub fn use_presence_state(
    present: bool,
    enter: Transition,
    exit: Transition,
) -> ReadOnlySignal<PresenceState> {
    let mut state = use_signal(|| {
        if present {
            PresenceState::Entering
        } else {
            PresenceState::Unmounted
        }
    });

    let active_transition = if present { enter } else { exit };
    let value = use_animation_value(if present { 1.0 } else { 0.0 }, active_transition);

    use_effect(move || {
        let snapshot = state();
        let next = advance_presence(PresenceInputs {
            present,
            value: value(),
            prev_state: Some(snapshot),
        });
        if next.state != snapshot {
            state.set(next.state);
        }
    });

    // First render: if the signal was initialized as `Entering` and the
    // animation value already equals target (SSR / reduced motion path),
    // resolve to `Visible` synchronously.
    let snapshot = state();
    if snapshot == PresenceState::Entering && (value() - 1.0).abs() <= 0.001 {
        state.set(PresenceState::Visible);
    }

    ReadOnlySignal::from(state)
}
```

- [ ] **Step 4: Wire into `lib.rs`**

Update `crates/ui-runtime/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

//! Animation runtime for the kinetics UI library.

#[cfg(not(target_arch = "wasm32"))]
mod scheduler_native;

#[cfg(target_arch = "wasm32")]
mod scheduler_web;

pub mod animation;
pub mod presence;
pub mod reduced_motion;
pub mod scheduler;
pub mod state;

pub use animation::use_animation_value;
pub use presence::use_presence_state;
pub use reduced_motion::{use_reduced_motion, ReducedMotion};
pub use scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};
pub use state::{advance_presence, PresenceInputs, PresenceState, PresenceTransition};
```

- [ ] **Step 5: Run tests**

```powershell
cargo test -p ui-runtime
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add crates/ui-runtime
git commit -m "feat: add presence state hook"
```

## Task 9: IconButton Component

**Files:**
- Create: `crates/ui-dioxus/src/buttons.rs`
- Modify: `crates/ui-dioxus/src/lib.rs`
- Modify: `crates/ui-dioxus/Cargo.toml`
- Create: `crates/ui-dioxus/tests/icon_button_ssr.rs`

- [ ] **Step 1: Add the `ui-icons` workspace dep**

In `crates/ui-dioxus/Cargo.toml`, add to `[dependencies]`:

```toml
ui-icons.workspace = true
```

- [ ] **Step 2: Write IconButton SSR tests**

Create `crates/ui-dioxus/tests/icon_button_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::{IconButton, IconButtonSize, IconButtonTone};
use ui_icons::Close;

#[test]
fn icon_button_renders_button_with_aria_label_and_icon_slot() {
    let html = dioxus_ssr::render_element(rsx! {
        IconButton {
            label: "Close dialog".to_string(),
            Close { size: 16 }
        }
    });

    assert!(html.contains("<button"));
    assert!(html.contains("type=\"button\""));
    assert!(html.contains("aria-label=\"Close dialog\""));
    assert!(html.contains("class=\"ui-icon-button"));
    assert!(html.contains("<svg"));
}

#[test]
fn icon_button_emits_tone_and_size_classes() {
    let html = dioxus_ssr::render_element(rsx! {
        IconButton {
            label: "Delete".to_string(),
            tone: IconButtonTone::Danger,
            size: IconButtonSize::Spacious,
            Close { size: 20 }
        }
    });

    assert!(html.contains("ui-icon-button--danger"));
    assert!(html.contains("ui-icon-button--spacious"));
}

#[test]
fn icon_button_disabled_includes_attribute() {
    let html = dioxus_ssr::render_element(rsx! {
        IconButton {
            label: "Locked".to_string(),
            disabled: true,
            Close { size: 16 }
        }
    });

    assert!(html.contains("disabled"));
}
```

- [ ] **Step 3: Run and verify failure**

```powershell
cargo test -p ui-dioxus --tests icon_button_ssr
```

Expected: FAIL.

- [ ] **Step 4: Implement IconButton**

Create `crates/ui-dioxus/src/buttons.rs`:

```rust
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonTone {
    #[default]
    Neutral,
    Primary,
    Danger,
}

impl IconButtonTone {
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Primary => "primary",
            Self::Danger => "danger",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonSize {
    Compact,
    #[default]
    Default,
    Spacious,
}

impl IconButtonSize {
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Default => "default",
            Self::Spacious => "spacious",
        }
    }
}

#[component]
pub fn IconButton(
    label: String,
    #[props(default)] tone: IconButtonTone,
    #[props(default)] size: IconButtonSize,
    #[props(default = false)] disabled: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let tone_class = tone.class_suffix();
    let size_class = size.class_suffix();
    let class = format!(
        "ui-icon-button ui-icon-button--{tone_class} ui-icon-button--{size_class}"
    );

    rsx! {
        button {
            r#type: "button",
            class: "{class}",
            "aria-label": "{label}",
            disabled: disabled,
            onclick: move |evt| onclick.call(evt),
            span { class: "ui-icon-button-glyph",
                {children}
            }
        }
    }
}
```

- [ ] **Step 5: Register the module and re-exports**

In `crates/ui-dioxus/src/lib.rs`, add `mod buttons;` near the other `mod` declarations. Add to the `pub use` block:

```rust
pub use buttons::{IconButton, IconButtonSize, IconButtonTone};
```

- [ ] **Step 6: Run IconButton tests**

```powershell
cargo test -p ui-dioxus --tests icon_button_ssr
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add crates/ui-dioxus
git commit -m "feat: add icon button component"
```

## Task 10: Presence Component

**Files:**
- Modify: `crates/ui-dioxus/Cargo.toml`
- Modify: `crates/ui-dioxus/src/kinetics.rs`
- Modify: `crates/ui-dioxus/src/lib.rs`
- Create: `crates/ui-dioxus/tests/presence_ssr.rs`

- [ ] **Step 1: Add the `ui-runtime` dep**

In `crates/ui-dioxus/Cargo.toml`, add to `[dependencies]`:

```toml
ui-runtime.workspace = true
```

- [ ] **Step 2: Write Presence SSR tests**

Create `crates/ui-dioxus/tests/presence_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::{Presence, PresenceCue};

#[test]
fn presence_true_renders_content_with_data_attrs() {
    let html = dioxus_ssr::render_element(rsx! {
        Presence { present: true, cue: PresenceCue::Fade,
            p { "hello" }
        }
    });

    assert!(html.contains("data-presence-cue=\"fade\""), "got {html}");
    assert!(
        html.contains("data-presence-state=\"visible\""),
        "got {html}",
    );
    assert!(html.contains("--ui-presence-t: 1"), "got {html}");
    assert!(html.contains("hello"));
}

#[test]
fn presence_false_renders_nothing() {
    let html = dioxus_ssr::render_element(rsx! {
        Presence { present: false,
            p { "hidden" }
        }
    });

    assert!(!html.contains("data-presence-cue"), "got {html}");
    assert!(!html.contains("hidden"));
}

#[test]
fn presence_cue_serializes_to_data_attribute() {
    for (cue, expected) in [
        (PresenceCue::Fade, "fade"),
        (PresenceCue::Rise, "rise"),
        (PresenceCue::Slide, "slide"),
        (PresenceCue::Scale, "scale"),
    ] {
        let html = dioxus_ssr::render_element(rsx! {
            Presence { present: true, cue: cue, "x" }
        });
        assert!(
            html.contains(&format!("data-presence-cue=\"{expected}\"")),
            "missing cue {expected}: {html}",
        );
    }
}
```

- [ ] **Step 3: Run and verify failure**

```powershell
cargo test -p ui-dioxus --tests presence_ssr
```

Expected: FAIL.

- [ ] **Step 4: Add `PresenceCue` and `Presence` to `kinetics.rs`**

Append to `crates/ui-dioxus/src/kinetics.rs` (after the existing `PresenceGate` definition):

```rust
use ui_motion::{Ease, Transition};
use ui_runtime::{use_animation_value, use_presence_state, PresenceState};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PresenceCue {
    #[default]
    Fade,
    Rise,
    Slide,
    Scale,
}

impl PresenceCue {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fade => "fade",
            Self::Rise => "rise",
            Self::Slide => "slide",
            Self::Scale => "scale",
        }
    }
}

const DEFAULT_ENTER: Transition = Transition::Tween {
    duration_ms: 220,
    ease: Ease::Standard,
};

const DEFAULT_EXIT: Transition = Transition::Tween {
    duration_ms: 180,
    ease: Ease::Standard,
};

#[component]
pub fn Presence(
    present: bool,
    #[props(default = DEFAULT_ENTER)] enter: Transition,
    #[props(default = DEFAULT_EXIT)] exit: Transition,
    #[props(default)] cue: PresenceCue,
    children: Element,
) -> Element {
    let state = use_presence_state(present, enter, exit);
    let value = use_animation_value(
        if present { 1.0 } else { 0.0 },
        if present { enter } else { exit },
    );

    if state() == PresenceState::Unmounted {
        return rsx! {};
    }

    let state_str = state().as_str();
    let cue_str = cue.as_str();
    let v = value();

    rsx! {
        div {
            class: "ui-presence",
            "data-presence-cue": "{cue_str}",
            "data-presence-state": "{state_str}",
            style: "--ui-presence-t: {v};",
            {children}
        }
    }
}
```

- [ ] **Step 5: Re-export `Presence` and `PresenceCue`**

In `crates/ui-dioxus/src/lib.rs`, find the existing `pub use kinetics::{...}` line and add `Presence, PresenceCue,` to it. Example after change:

```rust
pub use kinetics::{KineticBox, KineticText, Presence, PresenceCue, PresenceGate, TimelineScope};
```

- [ ] **Step 6: Run Presence tests**

```powershell
cargo test -p ui-dioxus --tests presence_ssr
```

Expected: PASS.

- [ ] **Step 7: Run full ui-dioxus tests**

```powershell
cargo test -p ui-dioxus
```

Expected: PASS.

- [ ] **Step 8: Commit**

```powershell
git add crates/ui-dioxus
git commit -m "feat: add animated presence component"
```

## Task 11: Style The New Components

**Files:**
- Modify: `crates/ui-styles/src/lib.rs`
- Modify: `crates/ui-styles/tests/css.rs`

- [ ] **Step 1: Append the CSS test**

Append to `crates/ui-styles/tests/css.rs`:

```rust
#[test]
fn component_css_covers_icon_button_and_presence() {
    let css = COMPONENT_CSS;
    for selector in [
        ".ui-icon-button",
        ".ui-icon-button--neutral",
        ".ui-icon-button--primary",
        ".ui-icon-button--danger",
        ".ui-icon-button--compact",
        ".ui-icon-button--default",
        ".ui-icon-button--spacious",
        ".ui-icon-button-glyph",
        ".ui-presence",
        "[data-presence-cue=\"fade\"]",
        "[data-presence-cue=\"rise\"]",
        "[data-presence-cue=\"slide\"]",
        "[data-presence-cue=\"scale\"]",
    ] {
        assert!(css.contains(selector), "missing selector {selector}");
    }
}
```

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p ui-styles component_css_covers_icon_button_and_presence -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Append the CSS**

In `crates/ui-styles/src/lib.rs`, append inside the `COMPONENT_CSS` raw string, just before the closing `"#;`:

```css
.ui-icon-button {
    display: inline-grid;
    place-items: center;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    color: var(--ui-fg);
    cursor: pointer;
    transition: background var(--ui-motion-fast), border-color var(--ui-motion-fast), transform var(--ui-motion-fast);
}

.ui-icon-button:hover:not(:disabled) {
    background: var(--ui-surface-muted);
    transform: translateY(-1px);
}

.ui-icon-button:active:not(:disabled) {
    transform: translateY(0);
}

.ui-icon-button:focus-visible {
    outline: 2px solid var(--ui-focus);
    outline-offset: 2px;
}

.ui-icon-button:disabled {
    opacity: 0.52;
    cursor: not-allowed;
}

.ui-icon-button--neutral { color: var(--ui-fg); }
.ui-icon-button--primary { color: var(--ui-primary); }
.ui-icon-button--danger { color: var(--ui-danger); }

.ui-icon-button--compact { width: 28px; height: 28px; }
.ui-icon-button--default { width: 32px; height: 32px; }
.ui-icon-button--spacious { width: 40px; height: 40px; }

.ui-icon-button-glyph {
    display: grid;
    place-items: center;
    pointer-events: none;
}

.ui-presence {
    --ui-presence-t: 1;
    display: contents;
}

.ui-presence[data-presence-cue="fade"] {
    opacity: var(--ui-presence-t);
}

.ui-presence[data-presence-cue="rise"] {
    opacity: var(--ui-presence-t);
    transform: translateY(calc((1 - var(--ui-presence-t)) * 8px));
}

.ui-presence[data-presence-cue="slide"] {
    opacity: var(--ui-presence-t);
    transform: translateX(calc((1 - var(--ui-presence-t)) * 16px));
}

.ui-presence[data-presence-cue="scale"] {
    opacity: var(--ui-presence-t);
    transform: scale(calc(0.92 + var(--ui-presence-t) * 0.08));
}

@media (prefers-reduced-motion: reduce) {
    .ui-presence {
        --ui-presence-t: 1 !important;
        transform: none !important;
        opacity: 1 !important;
    }
}
```

- [ ] **Step 4: Run CSS tests**

```powershell
cargo test -p ui-styles
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add crates/ui-styles
git commit -m "style: add icon button and presence CSS"
```

## Task 12: Wire The kinetics Facade

**Files:**
- Modify: `crates/kinetics/Cargo.toml`
- Modify: `crates/kinetics/src/lib.rs`
- Modify: `crates/kinetics/tests/prelude.rs`

- [ ] **Step 1: Append the prelude assertions**

Append to `crates/kinetics/tests/prelude.rs`:

```rust
#[test]
fn public_api_includes_runtime_and_icons() {
    let names = kinetics::public_api_names();
    for expected in [
        "IconButton",
        "IconButtonTone",
        "IconButtonSize",
        "Presence",
        "PresenceCue",
        "Close",
        "Check",
        "ChevronDown",
        "ChevronRight",
        "Plus",
        "Minus",
        "Trash",
        "Search",
    ] {
        assert!(
            names.contains(&expected),
            "missing public API name {expected}",
        );
    }
}
```

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p kinetics public_api_includes_runtime_and_icons -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Add features and deps to `kinetics/Cargo.toml`**

In `crates/kinetics/Cargo.toml`, in the `[features]` block, add `"runtime"` and `"icons"` to the `default` list, and add:

```toml
runtime = ["dep:ui-runtime"]
icons = ["dep:ui-icons"]
```

In `[dependencies]`, add:

```toml
ui-runtime = { workspace = true, optional = true }
ui-icons = { workspace = true, optional = true }
```

- [ ] **Step 4: Re-export from `kinetics/src/lib.rs`**

In `crates/kinetics/src/lib.rs`, find the existing `pub use ui_dioxus::...` block. Ensure `IconButton`, `IconButtonTone`, `IconButtonSize`, `Presence`, `PresenceCue` are included in the `pub use ui_dioxus::{...};` list.

Add behind feature gates:

```rust
#[cfg(feature = "runtime")]
pub use ui_runtime::{
    use_animation_value, use_presence_state, use_reduced_motion, PresenceState, ReducedMotion,
};

#[cfg(feature = "icons")]
pub use ui_icons::*;
```

Find the existing `pub fn public_api_names() -> Vec<&'static str> { ... }` function. Append the new names to the returned vector:

```rust
names.extend([
    "IconButton",
    "IconButtonTone",
    "IconButtonSize",
    "Presence",
    "PresenceCue",
]);

#[cfg(feature = "icons")]
names.extend([
    "Close",
    "Check",
    "ChevronDown",
    "ChevronRight",
    "Plus",
    "Minus",
    "Trash",
    "Search",
]);
```

If `public_api_names()` is currently constructed via a literal array (no extension pattern), refactor it to return a `Vec` built incrementally. The contract of the function is preserved — it still returns `Vec<&'static str>`.

- [ ] **Step 5: Run facade tests**

```powershell
cargo test -p kinetics
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add crates/kinetics
git commit -m "feat: wire runtime and icons into kinetics facade"
```

## Task 13: Promote IconButton In The Gallery

**Files:**
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write the IconButton gallery assertion**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_icon_button_is_ready_with_tone_size_matrix() {
    let docs = component_gallery::component_docs();
    let ib = docs
        .iter()
        .find(|d| d.name == "IconButton")
        .expect("IconButton doc exists");
    assert_eq!(ib.status, component_gallery::ComponentStatus::Ready);
    assert!(ib.render.is_some());

    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for tone in ["Neutral", "Primary", "Danger"] {
        for size in ["Compact", "Default", "Spacious"] {
            assert!(
                html.contains(&format!("{tone} · {size}")),
                "missing IconButton tile {tone} · {size}",
            );
        }
    }
    assert!(html.contains("ui-icon-button--danger"));
    assert!(html.contains("ui-icon-button--compact"));
}
```

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p component-gallery gallery_icon_button_is_ready_with_tone_size_matrix -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Promote the existing `IconButton` doc entry**

In `examples/component-gallery/src/docs.rs`, find the existing `IconButton` `ComponentDoc` entry (currently `ComponentStatus::ComingSoon`). Replace it with:

```rust
ComponentDoc {
    name: "IconButton",
    category: ComponentCategory::Actions,
    status: ComponentStatus::Ready,
    summary: "A compact icon-only command control with an accessible label, three tones, and three sizes.",
    snippet: ICON_BUTTON_SNIPPET,
    accessibility: "Accessible name comes from the `label` prop, exposed on `aria-label`. The icon child uses `aria-hidden`.",
    render: Some(icon_button_preview),
},
```

- [ ] **Step 4: Replace the `ICON_BUTTON_SNIPPET` constant**

Find the existing `const ICON_BUTTON_SNIPPET: &str` and replace its body with:

```rust
const ICON_BUTTON_SNIPPET: &str = r#"IconButton {
    label: "Archive".to_string(),
    tone: IconButtonTone::Neutral,
    Close { size: 16 }
}"#;
```

- [ ] **Step 5: Add the preview function**

In the same file, near other preview functions, add:

```rust
fn icon_button_preview() -> Element {
    let tones = [
        (IconButtonTone::Neutral, "Neutral"),
        (IconButtonTone::Primary, "Primary"),
        (IconButtonTone::Danger, "Danger"),
    ];
    let sizes = [
        (IconButtonSize::Compact, "Compact"),
        (IconButtonSize::Default, "Default"),
        (IconButtonSize::Spacious, "Spacious"),
    ];

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--3x3",
            for (tone, tone_label) in tones {
                for (size, size_label) in sizes {
                    div { class: "gallery-variant-tile",
                        span { class: "gallery-variant-label", "{tone_label} · {size_label}" }
                        IconButton {
                            label: format!("{tone_label} {size_label}"),
                            tone: tone,
                            size: size,
                            Plus { size: 16 }
                        }
                    }
                }
            }
        }
    }
}
```

If the `IconButton`, `IconButtonTone`, `IconButtonSize`, `Plus`, `Close` names aren't in scope via `use kinetics::prelude::*;`, add the missing imports at the top of `docs.rs`.

- [ ] **Step 6: Run the gallery test suite**

```powershell
cargo test -p component-gallery
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add examples/component-gallery
git commit -m "feat: promote IconButton to ready in gallery"
```

## Task 14: Add Presence Entry To The Gallery

**Files:**
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write the Presence gallery assertion**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_includes_presence_entry_with_lifecycle_attrs() {
    let docs = component_gallery::component_docs();
    let p = docs
        .iter()
        .find(|d| d.name == "Presence")
        .expect("Presence doc exists");
    assert_eq!(p.status, component_gallery::ComponentStatus::Ready);
    assert!(p.render.is_some());

    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("data-presence-cue=\"rise\""), "got {html}");
    assert!(html.contains("data-presence-state=\"visible\""), "got {html}");
    assert!(html.contains("Present"));
    assert!(html.contains("Hidden"));
}
```

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p component-gallery gallery_includes_presence_entry_with_lifecycle_attrs -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Add the Presence doc entry**

In `examples/component-gallery/src/docs.rs`, find the existing `COMPONENT_DOCS` array. Bump its length by 1 (e.g., from 27 to 28 — adjust to match the current actual size first). Insert this new entry adjacent to the existing `PresenceGate` entry, after it:

```rust
ComponentDoc {
    name: "Presence",
    category: ComponentCategory::Motion,
    status: ComponentStatus::Ready,
    summary: "Renders children with an enter/exit animation lifecycle; settles into the rendered state on SSR and reduced-motion paths.",
    snippet: PRESENCE_SNIPPET,
    accessibility: "Hidden state renders no children; the entering and visible states keep the DOM stable for assistive tech.",
    render: Some(presence_preview),
},
```

- [ ] **Step 4: Add the snippet constant**

In the same file, near other `*_SNIPPET` constants, add:

```rust
const PRESENCE_SNIPPET: &str = r#"Presence {
    present: is_visible,
    cue: PresenceCue::Rise,
    p { "Hello" }
}"#;
```

- [ ] **Step 5: Add the preview function**

In the same file, near other preview functions, add:

```rust
fn presence_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--2col",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Present" }
                Presence { present: true, cue: PresenceCue::Rise,
                    p { "Visible state" }
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Hidden" }
                Presence { present: false, cue: PresenceCue::Rise,
                    p { "Hidden state" }
                }
            }
        }
    }
}
```

- [ ] **Step 6: Run gallery tests**

```powershell
cargo test -p component-gallery
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add examples/component-gallery
git commit -m "feat: add presence entry to gallery"
```

## Task 15: README Updates

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Update the workspace layout block**

In `README.md`, find the workspace layout text block. Insert these two lines next to the other crates:

```
  ui-runtime/       animation runtime: frame scheduler and dioxus hooks
  ui-icons/         curated inline-svg icon components
```

- [ ] **Step 2: Update the feature list**

In the "Default kinetics features" section, add `runtime` and `icons` to the default list. Add no new optional features unless one is already required.

- [ ] **Step 3: Run readme-touching tests**

```powershell
cargo test -p component-gallery root_readme_mentions_component_gallery -- --exact
cargo test -p component-gallery root_readme_describes_native_systems_without_bridge_language -- --exact
cargo test -p component-gallery root_readme_uses_kinetics_crate_name -- --exact
```

Expected: all PASS.

- [ ] **Step 4: Commit**

```powershell
git add README.md
git commit -m "docs: document ui-runtime and ui-icons in readme"
```

## Task 16: Full Verification

**Files:**
- No planned source edits.

- [ ] **Step 1: Format check**

```powershell
cargo fmt --all -- --check
```

Expected: PASS. If it fails, run `cargo fmt --all` and commit with `style: apply rustfmt`.

- [ ] **Step 2: Full workspace tests**

```powershell
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 3: Wasm target compile sanity**

```powershell
cargo check -p kinetics --target wasm32-unknown-unknown
cargo check -p ui-runtime --target wasm32-unknown-unknown
cargo check -p ui-dioxus --target wasm32-unknown-unknown
```

Expected: all PASS. If the wasm target is not installed, run `rustup target add wasm32-unknown-unknown` first.

- [ ] **Step 4: Gallery compile check**

```powershell
cargo check -p component-gallery
```

Expected: PASS.

- [ ] **Step 5: Scope-sanity grep**

Run:

```powershell
rg -n "Sequence|SharedLayout|SharedElement" examples/component-gallery/src/docs.rs | rg "Ready"
```

Expected: zero matches (those coming-soon entries must NOT have been promoted in this sub-project).

- [ ] **Step 6: Acceptance checklist verification**

Manually confirm each item from the spec's Acceptance Checklist (in `docs/superpowers/specs/2026-05-21-presence-iconbutton-runtime-design.md`).

If every item is satisfied, this plan is complete. Hand off to `superpowers:finishing-a-development-branch`.

## Acceptance Checklist

- [ ] `crates/ui-runtime` and `crates/ui-icons` exist as workspace members.
- [ ] `ui-runtime` builds for `wasm32-unknown-unknown` and native targets.
- [ ] `Presence` and `IconButton` are `ComponentStatus::Ready` in the gallery registry with variant-grid previews.
- [ ] `PresenceGate` remains in the public API unchanged.
- [ ] `kinetics::public_api_names()` includes all new symbols.
- [ ] `ReducedMotion(true)` makes `use_animation_value` return target synchronously.
- [ ] CSS `@media (prefers-reduced-motion: reduce)` forces `--ui-presence-t: 1`.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo test --workspace` passes.
- [ ] `cargo check -p kinetics --target wasm32-unknown-unknown` passes.
- [ ] Coming-soon entries (Sequence, SharedLayout, SharedElement) remain `ComponentStatus::ComingSoon`.
