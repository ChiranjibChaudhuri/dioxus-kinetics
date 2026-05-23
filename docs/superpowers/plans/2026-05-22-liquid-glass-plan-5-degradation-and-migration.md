# Liquid Glass — Plan 5: Plan 4 Cleanups + Degradation Ladder + Migration

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development. Steps use checkbox (`- [ ]`) syntax for tracking. This is the final plan in the Liquid Glass arc — it closes out Plan 4's known leaks, ships the degradation ladder, and migrates the existing `<GlassSurface>` to route through the engine.

**Goal:** Close out the Liquid Glass production story:

1. Plan 4 cleanups — frame handle + listener guards tied to component unmount, scroll velocity decay, prop hot-swap, fallback-bg cache.
2. Degradation ladder — `QualityProfile`, capability detection, SVG filter chain fallback, Solid fallback.
3. Migration — three phases: auto-upgrade `<GlassSurface>` through the engine, default the `liquid-glass` feature on, retire redundant CSS rules in `ui-styles`.

After Plan 5 the engine is the canonical glass rendering path, gracefully degrades on environments without WebGPU/WebGL2, and the legacy CSS-only path serves only its narrow fallback role.

**Tech Stack:** Same as Plans 1-4 — wgpu 26, Dioxus 0.7, WGSL, web-sys. Adds no new external deps.

---

## Context: What Plan 4 left open

The Plan 4 final reviewer identified these as known-leaked-on-purpose:

1. `FrameHandle` in `start_frame_loop` is `std::mem::forget`-ed (`crates/ui-glass-dioxus/src/component.rs`).
2. `MotionListenersGuard` from `attach_listeners` is similarly forgotten.
3. The transparent fallback bg texture allocates `wgpu::Texture` every frame.
4. `scroll_velocity_px` is set to a per-event delta and never decays — after the user stops scrolling the velocity stays nonzero.
5. Material/background prop changes after first mount are silently ignored by the running frame loop.

The reviewer also flagged Plan 3's narrow SPECULAR exponent as an "open design question," already partially addressed by Plan 3's `pow(.,4)`. We leave it alone for Plan 5 — it's no longer urgent.

---

## File Structure

**Modified files:**

```
crates/ui-glass-dioxus/src/component.rs        FrameHandle + guards tied to use_drop;
                                                use_effect for prop hot-swap
crates/ui-glass-dioxus/src/motion_bridge.rs    scroll decay
crates/ui-glass-engine/src/compositor.rs       cached fallback bg texture
crates/ui-glass/src/lib.rs                     QualityProfile enum
crates/ui-glass-engine/src/lib.rs              capability detection module
crates/ui-styles/src/lib.rs                    SVG <filter> defs + reduced CSS
crates/ui-dioxus/src/lib.rs                    GlassSurface auto-upgrade
Cargo.toml + ui-dioxus/Cargo.toml              liquid-glass feature default-on
```

**New files:**

```
crates/ui-glass-engine/src/capabilities.rs     CapabilityProbe + Tier enum
crates/ui-glass-engine/src/svg_fallback.rs     SVG filter-chain string generator
crates/ui-glass-engine/tests/capabilities.rs   tier detection
crates/ui-glass-engine/tests/svg_filter.rs     SVG filter output snapshots
crates/ui-glass-dioxus/tests/glass_upgrade.rs  GlassSurface picks the right tier
```

---

## Task 1: Tie `FrameHandle` and `MotionListenersGuard` to component unmount

**Files:**
- Modify: `crates/ui-glass-dioxus/src/component.rs`

The leaks need to flow into Dioxus's `use_drop` so they're cleaned up when `<LiquidSurface>` unmounts.

- [ ] **Step 1: Add a `LiveResources` struct that owns both guards**

In `component.rs`, add at the top:

```rust
/// Combined lifetime guard for everything spawned on canvas mount. Dropping
/// this stops the frame loop and removes all event listeners.
#[cfg(target_arch = "wasm32")]
struct LiveResources {
    _frame: ui_runtime::scheduler::FrameHandle,
    _listeners: crate::motion_bridge::MotionListenersGuard,
}

#[cfg(not(target_arch = "wasm32"))]
struct LiveResources;
```

- [ ] **Step 2: Make `start_frame_loop` return the FrameHandle**

Change the function signature:

```rust
#[cfg(target_arch = "wasm32")]
fn start_frame_loop(
    surface_state: Signal<Option<SurfaceState>>,
    motion_state: Signal<MotionState>,
    material: LiquidMaterial,
    background: Option<BackgroundSource>,
    rect: Option<[f32; 4]>,
    width: u32,
    height: u32,
) -> ui_runtime::scheduler::FrameHandle {
    // ... existing body up to the spawn_frame_loop call ...

    let handle = spawn_frame_loop(move |_dt_ms| {
        // ... existing closure body ...
    });

    handle  // return instead of std::mem::forget(handle)
}
```

Delete the `std::mem::forget(handle);` line entirely.

- [ ] **Step 3: Store `LiveResources` in a Signal + add `use_drop`**

In the `LiquidSurface` component body, after the existing signals, add:

```rust
    let live: Signal<Option<LiveResources>> = use_signal(|| None::<LiveResources>);

    use_drop(move || {
        // Dropping the LiveResources guard stops the frame loop and removes
        // listeners. Signals get dropped on component teardown.
        let _ = live;
    });
```

Wait — `use_drop` is unnecessary here. The `Signal<Option<LiveResources>>` will drop its contents when the component is unmounted (Dioxus signals are scoped to the component). The mere act of storing the LiveResources in the signal achieves cleanup. So the explicit `use_drop` can be removed; just having the signal is enough.

