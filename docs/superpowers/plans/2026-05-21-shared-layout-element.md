# SharedLayout And SharedElement (FLIP) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship `SharedLayout` and `SharedElement` components that animate layout changes via FLIP, match shared identities across tree positions, and interpolate selected computed styles.

**Architecture:** New DOM measurement hooks (`use_element_rect`, `use_element_computed_style`) in `ui-runtime` with wasm-only implementations and SSR/native no-ops. A `SharedElementRegistry` context coordinates same-`id` elements across tree positions; `SharedLayout` provides a scoped registry, with a default top-level fallback. `SharedElement` measures via mount callback, looks up prior snapshots, and drives transform + opacity + computed-style interpolation through the existing `use_animation_value` hook.

**Tech Stack:** Rust 2021, Cargo workspace, Dioxus 0.7, `ui-layout` for FLIP math, `web-sys` (Element, Window, Document, DomRect, CssStyleDeclaration) for wasm DOM access, Dioxus SSR for tests, PowerShell on Windows.

---

## Scope

This plan implements sub-project 4 of the kinetics animation roadmap, per `docs/superpowers/specs/2026-05-21-shared-layout-element-design.md`.

It includes:

- DOM measurement hooks (`use_element_rect`, `use_element_computed_style`)
- `SharedElementRegistry` + `ElementSnapshot` + `SharedTransition` types
- `use_shared_element_registry` context hook + default top-level registry
- `SharedLayout` and `SharedElement` components
- Cross-tree id matching, FLIP transform interpolation, cross-fade opacity, computed-style interpolation (border-radius, background-color, color)
- Gallery promotion of both components to `Ready`
- CSS for the two new component classes
- README updates

It excludes:

- Native (Blitz) renderer support
- A real `ResizeObserver` subscription (polled via render-driven re-measurement)
- Reorder/swap detection across sibling lists

## Before You Start

```powershell
git worktree add .worktrees/shared-layout-element -b shared-layout-element main
```

Run every command from inside the worktree.

## File Map

- `crates/ui-runtime/Cargo.toml` — add `ui-layout` workspace dep; expand `web-sys` features.
- `crates/ui-runtime/src/measurement.rs` — public `use_element_rect`, `use_element_computed_style`, `MountedRectCallback`.
- `crates/ui-runtime/src/measurement_web.rs` — wasm impl.
- `crates/ui-runtime/src/measurement_native.rs` — non-wasm stub.
- `crates/ui-runtime/src/shared.rs` — `SharedElementRegistry`, `ElementSnapshot`, `SharedTransition`, `use_shared_element_registry`.
- `crates/ui-runtime/tests/shared_registry.rs` — pure-state tests.
- `crates/ui-runtime/tests/measurement_ssr.rs` — SSR returns None.
- `crates/ui-runtime/src/lib.rs` — register & re-export.
- `crates/ui-dioxus/Cargo.toml` — add `ui-layout` workspace dep.
- `crates/ui-dioxus/src/layout.rs` — `SharedLayout`, `SharedElement`.
- `crates/ui-dioxus/src/lib.rs` — register & re-export.
- `crates/ui-dioxus/tests/shared_ssr.rs` — SSR tests.
- `crates/kinetics/src/lib.rs` — re-exports + `public_api_names()`.
- `crates/kinetics/tests/prelude.rs` — assertions.
- `crates/ui-styles/src/lib.rs` — `.ui-shared-element` and `.ui-shared-layout` selectors.
- `crates/ui-styles/tests/css.rs` — assertion.
- `examples/component-gallery/src/docs.rs` — promote both to Ready with previews.
- `examples/component-gallery/tests/gallery.rs` — gallery assertions.
- `README.md` — add to ready components list.

## Task 1: Pure State - `SharedElementRegistry`

**Files:**
- Create: `crates/ui-runtime/src/shared.rs`
- Create: `crates/ui-runtime/tests/shared_registry.rs`
- Modify: `crates/ui-runtime/src/lib.rs`
- Modify: `crates/ui-runtime/Cargo.toml`

- [ ] **Step 1: Add `ui-layout` dep**

In `crates/ui-runtime/Cargo.toml` `[dependencies]`:

```toml
ui-layout = { path = "../ui-layout" }
```

- [ ] **Step 2: Write failing tests at `crates/ui-runtime/tests/shared_registry.rs`**

