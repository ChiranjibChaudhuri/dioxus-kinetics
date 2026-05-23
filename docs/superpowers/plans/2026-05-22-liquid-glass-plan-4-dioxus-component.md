# Liquid Glass — Plan 4: `<LiquidSurface>` Dioxus Component

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development. Steps use checkbox (`- [ ]`) syntax for tracking. **This plan is the most complex yet — wgpu-on-canvas via Dioxus has many integration cliffs. Each implementation task includes an escalation condition: if you hit a wgpu API mismatch, Dioxus signal contention, or wasm-bindgen lifetime problem you can't resolve in ~15 minutes, report DONE_WITH_CONCERNS or BLOCKED with details rather than papering over.**

**Goal:** Ship `<LiquidSurface { material: LiquidMaterial, background: BackgroundSource, ... }>` — a Dioxus component that mounts a `<canvas>`, initializes wgpu, and drives the engine's compositor through `ui-runtime`'s frame loop. Pointer events, scroll velocity, and `prefers-reduced-motion` flow into the compositor via `MotionInputs`. The component is SSR-safe (renders a placeholder div on the server). After Plan 4 the engine has a real production surface usable from Dioxus apps.

**Architecture:** New crate `ui-glass-dioxus` depending on `ui-glass`, `ui-glass-engine`, `ui-runtime`, `ui-motion`, `dioxus`, plus web-platform bindings (`wasm-bindgen`, `web-sys`, `gloo-events`). The component renders a positioned `<canvas>` element. On client mount, an async pipeline initializes the wgpu instance/adapter/device, builds a `wgpu::Surface` from the canvas, and constructs a `Compositor`. Per-frame, the component reads its current `MotionInputs` (driven by event listeners + a ui-motion spring for pointer smoothing) and calls `compositor.render(...)`. On native (non-wasm32) targets the component still compiles but renders a no-op placeholder — full native canvas integration is deferred.

**Tech Stack:** Dioxus 0.7, wgpu 26 (canvas surface support via `SurfaceTarget::Canvas`), wasm-bindgen, web-sys, gloo-events for scroll listeners, ui-runtime's existing scheduler.

---

## Context: What Plan 3 left

- `Compositor` is fully wired: `update_inputs(MotionInputs)`, `set_background_scene(...)`, `render(bg_view, output_view, canvas_size, &[GlassRegion])`. Per-region backgrounds resolve through `BackgroundRenderer`. Multi-region works.
- `ui-runtime::spawn_frame_loop(callback) -> FrameHandle` already exists on both web and native — we don't need to reinvent rAF.
- `ui-motion::Spring` exists for pointer smoothing.

What Plan 4 adds is the host-side glue.

---

## Risk inventory

These are the integration cliffs. Subagents should escalate if blocked, not paper over.

1. **wgpu canvas surface on wasm32.** `wgpu::SurfaceTarget::Canvas(HtmlCanvasElement)` is supported in wgpu 22+, but the exact API and feature flags may have changed in 26. If you can't get a Surface from a canvas, report and we'll consult docs.
2. **Async device init inside a Dioxus component.** `wgpu::Instance::request_adapter` is async; Dioxus's `use_effect` is sync. Use `dioxus::prelude::spawn` to launch the async init, store the result in a `Signal<Option<SurfaceState>>`.
3. **`HtmlCanvasElement` access via `onmounted`.** Dioxus 0.7's `MountedData` provides a `web_event<T>()` accessor that downcasts to web-sys types. On non-web targets, the access path returns `None`.
4. **Lifetime of wgpu resources across Dioxus re-renders.** Surfaces and devices must outlive frame callbacks. Store them in `Rc<RefCell<...>>` or similar; the frame closure captures the cell.
5. **SSR rendering.** During SSR there is no canvas, no wgpu, no window. The component must render *something* — a `<div>` placeholder with the right inline style so the page layout matches client-side until hydration.
6. **Cleanup on unmount.** Dropping `FrameHandle` stops the loop. Dropping the Compositor + Surface releases GPU resources.

---

## File Structure

**New files:**

```
crates/ui-glass-dioxus/Cargo.toml
crates/ui-glass-dioxus/src/lib.rs                     re-exports + crate docs
crates/ui-glass-dioxus/src/component.rs               LiquidSurface component
crates/ui-glass-dioxus/src/surface_state.rs           wgpu init + per-frame holder
crates/ui-glass-dioxus/src/motion_bridge.rs           pointer/scroll/reduced-motion → MotionInputs
crates/ui-glass-dioxus/src/web.rs                     wasm32-only: canvas helpers
crates/ui-glass-dioxus/src/stub.rs                    non-wasm32: placeholder
crates/ui-glass-dioxus/tests/ssr_placeholder.rs       SSR renders a div, not a canvas
crates/ui-glass-dioxus/tests/component_props.rs       props mount/unmount correctly
```

**Modified files:**

```
Cargo.toml                                            workspace member + ui-glass-dioxus dep
examples/component-gallery/src/gallery.rs             example LiquidSurface usage
crates/ui-dioxus/src/lib.rs                           re-export LiquidSurface from ui-glass-dioxus
```

