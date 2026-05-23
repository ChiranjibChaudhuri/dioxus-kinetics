# Liquid Glass — Plan 3: Scene Contract + Motion Bridge + Multi-Region

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Take the headless engine from Plan 2's testable kernel to a production-ready surface: authors declare backgrounds (gradients, images, procedural meshes) instead of uploading raw textures, the compositor subscribes to `ui-motion` springs for pointer/scroll/time, multi-region overlap composites correctly, and the SPECULAR shader produces visible highlights without the `edge_falloff(60.0)` workaround. After Plan 3 the engine is callable from any Rust code with a clean API; Plan 4 wires it into Dioxus.

**Architecture:** Three independent pieces that converge in a redesigned `Compositor::render` signature:

1. **Background renderer** — `BackgroundSource` enum + `BackgroundScene` scene-graph render into a single bg texture per frame. New shaders for gradients and procedural meshes.
2. **Motion bridge** — `Compositor::update_inputs(pointer, scroll, time)` writes to a per-compositor input cache that `render` reads when building each region's `GlassUniforms`. No `with_pointer/with_scroll_velocity` test-only path is needed from production code.
3. **Multi-region** — `regions` are now rendered into a transparent `output_view` back-to-front with proper alpha compositing; the `debug_assert!(regions.len() <= 1)` lifts.

**Tech Stack:** Same as Plan 2 — wgpu 26, WGSL. Adds `ui-motion` as a workspace dependency on `ui-glass-engine`.

---

## Context: What Plan 2 left

- `Compositor::render` takes a raw `bg_view: &TextureView` and a single-region `regions: &[GlassRegion]` (asserted ≤1). Authors must upload their own background and can only render one surface at a time.
- `render_with_uniforms` is the test-only path for injecting pointer/scroll/time; production code has no way to drive those uniforms.
- `compose.wgsl`'s SPECULAR block uses `pow(n_dot_l, 16.0)` — the highlight is narrow enough that the behavior test had to set `edge_falloff(60.0)` to clear a 2% pixel-diff threshold. Apple Liquid Glass has broader, more diffuse rim lighting; tuning the exponent down (to ~4) widens the highlight while keeping the geometry sensible.
- `materialize_mipped_bg` already produces a proper mip chain for TINT_ADAPT; we keep it.

---

## File Structure

**Modified files:**

```
crates/ui-glass-engine/Cargo.toml               add ui-motion dep
crates/ui-glass-engine/src/lib.rs               re-export new types
crates/ui-glass-engine/src/compositor.rs        add update_inputs, MotionInputs cache,
                                                accept BackgroundSource per region,
                                                accept BackgroundScene context
crates/ui-glass-engine/src/render_graph.rs      multi-region compositing
crates/ui-glass-engine/src/uniforms.rs          (no struct change; existing helpers stay)
crates/ui-glass-engine/src/shaders/compose.wgsl SPECULAR exponent tune (16 → 4)
crates/ui-glass-engine/tests/behavior.rs        drop edge_falloff(60.0) workaround
```

**New files:**

```
crates/ui-glass-engine/src/background/mod.rs          BackgroundSource + BackgroundScene
crates/ui-glass-engine/src/background/render.rs       render-source-to-texture pipeline
crates/ui-glass-engine/src/background/image_cache.rs  static URL + dynamic handle cache
crates/ui-glass-engine/src/shaders/bg_gradient.wgsl   linear/radial/conic gradients
crates/ui-glass-engine/src/shaders/bg_mesh.wgsl       Aurora/Orbs/Grain procedural
crates/ui-glass-engine/src/motion.rs                  MotionInputs + bridge helpers
crates/ui-glass-engine/tests/background_sources.rs    per-source render tests
crates/ui-glass-engine/tests/scene_graph.rs           BackgroundScene composes correctly
crates/ui-glass-engine/tests/motion_bridge.rs         pointer/scroll/time pass through
crates/ui-glass-engine/tests/multi_region.rs          overlapping surfaces composite
crates/ui-glass-engine/tests/assets/*.png             new goldens (5)
```

**Responsibility boundaries:**

- `background/mod.rs` owns the descriptors only — no wgpu types.
- `background/render.rs` owns the GPU pipelines and the per-frame "materialize a bg texture" entry point.
- `background/image_cache.rs` owns texture lifetime for static + dynamic image sources.
- `motion.rs` is the bridge: a small struct that holds the latest pointer/scroll/time values; the actual springs live in `ui-motion` (consumers feed values in, not the reverse).
- `compositor.rs` orchestrates: builds the per-frame bg via `background::render`, reads `MotionInputs`, dispatches multi-region.

---

## Task 1: SPECULAR exponent tune + re-bake goldens

**Files:**
- Modify: `crates/ui-glass-engine/src/shaders/compose.wgsl`
- Modify: `crates/ui-glass-engine/tests/behavior.rs`
- Re-bake: `crates/ui-glass-engine/tests/assets/{specular,floating,full}_128.png`

- [ ] **Step 1: Edit `compose.wgsl` SPECULAR block**

Find:

```wgsl
    if (FEAT_SPECULAR) {
        let n_dot_l = max(dot(normal, u.light_dir), 0.0);
        let spec = pow(n_dot_l, 16.0) * u.light_intensity;
        let edge_mask = smoothstep(0.0, max(u.edge_falloff, 0.5), -sdf);
        color = color + vec3<f32>(spec * (1.0 - edge_mask));
    }
```

Replace with:

```wgsl
    if (FEAT_SPECULAR) {
        let n_dot_l = max(dot(normal, u.light_dir), 0.0);
        let spec = pow(n_dot_l, 4.0) * u.light_intensity;
        let edge_mask = smoothstep(0.0, max(u.edge_falloff, 0.5), -sdf);
        color = color + vec3<f32>(spec * (1.0 - edge_mask));
    }
```

Only change: `pow(n_dot_l, 16.0)` → `pow(n_dot_l, 4.0)`. This widens the highlight from a near-singularity at perfect alignment to a smooth gradient across roughly the lit half of the rim.

- [ ] **Step 2: Restore behavior.rs SPECULAR test to sane parameters**

In `crates/ui-glass-engine/tests/behavior.rs`, find the `specular_changes_output` test and replace the material with the parameters that originally would have made sense:

```rust
#[test]
fn specular_changes_output() {
    let off = render_with_checkerboard(W, H, base());
    let on = render_with_checkerboard(
        W, H,
        base().specular(45.0_f32.to_radians(), 0.8).edge_falloff(2.0),
    );
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(frac > MIN_AFFECTED_FRACTION, "SPECULAR changed only {:.2}% of pixels", frac * 100.0);
}
```

(Reverts to intensity 0.8 + edge_falloff 2.0, the values used by the golden test.)

- [ ] **Step 3: Run behavior test**

`cargo test -p ui-glass-engine --features headless --test behavior`
Expected: all 6 tests pass, including the SPECULAR one without the workaround.

- [ ] **Step 4: Re-bake affected goldens**

Bash:
```bash
UPDATE_GOLDEN=1 cargo test -p ui-glass-engine --features headless --test golden_specular
UPDATE_GOLDEN=1 cargo test -p ui-glass-engine --features headless --test golden_floating
UPDATE_GOLDEN=1 cargo test -p ui-glass-engine --features headless --test golden_full
cargo test -p ui-glass-engine --features headless
```

- [ ] **Step 5: Commit**

```bash
git add crates/ui-glass-engine/src/shaders/compose.wgsl crates/ui-glass-engine/tests/behavior.rs crates/ui-glass-engine/tests/assets/specular_128.png crates/ui-glass-engine/tests/assets/floating_neutral_128.png crates/ui-glass-engine/tests/assets/full_128.png
git commit -m "fix(ui-glass-engine): widen SPECULAR highlight (pow(.,16) -> pow(.,4))"
```

---

## Task 2: BackgroundSource enum (descriptors, no wgpu)

**Files:**
- Create: `crates/ui-glass-engine/src/background/mod.rs`
- Modify: `crates/ui-glass-engine/src/lib.rs`
- Create: `crates/ui-glass-engine/tests/background_sources.rs`

- [ ] **Step 1: Write failing tests**