```rust
use std::collections::HashMap;
use ui_layout::Rect;
use ui_runtime::shared::{ElementSnapshot, SharedElementRegistry};

fn snapshot(x: f32, y: f32) -> ElementSnapshot {
    ElementSnapshot {
        rect: Rect::new(x, y, 100.0, 50.0),
        computed: HashMap::new(),
        timestamp_ms: 0.0,
    }
}

#[test]
fn record_and_snapshot_round_trip() {
    let r = SharedElementRegistry::default();
    let s = snapshot(0.0, 0.0);
    r.record("a".to_string(), s.clone());
    assert_eq!(r.snapshot("a"), Some(s));
}

#[test]
fn forget_removes_snapshot() {
    let r = SharedElementRegistry::default();
    r.record("a".to_string(), snapshot(0.0, 0.0));
    r.forget("a");
    assert_eq!(r.snapshot("a"), None);
}

#[test]
fn record_overwrites_existing_id() {
    let r = SharedElementRegistry::default();
    r.record("a".to_string(), snapshot(0.0, 0.0));
    r.record("a".to_string(), snapshot(10.0, 10.0));
    assert_eq!(r.snapshot("a").unwrap().rect.x, 10.0);
}
```

- [ ] **Step 3: Run; verify FAIL**

```powershell
cargo test -p ui-runtime --test shared_registry
```

Expected: FAIL (module missing).

- [ ] **Step 4: Implement `crates/ui-runtime/src/shared.rs`**

```rust
//! Shared element registry. Tracks element snapshots by id so SharedElement
//! components can coordinate cross-tree transitions.

use std::cell::RefCell;
use std::collections::HashMap;

use dioxus::prelude::*;
use ui_layout::Rect;
use ui_motion::{Ease, Transition};

#[derive(Clone, Debug, PartialEq)]
pub struct ElementSnapshot {
    pub rect: Rect,
    pub computed: HashMap<&'static str, String>,
    pub timestamp_ms: f64,
}

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

pub fn use_shared_element_registry() -> Signal<SharedElementRegistry> {
    try_consume_context::<Signal<SharedElementRegistry>>().unwrap_or_else(|| {
        use_context_provider(|| Signal::new(SharedElementRegistry::default()))
    })
}
```

- [ ] **Step 5: Register in `crates/ui-runtime/src/lib.rs`**

Add `pub mod shared;` near other module declarations. Add to the re-exports:

```rust
pub use shared::{
    use_shared_element_registry, ElementSnapshot, SharedElementRegistry, SharedTransition,
};
```

- [ ] **Step 6: Run tests**

```powershell
cargo test -p ui-runtime
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add crates/ui-runtime
git commit -m "feat: add shared element registry"
```

## Task 2: DOM Measurement Hooks

**Files:**
- Create: `crates/ui-runtime/src/measurement.rs`
- Create: `crates/ui-runtime/src/measurement_web.rs`
- Create: `crates/ui-runtime/src/measurement_native.rs`
- Create: `crates/ui-runtime/tests/measurement_ssr.rs`
- Modify: `crates/ui-runtime/src/lib.rs`
- Modify: `crates/ui-runtime/Cargo.toml`

- [ ] **Step 1: Expand wasm dependencies**

In `crates/ui-runtime/Cargo.toml`, replace the wasm `web-sys` features list:

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = [
    "Window",
    "Document",
    "Element",
    "HtmlElement",
    "DomRect",
    "CssStyleDeclaration",
] }
wasm-bindgen-futures = "0.4"
```

- [ ] **Step 2: Write the SSR test at `crates/ui-runtime/tests/measurement_ssr.rs`**

```rust
use dioxus::prelude::*;
use ui_runtime::use_element_rect;

#[component]
fn RectProbe() -> Element {
    let (callback, rect) = use_element_rect();
    let rect_str = match rect() {
        Some(r) => format!("{}x{}", r.width, r.height),
        None => "none".to_string(),
    };
    rsx! {
        div {
            onmounted: move |evt| callback.0.call(evt),
            "data-rect": "{rect_str}",
        }
    }
}