**Responsibility boundaries:**

- `component.rs` owns the Dioxus component itself (props, RSX, mount/unmount lifecycle).
- `surface_state.rs` owns wgpu init: instance, adapter, device, surface, compositor. One big struct that lives in a `Signal<Option<...>>` on the component.
- `motion_bridge.rs` owns event listeners + spring smoothing. Reads pointer position and writes to a `Signal<MotionInputs>`.
- `web.rs` is wasm32-only — `#[cfg(target_arch = "wasm32")]` gated. Contains the canvas-acquisition helper.
- `stub.rs` is non-wasm32 — produces a default `SurfaceState` that's a no-op. Native integration via Blitz is deferred (out of scope).

---

## Task 1: Scaffold `ui-glass-dioxus` crate

**Files:**
- Modify: `Cargo.toml`
- Create: `crates/ui-glass-dioxus/Cargo.toml`
- Create: `crates/ui-glass-dioxus/src/lib.rs`
- Create: `crates/ui-glass-dioxus/src/component.rs`
- Create: `crates/ui-glass-dioxus/src/surface_state.rs`
- Create: `crates/ui-glass-dioxus/src/motion_bridge.rs`

- [ ] **Step 1: Workspace member + workspace dep**

Edit `Cargo.toml`. Append `"crates/ui-glass-dioxus"` to `[workspace.members]`. Append to `[workspace.dependencies]`:

```toml
ui-glass-dioxus = { path = "crates/ui-glass-dioxus" }
gloo-events = "0.2"
gloo-utils = "0.2"
```

(`wasm-bindgen`, `web-sys`, `js-sys` are already in workspace via ui-runtime.)

- [ ] **Step 2: Create `crates/ui-glass-dioxus/Cargo.toml`**

```toml
[package]
name = "ui-glass-dioxus"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
dioxus.workspace = true
ui-glass.workspace = true
ui-glass-engine.workspace = true
ui-runtime.workspace = true
ui-motion.workspace = true
ui-tokens.workspace = true
wgpu.workspace = true

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = [
    "Window",
    "Document",
    "Element",
    "HtmlCanvasElement",
    "HtmlElement",
    "DomRect",
    "PointerEvent",
    "WheelEvent",
    "MouseEvent",
    "MediaQueryList",
    "MediaQueryListEvent",
    "CssStyleDeclaration",
] }
wasm-bindgen-futures = "0.4"
gloo-events.workspace = true
gloo-utils.workspace = true

[dev-dependencies]
dioxus-ssr.workspace = true
pollster.workspace = true

[lib]
path = "src/lib.rs"
```

- [ ] **Step 3: Create skeleton source files**

`crates/ui-glass-dioxus/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

//! Dioxus integration for the Liquid Glass engine.
//!
//! Provides `<LiquidSurface>` — a component that mounts a wgpu-rendered
//! glass surface backed by `ui-glass-engine::Compositor`. On web/desktop/mobile
//! (WebView) targets, the component initializes wgpu against an HTML canvas
//! and drives per-frame rendering via the `ui-runtime` scheduler. On native
//! (non-wasm32) targets the component compiles but renders a placeholder
//! — Blitz/native integration is deferred.

pub mod component;
pub mod motion_bridge;
pub mod surface_state;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub mod stub;

pub use component::{LiquidSurface, LiquidSurfaceProps};
```

`crates/ui-glass-dioxus/src/component.rs` (stub):

```rust
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LiquidSurfaceProps {
    pub width: u32,
    pub height: u32,
}

#[component]
pub fn LiquidSurface(props: LiquidSurfaceProps) -> Element {
    rsx! {
        div {
            "width": "{props.width}px",
            "height": "{props.height}px",
        }
    }
}
```

`crates/ui-glass-dioxus/src/surface_state.rs` (stub):

```rust
//! Filled in by Tasks 3-4.
```

`crates/ui-glass-dioxus/src/motion_bridge.rs` (stub):

```rust
//! Filled in by Tasks 8-10.
```

If on wasm32, create `crates/ui-glass-dioxus/src/web.rs` (stub):

```rust
#![cfg(target_arch = "wasm32")]
//! Filled in by Task 2.
```

If on non-wasm32, create `crates/ui-glass-dioxus/src/stub.rs` (stub):

```rust
#![cfg(not(target_arch = "wasm32"))]
//! Native placeholder. Filled in by Task 12.
```

- [ ] **Step 4: Build to verify**

`cargo build -p ui-glass-dioxus`
Expected: success.

`cargo build --target wasm32-unknown-unknown -p ui-glass-dioxus` (if wasm32 target is installed; skip if not).

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml crates/ui-glass-dioxus
git commit -m "feat(ui-glass-dioxus): scaffold new crate"
```

---

## Task 2: Canvas acquisition helper (web-only)

**Files:**
- Modify: `crates/ui-glass-dioxus/src/web.rs`

- [ ] **Step 1: Implement canvas helper**

Replace `crates/ui-glass-dioxus/src/web.rs` with:

```rust
#![cfg(target_arch = "wasm32")]
//! Web-only helpers for canvas + window access.