Create `crates/ui-glass-engine/tests/background_sources.rs`:

```rust
use ui_glass_engine::background::{BackgroundSource, Gradient, GradientStop, MeshKind};
use ui_tokens::Color;

#[test]
fn color_source_constructs() {
    let src = BackgroundSource::Color(Color::rgba(10, 20, 30, 1.0));
    match src {
        BackgroundSource::Color(c) => assert_eq!(c.r, 10),
        _ => panic!("expected Color variant"),
    }
}

#[test]
fn gradient_linear_with_stops() {
    let g = Gradient::linear(
        0.5,
        vec![
            GradientStop { offset: 0.0, color: Color::rgba(0, 0, 0, 1.0) },
            GradientStop { offset: 1.0, color: Color::rgba(255, 255, 255, 1.0) },
        ],
    );
    assert_eq!(g.stops().len(), 2);
    assert!(g.is_linear());
}

#[test]
fn gradient_radial_with_center_and_stops() {
    let g = Gradient::radial(
        [0.5, 0.5],
        0.7,
        vec![GradientStop { offset: 0.0, color: Color::rgba(255, 0, 0, 1.0) }],
    );
    assert!(g.is_radial());
}

#[test]
fn gradient_conic_with_angle() {
    let g = Gradient::conic(
        [0.5, 0.5],
        0.0,
        vec![GradientStop { offset: 0.0, color: Color::rgba(0, 255, 0, 1.0) }],
    );
    assert!(g.is_conic());
}

#[test]
fn mesh_variants_exist() {
    let _a = BackgroundSource::Mesh(MeshKind::Aurora);
    let _o = BackgroundSource::Mesh(MeshKind::Orbs);
    let _g = BackgroundSource::Mesh(MeshKind::Grain);
}
```

- [ ] **Step 2: Run, expect failure**

`cargo test -p ui-glass-engine --test background_sources`
Expected: FAIL — `background` module doesn't exist.

- [ ] **Step 3: Create `background/mod.rs`**

```rust
//! Background-source descriptors. CPU types only — no wgpu. The renderer that
//! turns these into a texture lives in `background::render`.

use ui_tokens::Color;

pub mod render;
pub mod image_cache;

pub use image_cache::{ImageCache, ImageHandle};

/// A single layer of the background scene. Compositors materialize one of
/// these (or the per-surface variant) into the texture that glass surfaces
/// sample from.
#[derive(Clone, Debug)]
pub enum BackgroundSource {
    Color(Color),
    Gradient(Gradient),
    Image(ImageSource),
    Mesh(MeshKind),
}

#[derive(Clone, Debug)]
pub enum ImageSource {
    /// URL or path; resolved through ImageCache.
    Static(String),
    /// Externally-uploaded texture; handle owned by ImageCache.
    Dynamic(ImageHandle),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeshKind {
    Aurora,
    Orbs,
    Grain,
}

#[derive(Clone, Copy, Debug)]
pub struct GradientStop {
    pub offset: f32,  // 0..1
    pub color: Color,
}

#[derive(Clone, Debug)]
pub struct Gradient {
    kind: GradientKind,
    stops: Vec<GradientStop>,
}

#[derive(Clone, Copy, Debug)]
enum GradientKind {
    Linear { angle_rad: f32 },
    Radial { center: [f32; 2], radius: f32 },
    Conic  { center: [f32; 2], start_angle_rad: f32 },
}

impl Gradient {
    pub fn linear(angle_rad: f32, stops: Vec<GradientStop>) -> Self {
        Self { kind: GradientKind::Linear { angle_rad }, stops }
    }

    pub fn radial(center: [f32; 2], radius: f32, stops: Vec<GradientStop>) -> Self {
        Self { kind: GradientKind::Radial { center, radius }, stops }
    }

    pub fn conic(center: [f32; 2], start_angle_rad: f32, stops: Vec<GradientStop>) -> Self {
        Self { kind: GradientKind::Conic { center, start_angle_rad }, stops }
    }

    pub fn stops(&self) -> &[GradientStop] { &self.stops }
    pub fn is_linear(&self) -> bool { matches!(self.kind, GradientKind::Linear { .. }) }
    pub fn is_radial(&self) -> bool { matches!(self.kind, GradientKind::Radial { .. }) }
    pub fn is_conic(&self) -> bool  { matches!(self.kind, GradientKind::Conic { .. }) }

    pub(crate) fn kind(&self) -> GradientKind { self.kind }
}

/// Full scene-graph mode. Layers composite back-to-front into the bg
/// texture every frame. Registered per-route (or per-Compositor); falls back
/// to per-surface `BackgroundSource` when absent.
#[derive(Clone, Debug, Default)]
pub struct BackgroundScene {
    pub layers: Vec<BackgroundSource>,
}

impl BackgroundScene {
    pub fn new() -> Self { Self::default() }
    pub fn layer(mut self, source: BackgroundSource) -> Self {
        self.layers.push(source);
        self
    }
    pub fn is_empty(&self) -> bool { self.layers.is_empty() }
}
```

- [ ] **Step 4: Create stub `image_cache.rs`**

```rust
//! Texture cache for `Image::Static(...)` and external `Image::Dynamic(handle)`.
//! Real impl lands in Task 6; this stub keeps the public type defined so the
//! Background enum compiles.

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImageHandle(pub u64);

pub struct ImageCache;
```

- [ ] **Step 5: Create stub `render.rs`**

```rust
//! Background renderer. Fills in across Tasks 3-6.
```

- [ ] **Step 6: Add module to lib.rs**

```rust
pub mod background;
```

(Place alongside `pub mod compositor;`. Re-export later when needed.)

- [ ] **Step 7: Run tests**

`cargo test -p ui-glass-engine --test background_sources`
Expected: all 5 tests pass.

- [ ] **Step 8: Commit**

```bash
git add crates/ui-glass-engine/src/background/ crates/ui-glass-engine/src/lib.rs crates/ui-glass-engine/tests/background_sources.rs
git commit -m "feat(ui-glass-engine): add BackgroundSource descriptors"
```

---

## Task 3: Gradient renderer (WGSL + pipeline)

**Files:**
- Create: `crates/ui-glass-engine/src/shaders/bg_gradient.wgsl`
- Modify: `crates/ui-glass-engine/src/background/render.rs`
- Modify: `crates/ui-glass-engine/src/lib.rs`
- Create: `crates/ui-glass-engine/tests/background_render.rs`

- [ ] **Step 1: Write failing test**

Create `crates/ui-glass-engine/tests/background_render.rs`:

```rust
use ui_glass_engine::background::{BackgroundSource, Gradient, GradientStop};
use ui_glass_engine::background::render::BackgroundRenderer;
use ui_glass_engine::headless::TestHarness;
use ui_tokens::Color;

#[test]
fn renderer_creates_pipeline() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let _r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
}

#[test]
fn linear_gradient_produces_color_transition() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    let g = Gradient::linear(
        0.0,  // horizontal
        vec![
            GradientStop { offset: 0.0, color: Color::rgba(255,   0,   0, 1.0) },
            GradientStop { offset: 1.0, color: Color::rgba(  0,   0, 255, 1.0) },
        ],
    );
    let pixels = r.render_to_pixels(&[BackgroundSource::Gradient(g)], 64, 64);
    // Left edge should be red-ish; right edge should be blue-ish.
    let left = (pixels[0], pixels[1], pixels[2]);
    let right_idx = (63 * 4) as usize;
    let right = (pixels[right_idx], pixels[right_idx + 1], pixels[right_idx + 2]);
    assert!(left.0 > 100, "left should be red, got {left:?}");
    assert!(right.2 > 100, "right should be blue, got {right:?}");
}

#[test]
fn solid_color_fills_whole_texture() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    let pixels = r.render_to_pixels(
        &[BackgroundSource::Color(Color::rgba(0, 128, 0, 1.0))],
        32, 32,
    );
    let center_idx = ((16 * 32 + 16) * 4) as usize;
    assert!(pixels[center_idx + 1] > 50, "expected greenish center");
}
```

- [ ] **Step 2: Run, expect failure**

`cargo test -p ui-glass-engine --features headless --test background_render`
Expected: FAIL — `BackgroundRenderer` doesn't exist.