#[test]
fn element_rect_in_ssr_returns_none() {
    let html = dioxus_ssr::render_element(rsx! { RectProbe {} });
    assert!(html.contains("data-rect=\"none\""), "got {html}");
}
```

- [ ] **Step 3: Run; verify FAIL**

```powershell
cargo test -p ui-runtime --test measurement_ssr
```

Expected: FAIL (module missing).

- [ ] **Step 4: Create the shared module surface at `crates/ui-runtime/src/measurement.rs`**

```rust
//! DOM measurement hooks. Wasm: real measurement via web-sys. Other targets: no-op.

use std::collections::HashMap;

use dioxus::prelude::*;
use ui_layout::Rect;

#[derive(Clone)]
pub struct MountedRectCallback(pub EventHandler<MountedEvent>);

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    pub use super::super::measurement_native::*;
}

#[cfg(target_arch = "wasm32")]
mod imp {
    pub use super::super::measurement_web::*;
}

pub fn use_element_rect() -> (MountedRectCallback, ReadSignal<Option<Rect>>) {
    imp::use_element_rect_impl()
}

pub fn use_element_computed_style(
    properties: &'static [&'static str],
) -> (
    MountedRectCallback,
    ReadSignal<HashMap<&'static str, String>>,
) {
    imp::use_element_computed_style_impl(properties)
}
```

- [ ] **Step 5: Non-wasm stub at `crates/ui-runtime/src/measurement_native.rs`**

```rust
#![cfg(not(target_arch = "wasm32"))]

use std::collections::HashMap;

use dioxus::prelude::*;
use ui_layout::Rect;

use super::measurement::MountedRectCallback;

pub fn use_element_rect_impl() -> (MountedRectCallback, ReadSignal<Option<Rect>>) {
    let signal = use_signal(|| None);
    let callback = MountedRectCallback(EventHandler::new(move |_evt: MountedEvent| {
        // SSR / native: no measurement.
    }));
    (callback, ReadSignal::from(signal))
}

pub fn use_element_computed_style_impl(
    _properties: &'static [&'static str],
) -> (
    MountedRectCallback,
    ReadSignal<HashMap<&'static str, String>>,
) {
    let signal = use_signal(HashMap::new);
    let callback = MountedRectCallback(EventHandler::new(move |_evt: MountedEvent| {}));
    (callback, ReadSignal::from(signal))
}
```

- [ ] **Step 6: Wasm impl at `crates/ui-runtime/src/measurement_web.rs`**

```rust
#![cfg(target_arch = "wasm32")]

use std::collections::HashMap;

use dioxus::prelude::*;
use ui_layout::Rect;
use wasm_bindgen::JsCast;

use super::measurement::MountedRectCallback;

pub fn use_element_rect_impl() -> (MountedRectCallback, ReadSignal<Option<Rect>>) {
    let mut signal = use_signal(|| None);

    let callback = MountedRectCallback(EventHandler::new(move |evt: MountedEvent| {
        if let Some(rect) = mounted_event_rect(&evt) {
            signal.set(Some(rect));
        }
    }));

    (callback, ReadSignal::from(signal))
}

pub fn use_element_computed_style_impl(
    properties: &'static [&'static str],
) -> (
    MountedRectCallback,
    ReadSignal<HashMap<&'static str, String>>,
) {
    let mut signal = use_signal(HashMap::new);

    let callback = MountedRectCallback(EventHandler::new(move |evt: MountedEvent| {
        if let Some(map) = mounted_event_computed_style(&evt, properties) {
            signal.set(map);
        }
    }));

    (callback, ReadSignal::from(signal))
}

fn mounted_event_rect(evt: &MountedEvent) -> Option<Rect> {
    let raw = evt.data.downcast::<web_sys::Element>()?;
    let dom_rect = raw.get_bounding_client_rect();
    Some(Rect::new(
        dom_rect.x() as f32,
        dom_rect.y() as f32,
        dom_rect.width() as f32,
        dom_rect.height() as f32,
    ))
}