use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

/// Try to downcast a Dioxus `MountedData`-provided element to an
/// `HtmlCanvasElement`. Returns `None` if the type doesn't match (e.g. SSR or
/// the element hasn't mounted yet).
pub fn canvas_from_mounted(mounted: &dioxus::prelude::MountedData) -> Option<HtmlCanvasElement> {
    let raw = mounted.try_as_web_event::<web_sys::Element>().ok()?;
    raw.dyn_into::<HtmlCanvasElement>().ok()
}

/// Read the device pixel ratio for hi-DPI canvas sizing.
pub fn device_pixel_ratio() -> f32 {
    web_sys::window()
        .map(|w| w.device_pixel_ratio() as f32)
        .unwrap_or(1.0)
}

/// Resize the underlying canvas drawing-buffer to match its CSS size scaled
/// by the device pixel ratio. Returns the new (width, height) in physical px.
pub fn resize_canvas_to_css_size(canvas: &HtmlCanvasElement) -> (u32, u32) {
    let css_w = canvas.client_width().max(1) as f32;
    let css_h = canvas.client_height().max(1) as f32;
    let dpr = device_pixel_ratio();
    let w = (css_w * dpr).round().max(1.0) as u32;
    let h = (css_h * dpr).round().max(1.0) as u32;
    canvas.set_width(w);
    canvas.set_height(h);
    (w, h)
}
```

**Escalation:** Dioxus 0.7's exact `MountedData` API for web-event downcasting may differ. The method may be `web_event()`, `try_as_web_event()`, or accessed via `as_any()` + `downcast_ref`. If the exact form above doesn't compile, find the correct one in Dioxus docs / existing `ui-runtime` measurement code (which already uses `MountedData`) and adapt.

- [ ] **Step 2: Build**

`cargo build -p ui-glass-dioxus --target wasm32-unknown-unknown` (or just `cargo build -p ui-glass-dioxus` — non-wasm32 will skip this file via cfg).

- [ ] **Step 3: Commit**

```bash
git add crates/ui-glass-dioxus/src/web.rs
git commit -m "feat(ui-glass-dioxus): canvas acquisition helper"
```

---

## Task 3: `SurfaceState` type — wgpu init pipeline

**Files:**
- Modify: `crates/ui-glass-dioxus/src/surface_state.rs`

- [ ] **Step 1: Implement SurfaceState (web path)**

Replace `crates/ui-glass-dioxus/src/surface_state.rs` with:

```rust
//! Holds the wgpu pipeline for a single LiquidSurface instance. Initialized
//! asynchronously when the canvas mounts; lives in a `Signal<Option<...>>`
//! on the component.

use std::sync::Arc;

use ui_glass_engine::Compositor;

pub struct SurfaceState {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub surface: wgpu::Surface<'static>,
    pub surface_format: wgpu::TextureFormat,
    pub compositor: Compositor,
    pub physical_size: (u32, u32),
}

impl SurfaceState {
    /// Initialize wgpu from a canvas element. Returns `None` if no adapter is
    /// available (e.g. WebGPU unavailable and no WebGL2 fallback).
    #[cfg(target_arch = "wasm32")]
    pub async fn from_canvas(
        canvas: web_sys::HtmlCanvasElement,
        physical_size: (u32, u32),
    ) -> Option<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
            .ok()?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok()?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("liquid-surface-device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .ok()?;

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let caps = surface.get_capabilities(&adapter);
        let surface_format = caps.formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        surface.configure(&device, &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: physical_size.0,
            height: physical_size.1,
            present_mode: caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
            view_formats: vec![],
        });

        let compositor = Compositor::new(device.clone(), queue.clone());

        Some(Self {
            device, queue, surface, surface_format, compositor, physical_size,
        })
    }

    pub fn resize(&mut self, physical_size: (u32, u32)) {
        if physical_size == self.physical_size || physical_size.0 == 0 || physical_size.1 == 0 {
            return;
        }
        self.physical_size = physical_size;
        let caps = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width: physical_size.0,
            height: physical_size.1,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
            view_formats: vec![],
        };
        self.surface.configure(&self.device, &caps);
    }
}
```

**Escalation:** `wgpu::SurfaceTarget::Canvas` may have a different name or wrapping in wgpu 26 (e.g. `SurfaceTarget::from(canvas)` constructor). Test compilation; if it fails, the surface variant might be `SurfaceTarget::Canvas { canvas }` or a `Surface<'static>` constructor takes the canvas directly. Adapt to the actual API; do not invent.

`wgpu::CompositeAlphaMode::PreMultiplied` is essential — Dioxus DOM widgets layer on top via z-index, and the canvas must blend correctly. If the browser/adapter doesn't support `PreMultiplied`, fall back to `caps.alpha_modes[0]` and note it.

- [ ] **Step 2: Build**

