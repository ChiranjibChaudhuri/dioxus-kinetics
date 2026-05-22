# Liquid Glass Engine — Design

Status: Draft for review
Author: Brainstormed with Claude
Date: 2026-05-22

## Goal

Replace the current CSS-only `backdrop-filter`-based glass implementation with a
Rust-native rendering engine that achieves iOS 26 / macOS Tahoe "Liquid Glass"
fidelity uniformly across Dioxus Web, Mobile, Desktop, and Native targets.

The current `ui-glass` + `ui-styles` implementation produces a frosted surface
with blur, saturation, and a tinted overlay. It does not produce real refraction,
chromatic dispersion at edges, pointer-reactive deformation, scroll parallax,
dynamic specular highlights, or background-aware tint adaptation. Those are the
defining traits of the target aesthetic. This spec defines an engine that
delivers all of them.

## Visual target

iOS 26 / macOS Tahoe Liquid Glass. Nine traits, all mandatory for v1:

1. Backdrop blur (multi-tap separable Gaussian).
2. Refraction (background sampled with displaced UVs driven by turbulence noise
   plus surface curvature derived from a signed-distance field).
3. Chromatic dispersion at edges (RGB channels sampled with slightly different
   displacement amounts).
4. Specular edge highlight (virtual light position drives a bright rim).
5. Inner shadow / ambient occlusion (soft dark ring inside the edge for depth).
6. Pointer-reactive deformation (refraction and specular track a spring-damped
   pointer position).
7. Scroll-reactive parallax (refraction noise scrolls against content velocity).
8. Mesh ambient lighting (slow animated mesh gradient layered into specular).
9. Tint adaptation (glass picks up dominant background color via mip sampling).

## Platform priority

Web > Mobile > Desktop > Native, with uniform fidelity required across all
four. Three of four targets render through a WebView; only Native uses Blitz +
direct wgpu.

## Architecture

### Crate layout

```
ui-glass            (existing, refactored)  descriptors only
  GlassLevel/Depth/Tone/Density/Edge        unchanged semantic recipes
  LiquidMaterial                            new full shader-param struct
  MaterialBuilder                           new DSL surface

ui-glass-engine     (new)                   render pipeline
  Compositor                                per-route render coordinator
  wgpu pipeline + WGSL uber-shader
  Naga transpile path for WebGPU and WebGL2
  BackgroundSource (scene-graph or texture)
  MotionBridge (subscribes to ui-motion)

ui-styles           (existing, narrowed)    CSS fallback only
  SVG filter chain for non-wgpu environments
  Solid surface for reduced-transparency policy
```

### Render flow per frame

1. `ui-runtime` collects all `<LiquidSurface>` mounts for the current route.
2. `ui-glass-engine::Compositor` receives a list of
   `(rect, LiquidMaterial, BackgroundSource)` tuples in z-order, plus the
   current pointer + scroll state from `ui-motion`.
3. Single wgpu render pass:
   a. Render background sources into texture `A`.
   b. For each glass region (back-to-front), sample `A` with the uber-shader,
      then write back into `A` so the next surface sees the correctly
      composited result.
   c. Present `A` to the canvas (web) or framebuffer (native).
4. DOM widgets (text, controls) layer over the canvas with transparent
   backgrounds.

### Compositor placement

- Web, Desktop-Wry, Mobile-WebView: a `<canvas>` absolutely positioned at
  `z-index: 0`, `pointer-events: none`. DOM widgets sit at `z-index: 1+`.
- Native (Blitz): a render pass injected into Blitz's render graph between
  background draw and foreground widget draw.

### Backend selection

Five tiers, auto-selected per surface; can be forced via `GlassPolicy` or
`QualityProfile`.

```
Tier 1  WebGPU + all features         all 9 traits, 13-tap blur, mip-6 adapt
Tier 2  WebGPU + reduced quality       9-tap blur, mip-4 adapt, clamp dispersion
Tier 3  wgpu via WebGL2                same shader transpiled by naga
Tier 4  SVG filter chain               backdrop-filter url chain, no reactivity
Tier 5  Solid CSS surface              var(--ui-glass-solid), no filters
```

Auto-detect inputs: `navigator.userAgentData`, `performance.memory` when
available, `prefers-reduced-motion`, and a rolling 60-frame timing average.

## API design

### Layer 1 — semantic recipes (unchanged)

Existing types continue to work:

```rust
let req = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Primary)
    .with_density(MaterialDensity::Comfortable)
    .with_edge(MaterialEdge::Hairline)
    .with_vibrancy(MaterialVibrancy::Standard);
```