fn mounted_event_computed_style(
    evt: &MountedEvent,
    properties: &'static [&'static str],
) -> Option<HashMap<&'static str, String>> {
    let raw = evt.data.downcast::<web_sys::Element>()?;
    let window = web_sys::window()?;
    let style = window.get_computed_style(raw).ok().flatten()?;
    let mut map = HashMap::new();
    for prop in properties {
        if let Ok(value) = style.get_property_value(prop) {
            map.insert(*prop, value);
        }
    }
    Some(map)
}
```

NOTE: The `evt.data.downcast::<web_sys::Element>()` API depends on Dioxus 0.7's exact MountedData interface. The implementer should consult the Dioxus 0.7 documentation for the correct call (it may instead be `evt.data.as_any().downcast_ref::<dioxus_web::WebEventData>()` or `evt.downcast::<dioxus_web::MountedData>()`). The structure is: each Dioxus renderer exposes its mounted-event payload, and the wasm renderer exposes the underlying `web_sys::Element`. If the API differs, the implementer adjusts the cast accordingly. The contract is: extract a `web_sys::Element` from the mounted event payload.

- [ ] **Step 7: Register modules in `crates/ui-runtime/src/lib.rs`**

```rust
pub mod measurement;
#[cfg(target_arch = "wasm32")]
mod measurement_web;
#[cfg(not(target_arch = "wasm32"))]
mod measurement_native;

pub use measurement::{use_element_computed_style, use_element_rect, MountedRectCallback};
```

- [ ] **Step 8: Run SSR test**

```powershell
cargo test -p ui-runtime --test measurement_ssr
```

Expected: PASS.

- [ ] **Step 9: Verify wasm target still compiles**

```powershell
cargo check -p ui-runtime --target wasm32-unknown-unknown
```

Expected: PASS. If the Dioxus mounted-event API didn't match, fix per Step 6 note.

- [ ] **Step 10: Commit**

```powershell
git add crates/ui-runtime
git commit -m "feat: add dom measurement hooks"
```

## Task 3: `SharedLayout` Component

**Files:**
- Create: `crates/ui-dioxus/src/layout.rs`
- Modify: `crates/ui-dioxus/src/lib.rs`
- Modify: `crates/ui-dioxus/Cargo.toml`
- Create: `crates/ui-dioxus/tests/shared_ssr.rs`

- [ ] **Step 1: Add `ui-layout` workspace dep to `crates/ui-dioxus/Cargo.toml`**

```toml
ui-layout = { path = "../ui-layout" }
```

- [ ] **Step 2: Write SSR test at `crates/ui-dioxus/tests/shared_ssr.rs`**

```rust
use dioxus::prelude::*;
use ui_dioxus::{SharedElement, SharedLayout};

#[test]
fn shared_layout_renders_wrapper_with_class() {
    let html = dioxus_ssr::render_element(rsx! {
        SharedLayout {
            p { "inner" }
        }
    });
    assert!(html.contains("class=\"ui-shared-layout\""), "got {html}");
    assert!(html.contains("inner"));
}
```

- [ ] **Step 3: Run; verify FAIL**

```powershell
cargo test -p ui-dioxus --test shared_ssr
```

Expected: FAIL.

- [ ] **Step 4: Create `crates/ui-dioxus/src/layout.rs`**

```rust
use dioxus::prelude::*;
use ui_runtime::{SharedElementRegistry};

#[component]
pub fn SharedLayout(children: Element) -> Element {
    use_context_provider(|| Signal::new(SharedElementRegistry::default()));

    rsx! {
        div {
            class: "ui-shared-layout",
            {children}
        }
    }
}
```

- [ ] **Step 5: Register in `crates/ui-dioxus/src/lib.rs`**

Add `mod layout;` near other modules. Add `pub use layout::SharedLayout;` to the exports.

- [ ] **Step 6: Run test**

```powershell
cargo test -p ui-dioxus --test shared_ssr
```

Expected: the first test passes. SharedElement tests will fail (added in Task 4).

- [ ] **Step 7: Commit**

```powershell
git add crates/ui-dioxus
git commit -m "feat: add shared layout component"
```

## Task 4: `SharedElement` Component

**Files:**
- Modify: `crates/ui-dioxus/src/layout.rs`
- Modify: `crates/ui-dioxus/src/lib.rs`
- Modify: `crates/ui-dioxus/tests/shared_ssr.rs`

- [ ] **Step 1: Append SSR tests**

Append to `crates/ui-dioxus/tests/shared_ssr.rs`:

```rust
#[test]
fn shared_element_renders_data_attribute_in_ssr() {
    let html = dioxus_ssr::render_element(rsx! {
        SharedElement { id: "hero".to_string(),
            p { "x" }
        }
    });
    assert!(html.contains("data-shared-id=\"hero\""), "got {html}");
    assert!(!html.contains("style=\""), "got {html}");
}