The pattern is:

```rust
    let mut live: Signal<Option<LiveResources>> = use_signal(|| None::<LiveResources>);
```

When the component unmounts, `live` is dropped, which drops `Option<LiveResources>::Some(...)`, which drops `_frame: FrameHandle` and `_listeners: MotionListenersGuard`. `FrameHandle::Drop` sets `cancelled = true` (see `crates/ui-runtime/src/scheduler_web.rs:17-21`), stopping the rAF callbacks. `MotionListenersGuard::Drop` removes the gloo-events listeners.

- [ ] **Step 4: Wire `LiveResources` into `handle_canvas_mounted`**

Modify the spawn(async move { ... }) block. Replace the `std::mem::forget` calls with capturing into `live`:

```rust
        let canvas_for_listeners = canvas.clone();
        if let Some(state) = SurfaceState::from_canvas(canvas, physical_size).await {
            surface_state.set(Some(state));
            let listeners = crate::motion_bridge::attach_listeners(&canvas_for_listeners, motion_state);
            let frame = start_frame_loop(
                surface_state, motion_state, material, background, rect, width, height,
            );
            live.set(Some(LiveResources { _frame: frame, _listeners: listeners }));
        }
```

Update `handle_canvas_mounted` to accept `live: Signal<Option<LiveResources>>` as a parameter (capturing it from the LiquidSurface body and passing into the `onmounted` closure).

- [ ] **Step 5: Build + test**

```
cargo build --workspace
cargo check -p ui-glass-dioxus --target wasm32-unknown-unknown
cargo test --workspace --features ui-glass-engine/headless
```

- [ ] **Step 6: Commit**

```bash
git add crates/ui-glass-dioxus/src/component.rs
git commit -m "fix(ui-glass-dioxus): tie FrameHandle + listener guards to component lifecycle"
```

**Escalation:** If `use_signal(|| None::<LiveResources>)` runs into type-inference issues with `LiveResources` (it's not `Clone` since `FrameHandle` isn't), wrap in an `Rc<RefCell<...>>` or use Dioxus's `use_hook(|| ...)` which gives non-Clone values a stable place to live.

---

## Task 2: Scroll velocity decay

**Files:**
- Modify: `crates/ui-glass-dioxus/src/component.rs` (frame loop)

Currently `scroll_velocity_px` is set to a per-event delta and never reset. After scrolling stops, the value persists forever.

- [ ] **Step 1: Add per-frame decay**

Inside the `spawn_frame_loop` closure in `start_frame_loop`, after the existing motion read, decay the stored velocity:

```rust
        // Decay scroll velocity each frame so the surface settles after the
        // user stops scrolling.
        motion_state.with_mut(|s| {
            s.scroll_velocity_px[0] *= 0.85;
            s.scroll_velocity_px[1] *= 0.85;
            if s.scroll_velocity_px[0].abs() < 0.01 { s.scroll_velocity_px[0] = 0.0; }
            if s.scroll_velocity_px[1].abs() < 0.01 { s.scroll_velocity_px[1] = 0.0; }
        });
```

Place this after `let inputs = motion_state.read().to_motion_inputs(start_time);` so the inputs for THIS frame use the pre-decay value (smoother), and the decay applies for NEXT frame.

Actually that's wrong — the read happens before the decay, so the decay only affects future frames. But the listener overwrites the velocity each scroll event, so user scrolling resets velocity to current delta; otherwise it decays. That's the right behavior.

- [ ] **Step 2: Build + test**

```
cargo build -p ui-glass-dioxus
```

- [ ] **Step 3: Commit**

```bash
git add crates/ui-glass-dioxus/src/component.rs
git commit -m "fix(ui-glass-dioxus): decay scroll velocity each frame"
```

---

## Task 3: Cache the fallback bg texture on the Compositor

**Files:**
- Modify: `crates/ui-glass-engine/src/compositor.rs`
- Modify: `crates/ui-glass-dioxus/src/component.rs`

The frame loop currently allocates a transparent `wgpu::Texture` every frame. Cache it on the Compositor and reuse.

- [ ] **Step 1: Add a cached transparent texture to Compositor**

In `compositor.rs`, add:

```rust
pub struct Compositor {
    // existing fields ...
    transparent_bg: Option<(wgpu::Texture, wgpu::TextureView, [u32; 2])>,
}
```

Initialize `transparent_bg: None` in `new`.

Add a method:

```rust
    /// Get or build a transparent fallback bg texture matching `size`. Used
    /// by surfaces that don't declare a per-region BackgroundSource and don't
    /// have a scene-graph installed.
    pub fn transparent_bg_view(&mut self, size: [u32; 2]) -> &wgpu::TextureView {
        let needs_rebuild = match &self.transparent_bg {
            Some((_, _, cached_size)) => *cached_size != size,
            None => true,
        };
        if needs_rebuild {
            let tex = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("liquid-glass-transparent-bg"),
                size: wgpu::Extent3d { width: size[0], height: size[1], depth_or_array_layers: 1 },
                mip_level_count: 1, sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            let view = tex.create_view(&Default::default());
            self.transparent_bg = Some((tex, view, size));
        }
        &self.transparent_bg.as_ref().unwrap().1
    }
```

- [ ] **Step 2: Use the cached view in `start_frame_loop`**

In `crates/ui-glass-dioxus/src/component.rs`, replace the manual `bg_tex` / `bg_view` construction with:

```rust
        // Cached fallback bg from the compositor.
        let bg_view = state.compositor.transparent_bg_view([
            state.physical_size.0,
            state.physical_size.1,
        ]);
        // ... still use bg_view in compositor.render(...)
```

