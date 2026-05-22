# SharedLayout And SharedElement (FLIP) Design

## Goal

Land `SharedLayout` and `SharedElement` components that animate layout
changes via FLIP, match shared identities across tree positions, and
interpolate selected computed styles (opacity, border-radius,
background-color) alongside the transform.

This is sub-project 4 of a 4-part animation effort. It completes the
roadmap: after this, all motion entries in the gallery are `Ready`.

## Scope

This spec lands:

- DOM measurement hooks in `ui-runtime`: `use_element_rect` and
  `use_element_computed_style`, both SSR-safe.
- A `SharedElementRegistry` context type with record/forget/snapshot
  operations.
- `SharedLayout` component that creates a scoped registry.
- A default top-level registry for use without a `SharedLayout`
  ancestor.
- `SharedElement` component that uses the registry to coordinate
  cross-tree transitions and animates FLIP delta plus computed-style
  deltas.
- Cross-fade behavior when both outgoing and incoming elements with
  the same id exist briefly.
- Gallery promotion of both components to `Ready` with previews.

It excludes:

- Native (Blitz) renderer support. Web and WebView only.
- DOM ResizeObserver as a real subscription. Initial implementation
  polls rect via `use_future` on each render and on `onresize` events
  if available; a true ResizeObserver wrapping is a follow-up.
- Reorder/swap-detection between siblings (advanced FLIP). Self-FLIP
  and cross-tree matching are the supported patterns.

## Non-Goals

- Replacing or breaking `Presence`, `Sequence`, `IconButton`,
  `TimelineScope`, `PresenceGate`.
- Animating CSS properties other than the documented set
  (transform, opacity, border-radius, background-color).
- A built-in router or focus-management integration. The runtime
  triggers transitions based on remount lifecycle; routing is the
  caller's concern.

## Tech Stack

- Rust 2021, Cargo workspace, Dioxus 0.7.
- `ui-layout` for FLIP math.
- `ui-runtime` for the animation hook + frame scheduler.
- `web-sys` features: `Element`, `CssStyleDeclaration`, `Window`,
  `Document`, `DomRect`.
- `tokio` (non-wasm) and `wasm-bindgen-futures` (wasm) — already in
  `ui-runtime`.
- Dioxus SSR for tests.

## Architecture

### Concern 1: DOM measurement hooks

A new `ui-runtime::measurement` module exposes:

```rust
pub struct MountedRectCallback(pub EventHandler<MountedEvent>);

pub fn use_element_rect() -> (MountedRectCallback, ReadSignal<Option<Rect>>);

pub fn use_element_computed_style(
    properties: &'static [&'static str],
) -> (MountedRectCallback, ReadSignal<HashMap<&'static str, String>>);
```

A single `MountedRectCallback` may be attached to the element via
`onmounted: {callback.0}`. The callback's handler captures the
`MountedData` reference and uses it to:

- Wasm: call `get_client_rect()` (async) and `get_computed_style()`
  on the underlying `web_sys::Element`.
- Non-wasm/SSR: write `None` / empty map.

The rect signal is refreshed on:

1. The initial mount.
2. Each subsequent render (via `use_future` polling the mounted ref's
   `get_client_rect()` if available).
3. Window `resize` events (wasm only) — attached once per element.

The hook should NOT introduce a runaway loop. The rect is updated
only when it differs from the last stored value.

When mounted ref's underlying API isn't available (no mount yet,
SSR, native), the signal stays at `None`.

### Concern 2: Shared element registry

A new `ui-runtime::shared` module:

```rust
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
    pub fn snapshot(&self, id: &str) -> Option<ElementSnapshot>;
    pub fn record(&self, id: String, snapshot: ElementSnapshot);
    pub fn forget(&self, id: &str);
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

pub fn use_shared_element_registry() -> Signal<SharedElementRegistry>;
```

`use_shared_element_registry()` returns the nearest `Signal<SharedElementRegistry>` in context. If none exists, it creates and provides a global default at the app root via `use_context_provider` on first access.