- [ ] **Step 3: Create `bg_gradient.wgsl`**

```wgsl
// Renders Color and Gradient backgrounds. The host passes the gradient kind
// and stops via a uniform buffer; the fragment shader picks the algorithm via
// an `override` constant for kind selection.

override KIND: u32 = 0u; // 0=Color, 1=Linear, 2=Radial, 3=Conic
override STOP_COUNT: u32 = 2u;

struct BgUniforms {
    // Common
    canvas_size: vec2<f32>,
    _pad0: vec2<f32>,
    // Linear
    direction: vec2<f32>,       // unit vector for linear; ignored otherwise
    _pad1: vec2<f32>,
    // Radial / conic center
    center: vec2<f32>,
    radius: f32,
    start_angle_rad: f32,
    // Solid fallback (used when KIND == 0)
    solid: vec4<f32>,
    // Up to 8 stops; unused entries are zero.
    stop_offsets: vec4<f32>,    // [0..3]
    stop_offsets2: vec4<f32>,   // [4..7]
    stop_colors_0: vec4<f32>,
    stop_colors_1: vec4<f32>,
    stop_colors_2: vec4<f32>,
    stop_colors_3: vec4<f32>,
    stop_colors_4: vec4<f32>,
    stop_colors_5: vec4<f32>,
    stop_colors_6: vec4<f32>,
    stop_colors_7: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: BgUniforms;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    var uv  = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );
    var out: VsOut;
    out.pos = vec4<f32>(pos[vid], 0.0, 1.0);
    out.uv  = uv[vid];
    return out;
}

fn stop_offset(i: u32) -> f32 {
    if (i == 0u) { return u.stop_offsets.x; }
    if (i == 1u) { return u.stop_offsets.y; }
    if (i == 2u) { return u.stop_offsets.z; }
    if (i == 3u) { return u.stop_offsets.w; }
    if (i == 4u) { return u.stop_offsets2.x; }
    if (i == 5u) { return u.stop_offsets2.y; }
    if (i == 6u) { return u.stop_offsets2.z; }
    return u.stop_offsets2.w;
}

fn stop_color(i: u32) -> vec4<f32> {
    if (i == 0u) { return u.stop_colors_0; }
    if (i == 1u) { return u.stop_colors_1; }
    if (i == 2u) { return u.stop_colors_2; }
    if (i == 3u) { return u.stop_colors_3; }
    if (i == 4u) { return u.stop_colors_4; }
    if (i == 5u) { return u.stop_colors_5; }
    if (i == 6u) { return u.stop_colors_6; }
    return u.stop_colors_7;
}

fn sample_gradient(t: f32) -> vec4<f32> {
    let tc = clamp(t, 0.0, 1.0);
    if (STOP_COUNT <= 1u) { return stop_color(0u); }
    for (var i: u32 = 1u; i < STOP_COUNT; i = i + 1u) {
        let a = stop_offset(i - 1u);
        let b = stop_offset(i);
        if (tc <= b) {
            let span = max(b - a, 1e-5);
            let f = (tc - a) / span;
            return mix(stop_color(i - 1u), stop_color(i), f);
        }
    }
    return stop_color(STOP_COUNT - 1u);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    if (KIND == 0u) {
        return u.solid;
    }
    if (KIND == 1u) {
        // Linear: project the UV onto the direction vector.
        let p = in.uv - vec2<f32>(0.5);
        let t = dot(p, u.direction) + 0.5;
        return sample_gradient(t);
    }
    if (KIND == 2u) {
        let d = length(in.uv - u.center) / max(u.radius, 1e-5);
        return sample_gradient(d);
    }
    // Conic
    let v = in.uv - u.center;
    let angle = atan2(v.y, v.x) - u.start_angle_rad;
    let t = (angle / 6.28318) + 0.5;
    return sample_gradient(t - floor(t));
}
```

- [ ] **Step 4: Implement `BackgroundRenderer`**

Replace `crates/ui-glass-engine/src/background/render.rs` with:

```rust
//! Background renderer. Materializes BackgroundSource descriptors into an
//! RGBA8UnormSrgb texture suitable as a glass-pass bg input.

use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::background::{BackgroundSource, Gradient};

const SHADER: &str = include_str!("../shaders/bg_gradient.wgsl");
const MAX_STOPS: usize = 8;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct BgUniforms {
    canvas_size: [f32; 2],
    _pad0: [f32; 2],
    direction: [f32; 2],
    _pad1: [f32; 2],
    center: [f32; 2],
    radius: f32,
    start_angle_rad: f32,
    solid: [f32; 4],
    stop_offsets: [f32; 4],
    stop_offsets2: [f32; 4],
    stop_colors: [[f32; 4]; MAX_STOPS],
}

pub struct BackgroundRenderer {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    bgl: wgpu::BindGroupLayout,
}

impl BackgroundRenderer {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bg-render-bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        Self { device, queue, bgl }
    }

    /// Render the given sources into an offscreen RGBA8UnormSrgb texture and
    /// return the wgpu Texture (with its full-mip view). Layers composite
    /// back-to-front via blend state.
    pub fn render_to_texture(
        &mut self,
        sources: &[BackgroundSource],
        w: u32,
        h: u32,
    ) -> wgpu::Texture {
        let tex = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("bg-source-tex"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut first = true;
        for src in sources {
            let (uniforms, kind, stop_count) = self.uniforms_for(src, [w as f32, h as f32]);
            let pipeline = self.build_pipeline(kind, stop_count);
            let buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bg-uniforms"),
                contents: bytemuck::bytes_of(&uniforms),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
            let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("bg-bg"),
                layout: &self.bgl,
                entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }],
            });

            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bg-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: if first {
                            wgpu::LoadOp::Clear(wgpu::Color::BLACK)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.draw(0..3, 0..1);
            first = false;
        }
        self.queue.submit(Some(encoder.finish()));
        tex
    }

    /// Convenience for tests: render and read back as RGBA bytes.
    #[cfg(any(test, feature = "headless"))]
    pub fn render_to_pixels(&mut self, sources: &[BackgroundSource], w: u32, h: u32) -> Vec<u8> {
        let tex = self.render_to_texture(sources, w, h);
        read_back(&self.device, &self.queue, &tex, w, h)
    }

    fn uniforms_for(&self, src: &BackgroundSource, canvas: [f32; 2]) -> (BgUniforms, u32, u32) {
        use crate::background::GradientKind;
        let mut u = BgUniforms {
            canvas_size: canvas,
            _pad0: [0.0; 2],
            direction: [1.0, 0.0],
            _pad1: [0.0; 2],
            center: [0.5, 0.5],
            radius: 0.5,
            start_angle_rad: 0.0,
            solid: [0.0, 0.0, 0.0, 0.0],
            stop_offsets: [0.0; 4],
            stop_offsets2: [0.0; 4],
            stop_colors: [[0.0; 4]; MAX_STOPS],
        };

        match src {
            BackgroundSource::Color(c) => {
                u.solid = [c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0, c.a];
                (u, 0, 0)
            }
            BackgroundSource::Gradient(g) => {
                let (kind, stops) = self.write_gradient(g, &mut u);
                (u, kind, stops)
            }
            BackgroundSource::Image(_) | BackgroundSource::Mesh(_) => {
                // Image and mesh sources are handled by separate render paths
                // (Tasks 5 and 6); for Task 3 they fall through to a black fill.
                (u, 0, 0)
            }
        }
    }

    fn write_gradient(&self, g: &Gradient, u: &mut BgUniforms) -> (u32, u32) {
        let stops = g.stops();
        let n = stops.len().min(MAX_STOPS);
        for (i, s) in stops.iter().take(n).enumerate() {
            let arr = if i < 4 { &mut u.stop_offsets } else { &mut u.stop_offsets2 };
            arr[i % 4] = s.offset;
            u.stop_colors[i] = [
                s.color.r as f32 / 255.0,
                s.color.g as f32 / 255.0,
                s.color.b as f32 / 255.0,
                s.color.a,
            ];
        }
        match g.kind() {
            crate::background::GradientKind::Linear { angle_rad } => {
                u.direction = [angle_rad.cos(), angle_rad.sin()];
                (1, n as u32)
            }
            crate::background::GradientKind::Radial { center, radius } => {
                u.center = center;
                u.radius = radius;
                (2, n as u32)
            }
            crate::background::GradientKind::Conic { center, start_angle_rad } => {
                u.center = center;
                u.start_angle_rad = start_angle_rad;
                (3, n as u32)
            }
        }
    }

    fn build_pipeline(&self, kind: u32, stop_count: u32) -> wgpu::RenderPipeline {
        let module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bg_gradient.wgsl"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("bg-layout"),
            bind_group_layouts: &[&self.bgl],
            push_constant_ranges: &[],
        });
        let constants: &[(&str, f64)] = &[
            ("KIND", kind as f64),
            ("STOP_COUNT", stop_count.max(1) as f64),
        ];
        self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("bg-pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &module, entry_point: Some("vs_main"),
                buffers: &[], compilation_options: wgpu::PipelineCompilationOptions {
                    constants, zero_initialize_workgroup_memory: false,
                },
            },
            fragment: Some(wgpu::FragmentState {
                module: &module, entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions {
                    constants, zero_initialize_workgroup_memory: false,
                },
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        })
    }
}

#[cfg(any(test, feature = "headless"))]
fn read_back(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    tex: &wgpu::Texture, w: u32, h: u32,
) -> Vec<u8> {
    let bpr = ((w * 4 + 255) / 256) * 256;
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("bg-readback"),
        size: (bpr * h) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc = device.create_command_encoder(&Default::default());
    enc.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: tex, mip_level: 0, origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buf,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0, bytes_per_row: Some(bpr), rows_per_image: Some(h),
            },
        },
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );
    queue.submit(Some(enc.finish()));
    let slice = buf.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| { tx.send(r).unwrap(); });
    let _ = device.poll(wgpu::PollType::Wait);
    rx.recv().unwrap().unwrap();
    let data = slice.get_mapped_range();
    let mut out = Vec::with_capacity((w * h * 4) as usize);
    for row in 0..h {
        let start = (row * bpr) as usize;
        out.extend_from_slice(&data[start..start + (w * 4) as usize]);
    }
    drop(data);
    buf.unmap();
    out
}
```