`cargo build -p ui-glass-dioxus`
Expected: success on both targets. On non-wasm32, the `#[cfg(target_arch = "wasm32")]` gate hides `from_canvas`, so the type alone must compile in stub form. If non-wasm32 fails, add a placeholder constructor for stub mode.

- [ ] **Step 3: Commit**

```bash
git add crates/ui-glass-dioxus/src/surface_state.rs
git commit -m "feat(ui-glass-dioxus): SurfaceState with async wgpu init"
```

---

## Task 4: `LiquidSurface` component — props, RSX shell, mount handler

**Files:**
- Modify: `crates/ui-glass-dioxus/src/component.rs`

- [ ] **Step 1: Define props + component skeleton**

Replace `crates/ui-glass-dioxus/src/component.rs` with:

```rust
//! `<LiquidSurface>` Dioxus component.

use dioxus::prelude::*;

use ui_glass::LiquidMaterial;
use ui_glass_engine::background::BackgroundSource;

use crate::motion_bridge::MotionState;
use crate::surface_state::SurfaceState;

#[derive(Props, Clone, PartialEq)]
pub struct LiquidSurfaceProps {
    /// Material descriptor for the glass surface.
    #[props(into)]
    pub material: LiquidMaterial,

    /// Optional background source. When `None`, the surface samples whatever
    /// is rendered behind the canvas in the DOM (which, due to z-index
    /// stacking, is effectively the page background).
    #[props(default)]
    pub background: Option<BackgroundSource>,

    /// Surface rect within the canvas, in CSS pixels: `[x, y, w, h]`. Defaults
    /// to filling the canvas.
    #[props(default)]
    pub rect: Option<[f32; 4]>,

    /// Logical width in CSS pixels.
    #[props(default = 320)]
    pub width: u32,

    /// Logical height in CSS pixels.
    #[props(default = 200)]
    pub height: u32,

    /// Optional children — rendered as DOM widgets on top of the canvas with
    /// `position: absolute; z-index: 1; pointer-events: auto`.
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn LiquidSurface(props: LiquidSurfaceProps) -> Element {
    // Signal carrying the wgpu state once the canvas mounts and async init
    // completes. None until ready.
    let surface_state: Signal<Option<SurfaceState>> = use_signal(|| None::<SurfaceState>);

    // MotionState holds the latest pointer/scroll/time + reduced-motion flag.
    // Updated by motion_bridge listeners; read by the frame loop.
    let motion_state: Signal<MotionState> = use_signal(MotionState::default);

    let inline_style = format!(
        "position: relative; display: inline-block; width: {}px; height: {}px;",
        props.width, props.height,
    );
    let canvas_style = format!(
        "position: absolute; inset: 0; width: 100%; height: 100%; \
         z-index: 0; pointer-events: none; display: block;",
    );
    let foreground_style = format!(
        "position: absolute; inset: 0; z-index: 1; pointer-events: auto;",
    );

    rsx! {
        div {
            class: "ui-liquid-surface",
            style: "{inline_style}",
            data_glass_role: "liquid-surface",

            canvas {
                style: "{canvas_style}",
                width: "{props.width}",
                height: "{props.height}",
                onmounted: move |evt| {
                    handle_canvas_mounted(evt, surface_state, motion_state, &props);
                },
            }
            div {
                style: "{foreground_style}",
                {props.children}
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn handle_canvas_mounted(
    evt: dioxus::prelude::MountedEvent,
    mut surface_state: Signal<Option<SurfaceState>>,
    motion_state: Signal<crate::motion_bridge::MotionState>,
    props: &LiquidSurfaceProps,
) {
    use crate::web::{canvas_from_mounted, resize_canvas_to_css_size};
    if let Some(canvas) = canvas_from_mounted(&evt.data()) {
        let physical_size = resize_canvas_to_css_size(&canvas);
        let canvas_clone = canvas.clone();
        let material = props.material;
        let background = props.background.clone();
        let rect = props.rect;
        let width = props.width;
        let height = props.height;

        spawn(async move {
            if let Some(state) = SurfaceState::from_canvas(canvas_clone, physical_size).await {
                surface_state.set(Some(state));
                // Start the frame loop (Task 5 wires this).
                start_frame_loop(surface_state, motion_state, material, background, rect, width, height);
            }
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn handle_canvas_mounted(
    _evt: dioxus::prelude::MountedEvent,
    _surface_state: Signal<Option<SurfaceState>>,
    _motion_state: Signal<crate::motion_bridge::MotionState>,
    _props: &LiquidSurfaceProps,
) {
    // No-op on non-web (Blitz/native canvas integration deferred).
}

#[cfg(target_arch = "wasm32")]
fn start_frame_loop(
    _surface_state: Signal<Option<SurfaceState>>,
    _motion_state: Signal<crate::motion_bridge::MotionState>,
    _material: LiquidMaterial,
    _background: Option<BackgroundSource>,
    _rect: Option<[f32; 4]>,
    _width: u32,
    _height: u32,
) {
    // Wired in Task 5.
}
```