The registry is held inside a `Signal` so updates trigger
re-renders for downstream consumers.

### Concern 3: `SharedLayout` component

```rust
#[component]
pub fn SharedLayout(children: Element) -> Element
```

Creates a new `Signal<SharedElementRegistry>` and provides it via
`use_context_provider`. Renders children inside a wrapper:

```rust
rsx! {
    div { class: "ui-shared-layout",
        {children}
    }
}
```

The new scope means descendant `SharedElement` instances with the same
`id` match only within this `SharedLayout`. Nesting `SharedLayout`
inside another `SharedLayout` creates a deeper scope.

### Concern 4: `SharedElement` component

```rust
#[component]
pub fn SharedElement(
    id: String,
    #[props(default)] transition: SharedTransition,
    children: Element,
) -> Element
```

Behavior:

1. **Measurement.** Call `use_element_rect()` and
   `use_element_computed_style(&["border-radius", "background-color", "color"])`.
   Attach both `MountedRectCallback`s to the wrapping `div` (Dioxus
   accepts a single `onmounted` handler; the implementation composes
   them into one handler that fans out to both hooks).

2. **Registry lookup.** Read the current `SharedElementRegistry` from
   context. Look up `id`. If a prior snapshot exists and its
   `timestamp_ms` is recent (within ~500ms), treat it as the "from"
   state. Otherwise, the element is "new" and animates from its
   own first-rendered state (self-FLIP) if it changes on subsequent
   renders.

3. **Animation drive.** Use the existing `use_animation_value` to drive
   a `t` value from 0 to 1 over `transition.layout.duration_ms`. The
   inline `style` is computed from `t`:

   - Translate/scale: lerp `(from.rect → current.rect)` via
     `compute_flip` reverse application. At `t=0`, the element is
     placed at the "from" position via inverse-flip transform. At
     `t=1`, the element is at identity (its natural position).
   - Opacity: lerp from `from.computed["opacity"]` to current
     computed opacity (defaults to 1.0 if missing).
   - Border-radius / background-color / color: lerp between the two
     computed values. For length values, parse `px` numerics; for
     color values, parse `rgb()` / `rgba()` / hex into RGBA and lerp.

4. **Cross-fade.** When a NEW SharedElement mounts with the same `id`
   that recently unmounted (or that exists elsewhere), the registry
   triggers a brief overlap: the outgoing element's last snapshot is
   recorded and the new element animates from that snapshot. The
   outgoing element's parent component is responsible for unmounting
   it after a configurable delay (defaults to
   `transition.fade.duration_ms`). The implementation does not
   forcibly keep outgoing elements alive — the SSR/Dioxus contract
   is that the caller controls mount/unmount, and `SharedElement`
   takes a snapshot before unmount via a destructor effect.

5. **Snapshot on every render.** After the first paint, record the
   current rect + computed style in the registry under `id`. This
   makes the latest position available to any future incoming
   element with the same id.

6. **SSR.** All measurement returns `None`/empty in SSR. The
   component renders:

   ```rust
   rsx! {
       div {
           class: "ui-shared-element",
           "data-shared-id": "{id}",
           onmounted: callback,
           {children}
       }
   }
   ```

   No inline `style` is set on SSR. The DOM HTML is byte-stable.

7. **Reduced motion.** When `use_reduced_motion()` returns true, all
   animations are skipped; the element renders at the final state
   immediately.

## File Map

- Create: `crates/ui-runtime/src/measurement.rs`
- Create: `crates/ui-runtime/src/measurement_web.rs`
- Create: `crates/ui-runtime/src/measurement_native.rs`
- Create: `crates/ui-runtime/src/shared.rs`
- Create: `crates/ui-runtime/tests/shared_registry.rs`
- Create: `crates/ui-runtime/tests/measurement_ssr.rs`
- Modify: `crates/ui-runtime/src/lib.rs`
- Modify: `crates/ui-runtime/Cargo.toml` (add `ui-layout` dep; expand
  `web-sys` features to include `Element`, `Window`, `Document`,
  `DomRect`, `CssStyleDeclaration`)