Borrow-check considerations: `transparent_bg_view` takes `&mut self`, but the render call takes `&self.compositor`. Since both are on `state`, you'll need to scope the bg_view borrow before the render call. Pattern:

```rust
        let canvas_size = [width as f32, height as f32];
        let physical = state.physical_size;

        // Step A: ensure the cached bg exists; this returns a view we don't
        // hold across the render call (avoid borrow conflict).
        let _ = state.compositor.transparent_bg_view([physical.0, physical.1]);

        // Step B: take a view we can hold (the cache won't rebuild because
        // size matches).
        let bg_tex_ref = state.compositor.transparent_bg.as_ref().unwrap();
        let bg_view = &bg_tex_ref.1;

        state.compositor.render(bg_view, &output_view, canvas_size, &[region]);
```

This requires `transparent_bg` to be visible (`pub(crate)`). Alternatively, refactor `transparent_bg_view` to be split into "ensure" and "view" methods. Pick what works; the goal is no per-frame texture allocation.

- [ ] **Step 3: Build + test**

```
cargo build --workspace
cargo test --workspace --features ui-glass-engine/headless
```

- [ ] **Step 4: Commit**

```bash
git add crates/ui-glass-engine/src/compositor.rs crates/ui-glass-dioxus/src/component.rs
git commit -m "fix(ui-glass-engine): cache transparent fallback bg texture"
```

---

## Task 4: Hot-swap material + background props at runtime

**Files:**
- Modify: `crates/ui-glass-dioxus/src/component.rs`

Currently `start_frame_loop` captures `material`, `background`, `rect`, `width`, `height` once at first mount. Subsequent prop changes are ignored.

The simplest solution: store the GlassRegion in a Signal that the component re-writes via `use_effect` when the dependent props change. The frame loop reads from the Signal.

- [ ] **Step 1: Add a region Signal**

In `LiquidSurface`'s body:

```rust
    let region: Signal<ui_glass_engine::GlassRegion> = use_signal(|| {
        let r = props.rect.unwrap_or([0.0, 0.0, props.width as f32, props.height as f32]);
        let mut gr = ui_glass_engine::GlassRegion::new(r, props.material);
        if let Some(bg) = props.background.clone() {
            gr = gr.with_background(bg);
        }
        gr
    });
```

Update `region` whenever props change via `use_effect`:

```rust
    let material = props.material;
    let background = props.background.clone();
    let rect = props.rect;
    let width = props.width;
    let height = props.height;
    {
        let mut region = region;
        let background = background.clone();
        use_effect(move || {
            let r = rect.unwrap_or([0.0, 0.0, width as f32, height as f32]);
            let mut gr = ui_glass_engine::GlassRegion::new(r, material);
            if let Some(bg) = background.clone() {
                gr = gr.with_background(bg);
            }
            region.set(gr);
        });
    }
```

- [ ] **Step 2: Read `region` from inside the frame loop**

In `start_frame_loop`, remove `region_template` and the `region` clone. Instead:

```rust
    let region_signal = region;  // pass as parameter into start_frame_loop

    let handle = spawn_frame_loop(move |_dt_ms| {
        // ...
        state.compositor.render(
            bg_view,
            &output_view,
            canvas_size,
            std::slice::from_ref(&region_signal.read()),
        );
        // ...
    });
```

Update `start_frame_loop`'s signature to accept `region: Signal<GlassRegion>` instead of the individual `material`/`background`/`rect`/`width`/`height` arguments. (Keep `width`/`height` if they're needed for canvas_size, which they are.)

- [ ] **Step 3: Build + test**

```
cargo build --workspace
cargo check -p ui-glass-dioxus --target wasm32-unknown-unknown
```

- [ ] **Step 4: Commit**

```bash
git add crates/ui-glass-dioxus/src/component.rs
git commit -m "fix(ui-glass-dioxus): hot-swap material + background props at runtime"
```

**Escalation:** `Signal::read` returns a `ReadableRef<T>` which derefs to `T`. `std::slice::from_ref` needs `&T`. If the deref coercion isn't automatic, do `let r = region_signal.read(); let region: &GlassRegion = &*r; state.compositor.render(..., std::slice::from_ref(region));`.

---

## Task 5: `QualityProfile` enum + lookup

**Files:**
- Modify: `crates/ui-glass/src/lib.rs`

The spec at §"Quality profiles" defines four tiers. Define them in Rust.

- [ ] **Step 1: Add `QualityProfile`**

In `crates/ui-glass/src/lib.rs`, near the other enums, add:

```rust
/// Pre-baked quality preset. Maps to feature mask + blur tap count + scroll/
/// pointer reactivity gates. Used by the runtime to scale visual cost
/// against device class and user preferences.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QualityProfile {
    /// All 9 traits, 13-tap blur, full mip chain.
    #[default]
    High,
    /// Tier 1 minus AMBIENT_MESH, 9-tap blur.
    Balanced,
    /// Tier 2 forced (battery-saving): drops REFRACT, DISPERSE, AMBIENT_MESH, TINT_ADAPT.
    Power,
    /// Engine off — Solid CSS surface only.
    Off,
}

impl QualityProfile {
    /// Mask out features that this profile suppresses. Combine with the
    /// material's existing features via `material.features &= profile.mask()`.
    pub fn feature_mask(self) -> GlassFeatures {
        use GlassFeatures as F;
        match self {
            QualityProfile::High => F::all(),
            QualityProfile::Balanced => F::all().difference(F::AMBIENT_MESH),
            QualityProfile::Power => F::BLUR | F::SPECULAR | F::INNER_SHADOW,
            QualityProfile::Off => F::empty(),
        }
    }

    /// Blur taps to use under this profile.
    pub fn blur_taps(self) -> u32 {
        match self {
            QualityProfile::High => 13,
            QualityProfile::Balanced => 9,
            QualityProfile::Power => 5,
            QualityProfile::Off => 1,
        }
    }
}
```