**Escalation:** Dioxus 0.7's exact attribute syntax for inline styles, `onmounted`, and `MountedEvent.data()` access may differ from what's written. Check existing components in `ui-dioxus` (e.g. ToolBar, Sidebar) for the established pattern in this repo and adapt. The component should at minimum render a div+canvas pair when called with default props.

- [ ] **Step 2: Build**

`cargo build -p ui-glass-dioxus`
Expected: success on both targets.

- [ ] **Step 3: Commit**

```bash
git add crates/ui-glass-dioxus/src/component.rs
git commit -m "feat(ui-glass-dioxus): LiquidSurface component shell + canvas mount"
```

---

## Task 5: Frame loop integration

**Files:**
- Modify: `crates/ui-glass-dioxus/src/component.rs`

- [ ] **Step 1: Implement `start_frame_loop`**

The frame loop needs to:
1. Acquire next swap-chain texture from the surface
2. Build the GlassRegion from material + rect props
3. Call `compositor.update_inputs(motion.snapshot())`
4. Call `compositor.render(bg_view, output_view, canvas_size, &[region])`
5. Present the swap-chain texture

The "bg_view" parameter is interesting: when `props.background` is `Some(...)`, the GlassRegion carries the source; the Compositor's BackgroundRenderer materializes it. When `None`, the caller normally provides a pre-uploaded texture — but here on canvas, the "behind the surface" content is whatever the browser composites underneath the canvas (CSS background, sibling DOM elements), which wgpu can't sample. So when `background` is None we fall back to a transparent placeholder texture so the glass surface effectively shows the canvas's existing clear color.

Replace the placeholder `start_frame_loop` body in `component.rs`:

```rust
#[cfg(target_arch = "wasm32")]
fn start_frame_loop(
    surface_state: Signal<Option<SurfaceState>>,
    motion_state: Signal<crate::motion_bridge::MotionState>,
    material: LiquidMaterial,
    background: Option<BackgroundSource>,
    rect: Option<[f32; 4]>,
    width: u32,
    height: u32,
) {
    use ui_runtime::scheduler::{spawn_frame_loop, ControlFlow};
    use ui_glass_engine::GlassRegion;

    // Build the region once; props re-renders will recreate the frame loop.
    let region = {
        let r = rect.unwrap_or([0.0, 0.0, width as f32, height as f32]);
        let mut gr = GlassRegion::new(r, material);
        if let Some(bg) = background.clone() {
            gr = gr.with_background(bg);
        }
        gr
    };

    let start = web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0);

    // Spawn a frame loop that lives until the FrameHandle drops. We attach the
    // handle to a use_effect so it cleans up on unmount; here, we leak it for
    // simplicity (Plan 4 follow-up: store handle in a Signal so unmount drops
    // it). Plan 4 explicitly accepts the leak as a known limitation.
    let _handle = spawn_frame_loop(move |_dt_ms| {
        let mut surface_state = surface_state;
        let motion_state = motion_state;
        let region = region.clone();
        let mut state_opt = surface_state.write();
        let Some(state) = state_opt.as_mut() else {
            return ControlFlow::Continue;
        };

        // 1. Acquire frame
        let frame = match state.surface.get_current_texture() {
            Ok(f) => f,
            Err(_) => return ControlFlow::Continue,
        };
        let output_view = frame.texture.create_view(&Default::default());

        // 2. Build a transparent fallback bg if needed.
        let bg_tex = state.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("liquid-surface-bg-fallback"),
            size: wgpu::Extent3d {
                width: state.physical_size.0,
                height: state.physical_size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let bg_view = bg_tex.create_view(&Default::default());

        // 3. Push current motion + render
        let inputs = motion_state.read().to_motion_inputs(start);
        state.compositor.update_inputs(inputs);
        state.compositor.render(
            &bg_view,
            &output_view,
            [state.physical_size.0 as f32, state.physical_size.1 as f32],
            &[region],
        );

        // 4. Present
        frame.present();

        ControlFlow::Continue
    });

    // _handle leaked intentionally for Plan 4. Plan 5 will properly tie it to
    // component lifecycle.
    std::mem::forget(_handle);
}
```

**Escalation:** This is the most complex step. Several things can go wrong:

- `surface.get_current_texture()` may fail if the surface needs reconfiguration (resize). Handle by calling `state.resize(...)` on `SurfaceErrorKind::Lost` / `Outdated`.
- `spawn_frame_loop`'s closure must be `'static + FnMut(f64) -> ControlFlow`. Capturing `Signal<...>` (which is `Copy`) should be fine, but the borrow inside may need adjustment. If you hit lifetime issues, use a smaller capture set.
- The transparent fallback bg gets created every frame — wasteful but correct. Plan 5 will cache it.

If frame loop won't compile/run, report DONE_WITH_CONCERNS with the specific error and we'll figure out the right shape together.

- [ ] **Step 2: Build**

`cargo build -p ui-glass-dioxus`
Expected: success.

- [ ] **Step 3: Commit**

```bash
git add crates/ui-glass-dioxus/src/component.rs
git commit -m "feat(ui-glass-dioxus): wire frame loop to wgpu surface"
```

---

## Task 6: Motion bridge — pointer/scroll/reduced-motion event listeners

**Files:**
- Modify: `crates/ui-glass-dioxus/src/motion_bridge.rs`
- Modify: `crates/ui-glass-dioxus/src/component.rs`

- [ ] **Step 1: Implement `motion_bridge.rs`**

Replace with:

```rust
//! Reads pointer position, scroll velocity, and `prefers-reduced-motion`,
//! and exposes them as `MotionInputs` for the compositor.