- Create: `crates/ui-dioxus/src/layout.rs`
- Create: `crates/ui-dioxus/tests/shared_ssr.rs`
- Modify: `crates/ui-dioxus/src/lib.rs`
- Modify: `crates/ui-dioxus/Cargo.toml` (add `ui-layout` dep)
- Modify: `crates/kinetics/src/lib.rs` (re-exports +
  `public_api_names()`)
- Modify: `crates/kinetics/tests/prelude.rs`
- Modify: `crates/ui-styles/src/lib.rs` (add `.ui-shared-element`
  and `.ui-shared-layout` selectors)
- Modify: `crates/ui-styles/tests/css.rs`
- Modify: `examples/component-gallery/src/docs.rs` (promote
  `SharedLayout` and `SharedElement` to `Ready` with previews)
- Modify: `examples/component-gallery/tests/gallery.rs`
- Modify: `README.md`

## Public API additions

`kinetics::prelude` gains:

- `SharedLayout`, `SharedElement`
- `SharedTransition`
- `SharedElementRegistry`, `ElementSnapshot`
- `use_shared_element_registry`, `use_element_rect`,
  `use_element_computed_style`

`public_api_names()` extended accordingly.

## Tests

### `ui-runtime`

```rust
// shared_registry.rs

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

```rust
// measurement_ssr.rs

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
            onmounted: callback.0,
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

### `ui-dioxus`

```rust
// shared_ssr.rs

use dioxus::prelude::*;
use ui_dioxus::{SharedElement, SharedLayout};

#[test]
fn shared_element_renders_data_attribute_in_ssr() {
    let html = dioxus_ssr::render_element(rsx! {
        SharedElement { id: "hero".to_string(),
            p { "x" }
        }
    });
    assert!(html.contains("data-shared-id=\"hero\""));
    // No inline style applied in SSR.
    assert!(!html.contains("style=\""), "got {html}");
}

#[test]
fn shared_layout_provides_scoped_registry_context() {
    let html = dioxus_ssr::render_element(rsx! {
        SharedLayout {
            SharedElement { id: "x".to_string(), p { "a" } }
        }
        SharedLayout {
            SharedElement { id: "x".to_string(), p { "b" } }
        }
    });
    // Both SharedElements render under separate registries; no panic.
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

### Gallery

A new test asserts `SharedLayout` and `SharedElement` are `Ready`,
have render functions, and the SSR HTML contains
`data-shared-id="..."` attributes for the preview's shared elements.

### Manual / visual verification

The actual FLIP animation, cross-tree matching, cross-fade overlap,
and computed-style interpolation require a browser. The gallery's
preview includes a toggle that swaps element positions to trigger the
shared-element transition. Manual verification is required to
confirm visual correctness.

## Acceptance Checklist

- [ ] `ui-runtime` exposes `use_element_rect`, `use_element_computed_style`.
- [ ] `SharedElementRegistry` supports record/forget/snapshot.
- [ ] `SharedLayout` provides a scoped `Signal<SharedElementRegistry>`.
- [ ] Default top-level registry is provided when no `SharedLayout` ancestor exists.
- [ ] `SharedElement` measures via `MountedRectCallback`.
- [ ] FLIP delta computed via `ui_layout::compute_flip`.
- [ ] Cross-fade applied when two SharedElements with the same id overlap.
- [ ] Computed-style interpolation supports `border-radius`,
      `background-color`, `color`.
- [ ] SSR renders `data-shared-id="..."` without inline style.
- [ ] Reduced-motion skips all animation.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo test --workspace` passes.
- [ ] `cargo check -p kinetics --target wasm32-unknown-unknown` passes.
- [ ] `cargo check -p ui-runtime --target wasm32-unknown-unknown` passes.
- [ ] Coming-soon entries removed from gallery; all entries now `Ready`.