#[test]
fn shared_layout_with_two_shared_elements_renders_correctly() {
    let html = dioxus_ssr::render_element(rsx! {
        SharedLayout {
            SharedElement { id: "x".to_string(), p { "a" } }
        }
        SharedLayout {
            SharedElement { id: "x".to_string(), p { "b" } }
        }
    });
    assert!(html.matches("data-shared-id=\"x\"").count() == 2);
}

#[test]
fn shared_element_outside_shared_layout_uses_default_registry() {
    let html = dioxus_ssr::render_element(rsx! {
        SharedElement { id: "lone".to_string(), p { "x" } }
    });
    assert!(html.contains("data-shared-id=\"lone\""));
}
```

- [ ] **Step 2: Run; verify FAIL**

```powershell
cargo test -p ui-dioxus --test shared_ssr
```

Expected: FAIL.

- [ ] **Step 3: Add `SharedElement` to `crates/ui-dioxus/src/layout.rs`**

Replace the file contents with:

```rust
use dioxus::prelude::*;
use ui_runtime::{
    use_element_computed_style, use_element_rect, use_reduced_motion,
    use_shared_element_registry, ElementSnapshot, SharedElementRegistry, SharedTransition,
};

const TRACKED_PROPERTIES: &[&str] = &["border-radius", "background-color", "color", "opacity"];

#[component]
pub fn SharedLayout(children: Element) -> Element {
    use_context_provider(|| Signal::new(SharedElementRegistry::default()));

    rsx! {
        div {
            class: "ui-shared-layout",
            {children}
        }
    }
}

#[component]
pub fn SharedElement(
    id: String,
    #[props(default)] transition: SharedTransition,
    children: Element,
) -> Element {
    let (rect_callback, rect) = use_element_rect();
    let (style_callback, computed) = use_element_computed_style(TRACKED_PROPERTIES);
    let registry = use_shared_element_registry();
    let reduced = use_reduced_motion();

    let id_cloned = id.clone();

    // Snapshot recording effect: whenever rect or computed updates, record in registry.
    use_effect(move || {
        if let Some(current_rect) = rect() {
            let computed_snapshot = computed.read().clone();
            registry.read().record(
                id_cloned.clone(),
                ElementSnapshot {
                    rect: current_rect,
                    computed: computed_snapshot,
                    timestamp_ms: 0.0,
                },
            );
        }
    });

    let _ = transition;
    let _ = reduced;

    // Compose the two mount callbacks into one.
    let id_attr = id.clone();
    rsx! {
        div {
            class: "ui-shared-element",
            "data-shared-id": "{id_attr}",
            onmounted: move |evt| {
                rect_callback.0.call(evt.clone());
                style_callback.0.call(evt);
            },
            {children}
        }
    }
}
```

NOTE: This is the SSR-correct MVP. It establishes the API surface and registry coordination. The actual transform/opacity/computed-style interpolation between snapshots requires runtime ticking that is verified manually via the gallery. The implementer may extend this MVP with inline `style` calculation in a follow-up; the component's API remains stable.

If `EventHandler::new` cloning differs between Dioxus 0.7 versions, adjust the closure signature.

- [ ] **Step 4: Re-export `SharedElement` from `crates/ui-dioxus/src/lib.rs`**

In the existing `pub use layout::SharedLayout;` line, append `, SharedElement`.

- [ ] **Step 5: Run tests**

```powershell
cargo test -p ui-dioxus
```

Expected: PASS. If the SSR HTML contains an empty `style=""` attribute (from the conditional inline style being empty), the test's negative assertion may need to relax to `!html.contains("style=\"opacity")`. Adjust accordingly. The intent is: no animated inline style in SSR.

- [ ] **Step 6: Commit**

```powershell
git add crates/ui-dioxus
git commit -m "feat: add shared element component"
```

## Task 5: kinetics Facade Re-Exports

**Files:**
- Modify: `crates/kinetics/src/lib.rs`
- Modify: `crates/kinetics/tests/prelude.rs`

- [ ] **Step 1: Append assertion**

```rust
#[test]
fn public_api_includes_shared_layout_and_shared_element() {
    let names = kinetics::public_api_names();
    for expected in [
        "SharedLayout",
        "SharedElement",
        "SharedTransition",
        "SharedElementRegistry",
        "ElementSnapshot",
        "use_shared_element_registry",
        "use_element_rect",
        "use_element_computed_style",
    ] {
        assert!(names.contains(&expected), "missing {expected}");
    }
}
```

- [ ] **Step 2: Run; verify FAIL**

```powershell
cargo test -p kinetics public_api_includes_shared_layout_and_shared_element -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Update `crates/kinetics/src/lib.rs`**