`From<MaterialRequest> for LiquidMaterial` derives a sensible default.

### Layer 2 — LiquidMaterial

```rust
pub struct LiquidMaterial {
    pub tint: Color,
    pub tint_alpha: f32,
    pub blur_radius_px: f32,
    pub saturation: f32,

    pub refraction_strength: f32,
    pub surface_curvature: f32,
    pub noise_frequency: f32,
    pub noise_seed: f32,

    pub dispersion_px: f32,

    pub light_angle_rad: f32,
    pub light_intensity: f32,
    pub edge_falloff_px: f32,

    pub inner_shadow_px: f32,
    pub inner_shadow_alpha: f32,

    pub pointer_reactive: bool,
    pub scroll_reactive: bool,
    pub ambient_mesh: Option<AmbientMesh>,

    pub adapt_to_background: f32,

    pub radius_px: f32,
    pub thickness_px: f32,

    pub features: GlassFeatures,  // bitflags driving spec constants
}
```

### Layer 3 — builder DSL

```rust
let mat = LiquidMaterial::floating()
    .blur(18.0)
    .refract(0.35)
    .disperse(2.0)
    .specular(angle_deg: 45.0, intensity: 0.8)
    .inner_shadow(4.0, 0.18)
    .pointer_reactive()
    .scroll_reactive()
    .ambient_mesh(AmbientMesh::aurora())
    .adapt_to_background(0.4)
    .radius(20.0);
```

Each builder method sets fields and flips bits in `GlassFeatures`.

### Presets

```
LiquidMaterial::chrome()       app shell, heavy blur, low refract
LiquidMaterial::floating()     cards, medium everything
LiquidMaterial::overlay()      modals, strong refract + dispersion
LiquidMaterial::sheet()        bottom sheets, mesh ambient on
LiquidMaterial::tooltip()      tight blur, no reactivity
LiquidMaterial::button()       small surface, pointer-reactive specular
```

## Shader design

One WGSL uber-shader, all nine traits gated by `override` specialization
constants so disabled features get branch-eliminated at pipeline-create time.
Pipelines cached keyed by `(GlassFeatures, BLUR_TAPS)`.

### Pipeline structure

Vertex shader is a trivial full-screen quad per glass region. The fragment
shader does the work.

### Fragment shader stages

1. Compute surface-local coords and signed distance to the rounded-rect edge.
2. Compute normal from SDF gradient plus thickness curvature.
3. Sample displacement: turbulence noise (uploaded 256x256 RGBA texture),
   optionally biased toward pointer and against scroll velocity.
4. Multi-tap separable Gaussian blur (13 taps default, 9 in Tier 2, 5 floor).
5. Chromatic dispersion: re-sample R and B channels with offset along normal.
6. Saturation + tint, with optional mip-6 background sampling for tint
   adaptation.
7. Specular edge: virtual light dot-product against surface normal, masked
   against edge falloff.
8. Inner shadow: smoothstep over signed distance into the surface.
9. Ambient mesh contribution: procedurally evaluated against `time_seconds`.

### Uniforms

One `GlassUniforms` buffer per surface, updated once per rAF tick. Layout:

```
rect             vec4<f32>   x, y, w, h in canvas pixels
radius           f32         corner radius
thickness        f32         affects refraction strength and edge highlight
tint             vec4<f32>   rgba
blur_radius      f32
saturation       f32
refract_strength f32
surface_curvature f32
noise_frequency  f32
noise_seed       f32
dispersion_px    f32
light_dir        vec2<f32>   unit vector from light_angle_rad
light_intensity  f32
edge_falloff     f32
inner_shadow_px  f32
inner_shadow_alpha f32
adapt_strength   f32
pointer          vec2<f32>   normalized to surface, -1..1
scroll_velocity  vec2<f32>
time_seconds     f32
```

### Geometry

SDF-only in v1. Surfaces are rounded rectangles (trivial radius-0 case included).
Custom-path glass is out of scope for v1; would require a separate pipeline with
uploaded path geometry.

### Noise

Uploaded once at engine start. 256x256 RGBA texture generated by combining
Worley and Perlin noise via `ui-glass-engine` build script or first-frame init.
~256KB asset, shared across all surfaces. Procedural noise rejected (about 2x
slower per fragment).

### Alpha

Premultiplied alpha throughout to keep overlapping surfaces blending correctly.

## Background scene contract

Two modes, can coexist:

### Mode A — per-surface texture