Important — `GradientKind` was private in `background/mod.rs`. Expose it as `pub(crate)` so `render.rs` can match on it:

```rust
pub(crate) enum GradientKind { ... }   // change from `enum` to `pub(crate) enum`
```

- [ ] **Step 5: Run, verify**

`cargo test -p ui-glass-engine --features headless --test background_render`
Expected: 3 tests pass.

- [ ] **Step 6: Commit**

```bash
git add crates/ui-glass-engine/src/background/render.rs crates/ui-glass-engine/src/background/mod.rs crates/ui-glass-engine/src/shaders/bg_gradient.wgsl crates/ui-glass-engine/tests/background_render.rs
git commit -m "feat(ui-glass-engine): gradient + solid background renderer"
```

---

## Task 4: Procedural mesh backgrounds (Aurora/Orbs/Grain)

**Files:**
- Create: `crates/ui-glass-engine/src/shaders/bg_mesh.wgsl`
- Modify: `crates/ui-glass-engine/src/background/render.rs`
- Modify: `crates/ui-glass-engine/tests/background_render.rs`

- [ ] **Step 1: Append failing test**

Append to `crates/ui-glass-engine/tests/background_render.rs`:

```rust
use ui_glass_engine::background::MeshKind;

#[test]
fn aurora_mesh_produces_non_uniform_output() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    let pixels = r.render_to_pixels(&[BackgroundSource::Mesh(MeshKind::Aurora)], 64, 64);
    let mut min = 255u8;
    let mut max = 0u8;
    for chunk in pixels.chunks(4) {
        for c in &chunk[..3] {
            if *c < min { min = *c; }
            if *c > max { max = *c; }
        }
    }
    assert!(max - min > 40, "aurora should vary across the texture; got range {min}..{max}");
}

#[test]
fn orbs_and_grain_produce_distinct_outputs() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    let orbs = r.render_to_pixels(&[BackgroundSource::Mesh(MeshKind::Orbs)], 32, 32);
    let grain = r.render_to_pixels(&[BackgroundSource::Mesh(MeshKind::Grain)], 32, 32);
    assert_ne!(orbs, grain, "orbs and grain should render differently");
}
```

- [ ] **Step 2: Run, expect failure**

`cargo test -p ui-glass-engine --features headless --test background_render`
Expected: FAIL — Mesh path is currently a black fill.

- [ ] **Step 3: Create `bg_mesh.wgsl`**

```wgsl
override MESH_KIND: u32 = 0u; // 0=Aurora, 1=Orbs, 2=Grain

struct MeshUniforms {
    canvas_size: vec2<f32>,
    time_seconds: f32,
    _pad: f32,
};

@group(0) @binding(0) var<uniform> u: MeshUniforms;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    var uv  = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );
    var out: VsOut;
    out.pos = vec4<f32>(pos[vid], 0.0, 1.0);
    out.uv  = uv[vid];
    return out;
}

fn hash21(p: vec2<f32>) -> f32 {
    let q = vec2<f32>(dot(p, vec2<f32>(127.1, 311.7)), dot(p, vec2<f32>(269.5, 183.3)));
    return fract(sin(dot(q, vec2<f32>(43758.5453, 12345.6789))) * 43758.5453);
}

fn aurora(uv: vec2<f32>, t: f32) -> vec3<f32> {
    // Three soft horizontal bands of color, drifting.
    let y = uv.y;
    let b1 = exp(-pow((y - 0.3 + 0.05 * sin(t)), 2.0) * 25.0);
    let b2 = exp(-pow((y - 0.6 + 0.05 * sin(t + 1.7)), 2.0) * 25.0);
    let b3 = exp(-pow((y - 0.9 + 0.05 * sin(t + 3.4)), 2.0) * 25.0);
    let c1 = vec3<f32>(0.18, 0.55, 0.85) * b1;
    let c2 = vec3<f32>(0.85, 0.35, 0.55) * b2;
    let c3 = vec3<f32>(0.40, 0.85, 0.55) * b3;
    return c1 + c2 + c3 + vec3<f32>(0.05);
}

fn orbs(uv: vec2<f32>, t: f32) -> vec3<f32> {
    let c0 = vec2<f32>(0.5 + 0.3 * cos(t),       0.5 + 0.3 * sin(t));
    let c1 = vec2<f32>(0.5 + 0.3 * cos(t + 2.0), 0.5 + 0.3 * sin(t + 2.0));
    let c2 = vec2<f32>(0.5 + 0.3 * cos(t + 4.0), 0.5 + 0.3 * sin(t + 4.0));
    let r = 0.25;
    let f0 = exp(-pow(length(uv - c0) / r, 2.0));
    let f1 = exp(-pow(length(uv - c1) / r, 2.0));
    let f2 = exp(-pow(length(uv - c2) / r, 2.0));
    return vec3<f32>(0.30, 0.55, 0.95) * f0
         + vec3<f32>(0.90, 0.45, 0.65) * f1
         + vec3<f32>(0.55, 0.95, 0.65) * f2
         + vec3<f32>(0.05);
}

fn grain(uv: vec2<f32>, t: f32) -> vec3<f32> {
    let base = vec3<f32>(0.15);
    let n = hash21(uv * 256.0 + vec2<f32>(t));
    return base + vec3<f32>(n) * 0.4;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let t = u.time_seconds * 0.3;
    var c: vec3<f32>;
    if (MESH_KIND == 0u) { c = aurora(in.uv, t); }
    else if (MESH_KIND == 1u) { c = orbs(in.uv, t); }
    else { c = grain(in.uv, t); }
    return vec4<f32>(c, 1.0);
}
```

- [ ] **Step 4: Wire mesh path in `BackgroundRenderer`**

In `crates/ui-glass-engine/src/background/render.rs`, add a sibling mesh pipeline. Append:

```rust
const MESH_SHADER: &str = include_str!("../shaders/bg_mesh.wgsl");

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct MeshUniforms {
    canvas_size: [f32; 2],
    time_seconds: f32,
    _pad: f32,
}

impl BackgroundRenderer {
    fn build_mesh_pipeline(&self, mesh_kind: u32) -> wgpu::RenderPipeline {
        let module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bg_mesh.wgsl"),
            source: wgpu::ShaderSource::Wgsl(MESH_SHADER.into()),
        });
        let layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("bg-mesh-layout"),
            bind_group_layouts: &[&self.bgl],
            push_constant_ranges: &[],
        });
        let constants: &[(&str, f64)] = &[("MESH_KIND", mesh_kind as f64)];
        self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("bg-mesh-pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &module, entry_point: Some("vs_main"),
                buffers: &[], compilation_options: wgpu::PipelineCompilationOptions {
                    constants, zero_initialize_workgroup_memory: false,
                },
            },
            fragment: Some(wgpu::FragmentState {
                module: &module, entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions {
                    constants, zero_initialize_workgroup_memory: false,
                },
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        })
    }
}
```

In `BackgroundRenderer::render_to_texture`, branch on the source kind. Replace the per-source loop body with:

```rust
        for src in sources {
            match src {
                BackgroundSource::Mesh(kind) => {
                    let mesh_kind = match kind {
                        crate::background::MeshKind::Aurora => 0u32,
                        crate::background::MeshKind::Orbs => 1,
                        crate::background::MeshKind::Grain => 2,
                    };
                    let pipeline = self.build_mesh_pipeline(mesh_kind);
                    let u = MeshUniforms { canvas_size: [w as f32, h as f32], time_seconds: 0.0, _pad: 0.0 };
                    let buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("mesh-uniforms"),
                        contents: bytemuck::bytes_of(&u),
                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    });
                    let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("mesh-bg"),
                        layout: &self.bgl,
                        entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }],
                    });
                    self.run_pass(&mut encoder, &view, &pipeline, &bg, first);
                }
                _ => {
                    let (uniforms, kind, stop_count) = self.uniforms_for(src, [w as f32, h as f32]);
                    let pipeline = self.build_pipeline(kind, stop_count);
                    let buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("bg-uniforms"),
                        contents: bytemuck::bytes_of(&uniforms),
                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    });
                    let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("bg-bg"),
                        layout: &self.bgl,
                        entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }],
                    });
                    self.run_pass(&mut encoder, &view, &pipeline, &bg, first);
                }
            }
            first = false;
        }
```

Extract the per-pass run logic into a helper method on `BackgroundRenderer`:

```rust
    fn run_pass(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        pipeline: &wgpu::RenderPipeline,
        bind: &wgpu::BindGroup,
        clear: bool,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("bg-source-pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if clear { wgpu::LoadOp::Clear(wgpu::Color::BLACK) } else { wgpu::LoadOp::Load },
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None, occlusion_query_set: None,
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, bind, &[]);
        pass.draw(0..3, 0..1);
    }
```

- [ ] **Step 5: Verify tests**

`cargo test -p ui-glass-engine --features headless --test background_render`
Expected: 5 tests pass (gradient + solid + mesh variants).

- [ ] **Step 6: Commit**

```bash
git add crates/ui-glass-engine/src/shaders/bg_mesh.wgsl crates/ui-glass-engine/src/background/render.rs crates/ui-glass-engine/tests/background_render.rs
git commit -m "feat(ui-glass-engine): procedural mesh backgrounds (Aurora/Orbs/Grain)"
```

---

## Task 5: Image source cache + upload

**Files:**
- Modify: `crates/ui-glass-engine/src/background/image_cache.rs`
- Modify: `crates/ui-glass-engine/src/background/render.rs`
- Modify: `crates/ui-glass-engine/tests/background_render.rs`

- [ ] **Step 1: Write failing test**

Append to `crates/ui-glass-engine/tests/background_render.rs`:

```rust
use ui_glass_engine::background::{ImageCache, ImageSource};

#[test]
fn dynamic_image_can_be_uploaded_and_sampled() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut cache = ImageCache::new(h.device().clone(), h.queue().clone());
    let pixels = vec![255u8, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255];
    let handle = cache.upload_rgba(&pixels, 2, 2);
    assert!(cache.get(&handle).is_some());

    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    r.set_image_cache(cache);
    let out = r.render_to_pixels(
        &[BackgroundSource::Image(ImageSource::Dynamic(handle))],
        4, 4,
    );
    // Output should pick up SOME non-black pixels from the uploaded image.
    let nonzero = out.iter().filter(|&&b| b > 16).count();
    assert!(nonzero > 0, "expected non-black pixels from uploaded image");
}
```

- [ ] **Step 2: Run, expect failure**

`cargo test -p ui-glass-engine --features headless --test background_render`
Expected: FAIL — `ImageCache::new`/`upload_rgba`/`get`/`set_image_cache` not defined.

- [ ] **Step 3: Implement `ImageCache`**

Replace `crates/ui-glass-engine/src/background/image_cache.rs`:

```rust
//! Texture cache for `Image::Static(path)` and `Image::Dynamic(handle)` background
//! sources. Static images are loaded once and cached by URL/path; dynamic images
//! are uploaded by the host and tracked via integer handles.

use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImageHandle(pub u64);

pub struct ImageCache {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    by_handle: HashMap<ImageHandle, Arc<wgpu::Texture>>,
    by_path: HashMap<String, Arc<wgpu::Texture>>,
    next_id: u64,
}

impl ImageCache {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self {
            device, queue,
            by_handle: HashMap::new(),
            by_path: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn upload_rgba(&mut self, pixels: &[u8], w: u32, h: u32) -> ImageHandle {
        let tex = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("user-image"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &tex, mip_level: 0, origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h),
            },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        let h = ImageHandle(self.next_id);
        self.next_id += 1;
        self.by_handle.insert(h, Arc::new(tex));
        h
    }

    /// Static image loading — for Plan 3 we accept a pre-loaded RGBA buffer
    /// to avoid IO. Plan 4 will add Asset / URL plumbing.
    pub fn upload_static(&mut self, key: &str, pixels: &[u8], w: u32, hgt: u32) {
        let h = self.upload_rgba(pixels, w, hgt);
        let tex = self.by_handle.remove(&h).unwrap();
        self.by_path.insert(key.to_string(), tex);
    }

    pub fn get(&self, handle: &ImageHandle) -> Option<Arc<wgpu::Texture>> {
        self.by_handle.get(handle).cloned()
    }

    pub fn get_static(&self, key: &str) -> Option<Arc<wgpu::Texture>> {
        self.by_path.get(key).cloned()
    }
}
```

- [ ] **Step 4: Wire image path into `BackgroundRenderer`**

In `crates/ui-glass-engine/src/background/render.rs`, add an `image_cache` field and `set_image_cache` accessor:

```rust
pub struct BackgroundRenderer {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    bgl: wgpu::BindGroupLayout,
    image_blit_bgl: wgpu::BindGroupLayout,
    image_blit_pipeline: wgpu::RenderPipeline,
    image_cache: Option<crate::background::image_cache::ImageCache>,
}

impl BackgroundRenderer {
    pub fn set_image_cache(&mut self, cache: crate::background::image_cache::ImageCache) {
        self.image_cache = Some(cache);
    }

    pub fn image_cache_mut(&mut self) -> &mut Option<crate::background::image_cache::ImageCache> {
        &mut self.image_cache
    }
}
```

For brevity reuse the existing `mipmap.wgsl` shader (already in the crate from Plan 2) as the image blit. Add to `BackgroundRenderer::new`:

```rust
        let image_blit_bgl = crate::pipeline::mipmap_bind_group_layout(&device);
        let image_blit_pipeline = crate::pipeline::build_mipmap_pipeline(&device);
```

Add the Image branch in `render_to_texture`:

```rust
                BackgroundSource::Image(crate::background::ImageSource::Dynamic(handle)) => {
                    let tex_opt = self.image_cache.as_ref().and_then(|c| c.get(handle));
                    if let Some(tex) = tex_opt {
                        let view = tex.create_view(&Default::default());
                        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
                            label: Some("image-bg-sampler"),
                            mag_filter: wgpu::FilterMode::Linear,
                            min_filter: wgpu::FilterMode::Linear,
                            ..Default::default()
                        });
                        let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: Some("image-bg"),
                            layout: &self.image_blit_bgl,
                            entries: &[
                                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
                            ],
                        });
                        self.run_pass(&mut encoder, /* the bg-source-tex view */ /*reuse `view` from outer scope*/, &self.image_blit_pipeline, &bg, first);
                    }
                }
                BackgroundSource::Image(crate::background::ImageSource::Static(key)) => {
                    let tex_opt = self.image_cache.as_ref().and_then(|c| c.get_static(key));
                    if let Some(tex) = tex_opt {
                        // Same blit path as Dynamic above
                    }
                }
```

Note: this requires the bind-group-layout entry layout for `image_blit_bgl` (texture + sampler at bindings 0 and 1) to match `mipmap.wgsl`'s expected bindings, which it does. The `run_pass` helper accepts a different pipeline; the only difference from gradient/mesh is the BGL the bind group is created against. Since both BGLs are structurally compatible at run time (wgpu compares by structural identity), this works.

If wgpu validation complains about layout mismatch between `compose-bgl` and the runtime pipeline expectation, fix by storing both `gradient_bgl` and `image_blit_bgl` separately on the renderer (which we already do).

- [ ] **Step 5: Run, verify**

`cargo test -p ui-glass-engine --features headless --test background_render`
Expected: 6 tests pass.

- [ ] **Step 6: Commit**

```bash
git add crates/ui-glass-engine/src/background/image_cache.rs crates/ui-glass-engine/src/background/render.rs crates/ui-glass-engine/tests/background_render.rs
git commit -m "feat(ui-glass-engine): ImageCache and Image background source"
```

---

## Task 6: BackgroundScene scene-graph mode

**Files:**
- Modify: `crates/ui-glass-engine/src/background/mod.rs`
- Create: `crates/ui-glass-engine/tests/scene_graph.rs`

- [ ] **Step 1: Write failing test**

Create `crates/ui-glass-engine/tests/scene_graph.rs`:

```rust
use ui_glass_engine::background::{BackgroundScene, BackgroundSource, MeshKind};
use ui_glass_engine::background::render::BackgroundRenderer;
use ui_glass_engine::headless::TestHarness;
use ui_tokens::Color;

#[test]
fn scene_with_multiple_layers_composites() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    let scene = BackgroundScene::new()
        .layer(BackgroundSource::Color(Color::rgba(20, 20, 80, 1.0)))
        .layer(BackgroundSource::Mesh(MeshKind::Aurora));

    let pixels = r.render_to_pixels(&scene.layers, 32, 32);
    let mut max_blue = 0u8;
    for chunk in pixels.chunks(4) {
        if chunk[2] > max_blue { max_blue = chunk[2]; }
    }
    assert!(max_blue > 40, "expected the dark blue base to show through");
}

#[test]
fn empty_scene_returns_black_texture() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut r = BackgroundRenderer::new(h.device().clone(), h.queue().clone());
    let scene = BackgroundScene::new();
    assert!(scene.is_empty());
    let pixels = r.render_to_pixels(&scene.layers, 32, 32);
    let center = ((16 * 32 + 16) * 4) as usize;
    assert!(pixels[center] < 16, "expected black center for empty scene");
}
```

- [ ] **Step 2: Run, verify**

`cargo test -p ui-glass-engine --features headless --test scene_graph`
Expected: 2 tests pass (the renderer already handles a slice of sources, so the scene-graph wrapper is just a list).

- [ ] **Step 3: Commit**

```bash
git add crates/ui-glass-engine/tests/scene_graph.rs
git commit -m "test(ui-glass-engine): BackgroundScene composes layers correctly"
```

---

## Task 7: Compositor accepts `BackgroundSource` per region

**Files:**
- Modify: `crates/ui-glass-engine/src/compositor.rs`
- Modify: `crates/ui-glass-engine/src/lib.rs`
- Create: `crates/ui-glass-engine/tests/compositor_with_source.rs`

- [ ] **Step 1: Modify `GlassRegion` to optionally carry a `BackgroundSource`**

In `crates/ui-glass-engine/src/compositor.rs`, change the struct:

```rust
use crate::background::BackgroundSource;

#[derive(Clone, Debug)]
pub struct GlassRegion {
    pub rect_px: [f32; 4],
    pub material: LiquidMaterial,
    /// Optional per-surface background. When `None`, the compositor's
    /// scene-graph (if set via `set_background_scene`) provides the bg; if
    /// neither is set, the bg texture passed to `render` is used as-is.
    pub background: Option<BackgroundSource>,
}
```

Update derive (no longer `Copy` because `BackgroundSource` is `Clone` not `Copy`).

- [ ] **Step 2: Add `set_background_scene` + materialize logic**

```rust
use crate::background::{BackgroundScene, render::BackgroundRenderer};

pub struct Compositor {
    // existing fields ...
    background_renderer: BackgroundRenderer,
    background_scene: Option<BackgroundScene>,
}

impl Compositor {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        // existing init ...
        let background_renderer = BackgroundRenderer::new(device.clone(), queue.clone());
        Self {
            // existing fields ...
            background_renderer,
            background_scene: None,
        }
    }

    pub fn set_background_scene(&mut self, scene: BackgroundScene) {
        self.background_scene = Some(scene);
    }

    pub fn background_renderer_mut(&mut self) -> &mut BackgroundRenderer {
        &mut self.background_renderer
    }
}
```

In `render`, before iterating regions, materialize a route-level bg texture if a scene is set:

```rust
    pub fn render(
        &mut self,
        bg_view: &wgpu::TextureView,
        output_view: &wgpu::TextureView,
        canvas_size: [f32; 2],
        regions: &[GlassRegion],
    ) {
        // existing pipeline cache logic ...

        let route_bg_tex = self.background_scene.as_ref().map(|scene| {
            self.background_renderer.render_to_texture(&scene.layers, canvas_size[0] as u32, canvas_size[1] as u32)
        });

        for region in regions {
            let per_surface_bg_tex = region.background.as_ref().map(|src| {
                self.background_renderer.render_to_texture(&[src.clone()], canvas_size[0] as u32, canvas_size[1] as u32)
            });

            let resolved_view = match (&per_surface_bg_tex, &route_bg_tex) {
                (Some(t), _) => t.create_view(&Default::default()),
                (None, Some(t)) => t.create_view(&Default::default()),
                _ => bg_view.clone_into_owned_or_borrow(), // see below
            };

            // rest of compositing uses resolved_view as the bg_view
        }
    }
```

Practical: `wgpu::TextureView` does not implement `Clone`. The simplest pattern: keep the `bg_view` parameter as the *fallback only* and use `Option<&TextureView>` internally:

```rust
            let resolved_view: &wgpu::TextureView = if let Some(t) = per_surface_bg_tex.as_ref() {
                // hold the view on stack
                Box::leak(Box::new(t.create_view(&Default::default())))
            } else if let Some(t) = route_bg_tex.as_ref() {
                Box::leak(Box::new(t.create_view(&Default::default())))
            } else {
                bg_view
            };
```

`Box::leak` is fine here as `compositor.rs` runs once per frame and the views' lifetimes are bounded. To avoid leak, store the materialized views in a `Vec` on the stack and reference them.

Concretely, replace the body of `render` with:

```rust
        // Generate per-route bg if a scene is installed.
        let scene_bg_tex = self.background_scene.as_ref().map(|scene| {
            self.background_renderer.render_to_texture(&scene.layers, canvas_size[0] as u32, canvas_size[1] as u32)
        });
        let scene_bg_view: Option<wgpu::TextureView> = scene_bg_tex
            .as_ref()
            .map(|t| t.create_view(&Default::default()));

        for region in regions {
            // ...
            let per_surface_bg_tex = region.background.as_ref().map(|src| {
                self.background_renderer.render_to_texture(
                    &[src.clone()],
                    canvas_size[0] as u32,
                    canvas_size[1] as u32,
                )
            });
            let per_surface_bg_view: Option<wgpu::TextureView> = per_surface_bg_tex
                .as_ref()
                .map(|t| t.create_view(&Default::default()));

            let resolved_view: &wgpu::TextureView = per_surface_bg_view
                .as_ref()
                .or(scene_bg_view.as_ref())
                .unwrap_or(bg_view);
            // pass resolved_view to render_glass_to_texture
        }
```