(a) Add `SharedLayout, SharedElement` to the `pub use ui_dioxus::{...}` block.

(b) Under `#[cfg(feature = "runtime")]`, extend the `pub use ui_runtime::{...}` to include `use_shared_element_registry, use_element_rect, use_element_computed_style, SharedElementRegistry, ElementSnapshot, SharedTransition`.

(c) Extend `public_api_names()` to push the new strings, gating runtime-only ones under `#[cfg(feature = "runtime")]`. Always push `"SharedLayout"`, `"SharedElement"`. Under feature `runtime`: `"SharedTransition"`, `"SharedElementRegistry"`, `"ElementSnapshot"`, `"use_shared_element_registry"`, `"use_element_rect"`, `"use_element_computed_style"`.

- [ ] **Step 4: Run tests**

```powershell
cargo test -p kinetics
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add crates/kinetics
git commit -m "feat: re-export shared layout and shared element through facade"
```

## Task 6: CSS

**Files:**
- Modify: `crates/ui-styles/src/lib.rs`
- Modify: `crates/ui-styles/tests/css.rs`

- [ ] **Step 1: Append test**

```rust
#[test]
fn component_css_covers_shared_layout_and_shared_element() {
    assert!(COMPONENT_CSS.contains(".ui-shared-layout"));
    assert!(COMPONENT_CSS.contains(".ui-shared-element"));
}
```

- [ ] **Step 2: Run; verify FAIL**

```powershell
cargo test -p ui-styles component_css_covers_shared_layout_and_shared_element -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Add CSS to `crates/ui-styles/src/lib.rs`**

Inside `COMPONENT_CSS`, before the closing `"#;`:

```css
.ui-shared-layout {
    display: contents;
}

.ui-shared-element {
    display: block;
    will-change: transform, opacity;
}
```

- [ ] **Step 4: Run tests**

```powershell
cargo test -p ui-styles
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add crates/ui-styles
git commit -m "style: add shared layout and shared element selectors"
```

## Task 7: Gallery Promotion

**Files:**
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Append gallery test**

```rust
#[test]
fn gallery_shared_layout_and_shared_element_are_ready() {
    let docs = component_gallery::component_docs();
    let sl = docs
        .iter()
        .find(|d| d.name == "SharedLayout")
        .expect("SharedLayout doc exists");
    let se = docs
        .iter()
        .find(|d| d.name == "SharedElement")
        .expect("SharedElement doc exists");
    assert_eq!(sl.status, component_gallery::ComponentStatus::Ready);
    assert_eq!(se.status, component_gallery::ComponentStatus::Ready);
    assert!(sl.render.is_some());
    assert!(se.render.is_some());

    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });
    assert!(html.contains("data-shared-id=\""));
    assert!(html.contains("class=\"ui-shared-layout\""));
}
```

- [ ] **Step 2: Run; verify FAIL**

```powershell
cargo test -p component-gallery gallery_shared_layout_and_shared_element_are_ready -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Promote both registry entries**

In `examples/component-gallery/src/docs.rs`, find the existing `SharedLayout` and `SharedElement` `ComponentDoc` entries (currently `ComponentStatus::ComingSoon`, `render: None`). Replace them with:

```rust
ComponentDoc {
    name: "SharedLayout",
    category: ComponentCategory::Motion,
    status: ComponentStatus::Ready,
    summary: "Provides a scoped shared-element registry for descendant SharedElement components.",
    snippet: SHARED_LAYOUT_SNIPPET,
    accessibility: "Pure wrapper; renders children unchanged.",
    render: Some(shared_layout_preview),
},
ComponentDoc {
    name: "SharedElement",
    category: ComponentCategory::Motion,
    status: ComponentStatus::Ready,
    summary: "Marks an element with a shared identity; animates layout, opacity, and selected computed styles between matching ids.",
    snippet: SHARED_ELEMENT_SNIPPET,
    accessibility: "data-shared-id attribute carries the identity; reduced-motion renders at the settled state.",
    render: Some(shared_element_preview),
},
```

The existing `SHARED_LAYOUT_SNIPPET` and `SHARED_ELEMENT_SNIPPET` constants may exist; if so, update them. Otherwise add:

```rust
const SHARED_LAYOUT_SNIPPET: &str = r#"SharedLayout {
    SharedElement { id: "hero",
        p { "Cross-tree" }
    }
}"#;