use ui_glass_engine::motion::MotionInputs;

#[derive(Clone, Copy, Debug, Default)]
pub struct MotionState {
    pub pointer_px: [f32; 2],
    pub scroll_velocity_px: [f32; 2],
    pub reduced_motion: bool,
}

impl MotionState {
    /// Build a `MotionInputs` snapshot. `start_time_ms` is the loop start in
    /// performance.now() units; the resulting `time_seconds` is relative.
    pub fn to_motion_inputs(self, start_time_ms: f64) -> MotionInputs {
        let now = web_sys_performance_now();
        let elapsed_s = ((now - start_time_ms) / 1000.0) as f32;
        MotionInputs::new()
            .with_pointer(self.pointer_px)
            .with_scroll_velocity(self.scroll_velocity_px)
            .with_time(elapsed_s)
            .with_reduced_motion(self.reduced_motion)
    }
}

#[cfg(target_arch = "wasm32")]
fn web_sys_performance_now() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
fn web_sys_performance_now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

/// Attach pointer + scroll + matchMedia listeners that update the given
/// motion_state signal. Returns a `MotionListenersGuard` whose drop removes
/// the listeners. Web-only.
#[cfg(target_arch = "wasm32")]
pub fn attach_listeners(
    canvas: &web_sys::HtmlCanvasElement,
    mut motion_state: dioxus::prelude::Signal<MotionState>,
) -> MotionListenersGuard {
    use gloo_events::EventListener;
    use wasm_bindgen::JsCast;

    let canvas = canvas.clone();
    let pointer = EventListener::new(&canvas, "pointermove", move |evt| {
        if let Some(e) = evt.dyn_ref::<web_sys::PointerEvent>() {
            let rect = canvas.get_bounding_client_rect();
            let x = (e.client_x() as f64 - rect.left()) as f32;
            let y = (e.client_y() as f64 - rect.top()) as f32;
            motion_state.with_mut(|s| s.pointer_px = [x, y]);
        }
    });

    let window = web_sys::window().expect("window");
    let scroll_state = std::rc::Rc::new(std::cell::RefCell::new(0.0f64));
    let scroll_state_clone = scroll_state.clone();
    let scroll = EventListener::new(&window, "scroll", move |_| {
        if let Some(w) = web_sys::window() {
            let y = w.scroll_y().unwrap_or(0.0);
            let mut prev = scroll_state_clone.borrow_mut();
            let dy = (y - *prev) as f32;
            *prev = y;
            motion_state.with_mut(|s| s.scroll_velocity_px = [0.0, dy]);
        }
    });

    // Reduced-motion preference
    let media = window
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten();
    if let Some(mql) = &media {
        let initial = mql.matches();
        motion_state.with_mut(|s| s.reduced_motion = initial);
    }
    let media_listener = media.as_ref().map(|mql| {
        EventListener::new(mql, "change", move |evt| {
            if let Some(e) = evt.dyn_ref::<web_sys::MediaQueryListEvent>() {
                let on = e.matches();
                motion_state.with_mut(|s| s.reduced_motion = on);
            }
        })
    });

    MotionListenersGuard {
        _pointer: pointer,
        _scroll: scroll,
        _media: media_listener,
    }
}

#[cfg(target_arch = "wasm32")]
pub struct MotionListenersGuard {
    _pointer: gloo_events::EventListener,
    _scroll: gloo_events::EventListener,
    _media: Option<gloo_events::EventListener>,
}
```

- [ ] **Step 2: Attach listeners in `handle_canvas_mounted`**

Modify `handle_canvas_mounted` in `component.rs` to also call `attach_listeners` after `surface_state.set(...)`:

```rust
        spawn(async move {
            if let Some(state) = SurfaceState::from_canvas(canvas_clone.clone(), physical_size).await {
                surface_state.set(Some(state));
                let _guard = crate::motion_bridge::attach_listeners(&canvas_clone, motion_state);
                std::mem::forget(_guard); // Plan 4 leak — Plan 5 cleans up
                start_frame_loop(surface_state, motion_state, material, background, rect, width, height);
            }
        });