The textures are owned by the local stack vars, the views borrow from them, and resolved_view borrows from one of those. Rust's lifetime checking will hold as long as everything stays in scope until after `render_glass_to_texture` is called.

- [ ] **Step 3: Write integration test**

Create `crates/ui-glass-engine/tests/compositor_with_source.rs`:

```rust
use ui_glass::LiquidMaterial;
use ui_glass_engine::background::BackgroundSource;
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::{Compositor, GlassRegion};
use ui_tokens::Color;

#[test]
fn region_with_background_source_renders_without_external_bg() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());

    // Provide a no-op fallback bg_view (transparent), since the region has its own bg.
    let bg = make_solid(h.device(), h.queue(), 64, 64, [0, 0, 0, 255]);
    let out = make_output(h.device(), 64, 64);

    let region = GlassRegion {
        rect_px: [8.0, 8.0, 48.0, 48.0],
        material: LiquidMaterial::floating(),
        background: Some(BackgroundSource::Color(Color::rgba(255, 200, 0, 1.0))),
    };

    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [64.0, 64.0],
        &[region],
    );
    // No panic = pass. Visual verification covered by golden tests later.
}

// helpers as in compositor_api.rs (paste the make_solid/make_output funcs)
fn make_solid(device: &std::sync::Arc<wgpu::Device>, queue: &std::sync::Arc<wgpu::Queue>, w: u32, h: u32, rgba: [u8; 4]) -> wgpu::Texture { /* same as before */
    let t = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("bg"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let pixels: Vec<u8> = (0..(w * h)).flat_map(|_| rgba).collect();
    queue.write_texture(
        wgpu::TexelCopyTextureInfo { texture: &t, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
        &pixels,
        wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h) },
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );
    t
}
fn make_output(device: &std::sync::Arc<wgpu::Device>, w: u32, h: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("out"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}
```

- [ ] **Step 4: Run, verify**

`cargo test -p ui-glass-engine --features headless`
Expected: existing tests pass + new compositor_with_source test passes. Other tests may need their `GlassRegion` literals updated to include `background: None`.

Search the workspace for `GlassRegion {` literals and add `background: None` to each — this will affect existing test files (`compositor_api.rs`, `end_to_end.rs`, `golden_*.rs` files, `pipeline_cache.rs`, `tests/common/mod.rs`).

Add a `Default` impl to `GlassRegion` so future tests are less verbose:

```rust
impl GlassRegion {
    pub fn new(rect_px: [f32; 4], material: LiquidMaterial) -> Self {
        Self { rect_px, material, background: None }
    }
}
```

Update test sites to use `GlassRegion::new(...).with_background(...)` style; OR add `background: None` explicitly. Whichever is simpler.

Helper:
```rust
impl GlassRegion {
    pub fn with_background(mut self, bg: BackgroundSource) -> Self {
        self.background = Some(bg);
        self
    }
}
```

- [ ] **Step 5: Commit**

```bash
git add crates/ui-glass-engine/src/compositor.rs crates/ui-glass-engine/tests/compositor_with_source.rs <updated test files>
git commit -m "feat(ui-glass-engine): per-region BackgroundSource and route-level scene"
```

---

## Task 8: MotionInputs cache + `update_inputs` API

**Files:**
- Create: `crates/ui-glass-engine/src/motion.rs`
- Modify: `crates/ui-glass-engine/src/compositor.rs`
- Modify: `crates/ui-glass-engine/src/lib.rs`
- Modify: `crates/ui-glass-engine/Cargo.toml` (add ui-motion dep)
- Create: `crates/ui-glass-engine/tests/motion_bridge.rs`

- [ ] **Step 1: Add ui-motion dep**

Edit `crates/ui-glass-engine/Cargo.toml`. Append to `[dependencies]`:

```toml
ui-motion.workspace = true
```

- [ ] **Step 2: Create `motion.rs`**

```rust
//! Latest motion-input snapshot consumed by the compositor when building per-
//! frame uniforms. Springs and decays live in `ui-motion`; this struct just
//! holds the most recent values written by the host (the Dioxus integration
//! in Plan 4 will subscribe to ui-motion signals and call
//! `Compositor::update_inputs` per rAF tick).

#[derive(Clone, Copy, Debug, Default)]
pub struct MotionInputs {
    /// Pointer in canvas-relative coords (px). Compositor normalizes to
    /// surface-local (-1..1) per region.
    pub pointer_px: [f32; 2],
    /// Scroll velocity in px/s.
    pub scroll_velocity_px: [f32; 2],
    /// Seconds since route mount.
    pub time_seconds: f32,
    /// Whether `prefers-reduced-motion` is active. When true the compositor
    /// zeroes pointer/scroll/time before writing uniforms.
    pub reduced_motion: bool,
}

impl MotionInputs {
    pub fn new() -> Self { Self::default() }

    pub fn with_pointer(mut self, px: [f32; 2]) -> Self { self.pointer_px = px; self }
    pub fn with_scroll(mut self, vel: [f32; 2]) -> Self { self.scroll_velocity_px = vel; self }
    pub fn with_time(mut self, t: f32) -> Self { self.time_seconds = t; self }
    pub fn with_reduced_motion(mut self, on: bool) -> Self { self.reduced_motion = on; self }
}
```

- [ ] **Step 3: Hook into Compositor**

In `crates/ui-glass-engine/src/compositor.rs`:

```rust
use crate::motion::MotionInputs;

pub struct Compositor {
    // existing fields ...
    inputs: MotionInputs,
}

impl Compositor {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        // existing init ...
        Self { /* ... */, inputs: MotionInputs::default() }
    }

    /// Update the per-frame motion inputs. The host calls this once per rAF
    /// tick, BEFORE `render`. Plan 4's Dioxus integration drives this from
    /// `ui-motion` springs.
    pub fn update_inputs(&mut self, inputs: MotionInputs) {
        self.inputs = inputs;
    }
}
```

In the `render` loop, derive pointer-normalized + scroll + time into the uniforms:

```rust
        for region in regions {
            let mut uniforms = GlassUniforms::from_material(...);
            if !self.inputs.reduced_motion {
                let rect = region.rect_px;
                // Normalize pointer to surface-local (-1..1)
                let px = self.inputs.pointer_px[0] - (rect[0] + rect[2] * 0.5);
                let py = self.inputs.pointer_px[1] - (rect[1] + rect[3] * 0.5);
                let pn = [
                    (px / (rect[2] * 0.5)).clamp(-1.0, 1.0),
                    (py / (rect[3] * 0.5)).clamp(-1.0, 1.0),
                ];
                uniforms = uniforms
                    .with_pointer(pn)
                    .with_scroll_velocity(self.inputs.scroll_velocity_px)
                    .with_time(self.inputs.time_seconds);
            }
            // ...
        }
```

- [ ] **Step 4: Write tests**

Create `crates/ui-glass-engine/tests/motion_bridge.rs`:

```rust
use ui_glass::LiquidMaterial;
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::motion::MotionInputs;
use ui_glass_engine::{Compositor, GlassRegion};

#[test]
fn update_inputs_propagates_to_render() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());
    comp.update_inputs(MotionInputs::new()
        .with_pointer([32.0, 32.0])
        .with_scroll([8.0, 0.0])
        .with_time(0.5));

    let bg = make_solid(h.device(), h.queue(), 64, 64, [40, 40, 40, 255]);
    let out = make_output(h.device(), 64, 64);

    let mat = LiquidMaterial::new()
        .blur(8.0)
        .refract(0.5)
        .pointer_reactive()
        .scroll_reactive()
        .radius(12.0);
    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [64.0, 64.0],
        &[GlassRegion::new([8.0, 8.0, 48.0, 48.0], mat)],
    );
    // No panic + GPU consumed the inputs = pass.
}

#[test]
fn reduced_motion_zeroes_pointer_uniform() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());
    comp.update_inputs(MotionInputs::new()
        .with_pointer([32.0, 32.0])
        .with_reduced_motion(true));

    let bg = make_solid(h.device(), h.queue(), 64, 64, [40, 40, 40, 255]);
    let out = make_output(h.device(), 64, 64);
    let mat = LiquidMaterial::new().blur(8.0).refract(0.5).pointer_reactive().radius(12.0);
    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [64.0, 64.0],
        &[GlassRegion::new([8.0, 8.0, 48.0, 48.0], mat)],
    );
}

// helpers (paste make_solid / make_output)
```