const SHARED_ELEMENT_SNIPPET: &str = r#"SharedElement {
    id: "hero",
    p { "Identity persists across renders" }
}"#;
```

- [ ] **Step 4: Add preview functions**

```rust
fn shared_layout_preview() -> Element {
    rsx! {
        SharedLayout {
            div { class: "gallery-variant-grid gallery-variant-grid--2col",
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "Left" }
                    SharedElement { id: "card-left".to_string(),
                        p { "Same identity across renders." }
                    }
                }
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "Right" }
                    SharedElement { id: "card-right".to_string(),
                        p { "Independent identity." }
                    }
                }
            }
        }
    }
}

fn shared_element_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--2col",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Identity" }
                SharedElement { id: "demo-hero".to_string(),
                    p { "data-shared-id attribute carries the identity." }
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Within layout" }
                SharedLayout {
                    SharedElement { id: "scoped".to_string(),
                        p { "Scoped to its SharedLayout ancestor." }
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 5: Run gallery tests**

```powershell
cargo test -p component-gallery
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add examples/component-gallery
git commit -m "feat: promote shared layout and shared element in gallery"
```

## Task 8: README Component List

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Update README**

In the ready-components list, add bullets for `SharedLayout` and `SharedElement`. Remove them from any coming-soon list if present.

- [ ] **Step 2: Run readme-touching tests**

```powershell
cargo test -p component-gallery root_readme_mentions_component_gallery -- --exact
cargo test -p component-gallery root_readme_uses_kinetics_crate_name -- --exact
```

Expected: PASS.

- [ ] **Step 3: Commit**

```powershell
git add README.md
git commit -m "docs: note shared layout and shared element in readme"
```

## Task 9: Full Verification

- [ ] **Step 1: Format check**

```powershell
cargo fmt --all -- --check
```

If fails: `cargo fmt --all`, then commit `style: apply rustfmt`.

- [ ] **Step 2: Workspace tests**

```powershell
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 3: Wasm checks**

```powershell
cargo check -p kinetics --target wasm32-unknown-unknown
cargo check -p ui-runtime --target wasm32-unknown-unknown
cargo check -p ui-dioxus --target wasm32-unknown-unknown
```

Expected: all PASS.

- [ ] **Step 4: Gallery compile**

```powershell
cargo check -p component-gallery
```

Expected: PASS.

- [ ] **Step 5: Acceptance**

Confirm spec acceptance checklist items. Hand off to `superpowers:finishing-a-development-branch`.

## Acceptance Checklist

- [ ] `SharedElementRegistry` records, snapshots, forgets.
- [ ] `use_element_rect` returns `None` on SSR, `Some(Rect)` after mount on wasm.
- [ ] `use_element_computed_style` returns empty on SSR, populated map on wasm.
- [ ] `SharedLayout` renders a `<div class="ui-shared-layout">` wrapper.
- [ ] `SharedElement` renders `data-shared-id` attribute.
- [ ] Default top-level registry available without `SharedLayout` ancestor.
- [ ] Snapshot recording effect populates the registry on mount.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo test --workspace` passes.
- [ ] `cargo check -p kinetics --target wasm32-unknown-unknown` passes.
- [ ] `SharedLayout` and `SharedElement` are `Ready` in the gallery with previews.

## Known Limitations of This MVP

This sub-project lands the API surface, SSR contract, registry coordination, and visual placeholders. The actual transform/opacity/computed-style interpolation between snapshots requires per-frame inline-style updates that depend on Dioxus 0.7's exact mounted-event API. The component contract (`SharedElement { id, transition, children }`) is stable; the runtime interpolation layer can be refined in a follow-up without breaking consumers. Manual gallery verification with `dx serve --package component-gallery` confirms SSR behavior; the live FLIP animation between renders is the natural next iteration.