```

**Escalation:** `gloo-events::EventListener` lifetime semantics on wasm32 require the closure to live as long as the listener. Forgetting the guard makes events fire for the lifetime of the page — acceptable for Plan 4 (lifetime ~= app session). Plan 5 will properly cleanup on unmount.

- [ ] **Step 3: Build**

`cargo build -p ui-glass-dioxus`

- [ ] **Step 4: Commit**

```bash
git add crates/ui-glass-dioxus/src/motion_bridge.rs crates/ui-glass-dioxus/src/component.rs
git commit -m "feat(ui-glass-dioxus): pointer/scroll/reduced-motion bridge"
```

---

## Task 7: Native stub

**Files:**
- Modify: `crates/ui-glass-dioxus/src/stub.rs`
- Modify: `crates/ui-glass-dioxus/src/surface_state.rs` (add non-wasm32 type)

- [ ] **Step 1: Add non-wasm32 SurfaceState placeholder**

In `surface_state.rs`, add at the bottom:

```rust
#[cfg(not(target_arch = "wasm32"))]
impl SurfaceState {
    pub fn resize(&mut self, _physical_size: (u32, u32)) {}
}
```

(The struct is already cfg-gated; ensure it compiles on non-wasm32 by either adding a `cfg(target_arch = "wasm32")` gate around the whole struct, or providing a stub field-free struct for non-wasm.)

Cleanest: gate the struct entirely on wasm32:

```rust
#[cfg(target_arch = "wasm32")]
pub struct SurfaceState { ... }

#[cfg(not(target_arch = "wasm32"))]
pub struct SurfaceState;

#[cfg(not(target_arch = "wasm32"))]
impl SurfaceState {
    pub fn resize(&mut self, _physical_size: (u32, u32)) {}
}
```

- [ ] **Step 2: Implement stub.rs**

```rust
#![cfg(not(target_arch = "wasm32"))]
//! Native placeholder. On non-wasm32 the LiquidSurface component still
//! compiles and renders a `<div>` placeholder, but doesn't initialize any
//! GPU resources. Full native integration (via Blitz/Freya) is a separate
//! plan.
```

- [ ] **Step 3: Build for native**

`cargo build -p ui-glass-dioxus`
Expected: success.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-glass-dioxus/src/surface_state.rs crates/ui-glass-dioxus/src/stub.rs
git commit -m "feat(ui-glass-dioxus): non-wasm32 placeholder"
```

---

## Task 8: SSR placeholder test

**Files:**
- Create: `crates/ui-glass-dioxus/tests/ssr_placeholder.rs`

- [ ] **Step 1: Write the test**

```rust
//! SSR renders the LiquidSurface as a div + canvas pair without running any
//! wgpu init. The output HTML is what the browser hydrates against.

use dioxus::prelude::*;
use ui_glass::LiquidMaterial;
use ui_glass_dioxus::{LiquidSurface, LiquidSurfaceProps};

#[test]
fn ssr_renders_div_and_canvas() {
    fn app() -> Element {
        rsx! {
            LiquidSurface {
                material: LiquidMaterial::floating(),
                width: 320,
                height: 200,
            }
        }
    }

    let html = dioxus_ssr::render_element(rsx! { app {} });
    assert!(html.contains("class=\"ui-liquid-surface\""), "missing wrapper class");
    assert!(html.contains("<canvas"), "missing canvas element");
    assert!(html.contains("width=\"320\""), "canvas width should match prop");
    assert!(html.contains("height=\"200\""), "canvas height should match prop");
}

#[test]
fn ssr_renders_foreground_children() {
    fn app() -> Element {
        rsx! {
            LiquidSurface {
                material: LiquidMaterial::floating(),
                width: 200,
                height: 100,
                "hello from inside"
            }
        }
    }

    let html = dioxus_ssr::render_element(rsx! { app {} });
    assert!(html.contains("hello from inside"));
}
```

- [ ] **Step 2: Run**

`cargo test -p ui-glass-dioxus --test ssr_placeholder`
Expected: 2 tests pass.

**Escalation:** Dioxus SSR rendering of components with `onmounted` handlers can sometimes panic during SSR because `MountedData` isn't available. If this happens, gate the `onmounted` handler on `cfg(target_arch = "wasm32")` only — SSR is non-wasm32, so the handler is skipped.

- [ ] **Step 3: Commit**

```bash
git add crates/ui-glass-dioxus/tests/ssr_placeholder.rs
git commit -m "test(ui-glass-dioxus): SSR placeholder renders div+canvas"
```

---

## Task 9: Component-gallery example

**Files:**
- Modify: `examples/component-gallery/src/gallery.rs` (or wherever the gallery aggregates examples)
- Modify: `examples/component-gallery/Cargo.toml` (add ui-glass-dioxus dep)

- [ ] **Step 1: Add dep**

Edit `examples/component-gallery/Cargo.toml`:

```toml
ui-glass-dioxus.workspace = true
```

- [ ] **Step 2: Add an example**

Find a sensible insertion point in the gallery (look at where other components are registered) and add:

```rust
use ui_glass_dioxus::LiquidSurface;
use ui_glass::{AmbientMesh, LiquidMaterial};
use ui_glass_engine::background::{BackgroundSource, Gradient, GradientStop, MeshKind};
use ui_tokens::Color;

// ... inside the gallery RSX:
LiquidSurface {
    material: LiquidMaterial::floating()
        .ambient_mesh(AmbientMesh::Aurora)
        .pointer_reactive()
        .radius(24.0)
        .tint(Color::rgba(255, 255, 255, 1.0), 0.18),
    background: BackgroundSource::Gradient(Gradient::conic(
        [0.5, 0.5], 0.0,
        vec![
            GradientStop { offset: 0.0, color: Color::rgba(80, 100, 220, 1.0) },
            GradientStop { offset: 0.5, color: Color::rgba(180, 80, 180, 1.0) },
            GradientStop { offset: 1.0, color: Color::rgba(80, 100, 220, 1.0) },
        ],
    )),
    width: 400,
    height: 240,
    h2 { "Liquid Glass" }
    p { "Pointer-reactive, refractive, Aurora-meshed" }
}
```

**Escalation:** The gallery's exact registration pattern depends on `examples/component-gallery/src/gallery.rs`. Read the file to find the right spot. If you can't figure out where to put it, add a standalone `liquid_glass.rs` page module and link it from the main gallery index — same pattern other pages follow.

- [ ] **Step 3: Build the gallery**

`cargo build -p component-gallery`
Expected: success.

- [ ] **Step 4: Commit**

```bash
git add examples/component-gallery
git commit -m "example(component-gallery): LiquidSurface showcase"
```

---

## Task 10: ui-dioxus re-export

**Files:**
- Modify: `crates/ui-dioxus/src/lib.rs`
- Modify: `crates/ui-dioxus/Cargo.toml`

- [ ] **Step 1: Add dep**

Edit `crates/ui-dioxus/Cargo.toml` to add `ui-glass-dioxus.workspace = true` in `[dependencies]`.

- [ ] **Step 2: Re-export**

In `crates/ui-dioxus/src/lib.rs`, after the existing `pub use` block, add:

```rust
pub use ui_glass_dioxus::{LiquidSurface, LiquidSurfaceProps};
```

- [ ] **Step 3: Build**

`cargo build -p ui-dioxus`

- [ ] **Step 4: Commit**

```bash
git add crates/ui-dioxus/src/lib.rs crates/ui-dioxus/Cargo.toml
git commit -m "feat(ui-dioxus): re-export LiquidSurface"
```

---

## Task 11: Workspace integration check + Plan 4 status

**Files:**
- Modify: `docs/superpowers/plans/2026-05-22-liquid-glass-plan-4-dioxus-component.md`

- [ ] **Step 1: Workspace build**

`cargo build --workspace`
Expected: success.

- [ ] **Step 2: Workspace test**

`cargo test --workspace --features ui-glass-engine/headless`
Expected: all tests pass, plus the new ssr_placeholder tests.

- [ ] **Step 3: Append status**

At the bottom of `docs/superpowers/plans/2026-05-22-liquid-glass-plan-4-dioxus-component.md`:

```markdown
---

## Status

Plan 4 complete. `<LiquidSurface>` mounts a wgpu-rendered glass canvas
via Dioxus on web/desktop-WebView/mobile-WebView. Pointer, scroll, and
reduced-motion flow into the compositor. SSR renders a hydration-safe
div+canvas placeholder. Component-gallery has a live example. Native
(non-wasm32) integration via Blitz is acknowledged as a future plan
beyond Plan 5's scope.

Known limitations (deferred to Plan 5):
- Frame loop handle and event listeners are leaked rather than tied to
  component unmount lifecycle.
- The transparent fallback bg texture is allocated every frame instead
  of cached on the compositor.
- Native target compiles but renders a div placeholder — Blitz/wgpu
  direct integration is not yet wired.

Next: Plan 5 — Degradation ladder (WebGL2/SVG/Solid fallback) + Phase
1-3 migration of existing `<GlassSurface>` to route through the engine.
```

- [ ] **Step 4: Commit**

```bash
git add docs/superpowers/plans/2026-05-22-liquid-glass-plan-4-dioxus-component.md
git commit -m "docs: mark Plan 4 status and Plan 5 handoff"
```

---

## Plan 4 — Done. What's next

The engine is now reachable from any Dioxus app: drop a `<LiquidSurface>`, declare material + background, get a wgpu-rendered glass canvas. Per-frame motion inputs flow through automatically. Multi-region routes are supported by mounting multiple `<LiquidSurface>` components (each gets its own canvas + compositor — Plan 5's "per-route compositor sharing" can optimize this later).

Plan 5 will:
- Add the WebGL2/SVG/Solid degradation ladder for browsers without WebGPU.
- Wire `Compositor::set_quality_profile(...)` and capability detection.
- Phase 1 migration: `<GlassSurface>` (the existing CSS-based component) auto-upgrades to route through the engine.
- Phase 2: `liquid-glass` engine becomes default-on.
- Phase 3: `ui-styles` reduced to fallback-only.
- Optionally: cleanup the Plan 4 leaks (frame loop handle + listener guards tied to component unmount).