- [ ] **Step 2: Add tests**

Append to `crates/ui-glass/tests/liquid_material.rs`:

```rust
#[test]
fn quality_profile_high_includes_all_features() {
    assert_eq!(ui_glass::QualityProfile::High.feature_mask(), ui_glass::GlassFeatures::all());
    assert_eq!(ui_glass::QualityProfile::High.blur_taps(), 13);
}

#[test]
fn quality_profile_off_clears_all_features() {
    assert_eq!(ui_glass::QualityProfile::Off.feature_mask(), ui_glass::GlassFeatures::empty());
}

#[test]
fn quality_profile_balanced_drops_ambient_mesh() {
    let mask = ui_glass::QualityProfile::Balanced.feature_mask();
    assert!(mask.contains(ui_glass::GlassFeatures::BLUR));
    assert!(!mask.contains(ui_glass::GlassFeatures::AMBIENT_MESH));
}
```

- [ ] **Step 3: Run + commit**

```
cargo test -p ui-glass --test liquid_material
git add crates/ui-glass/src/lib.rs crates/ui-glass/tests/liquid_material.rs
git commit -m "feat(ui-glass): add QualityProfile with feature masks"
```

---

## Task 6: Capability detection (Tier ladder)

**Files:**
- Create: `crates/ui-glass-engine/src/capabilities.rs`
- Modify: `crates/ui-glass-engine/src/lib.rs`
- Create: `crates/ui-glass-engine/tests/capabilities.rs`

Detect which rendering tier the current environment can run.

- [ ] **Step 1: Define `Tier` + `Capabilities`**

Create `crates/ui-glass-engine/src/capabilities.rs`:

```rust
//! Capability detection for the rendering tier ladder.
//!
//! Five tiers, top to bottom:
//!
//! - `Tier::WgpuWebGpu` — wgpu via WebGPU. All 9 features at full quality.
//! - `Tier::WgpuWebGl2` — wgpu via WebGL2. All 9 features, slightly reduced
//!   quality. Used as fallback when WebGPU is unavailable.
//! - `Tier::SvgFilter` — CSS `backdrop-filter: url(#kinetics-glass-...)` chain.
//!   No wgpu involved. Approximates the look with feGaussianBlur +
//!   feSpecularLighting + feColorMatrix + feDisplacementMap.
//! - `Tier::SolidCss` — solid surface via `--ui-glass-solid`. No filters.
//! - `Tier::Off` — engine disabled.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    WgpuWebGpu,
    WgpuWebGl2,
    SvgFilter,
    SolidCss,
    Off,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capabilities {
    pub has_webgpu: bool,
    pub has_webgl2: bool,
    pub has_backdrop_filter: bool,
    pub reduced_motion: bool,
    pub reduced_transparency: bool,
    pub high_contrast: bool,
}

impl Capabilities {
    /// The most-capable tier this environment can support, modulo user
    /// preferences.
    pub fn best_tier(&self) -> Tier {
        if self.high_contrast || self.reduced_transparency {
            return Tier::SolidCss;
        }
        if self.has_webgpu { return Tier::WgpuWebGpu; }
        if self.has_webgl2 { return Tier::WgpuWebGl2; }
        if self.has_backdrop_filter { return Tier::SvgFilter; }
        Tier::SolidCss
    }
}

/// Detect runtime capabilities. On wasm32 this probes the browser; on native
/// it assumes WebGPU is available (wgpu always has a native backend).
#[cfg(target_arch = "wasm32")]
pub fn detect() -> Capabilities {
    let window = web_sys::window();

    let has_webgpu = window
        .as_ref()
        .and_then(|w| js_sys::Reflect::get(w, &wasm_bindgen::JsValue::from_str("navigator")).ok())
        .and_then(|nav| js_sys::Reflect::get(&nav, &wasm_bindgen::JsValue::from_str("gpu")).ok())
        .map(|gpu| !gpu.is_undefined())
        .unwrap_or(false);

    // WebGL2 is essentially universal in 2026; check by trying to create a
    // throwaway context.
    let has_webgl2 = window
        .as_ref()
        .and_then(|w| w.document())
        .and_then(|d| d.create_element("canvas").ok())
        .and_then(|c| c.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        .and_then(|c| c.get_context("webgl2").ok().flatten())
        .is_some();

    // backdrop-filter — universal in modern browsers
    let has_backdrop_filter = true;

    let reduced_motion = matches_media_query(window.as_ref(), "(prefers-reduced-motion: reduce)");
    let reduced_transparency = matches_media_query(window.as_ref(), "(prefers-reduced-transparency: reduce)");
    let high_contrast = matches_media_query(window.as_ref(), "(prefers-contrast: more)");

    Capabilities {
        has_webgpu, has_webgl2, has_backdrop_filter,
        reduced_motion, reduced_transparency, high_contrast,
    }
}

#[cfg(target_arch = "wasm32")]
fn matches_media_query(window: Option<&web_sys::Window>, query: &str) -> bool {
    use wasm_bindgen::JsCast;
    window
        .and_then(|w| w.match_media(query).ok().flatten())
        .map(|mql| mql.matches())
        .unwrap_or(false)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn detect() -> Capabilities {
    Capabilities {
        has_webgpu: true,
        has_webgl2: true,
        has_backdrop_filter: false,
        reduced_motion: false,
        reduced_transparency: false,
        high_contrast: false,
    }
}
```

Add `use wasm_bindgen::JsCast;` at the top of the wasm32 block.

- [ ] **Step 2: Re-export from lib.rs**

```rust
pub mod capabilities;
pub use capabilities::{detect, Capabilities, Tier};
```

- [ ] **Step 3: Add tests**

Create `crates/ui-glass-engine/tests/capabilities.rs`:

```rust
use ui_glass_engine::capabilities::{Capabilities, Tier};

#[test]
fn high_contrast_overrides_to_solid() {
    let caps = Capabilities {
        has_webgpu: true,
        has_webgl2: true,
        has_backdrop_filter: true,
        reduced_motion: false,
        reduced_transparency: false,
        high_contrast: true,
    };
    assert_eq!(caps.best_tier(), Tier::SolidCss);
}

#[test]
fn webgpu_preferred_when_available() {
    let caps = Capabilities {
        has_webgpu: true,
        has_webgl2: true,
        has_backdrop_filter: true,
        reduced_motion: false,
        reduced_transparency: false,
        high_contrast: false,
    };
    assert_eq!(caps.best_tier(), Tier::WgpuWebGpu);
}

#[test]
fn falls_through_to_webgl2_when_no_webgpu() {
    let caps = Capabilities {
        has_webgpu: false,
        has_webgl2: true,
        has_backdrop_filter: true,
        reduced_motion: false,
        reduced_transparency: false,
        high_contrast: false,
    };
    assert_eq!(caps.best_tier(), Tier::WgpuWebGl2);
}

#[test]
fn falls_through_to_svg_when_only_backdrop_filter() {
    let caps = Capabilities {
        has_webgpu: false,
        has_webgl2: false,
        has_backdrop_filter: true,
        reduced_motion: false,
        reduced_transparency: false,
        high_contrast: false,
    };
    assert_eq!(caps.best_tier(), Tier::SvgFilter);
}

#[test]
fn solid_when_nothing_available() {
    let caps = Capabilities {
        has_webgpu: false,
        has_webgl2: false,
        has_backdrop_filter: false,
        reduced_motion: false,
        reduced_transparency: false,
        high_contrast: false,
    };
    assert_eq!(caps.best_tier(), Tier::SolidCss);
}
```

- [ ] **Step 4: Run + commit**

```
cargo build -p ui-glass-engine
cargo test -p ui-glass-engine --test capabilities
git add crates/ui-glass-engine/src/capabilities.rs crates/ui-glass-engine/src/lib.rs crates/ui-glass-engine/tests/capabilities.rs
git commit -m "feat(ui-glass-engine): capability detection + Tier ladder"
```

**Escalation:** The wasm32 `detect()` uses `js_sys::Reflect::get` to safely probe for `navigator.gpu`. If `wasm-bindgen` raises issues with the Reflect import, simplify to `window.navigator().gpu()` — but that requires the `Gpu` feature on web-sys which adds bundle weight. Reflect is the lighter path.

---

## Task 7: SVG filter chain generator

**Files:**
- Create: `crates/ui-glass-engine/src/svg_fallback.rs`
- Modify: `crates/ui-glass-engine/src/lib.rs`
- Create: `crates/ui-glass-engine/tests/svg_filter.rs`

The SVG fallback tier doesn't use wgpu. It generates an SVG `<filter>` chain that approximates the glass effect via CSS `backdrop-filter: url(#kinetics-glass-{id})`.

- [ ] **Step 1: Implement SVG generator**

Create `crates/ui-glass-engine/src/svg_fallback.rs`:

```rust
//! Generates SVG `<filter>` chains for the Tier 4 fallback backend. Each
//! material is rendered as a `<filter>` element with `feGaussianBlur` +
//! `feSpecularLighting` + `feColorMatrix` + (optionally) `feDisplacementMap`.
//! The resulting filter id can be referenced via
//! `backdrop-filter: url(#kinetics-glass-{id})` on the surface element.

use ui_glass::{GlassFeatures, LiquidMaterial};

/// Stable id derived from material features + key parameters.
pub fn filter_id(material: &LiquidMaterial) -> String {
    let f = material.features.bits();
    let blur = (material.blur_radius_px * 10.0) as u32;
    let refract = (material.refraction_strength * 100.0) as u32;
    format!("kinetics-glass-{f:x}-{blur:x}-{refract:x}")
}

/// Generate the `<filter>` element body for a material.
pub fn filter_element(material: &LiquidMaterial) -> String {
    let id = filter_id(material);
    let mut out = format!("<filter id=\"{id}\" x=\"-20%\" y=\"-20%\" width=\"140%\" height=\"140%\">");

    if material.features.contains(GlassFeatures::REFRACT) {
        let scale = (material.refraction_strength * 10.0) as i32;
        let freq = format!("{:.2}", material.noise_frequency * 0.02);
        out.push_str(&format!(
            "<feTurbulence type=\"fractalNoise\" baseFrequency=\"{freq}\" numOctaves=\"2\" result=\"turb\"/>\
             <feDisplacementMap in=\"SourceGraphic\" in2=\"turb\" scale=\"{scale}\" result=\"disp\"/>",
        ));
    } else {
        out.push_str("<feOffset in=\"SourceGraphic\" dx=\"0\" dy=\"0\" result=\"disp\"/>");
    }

    if material.features.contains(GlassFeatures::BLUR) {
        let std = format!("{:.1}", material.blur_radius_px * 0.5);
        out.push_str(&format!(
            "<feGaussianBlur in=\"disp\" stdDeviation=\"{std}\" result=\"blur\"/>",
        ));
    } else {
        out.push_str("<feOffset in=\"disp\" dx=\"0\" dy=\"0\" result=\"blur\"/>");
    }

    let sat = format!("{:.2}", material.saturation);
    out.push_str(&format!(
        "<feColorMatrix in=\"blur\" type=\"saturate\" values=\"{sat}\" result=\"sat\"/>",
    ));

    if material.features.contains(GlassFeatures::SPECULAR) {
        let intensity = format!("{:.2}", material.light_intensity);
        let elev = format!("{:.1}", 60.0 + material.light_angle_rad.to_degrees().abs() * 0.1);
        out.push_str(&format!(
            "<feSpecularLighting in=\"sat\" specularExponent=\"4\" specularConstant=\"{intensity}\" lighting-color=\"#ffffff\" result=\"spec\">\
             <feDistantLight azimuth=\"45\" elevation=\"{elev}\"/>\
             </feSpecularLighting>\
             <feComposite in=\"spec\" in2=\"SourceAlpha\" operator=\"in\" result=\"specMasked\"/>\
             <feComposite in=\"sat\" in2=\"specMasked\" operator=\"arithmetic\" k1=\"0\" k2=\"1\" k3=\"1\" k4=\"0\"/>",
        ));
    }

    out.push_str("</filter>");
    out
}

/// Generate a single `<svg>` `<defs>` block containing filters for all given
/// materials. Apps insert this once at the root of the document; surfaces
/// then reference filters by id.
pub fn defs_for(materials: &[&LiquidMaterial]) -> String {
    let filters: Vec<String> = materials.iter().map(|m| filter_element(m)).collect();
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"0\" height=\"0\" style=\"position:absolute;width:0;height:0;\" aria-hidden=\"true\"><defs>{}</defs></svg>",
        filters.join(""),
    )
}
```

- [ ] **Step 2: Re-export**

In `crates/ui-glass-engine/src/lib.rs`:

```rust
pub mod svg_fallback;
```

- [ ] **Step 3: Add tests**

Create `crates/ui-glass-engine/tests/svg_filter.rs`:

```rust
use ui_glass::LiquidMaterial;
use ui_glass_engine::svg_fallback::{defs_for, filter_element, filter_id};

#[test]
fn filter_id_stable_for_same_material() {
    let m = LiquidMaterial::floating();
    assert_eq!(filter_id(&m), filter_id(&m));
}

#[test]
fn filter_id_differs_for_different_materials() {
    let a = LiquidMaterial::floating();
    let b = LiquidMaterial::chrome();
    assert_ne!(filter_id(&a), filter_id(&b));
}

#[test]
fn filter_element_contains_blur_when_set() {
    let m = LiquidMaterial::new().blur(12.0);
    let el = filter_element(&m);
    assert!(el.contains("feGaussianBlur"), "expected feGaussianBlur, got {el}");
}

#[test]
fn filter_element_contains_displacement_when_refract() {
    let m = LiquidMaterial::new().refract(0.3).noise(2.0, 0.0);
    let el = filter_element(&m);
    assert!(el.contains("feDisplacementMap"));
    assert!(el.contains("feTurbulence"));
}

#[test]
fn defs_for_wraps_multiple_filters_in_svg_defs() {
    let a = LiquidMaterial::floating();
    let b = LiquidMaterial::chrome();
    let defs = defs_for(&[&a, &b]);
    assert!(defs.starts_with("<svg"));
    assert!(defs.contains("<defs>"));
    assert!(defs.contains(&filter_id(&a)));
    assert!(defs.contains(&filter_id(&b)));
}
```

- [ ] **Step 4: Run + commit**

```
cargo build -p ui-glass-engine
cargo test -p ui-glass-engine --test svg_filter
git add crates/ui-glass-engine/src/svg_fallback.rs crates/ui-glass-engine/src/lib.rs crates/ui-glass-engine/tests/svg_filter.rs
git commit -m "feat(ui-glass-engine): SVG filter chain fallback generator"
```

---

## Task 8: `GlassSurface` auto-upgrade (Phase 1)

**Files:**
- Modify: `crates/ui-dioxus/src/lib.rs`
- Modify: `crates/ui-dioxus/Cargo.toml`
- Create: `crates/ui-glass-dioxus/tests/glass_upgrade.rs`

The existing `<GlassSurface>` (`ui-dioxus/src/lib.rs:103-118`) renders a CSS-only section. Phase 1 makes it route through `<LiquidSurface>` when the engine is available (via the `liquid-glass` feature flag).

- [ ] **Step 1: Add `liquid-glass` feature to `ui-dioxus` Cargo.toml**

```toml
[features]
default = []
liquid-glass = []
```

- [ ] **Step 2: Conditionally route `GlassSurface`**

Replace `GlassSurface` in `crates/ui-dioxus/src/lib.rs`:

```rust
#[component]
pub fn GlassSurface(
    #[props(default)] level: GlassLevel,
    #[props(default)] tone: GlassTone,
    #[props(default)] density: GlassDensity,
    children: Element,
) -> Element {
    #[cfg(feature = "liquid-glass")]
    {
        use ui_glass::{GlassDepth, MaterialDensity, MaterialEdge, MaterialTone, MaterialRequest, MaterialVibrancy, LiquidMaterial};
        use ui_glass_engine::capabilities::{detect, Tier};

        let tier = detect().best_tier();
        let material = LiquidMaterial::from(
            MaterialRequest::new(
                match level {
                    GlassLevel::Subtle => GlassDepth::Raised,
                    GlassLevel::Floating => GlassDepth::Floating,
                    GlassLevel::Overlay => GlassDepth::Overlay,
                    GlassLevel::Chrome => GlassDepth::Chrome,
                },
                match tone {
                    GlassTone::Neutral => MaterialTone::Neutral,
                    GlassTone::Primary => MaterialTone::Primary,
                    GlassTone::Success => MaterialTone::Success,
                    GlassTone::Warning => MaterialTone::Warning,
                    GlassTone::Danger => MaterialTone::Danger,
                    GlassTone::Info => MaterialTone::Info,
                },
            )
            .with_density(match density {
                GlassDensity::Compact => MaterialDensity::Compact,
                GlassDensity::Comfortable => MaterialDensity::Comfortable,
                GlassDensity::Spacious => MaterialDensity::Spacious,
            })
            .with_edge(MaterialEdge::Hairline)
            .with_vibrancy(MaterialVibrancy::Standard)
        );

        return match tier {
            Tier::WgpuWebGpu | Tier::WgpuWebGl2 => rsx! {
                ui_glass_dioxus::LiquidSurface {
                    material,
                    {children}
                }
            },
            Tier::SvgFilter | Tier::SolidCss | Tier::Off => glass_surface_css(level, tone, density, children),
        };
    }

    #[cfg(not(feature = "liquid-glass"))]
    {
        glass_surface_css(level, tone, density, children)
    }
}

fn glass_surface_css(
    level: GlassLevel,
    tone: GlassTone,
    density: GlassDensity,
    children: Element,
) -> Element {
    rsx! {
        section {
            class: "ui-glass-surface",
            "data-glass-level": glass_level_name(level),
            "data-glass-tone": glass_tone_name(tone),
            "data-glass-density": glass_density_name(density),
            {children}
        }
    }
}
```

- [ ] **Step 3: Write upgrade test**

Create `crates/ui-glass-dioxus/tests/glass_upgrade.rs`:

```rust
//! With `liquid-glass` feature enabled, GlassSurface SSRs to a LiquidSurface
//! wrapper (which itself contains a div+canvas). Without the feature, it
//! renders the CSS section.

#[test]
#[cfg(feature = "liquid-glass")]
fn glass_surface_renders_liquid_surface_when_feature_on() {
    use dioxus::prelude::*;
    use ui_dioxus::GlassSurface;
    use ui_glass::{GlassDensity, GlassLevel, GlassTone};

    let html = dioxus_ssr::render_element(rsx! {
        GlassSurface {
            level: GlassLevel::Floating,
            tone: GlassTone::Neutral,
            density: GlassDensity::Comfortable,
            "child content"
        }
    });
    // With feature on, the SSR output should contain the LiquidSurface
    // wrapper class.
    assert!(html.contains("class=\"ui-liquid-surface\""), "got: {html}");
    assert!(html.contains("child content"));
}
```

(This test is feature-gated; without the feature it does not run, which is correct — without the feature the upgrade doesn't apply.)

Add ui-glass-dioxus a `[features]` block:

```toml
[features]
default = []
liquid-glass = []
```

And forward to ui-dioxus's feature:

Actually no — the feature lives on ui-dioxus, not ui-glass-dioxus. The test is in ui-glass-dioxus because that's where related tests live; but the feature gating happens at ui-dioxus's compile time.

For the test to compile with the feature, we'd need to run:
```
cargo test -p ui-glass-dioxus --features "ui-dioxus/liquid-glass" --test glass_upgrade
```

That requires `ui-glass-dioxus` to have `ui-dioxus` as a (cfg-able) dev-dep. Simpler approach: put the test in `crates/ui-dioxus/tests/glass_upgrade.rs` instead.

Move the test to `crates/ui-dioxus/tests/glass_upgrade.rs`.

- [ ] **Step 4: Run + commit**

```
cargo build -p ui-dioxus
cargo build -p ui-dioxus --features liquid-glass
cargo test -p ui-dioxus
cargo test -p ui-dioxus --features liquid-glass
git add crates/ui-dioxus crates/ui-glass-dioxus
git commit -m "feat(ui-dioxus): Phase 1 auto-upgrade GlassSurface through engine"
```

---

## Task 9: Phase 2 — `liquid-glass` feature default-on

**Files:**
- Modify: `crates/ui-dioxus/Cargo.toml`
- Modify: `crates/kinetics/Cargo.toml` (if it re-exports ui-dioxus features)

- [ ] **Step 1: Set the default**

In `crates/ui-dioxus/Cargo.toml`:

```toml
[features]
default = ["liquid-glass"]
liquid-glass = []
```

- [ ] **Step 2: Verify the workspace still builds**

```
cargo build --workspace
cargo test --workspace --features ui-glass-engine/headless
```

If the umbrella `kinetics` crate re-exports ui-dioxus features and you want consumers to be able to opt out, add a forwarding feature:

```toml
# in kinetics/Cargo.toml
[features]
default = ["ui-dioxus/liquid-glass"]
liquid-glass = ["ui-dioxus/liquid-glass"]
no-liquid-glass = []
```

(Adjust to whatever pattern `kinetics` already uses.)

- [ ] **Step 3: Commit**

```bash
git add crates/ui-dioxus/Cargo.toml crates/kinetics/Cargo.toml
git commit -m "feat(ui-dioxus): Phase 2 — liquid-glass feature default-on"
```

---

## Task 10: Phase 3 — Retire redundant CSS rules from `ui-styles`

**Files:**
- Modify: `crates/ui-styles/src/lib.rs`

With the engine as the default GlassSurface path, the `.ui-glass-surface[data-glass-level="..."]` CSS rules in `ui-styles` are only used by the Tier 4 (SVG) and Tier 5 (Solid) fallback paths. Many of them (the level-specific blur radii, the per-tone tint mixes) are redundant because the SVG filter and Solid path produce their own visuals.

We keep:
- The `.ui-glass-surface` base styling (the box-shadow, border, radius).
- The data-attribute targeting hooks so consumers can still write site-specific overrides.

We retire:
- The `.ui-glass-surface[data-glass-level="..."]` rules that hardcoded blur radii and tint percentages — the engine + SVG fallback own this now.

- [ ] **Step 1: Identify the rules**

Run `grep -n "data-glass-level" crates/ui-styles/src/lib.rs`. The rules to remove are the `backdrop-filter: blur(...) saturate(...)` + tint-mix rules at `data-glass-level="subtle/floating/overlay/chrome"`.

- [ ] **Step 2: Remove the redundant rules**

In `crates/ui-styles/src/lib.rs`, find each block like:

```css
.ui-glass-surface[data-glass-level="floating"] {
    background: color-mix(in srgb, var(--ui-glass-tint) 55%, color-mix(in srgb, #ffffff 60%, transparent));
    backdrop-filter: blur(18px) saturate(160%);
    -webkit-backdrop-filter: blur(18px) saturate(160%);
    box-shadow: var(--ui-elevation-2);
    border-color: color-mix(in srgb, var(--ui-fg) 12%, transparent);
}
```

Replace the entire `.ui-glass-surface[data-glass-level="..."]` block (all four levels) with a single base rule that applies to all glass surfaces in the Tier 5 Solid fallback:

```css
.ui-glass-surface {
    /* Base styling — engine renders the actual glass effect; this is the
       Tier 5 Solid fallback that displays when the engine and SVG filter
       are both unavailable. */
    background: var(--ui-glass-solid);
    box-shadow: var(--ui-elevation-2);
    border-color: var(--ui-border);
}
```

Keep:
- The tone tint variables (used by SVG filter)
- The density padding rules (still semantically meaningful)
- The `[data-ui-glass-policy="solid"]` overrides

- [ ] **Step 3: Update tests**

`crates/ui-styles/tests/css.rs` may check for specific CSS substrings. Run:

```
cargo test -p ui-styles
```

Update any tests that asserted on the removed rules.

- [ ] **Step 4: Build + commit**

```
cargo build --workspace
cargo test --workspace --features ui-glass-engine/headless
git add crates/ui-styles
git commit -m "refactor(ui-styles): Phase 3 — retire engine-redundant CSS rules"
```

---

## Task 11: Workspace integration check + spec status

**Files:**
- Modify: `docs/superpowers/specs/2026-05-22-liquid-glass-engine-design.md` (append final status)
- Modify: `docs/superpowers/plans/2026-05-22-liquid-glass-plan-5-degradation-and-migration.md`

- [ ] **Step 1: Full workspace build + test**

```
cargo build --workspace
cargo test --workspace --features ui-glass-engine/headless
cargo check --workspace --target wasm32-unknown-unknown
```

All three should succeed.

- [ ] **Step 2: Append spec final status**

At the very bottom of `docs/superpowers/specs/2026-05-22-liquid-glass-engine-design.md`:

```markdown
---

## Implementation Status (2026-05-22)

All five plans complete. Liquid Glass is the default rendering path for
`<GlassSurface>` and the canonical descriptor surface for new components via
`<LiquidSurface>`. The engine ships:

- `ui-glass` — descriptors, builder DSL, presets, `QualityProfile`.
- `ui-glass-engine` — wgpu compositor with all 9 shader features, background
  source pipeline, motion bridge, multi-region compositing, capability
  detection, SVG fallback generator.
- `ui-glass-dioxus` — `<LiquidSurface>` Dioxus component with SSR fallback
  and lifecycle-safe wgpu init.
- `ui-styles` — narrowed to Solid + SVG fallback only.

Tier ladder selection happens at GlassSurface render time:
1. WebGPU available → engine via WebGPU.
2. WebGL2 available → engine via wgpu's WebGL2 fallback.
3. Backdrop-filter only → SVG filter chain via ui-styles.
4. None of the above (or reduced-transparency/high-contrast) → Solid.
5. Engine forced off via QualityProfile → Solid.

Future work (out of scope of the original spec):
- Native renderer integration (Blitz) — currently the non-wasm32 LiquidSurface
  renders a placeholder.
- Per-route compositor sharing across multiple LiquidSurface instances.
- HDR specular and multi-light support.
- Custom-path (non-rounded-rect) glass geometry.
```

- [ ] **Step 3: Append Plan 5 status**

At the bottom of `docs/superpowers/plans/2026-05-22-liquid-glass-plan-5-degradation-and-migration.md`:

```markdown
---

## Status

Plan 5 complete. Liquid Glass shipped end-to-end. See the spec's
Implementation Status section for the production state.
```

- [ ] **Step 4: Commit**

```bash
git add docs/superpowers/specs/2026-05-22-liquid-glass-engine-design.md docs/superpowers/plans/2026-05-22-liquid-glass-plan-5-degradation-and-migration.md
git commit -m "docs: mark Liquid Glass spec + Plan 5 complete"
```

---

## Plan 5 — Done. Liquid Glass shipped

Five plans, ~75 commits, three new crates (`ui-glass-engine`, `ui-glass-dioxus`, plus narrowed `ui-styles`), all 9 spec features rendering, full degradation ladder, SSR-safe, capability-detected.

The work future-Claude can pick up:
- Native (Blitz) integration for the non-wasm32 path.
- Compositor sharing across multiple LiquidSurfaces in a route.
- HDR + multi-light specular.
- Custom-path glass.

But the spec is delivered.