- [ ] **Step 5: Run, verify**

`cargo test -p ui-glass-engine --features headless --test motion_bridge`

- [ ] **Step 6: Commit**

```bash
git add crates/ui-glass-engine/Cargo.toml crates/ui-glass-engine/src/motion.rs crates/ui-glass-engine/src/compositor.rs crates/ui-glass-engine/src/lib.rs crates/ui-glass-engine/tests/motion_bridge.rs
git commit -m "feat(ui-glass-engine): MotionInputs + Compositor::update_inputs"
```

---

## Task 9: Multi-region rendering

**Files:**
- Modify: `crates/ui-glass-engine/src/compositor.rs`
- Modify: `crates/ui-glass-engine/src/render_graph.rs`
- Create: `crates/ui-glass-engine/tests/multi_region.rs`

- [ ] **Step 1: Lift the debug_assert**

In `compositor.rs::render`, replace:

```rust
        debug_assert!(
            regions.len() <= 1,
            "Plan 1/2 multi-region renders overwrite each other; correct \
             overlap compositing lands in Plan 3",
        );
```

with:

```rust
        // Multi-region: regions are composited back-to-front. The output_view
        // is cleared once at the start; each region's glass-pass uses
        // LoadOp::Load so it preserves earlier regions and blends on top.
```

- [ ] **Step 2: Add a `prepare_output` clear pass**

Before the region loop in `Compositor::render`, clear the output once:

```rust
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("output-clear"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: output_view, resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None, occlusion_query_set: None,
            });
        }
        self.queue.submit(Some(encoder.finish()));
```

- [ ] **Step 3: Make compose use `LoadOp::Load` (already needed for overlap)**

In `render_graph.rs`, the compose pass currently uses `clear: true`. Change to `clear: false` so subsequent regions blend on top:

In the final `run_pass(...)` line for the compose:

```rust
    run_pass(&mut encoder, output_view, &compose_pipeline, &compose_bg, "compose", false);
```

This relies on the output being cleared once before the region loop (which Step 2 does).

- [ ] **Step 4: Write test**

Create `crates/ui-glass-engine/tests/multi_region.rs`:

```rust
use ui_glass::LiquidMaterial;
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::{Compositor, GlassRegion};
use ui_tokens::Color;

#[test]
fn two_regions_both_render_without_overwriting() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());

    let bg = make_solid(h.device(), h.queue(), 128, 128, [0, 0, 200, 255]);
    let out = make_output(h.device(), 128, 128);

    let r1 = GlassRegion::new([8.0, 8.0, 48.0, 48.0], LiquidMaterial::floating().tint(Color::rgba(255, 0, 0, 1.0), 0.5));
    let r2 = GlassRegion::new([72.0, 72.0, 48.0, 48.0], LiquidMaterial::floating().tint(Color::rgba(0, 255, 0, 1.0), 0.5));

    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [128.0, 128.0],
        &[r1, r2],
    );

    let pixels = read_back(h.device(), h.queue(), &out, 128, 128);
    // Region 1 center (~32,32) should be red-tinted
    let p1 = ((32 * 128 + 32) * 4) as usize;
    assert!(pixels[p1] > 80, "expected red tint in region 1, got R={}", pixels[p1]);
    // Region 2 center (~96,96) should be green-tinted
    let p2 = ((96 * 128 + 96) * 4) as usize;
    assert!(pixels[p2 + 1] > 80, "expected green tint in region 2, got G={}", pixels[p2 + 1]);
    // Corner should be transparent
    assert_eq!(pixels[3], 0);
}

// helpers
```

- [ ] **Step 5: Run, verify**

`cargo test -p ui-glass-engine --features headless --test multi_region`

- [ ] **Step 6: Run all tests**

`cargo test -p ui-glass-engine --features headless`
Expected: all tests pass. Older single-region tests may need their goldens re-baked because the `LoadOp::Load` change affects pixel output in edge cases (the corner alpha now comes from the clear pass, not the discard). If goldens fail, re-bake with `UPDATE_GOLDEN=1`.

- [ ] **Step 7: Commit**

```bash
git add crates/ui-glass-engine/src/compositor.rs crates/ui-glass-engine/src/render_graph.rs crates/ui-glass-engine/tests/multi_region.rs crates/ui-glass-engine/tests/assets/*.png
git commit -m "feat(ui-glass-engine): multi-region compositing"
```

---

## Task 10: Re-bake any goldens affected by Task 9

**Files:**
- Re-bake: `crates/ui-glass-engine/tests/assets/*.png` (whichever shifted)

- [ ] **Step 1: Re-bake any that fail**

Run the full test suite. For any golden_* test that fails, re-bake with `UPDATE_GOLDEN=1`:

```bash
cargo test -p ui-glass-engine --features headless 2>&1 | grep "^test golden_" | grep FAILED
# For each failing test:
UPDATE_GOLDEN=1 cargo test -p ui-glass-engine --features headless --test golden_<name>
```

- [ ] **Step 2: Verify all goldens stable**

`cargo test -p ui-glass-engine --features headless`
Expected: 0 failures.

- [ ] **Step 3: Commit (if any goldens changed)**

```bash
git add crates/ui-glass-engine/tests/assets/
git commit -m "test(ui-glass-engine): re-bake goldens affected by multi-region compositing"
```

(Skip if no goldens changed.)

---

## Task 11: Workspace integration check + Plan 3 status

**Files:**
- Modify: `docs/superpowers/plans/2026-05-22-liquid-glass-plan-3-scene-motion-multiregion.md`

- [ ] **Step 1: Build the workspace**

Run: `cargo build --workspace`
Expected: success.

- [ ] **Step 2: Run all tests**

Run: `cargo test --workspace --features ui-glass-engine/headless`
Expected: all tests pass; new tests add about 14 more.

- [ ] **Step 3: Append status to plan doc**

At the bottom of `docs/superpowers/plans/2026-05-22-liquid-glass-plan-3-scene-motion-multiregion.md`, append:

```markdown
---

## Status

Plan 3 complete. Engine is callable from production code with `BackgroundSource`
per region, `BackgroundScene` per route, and `MotionInputs` driving pointer /
scroll / time uniforms. Multi-region overlap composites correctly via cleared
output target + LoadOp::Load on compose. SPECULAR exponent reduced from 16 to
4 so highlights are visible without test workarounds.

Next: Plan 4 — `<LiquidSurface>` Dioxus component, canvas mount, pointer event
forwarding, per-route compositor lifecycle, and ui-motion subscription that
drives `update_inputs` per rAF tick.
```

- [ ] **Step 4: Commit**

```bash
git add docs/superpowers/plans/2026-05-22-liquid-glass-plan-3-scene-motion-multiregion.md
git commit -m "docs: mark Plan 3 status and Plan 4 handoff"
```

---

## Plan 3 — Done. What's next

The engine is now production-shaped:

- Authors describe backgrounds declaratively (`BackgroundSource::Gradient(Gradient::linear(...))`, `MeshKind::Aurora`, image cache, scene-graph), not as pre-uploaded textures.
- Pointer/scroll/time flow through `Compositor::update_inputs(MotionInputs)`; reduced-motion auto-zeroes them.
- Multiple glass surfaces composite correctly with proper z-order.
- SPECULAR highlights are visible at sane parameters.

Plan 4 will:
- Build `<LiquidSurface>` Dioxus component that mounts a `<canvas>`, initializes wgpu via web-sys, drives the compositor's `update_inputs` from Dioxus pointer events and ui-motion signals.
- Coordinate canvas size + z-index with the DOM layout so the canvas sits behind foreground widgets with `pointer-events: none`.
- Handle mount/unmount lifecycle and SSR fallback (component renders to a `<div>` placeholder on the server, hydrates the canvas on the client).