```rust
LiquidSurface::new(material)
    .background(BackgroundSource::gradient(Gradient::conic(...)))
    .background(BackgroundSource::image("/hero.jpg"))
    .background(BackgroundSource::mesh(AmbientMesh::aurora()))
```

Engine renders these into a small offscreen target sized to the surface's rect
plus an ~80px bleed so refraction near edges still has content.

### Mode B — full scene-graph

```rust
BackgroundScene::new()
    .layer(Gradient::linear(...))
    .layer(Image::static_("/hero.jpg"))
    .layer(Mesh::aurora())
    .install(cx);
```

Per-route, hosted by `ui-runtime`. Renders into one viewport-sized texture that
all surfaces sample from. Cheaper for many overlapping surfaces.

Selection is automatic. If a `BackgroundScene` is installed on the route,
surfaces sample from it. A surface with explicit `.background(...)` overrides
the scene-graph for its region.

### Sources

```
Gradient::linear/radial/conic    generated in shader, no upload
Image::static_(url)              loaded once, cached, uploaded as wgpu texture
Image::dynamic(handle)           author-driven texture handle hosted by ui-runtime
Mesh::aurora/orbs/grain          procedural shader passes
Color::solid(c)                  trivial fill
```

### Lifecycle

Background texture double-buffered. Mipmaps uploaded for `adapt_to_background`'s
mip-6 sample. Re-rendered only when source layers change or animate.

## Motion integration

`ui-motion` provides the springs and decays. The engine subscribes.

### Pointer

```rust
let pointer_spring = use_spring(MotionConfig {
    stiffness: 180.0,
    damping: 22.0,
    initial: Vec2::ZERO,
});
```

Spring lag of ~80ms is what makes the surface read as liquid rather than as a
hard cursor-tracker.

### Scroll

```rust
let scroll_velocity = use_decay(MotionConfig { decay: 0.92 });
```

Decays to zero in ~400ms after scroll stops.

### Time

`uniform.time_seconds` set from `now() - mount_time`.

### Cadence

All three uniforms written once per rAF tick, just before the render pass.
Surfaces without reactive flags skip the write; default-zero uniforms produce
no contribution. `prefers-reduced-motion: reduce` disables springs and decays
upstream in `ui-motion`.

## Quality profiles

```
QualityProfile::High      Tier 1 defaults
QualityProfile::Balanced  Tier 1 with 9-tap blur, mesh ambient off
QualityProfile::Power     Tier 2 forced (battery-saving)
QualityProfile::Off       Tier 5 forced
```

`cx.set_glass_quality(...)` overrides auto-detection.

## Migration

Four phases.

1. **Phase 0** — `ui-glass-engine` ships behind a `liquid-glass` feature flag.
   `ui-styles` unchanged. Existing `<GlassSurface>` keeps rendering through CSS.
   New `<LiquidSurface>` is the opt-in entry point.
2. **Phase 1** — `<GlassSurface>` checks for engine availability at mount; if
   engine is present and Tier ≥3 is achievable, it routes through the engine
   via `MaterialRequest::into::<LiquidMaterial>()`. Falls back to CSS otherwise.
   No code change for existing consumers.
3. **Phase 2** — `liquid-glass` feature default-on. CSS path remains for
   Tier 4/5.
4. **Phase 3** — deprecate redundant CSS. `ui-styles` keeps only the SVG filter
   chain (Tier 4) and solid fallback (Tier 5). `.ui-glass-surface[data-glass-*]`
   rules are removed.

## Testing

- **Golden-image tests.** Each preset rendered on each tier into a PNG,
  compared against checked-in references with per-pixel tolerance. Captured
  via `ui-capture`. Headless wgpu for native, Playwright for WebView targets.
- **Shader unit tests.** WGSL `@compute` test shaders exercise SDF, blur, and
  refraction in isolation; outputs written to a storage buffer and compared
  bit-exact.
- **Performance benchmarks.** `criterion` suite measuring uniform-update cost,
  pipeline-create cost, and frame time at 1, 4, and 16 simultaneous glass
  surfaces.
- **Accessibility.** Extend `ui-capture`'s contrast validator to evaluate
  against the blurred-glass surface in Tier 1-3, not just the solid fallback.

## Out of scope for v1

- Custom-path (non-rounded-rect) glass geometry.
- Glass-on-video (treats video as background; expensive re-upload per frame).
- HDR specular highlights.
- Light source array (more than one virtual light per surface).
- Per-surface noise customization beyond `noise_frequency` and `noise_seed`.
- Procedural noise (uploaded texture is the chosen path).
