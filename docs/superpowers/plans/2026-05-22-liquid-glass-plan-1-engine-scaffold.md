# Liquid Glass — Plan 1: Engine Scaffold + Minimal Shader

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Stand up the `ui-glass-engine` crate with a headless wgpu render pipeline that takes a `LiquidMaterial` descriptor plus a background texture and produces a rounded-rect frosted glass surface (separable Gaussian blur + SDF mask + tint). No Dioxus integration in this plan — that's Plan 2's job. Output is a Rust library testable via headless render-to-PNG.

**Architecture:** Two-pass separable Gaussian blur into intermediate textures, then a composite pass that uses a signed-distance-field for the rounded-rect shape, samples the blurred background, and applies tint. WGSL `override` specialization constants are wired in (set to `false` for unused features) so Plan 2 can light them up without re-architecting. Pipelines cached by `(GlassFeatures, BLUR_TAPS)`.

**Tech Stack:** Rust 2021, `wgpu` (cross-platform GPU API, works native + WebGPU/WebGL2), `bytemuck` (uniform struct casting), `bitflags` (`GlassFeatures`), `pollster` (block-on async in tests), `image` (PNG read/write in tests).

---

## File Structure

**Modified files:**

```
Cargo.toml                                workspace member + workspace dep entry
crates/ui-glass/Cargo.toml                add bitflags dep
crates/ui-glass/src/lib.rs                add LiquidMaterial, GlassFeatures,
                                          builder methods, presets, From impl
```

**New files:**

```
crates/ui-glass/tests/liquid_material.rs              unit tests for new types

crates/ui-glass-engine/Cargo.toml                     new crate manifest
crates/ui-glass-engine/src/lib.rs                     module re-exports + crate docs
crates/ui-glass-engine/src/uniforms.rs                GlassUniforms (Pod/Zeroable)
crates/ui-glass-engine/src/pipeline.rs                pipeline + bind group construction,
                                                      cache keyed by features+taps
crates/ui-glass-engine/src/compositor.rs              public Compositor::render API
crates/ui-glass-engine/src/render_graph.rs            two-pass blur + composite orchestration
crates/ui-glass-engine/src/shaders/blur.wgsl          separable Gaussian blur
crates/ui-glass-engine/src/shaders/compose.wgsl       uber-shader (Plan 1: SDF + tint only)
crates/ui-glass-engine/src/headless.rs                test-only headless device factory
crates/ui-glass-engine/tests/headless_render.rs       smoke tests
crates/ui-glass-engine/tests/golden_floating.rs       golden PNG comparison
crates/ui-glass-engine/tests/assets/.gitkeep          golden image directory
```

**Responsibility boundaries:**

- `uniforms.rs` owns the GPU/CPU uniform layout contract. Nothing else writes uniform memory.
- `pipeline.rs` owns wgpu pipeline + bind group construction. Caches by feature key.
- `render_graph.rs` owns the *order* of passes (blur H → blur V → composite). It does not own pipelines or uniforms.
- `compositor.rs` is the public API surface. Holds device, queue, pipeline cache, and a single `render(...)` entry point.
- `headless.rs` is test-only: provides a `TestHarness` that wraps device + queue + render-to-Vec<u8>.
- WGSL files in `src/shaders/` are included via `include_str!` (compile-time).

---

## Task 1: Add `bitflags` workspace dependency + `GlassFeatures` to `ui-glass`

**Files:**
- Modify: `Cargo.toml`
- Modify: `crates/ui-glass/Cargo.toml`
- Modify: `crates/ui-glass/src/lib.rs`
- Create: `crates/ui-glass/tests/liquid_material.rs`

- [ ] **Step 1: Add `bitflags` to workspace dependencies**

Edit `Cargo.toml`. Append to `[workspace.dependencies]`:

```toml
bitflags = "2.6"
```

- [ ] **Step 2: Add `bitflags` to `ui-glass`**

Edit `crates/ui-glass/Cargo.toml`. Under `[dependencies]`:

```toml
[dependencies]
ui-tokens.workspace = true
bitflags.workspace = true
```

- [ ] **Step 3: Write failing test for `GlassFeatures`**

Create `crates/ui-glass/tests/liquid_material.rs`:

```rust
use ui_glass::GlassFeatures;

#[test]
fn glass_features_empty_has_no_bits_set() {
    let f = GlassFeatures::empty();
    assert!(!f.contains(GlassFeatures::BLUR));
    assert!(!f.contains(GlassFeatures::REFRACT));
    assert!(!f.contains(GlassFeatures::DISPERSE));
    assert!(!f.contains(GlassFeatures::SPECULAR));
    assert!(!f.contains(GlassFeatures::INNER_SHADOW));
    assert!(!f.contains(GlassFeatures::POINTER));
    assert!(!f.contains(GlassFeatures::SCROLL));
    assert!(!f.contains(GlassFeatures::AMBIENT_MESH));
    assert!(!f.contains(GlassFeatures::TINT_ADAPT));
}

#[test]
fn glass_features_compose_with_bitwise_or() {
    let f = GlassFeatures::BLUR | GlassFeatures::SPECULAR;
    assert!(f.contains(GlassFeatures::BLUR));
    assert!(f.contains(GlassFeatures::SPECULAR));
    assert!(!f.contains(GlassFeatures::REFRACT));
}
```

- [ ] **Step 4: Run the failing test**

Run: `cargo test -p ui-glass --test liquid_material`
Expected: FAIL — `GlassFeatures` not in scope.

- [ ] **Step 5: Implement `GlassFeatures`**

In `crates/ui-glass/src/lib.rs`, add at the top (after `use` statements):

```rust
bitflags::bitflags! {
    /// Per-trait toggles for the glass uber-shader. Each bit corresponds to a
    /// WGSL `override` specialization constant in `compose.wgsl`. Pipelines are
    /// cached keyed by the feature set, so a surface with only `BLUR | TINT_ADAPT`
    /// runs a pipeline where every other branch is eliminated at compile time.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct GlassFeatures: u32 {
        const BLUR         = 1 << 0;
        const REFRACT      = 1 << 1;
        const DISPERSE     = 1 << 2;
        const SPECULAR     = 1 << 3;
        const INNER_SHADOW = 1 << 4;
        const AMBIENT_MESH = 1 << 5;
        const POINTER      = 1 << 6;
        const SCROLL       = 1 << 7;
        const TINT_ADAPT   = 1 << 8;
    }
}
```

- [ ] **Step 6: Run the test to verify it passes**

Run: `cargo test -p ui-glass --test liquid_material`
Expected: PASS, both tests.

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml crates/ui-glass/Cargo.toml crates/ui-glass/src/lib.rs crates/ui-glass/tests/liquid_material.rs
git commit -m "feat(ui-glass): add GlassFeatures bitflags"
```

---

## Task 2: Add `LiquidMaterial` struct

**Files:**
- Modify: `crates/ui-glass/src/lib.rs`
- Modify: `crates/ui-glass/tests/liquid_material.rs`

- [ ] **Step 1: Write failing test**

Append to `crates/ui-glass/tests/liquid_material.rs`:

```rust
use ui_glass::LiquidMaterial;
use ui_tokens::Color;

#[test]
fn liquid_material_new_has_neutral_defaults() {
    let m = LiquidMaterial::new();
    assert_eq!(m.tint, Color::rgba(255, 255, 255, 1.0));
    assert_eq!(m.tint_alpha, 0.0);
    assert_eq!(m.blur_radius_px, 0.0);
    assert_eq!(m.saturation, 1.0);
    assert_eq!(m.refraction_strength, 0.0);
    assert_eq!(m.dispersion_px, 0.0);
    assert_eq!(m.light_intensity, 0.0);
    assert_eq!(m.inner_shadow_alpha, 0.0);
    assert_eq!(m.adapt_to_background, 0.0);
    assert_eq!(m.radius_px, 0.0);
    assert_eq!(m.thickness_px, 1.0);
    assert!(!m.pointer_reactive);
    assert!(!m.scroll_reactive);
    assert!(m.ambient_mesh.is_none());
    assert_eq!(m.features, ui_glass::GlassFeatures::empty());
}
```

- [ ] **Step 2: Run to confirm failure**

Run: `cargo test -p ui-glass --test liquid_material liquid_material_new_has_neutral_defaults`
Expected: FAIL — `LiquidMaterial` unresolved.

- [ ] **Step 3: Add `AmbientMesh` placeholder enum + `LiquidMaterial` struct**

Append to `crates/ui-glass/src/lib.rs`:

```rust
/// Ambient mesh contribution variants. Plan 1 carries the descriptor; the
/// shader binding lands in Plan 2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AmbientMesh {
    Aurora,
    Orbs,
    Grain,
}

/// Full shader-parameter descriptor for a Liquid Glass surface.
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub features: GlassFeatures,
}

impl LiquidMaterial {
    pub const fn new() -> Self {
        Self {
            tint: Color::rgba(255, 255, 255, 1.0),
            tint_alpha: 0.0,
            blur_radius_px: 0.0,
            saturation: 1.0,
            refraction_strength: 0.0,
            surface_curvature: 0.0,
            noise_frequency: 1.0,
            noise_seed: 0.0,
            dispersion_px: 0.0,
            light_angle_rad: 0.0,
            light_intensity: 0.0,
            edge_falloff_px: 0.0,
            inner_shadow_px: 0.0,
            inner_shadow_alpha: 0.0,
            pointer_reactive: false,
            scroll_reactive: false,
            ambient_mesh: None,
            adapt_to_background: 0.0,
            radius_px: 0.0,
            thickness_px: 1.0,
            features: GlassFeatures::empty(),
        }
    }
}

impl Default for LiquidMaterial {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 4: Run test, verify pass**

Run: `cargo test -p ui-glass --test liquid_material`
Expected: 3 tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-glass/src/lib.rs crates/ui-glass/tests/liquid_material.rs
git commit -m "feat(ui-glass): add LiquidMaterial descriptor"
```

---

## Task 3: Add builder methods

**Files:**
- Modify: `crates/ui-glass/src/lib.rs`
- Modify: `crates/ui-glass/tests/liquid_material.rs`

- [ ] **Step 1: Write failing tests**

Append to `crates/ui-glass/tests/liquid_material.rs`:

```rust
#[test]
fn builder_blur_sets_radius_and_feature() {
    let m = LiquidMaterial::new().blur(18.0);
    assert_eq!(m.blur_radius_px, 18.0);
    assert!(m.features.contains(ui_glass::GlassFeatures::BLUR));
}

#[test]
fn builder_tint_sets_color_and_alpha() {
    let m = LiquidMaterial::new().tint(Color::rgba(10, 20, 30, 1.0), 0.4);
    assert_eq!(m.tint, Color::rgba(10, 20, 30, 1.0));
    assert_eq!(m.tint_alpha, 0.4);
}

#[test]
fn builder_refract_sets_strength_and_feature() {
    let m = LiquidMaterial::new().refract(0.35);
    assert_eq!(m.refraction_strength, 0.35);
    assert!(m.features.contains(ui_glass::GlassFeatures::REFRACT));
}

#[test]
fn builder_disperse_sets_px_and_feature() {
    let m = LiquidMaterial::new().disperse(2.0);
    assert_eq!(m.dispersion_px, 2.0);
    assert!(m.features.contains(ui_glass::GlassFeatures::DISPERSE));
}

#[test]
fn builder_specular_sets_light_params_and_feature() {
    let m = LiquidMaterial::new().specular(45.0_f32.to_radians(), 0.8);
    assert!((m.light_angle_rad - 45.0_f32.to_radians()).abs() < 1e-5);
    assert_eq!(m.light_intensity, 0.8);
    assert!(m.features.contains(ui_glass::GlassFeatures::SPECULAR));
}

#[test]
fn builder_inner_shadow_sets_px_alpha_and_feature() {
    let m = LiquidMaterial::new().inner_shadow(4.0, 0.18);
    assert_eq!(m.inner_shadow_px, 4.0);
    assert_eq!(m.inner_shadow_alpha, 0.18);
    assert!(m.features.contains(ui_glass::GlassFeatures::INNER_SHADOW));
}

#[test]
fn builder_pointer_reactive_sets_flag_and_feature() {
    let m = LiquidMaterial::new().pointer_reactive();
    assert!(m.pointer_reactive);
    assert!(m.features.contains(ui_glass::GlassFeatures::POINTER));
}

#[test]
fn builder_scroll_reactive_sets_flag_and_feature() {
    let m = LiquidMaterial::new().scroll_reactive();
    assert!(m.scroll_reactive);
    assert!(m.features.contains(ui_glass::GlassFeatures::SCROLL));
}

#[test]
fn builder_ambient_mesh_sets_variant_and_feature() {
    let m = LiquidMaterial::new().ambient_mesh(ui_glass::AmbientMesh::Aurora);
    assert_eq!(m.ambient_mesh, Some(ui_glass::AmbientMesh::Aurora));
    assert!(m.features.contains(ui_glass::GlassFeatures::AMBIENT_MESH));
}

#[test]
fn builder_adapt_to_background_sets_strength_and_feature() {
    let m = LiquidMaterial::new().adapt_to_background(0.4);
    assert_eq!(m.adapt_to_background, 0.4);
    assert!(m.features.contains(ui_glass::GlassFeatures::TINT_ADAPT));
}

#[test]
fn builder_radius_and_saturation_do_not_flip_features() {
    let m = LiquidMaterial::new().radius(20.0).saturation(1.6);
    assert_eq!(m.radius_px, 20.0);
    assert_eq!(m.saturation, 1.6);
    assert_eq!(m.features, ui_glass::GlassFeatures::empty());
}

#[test]
fn builder_chains_compose_features() {
    let m = LiquidMaterial::new()
        .blur(18.0)
        .refract(0.3)
        .specular(0.78, 0.7)
        .pointer_reactive();
    assert!(m.features.contains(ui_glass::GlassFeatures::BLUR));
    assert!(m.features.contains(ui_glass::GlassFeatures::REFRACT));
    assert!(m.features.contains(ui_glass::GlassFeatures::SPECULAR));
    assert!(m.features.contains(ui_glass::GlassFeatures::POINTER));
}
```

- [ ] **Step 2: Run to confirm failures**

Run: `cargo test -p ui-glass --test liquid_material`
Expected: FAIL — builder methods don't exist.

- [ ] **Step 3: Implement builder methods**

Append to the `impl LiquidMaterial` block in `crates/ui-glass/src/lib.rs`:

```rust
    pub fn blur(mut self, radius_px: f32) -> Self {
        self.blur_radius_px = radius_px;
        self.features |= GlassFeatures::BLUR;
        self
    }

    pub fn tint(mut self, color: Color, alpha: f32) -> Self {
        self.tint = color;
        self.tint_alpha = alpha;
        self
    }

    pub fn saturation(mut self, value: f32) -> Self {
        self.saturation = value;
        self
    }

    pub fn refract(mut self, strength: f32) -> Self {
        self.refraction_strength = strength;
        self.features |= GlassFeatures::REFRACT;
        self
    }

    pub fn surface_curvature(mut self, value: f32) -> Self {
        self.surface_curvature = value;
        self
    }

    pub fn noise(mut self, frequency: f32, seed: f32) -> Self {
        self.noise_frequency = frequency;
        self.noise_seed = seed;
        self
    }

    pub fn disperse(mut self, px: f32) -> Self {
        self.dispersion_px = px;
        self.features |= GlassFeatures::DISPERSE;
        self
    }

    pub fn specular(mut self, angle_rad: f32, intensity: f32) -> Self {
        self.light_angle_rad = angle_rad;
        self.light_intensity = intensity;
        self.features |= GlassFeatures::SPECULAR;
        self
    }

    pub fn edge_falloff(mut self, px: f32) -> Self {
        self.edge_falloff_px = px;
        self
    }

    pub fn inner_shadow(mut self, px: f32, alpha: f32) -> Self {
        self.inner_shadow_px = px;
        self.inner_shadow_alpha = alpha;
        self.features |= GlassFeatures::INNER_SHADOW;
        self
    }

    pub fn pointer_reactive(mut self) -> Self {
        self.pointer_reactive = true;
        self.features |= GlassFeatures::POINTER;
        self
    }

    pub fn scroll_reactive(mut self) -> Self {
        self.scroll_reactive = true;
        self.features |= GlassFeatures::SCROLL;
        self
    }

    pub fn ambient_mesh(mut self, mesh: AmbientMesh) -> Self {
        self.ambient_mesh = Some(mesh);
        self.features |= GlassFeatures::AMBIENT_MESH;
        self
    }

    pub fn adapt_to_background(mut self, strength: f32) -> Self {
        self.adapt_to_background = strength;
        self.features |= GlassFeatures::TINT_ADAPT;
        self
    }

    pub fn radius(mut self, px: f32) -> Self {
        self.radius_px = px;
        self
    }

    pub fn thickness(mut self, px: f32) -> Self {
        self.thickness_px = px;
        self
    }
```

- [ ] **Step 4: Run tests, verify pass**

Run: `cargo test -p ui-glass --test liquid_material`
Expected: All ~14 tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-glass/src/lib.rs crates/ui-glass/tests/liquid_material.rs
git commit -m "feat(ui-glass): add LiquidMaterial builder methods"
```

---

## Task 4: Add presets

**Files:**
- Modify: `crates/ui-glass/src/lib.rs`
- Modify: `crates/ui-glass/tests/liquid_material.rs`

- [ ] **Step 1: Write failing test**

Append to `crates/ui-glass/tests/liquid_material.rs`:

```rust
use ui_glass::GlassFeatures as F;

#[test]
fn preset_floating_has_blur_specular_inner_shadow() {
    let m = LiquidMaterial::floating();
    assert!(m.features.contains(F::BLUR));
    assert!(m.features.contains(F::SPECULAR));
    assert!(m.features.contains(F::INNER_SHADOW));
    assert!(m.blur_radius_px > 0.0);
    assert!(m.radius_px > 0.0);
}

#[test]
fn preset_chrome_has_heavy_blur_low_refract() {
    let m = LiquidMaterial::chrome();
    assert!(m.features.contains(F::BLUR));
    assert!(m.blur_radius_px >= 28.0);
    if m.features.contains(F::REFRACT) {
        assert!(m.refraction_strength <= 0.2);
    }
}

#[test]
fn preset_overlay_has_strong_refract_and_disperse() {
    let m = LiquidMaterial::overlay();
    assert!(m.features.contains(F::REFRACT));
    assert!(m.features.contains(F::DISPERSE));
    assert!(m.refraction_strength >= 0.3);
}

#[test]
fn preset_sheet_has_ambient_mesh() {
    let m = LiquidMaterial::sheet();
    assert!(m.features.contains(F::AMBIENT_MESH));
}

#[test]
fn preset_tooltip_has_no_reactivity() {
    let m = LiquidMaterial::tooltip();
    assert!(!m.features.contains(F::POINTER));
    assert!(!m.features.contains(F::SCROLL));
}

#[test]
fn preset_button_is_pointer_reactive() {
    let m = LiquidMaterial::button();
    assert!(m.features.contains(F::POINTER));
}
```

- [ ] **Step 2: Run, expect failure**

Run: `cargo test -p ui-glass --test liquid_material`
Expected: FAIL — presets not defined.

- [ ] **Step 3: Implement presets**

Append to the `impl LiquidMaterial` block in `crates/ui-glass/src/lib.rs`:

```rust
    pub fn chrome() -> Self {
        Self::new()
            .blur(32.0)
            .saturation(1.6)
            .refract(0.15)
            .specular(0.78, 0.5)
            .inner_shadow(6.0, 0.18)
            .edge_falloff(2.0)
            .radius(0.0)
            .thickness(2.0)
    }

    pub fn floating() -> Self {
        Self::new()
            .blur(18.0)
            .saturation(1.6)
            .refract(0.25)
            .disperse(1.0)
            .specular(0.78, 0.6)
            .inner_shadow(4.0, 0.14)
            .edge_falloff(1.5)
            .radius(14.0)
            .thickness(1.5)
    }

    pub fn overlay() -> Self {
        Self::new()
            .blur(24.0)
            .saturation(1.8)
            .refract(0.35)
            .disperse(2.0)
            .specular(0.78, 0.7)
            .inner_shadow(6.0, 0.22)
            .edge_falloff(2.0)
            .radius(18.0)
            .thickness(2.0)
    }

    pub fn sheet() -> Self {
        Self::floating()
            .ambient_mesh(AmbientMesh::Aurora)
            .radius(20.0)
    }

    pub fn tooltip() -> Self {
        Self::new()
            .blur(10.0)
            .saturation(1.3)
            .inner_shadow(2.0, 0.10)
            .radius(8.0)
            .thickness(1.0)
    }

    pub fn button() -> Self {
        Self::new()
            .blur(12.0)
            .saturation(1.4)
            .specular(0.78, 0.5)
            .inner_shadow(2.0, 0.12)
            .pointer_reactive()
            .radius(10.0)
            .thickness(1.0)
    }
```

- [ ] **Step 4: Run, verify pass**

Run: `cargo test -p ui-glass --test liquid_material`
Expected: All tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-glass/src/lib.rs crates/ui-glass/tests/liquid_material.rs
git commit -m "feat(ui-glass): add LiquidMaterial presets"
```

---

## Task 5: Add `From<MaterialRequest> for LiquidMaterial`

**Files:**
- Modify: `crates/ui-glass/src/lib.rs`
- Modify: `crates/ui-glass/tests/liquid_material.rs`

- [ ] **Step 1: Write failing test**

Append to `crates/ui-glass/tests/liquid_material.rs`:

```rust
use ui_glass::{
    GlassDepth, MaterialDensity, MaterialEdge, MaterialPolicy, MaterialRequest, MaterialTone,
    MaterialVibrancy,
};

#[test]
fn material_request_floating_neutral_maps_to_floating_preset_baseline() {
    let req = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral);
    let m: LiquidMaterial = req.into();
    assert!(m.features.contains(F::BLUR));
    assert!(m.blur_radius_px >= 16.0 && m.blur_radius_px <= 20.0);
}

#[test]
fn material_request_modal_maps_to_overlay_preset_strength() {
    let req = MaterialRequest::new(GlassDepth::Modal, MaterialTone::Primary);
    let m: LiquidMaterial = req.into();
    assert!(m.features.contains(F::REFRACT));
    assert!(m.refraction_strength >= 0.3);
}

#[test]
fn material_request_high_contrast_clears_reactive_and_visual_features() {
    let req = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_policy(MaterialPolicy::HighContrast);
    let m: LiquidMaterial = req.into();
    assert!(!m.features.contains(F::REFRACT));
    assert!(!m.features.contains(F::DISPERSE));
    assert!(!m.features.contains(F::SPECULAR));
    assert!(!m.features.contains(F::POINTER));
    assert!(!m.features.contains(F::SCROLL));
    assert!(!m.features.contains(F::AMBIENT_MESH));
}

#[test]
fn material_request_vivid_vibrancy_increases_saturation_and_dispersion() {
    let std = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_vibrancy(MaterialVibrancy::Standard);
    let vivid = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_vibrancy(MaterialVibrancy::Vivid);
    let ms: LiquidMaterial = std.into();
    let mv: LiquidMaterial = vivid.into();
    assert!(mv.saturation > ms.saturation);
    assert!(mv.dispersion_px >= ms.dispersion_px);
}

#[test]
fn material_request_emphasized_edge_increases_falloff_and_thickness() {
    let hair = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_edge(MaterialEdge::Hairline);
    let emph = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_edge(MaterialEdge::Emphasized);
    let mh: LiquidMaterial = hair.into();
    let me: LiquidMaterial = emph.into();
    assert!(me.edge_falloff_px > mh.edge_falloff_px);
    assert!(me.thickness_px > mh.thickness_px);
}

#[test]
fn material_request_compact_density_reduces_radius() {
    let comp = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_density(MaterialDensity::Compact);
    let spac = MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
        .with_density(MaterialDensity::Spacious);
    let mc: LiquidMaterial = comp.into();
    let ms: LiquidMaterial = spac.into();
    assert!(mc.radius_px < ms.radius_px);
}
```

- [ ] **Step 2: Run, expect failure**

Run: `cargo test -p ui-glass --test liquid_material`
Expected: FAIL — `From` not implemented.

- [ ] **Step 3: Implement `From<MaterialRequest>`**

Append to `crates/ui-glass/src/lib.rs`:

```rust
impl From<MaterialRequest> for LiquidMaterial {
    fn from(req: MaterialRequest) -> Self {
        let mut m = match req.depth {
            GlassDepth::Inline | GlassDepth::Raised => LiquidMaterial::floating().blur(12.0),
            GlassDepth::Floating => LiquidMaterial::floating(),
            GlassDepth::Chrome => LiquidMaterial::chrome(),
            GlassDepth::Overlay | GlassDepth::Modal => LiquidMaterial::overlay(),
        };

        // Tone → tint (alpha applied below from depth)
        m.tint = match req.tone {
            MaterialTone::Neutral => Color::rgba(255, 255, 255, 1.0),
            MaterialTone::Primary => Color::rgba(0, 102, 204, 1.0),
            MaterialTone::Success => Color::rgba(36, 138, 61, 1.0),
            MaterialTone::Warning => Color::rgba(176, 105, 0, 1.0),
            MaterialTone::Danger => Color::rgba(196, 43, 43, 1.0),
            MaterialTone::Info => Color::rgba(20, 118, 191, 1.0),
        };
        m.tint_alpha = match req.depth {
            GlassDepth::Inline => 0.58,
            GlassDepth::Raised => 0.64,
            GlassDepth::Floating => 0.72,
            GlassDepth::Chrome => 0.68,
            GlassDepth::Overlay => 0.80,
            GlassDepth::Modal => 0.84,
        };

        // Vibrancy → saturation + dispersion
        let (sat, disp) = match req.vibrancy {
            MaterialVibrancy::Muted => (1.3, 0.0),
            MaterialVibrancy::Standard => (1.6, 1.0),
            MaterialVibrancy::Vivid => (1.8, 2.0),
        };
        m.saturation = sat;
        if m.features.contains(GlassFeatures::DISPERSE) || disp > 0.0 {
            m = m.disperse(disp);
        }

        // Edge → falloff + thickness
        let (fall, thick) = match req.edge {
            MaterialEdge::None => (0.0, 1.0),
            MaterialEdge::Hairline => (1.0, 1.0),
            MaterialEdge::Standard => (1.5, 1.5),
            MaterialEdge::Emphasized => (2.5, 2.5),
        };
        m.edge_falloff_px = fall;
        m.thickness_px = thick;

        // Density → radius scaling against current radius_px
        let scale = match req.density {
            MaterialDensity::Compact => 0.75,
            MaterialDensity::Comfortable => 1.0,
            MaterialDensity::Spacious => 1.4,
        };
        m.radius_px *= scale;

        // Policy → feature masking
        if matches!(
            req.policy,
            MaterialPolicy::HighContrast
                | MaterialPolicy::ReducedTransparency
                | MaterialPolicy::SolidFallback
        ) {
            m.features.remove(
                GlassFeatures::REFRACT
                    | GlassFeatures::DISPERSE
                    | GlassFeatures::SPECULAR
                    | GlassFeatures::POINTER
                    | GlassFeatures::SCROLL
                    | GlassFeatures::AMBIENT_MESH,
            );
            m.refraction_strength = 0.0;
            m.dispersion_px = 0.0;
            m.light_intensity = 0.0;
            m.pointer_reactive = false;
            m.scroll_reactive = false;
            m.ambient_mesh = None;
        }

        m
    }
}
```

- [ ] **Step 4: Run, verify pass**

Run: `cargo test -p ui-glass --test liquid_material`
Expected: All tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-glass/src/lib.rs crates/ui-glass/tests/liquid_material.rs
git commit -m "feat(ui-glass): map MaterialRequest to LiquidMaterial"
```

---

## Task 6: Scaffold `ui-glass-engine` crate

**Files:**
- Modify: `Cargo.toml`
- Create: `crates/ui-glass-engine/Cargo.toml`
- Create: `crates/ui-glass-engine/src/lib.rs`

- [ ] **Step 1: Add crate to workspace**

Edit `Cargo.toml`. Append `"crates/ui-glass-engine"` to `[workspace.members]`. Append to `[workspace.dependencies]`:

```toml
ui-glass-engine = { path = "crates/ui-glass-engine" }
wgpu = { version = "26", default-features = false, features = ["wgsl"] }
bytemuck = { version = "1.16", features = ["derive"] }
pollster = "0.4"
image = { version = "0.25", default-features = false, features = ["png"] }
```

- [ ] **Step 2: Create `crates/ui-glass-engine/Cargo.toml`**

```toml
[package]
name = "ui-glass-engine"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
ui-glass.workspace = true
ui-tokens.workspace = true
wgpu.workspace = true
bytemuck.workspace = true

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
wgpu = { workspace = true, features = ["vulkan", "metal", "dx12", "gles", "wgsl"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wgpu = { workspace = true, features = ["webgpu", "webgl", "wgsl"] }

[dev-dependencies]
pollster.workspace = true
image.workspace = true

[lib]
path = "src/lib.rs"
```

- [ ] **Step 3: Create `crates/ui-glass-engine/src/lib.rs`**

```rust
#![forbid(unsafe_code)]

//! Headless wgpu render engine for Liquid Glass surfaces.
//!
//! See `docs/superpowers/specs/2026-05-22-liquid-glass-engine-design.md` for
//! the design that drives this crate. Plan 1 covers the engine scaffold,
//! pipeline cache, and minimal shader (blur + SDF + tint).

pub mod compositor;
pub mod pipeline;
pub mod render_graph;
pub mod uniforms;

#[cfg(feature = "headless")]
pub mod headless;

pub use compositor::{Compositor, GlassRegion};
// `GlassUniforms` is re-exported from Task 7 once the struct exists.
```

- [ ] **Step 4: Add stub modules so the crate compiles**

Create `crates/ui-glass-engine/src/uniforms.rs`:

```rust
// Filled in by Task 7.
```

Create `crates/ui-glass-engine/src/pipeline.rs`:

```rust
// Filled in by Task 8.
```

Create `crates/ui-glass-engine/src/render_graph.rs`:

```rust
// Filled in by Task 11.
```

Create `crates/ui-glass-engine/src/compositor.rs`:

```rust
//! Public render entry point. Filled in across Tasks 9–12.

use ui_glass::LiquidMaterial;

#[derive(Clone, Copy, Debug)]
pub struct GlassRegion {
    pub rect_px: [f32; 4], // x, y, w, h
    pub material: LiquidMaterial,
}

pub struct Compositor;
```

- [ ] **Step 5: Verify crate compiles**

Run: `cargo build -p ui-glass-engine`
Expected: success, possibly warnings about unused modules.

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml crates/ui-glass-engine
git commit -m "feat(ui-glass-engine): scaffold new crate"
```

---

## Task 7: Implement `GlassUniforms` (Pod/Zeroable, std140-compatible layout)

**Files:**
- Modify: `crates/ui-glass-engine/src/lib.rs` (add re-export)
- Modify: `crates/ui-glass-engine/src/uniforms.rs`
- Create: `crates/ui-glass-engine/tests/uniforms_layout.rs`

- [ ] **Step 1: Write failing test**

Create `crates/ui-glass-engine/tests/uniforms_layout.rs`:

```rust
use ui_glass_engine::GlassUniforms;

#[test]
fn uniforms_size_is_multiple_of_16_bytes() {
    let size = std::mem::size_of::<GlassUniforms>();
    assert_eq!(size % 16, 0, "uniform struct must be 16-byte aligned for wgpu");
}

#[test]
fn uniforms_zeroed_construction_compiles() {
    let _u: GlassUniforms = bytemuck::Zeroable::zeroed();
}

#[test]
fn uniforms_default_has_unit_thickness() {
    let u = GlassUniforms::default();
    assert_eq!(u.thickness, 1.0);
}
```

- [ ] **Step 2: Run, expect failure**

Run: `cargo test -p ui-glass-engine --test uniforms_layout`
Expected: FAIL — `GlassUniforms` not defined.

- [ ] **Step 3: Implement `GlassUniforms`**

Replace contents of `crates/ui-glass-engine/src/uniforms.rs`:

```rust
//! GPU-aligned uniform layout for the glass shader. The struct mirrors
//! `compose.wgsl`'s `GlassUniforms` block. Always 16-byte aligned; vec2 fields
//! pad to 8, vec4 to 16. See the WGSL file for the binding contract.

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GlassUniforms {
    pub rect: [f32; 4],          // x, y, w, h in canvas px
    pub tint: [f32; 4],          // rgba

    pub canvas_size: [f32; 2],   // px
    pub pointer: [f32; 2],       // -1..1 normalized to surface

    pub scroll_velocity: [f32; 2],
    pub light_dir: [f32; 2],     // unit vector from light_angle_rad

    pub radius: f32,
    pub thickness: f32,
    pub blur_radius: f32,
    pub saturation: f32,

    pub refract_strength: f32,
    pub surface_curvature: f32,
    pub noise_frequency: f32,
    pub noise_seed: f32,

    pub dispersion_px: f32,
    pub light_intensity: f32,
    pub edge_falloff: f32,
    pub inner_shadow_px: f32,

    pub inner_shadow_alpha: f32,
    pub adapt_strength: f32,
    pub time_seconds: f32,
    pub _pad0: f32,
}

impl Default for GlassUniforms {
    fn default() -> Self {
        Self {
            rect: [0.0; 4],
            tint: [1.0; 4],
            canvas_size: [1.0, 1.0],
            pointer: [0.0; 2],
            scroll_velocity: [0.0; 2],
            light_dir: [1.0, 0.0],
            radius: 0.0,
            thickness: 1.0,
            blur_radius: 0.0,
            saturation: 1.0,
            refract_strength: 0.0,
            surface_curvature: 0.0,
            noise_frequency: 1.0,
            noise_seed: 0.0,
            dispersion_px: 0.0,
            light_intensity: 0.0,
            edge_falloff: 0.0,
            inner_shadow_px: 0.0,
            inner_shadow_alpha: 0.0,
            adapt_strength: 0.0,
            time_seconds: 0.0,
            _pad0: 0.0,
        }
    }
}

impl GlassUniforms {
    pub fn from_material(
        material: &ui_glass::LiquidMaterial,
        rect_px: [f32; 4],
        canvas_size: [f32; 2],
    ) -> Self {
        Self {
            rect: rect_px,
            tint: [
                material.tint.r as f32 / 255.0,
                material.tint.g as f32 / 255.0,
                material.tint.b as f32 / 255.0,
                material.tint_alpha,
            ],
            canvas_size,
            pointer: [0.0; 2],
            scroll_velocity: [0.0; 2],
            light_dir: [
                material.light_angle_rad.cos(),
                material.light_angle_rad.sin(),
            ],
            radius: material.radius_px,
            thickness: material.thickness_px,
            blur_radius: material.blur_radius_px,
            saturation: material.saturation,
            refract_strength: material.refraction_strength,
            surface_curvature: material.surface_curvature,
            noise_frequency: material.noise_frequency,
            noise_seed: material.noise_seed,
            dispersion_px: material.dispersion_px,
            light_intensity: material.light_intensity,
            edge_falloff: material.edge_falloff_px,
            inner_shadow_px: material.inner_shadow_px,
            inner_shadow_alpha: material.inner_shadow_alpha,
            adapt_strength: material.adapt_to_background,
            time_seconds: 0.0,
            _pad0: 0.0,
        }
    }
}
```

- [ ] **Step 4: Add the re-export**

Edit `crates/ui-glass-engine/src/lib.rs`. Replace the `// GlassUniforms` placeholder comment with:

```rust
pub use uniforms::GlassUniforms;
```

- [ ] **Step 5: Run, verify pass**

Run: `cargo test -p ui-glass-engine --test uniforms_layout`
Expected: 3 tests pass.

- [ ] **Step 6: Commit**

```bash
git add crates/ui-glass-engine/src/lib.rs crates/ui-glass-engine/src/uniforms.rs crates/ui-glass-engine/tests/uniforms_layout.rs
git commit -m "feat(ui-glass-engine): add GlassUniforms layout"
```

---

## Task 8: Write WGSL shader files

**Files:**
- Create: `crates/ui-glass-engine/src/shaders/blur.wgsl`
- Create: `crates/ui-glass-engine/src/shaders/compose.wgsl`

These files are validated at pipeline creation time by wgpu/naga; the test landing point is Task 10 (pipeline construction).

- [ ] **Step 1: Create `blur.wgsl`**

Create `crates/ui-glass-engine/src/shaders/blur.wgsl`:

```wgsl
// Separable Gaussian blur. Direction is set via the `BLUR_DIRECTION_X` /
// `BLUR_DIRECTION_Y` override pair: (1.0, 0.0) for horizontal, (0.0, 1.0) for
// vertical. The pipeline cache instantiates each direction once.

override BLUR_DIRECTION_X: f32 = 1.0;
override BLUR_DIRECTION_Y: f32 = 0.0;
override BLUR_TAPS: u32 = 13u;

struct BlurUniforms {
    canvas_size: vec2<f32>,
    blur_radius_px: f32,
    _pad: f32,
};

@group(0) @binding(0) var<uniform> u: BlurUniforms;
@group(0) @binding(1) var src_tex: texture_2d<f32>;
@group(0) @binding(2) var src_samp: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    // Full-screen triangle: covers NDC -1..1 with 3 verts.
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

fn gaussian_weight(i: i32, sigma: f32) -> f32 {
    let x = f32(i);
    return exp(-0.5 * (x * x) / (sigma * sigma));
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let dir = vec2<f32>(BLUR_DIRECTION_X, BLUR_DIRECTION_Y) / u.canvas_size;
    let radius = max(u.blur_radius_px, 0.0);
    let sigma = max(radius * 0.5, 0.5);
    let half_taps = i32(BLUR_TAPS / 2u);

    var acc = vec4<f32>(0.0);
    var weight_sum = 0.0;
    for (var i = -half_taps; i <= half_taps; i = i + 1) {
        let w = gaussian_weight(i, sigma);
        let offset = dir * f32(i) * radius;
        acc = acc + textureSample(src_tex, src_samp, in.uv + offset) * w;
        weight_sum = weight_sum + w;
    }
    return acc / max(weight_sum, 1e-5);
}
```

- [ ] **Step 2: Create `compose.wgsl`**

Create `crates/ui-glass-engine/src/shaders/compose.wgsl`:

```wgsl
// Composite pass. Plan 1 implements: SDF rounded-rect mask, sample blurred
// backdrop, apply tint. Later plans light up REFRACT, DISPERSE, SPECULAR,
// INNER_SHADOW, AMBIENT_MESH, POINTER, SCROLL, TINT_ADAPT via the override
// constants below.

override FEAT_BLUR:         bool = false;
override FEAT_REFRACT:      bool = false;
override FEAT_DISPERSE:     bool = false;
override FEAT_SPECULAR:     bool = false;
override FEAT_INNER_SHADOW: bool = false;
override FEAT_AMBIENT_MESH: bool = false;
override FEAT_POINTER:      bool = false;
override FEAT_SCROLL:       bool = false;
override FEAT_TINT_ADAPT:   bool = false;

struct GlassUniforms {
    rect:               vec4<f32>,
    tint:               vec4<f32>,
    canvas_size:        vec2<f32>,
    pointer:            vec2<f32>,
    scroll_velocity:    vec2<f32>,
    light_dir:          vec2<f32>,
    radius:             f32,
    thickness:          f32,
    blur_radius:        f32,
    saturation:         f32,
    refract_strength:   f32,
    surface_curvature:  f32,
    noise_frequency:    f32,
    noise_seed:         f32,
    dispersion_px:      f32,
    light_intensity:    f32,
    edge_falloff:       f32,
    inner_shadow_px:    f32,
    inner_shadow_alpha: f32,
    adapt_strength:     f32,
    time_seconds:       f32,
    _pad0:              f32,
};

@group(0) @binding(0) var<uniform> u: GlassUniforms;
@group(0) @binding(1) var bg_tex:  texture_2d<f32>;
@group(0) @binding(2) var bg_samp: sampler;

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
    var uv = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );
    var out: VsOut;
    out.pos = vec4<f32>(pos[vid], 0.0, 1.0);
    out.uv  = uv[vid];
    return out;
}

fn rounded_rect_sdf(p: vec2<f32>, half_size: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(r, r);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

fn apply_saturation(c: vec3<f32>, sat: f32) -> vec3<f32> {
    let luma = dot(c, vec3<f32>(0.2126, 0.7152, 0.0722));
    return mix(vec3<f32>(luma), c, sat);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    // Map the fragment's UV onto the surface rect.
    let frag = in.uv * u.canvas_size;
    let local = frag - (u.rect.xy + u.rect.zw * 0.5);
    let sdf = rounded_rect_sdf(local, u.rect.zw * 0.5, u.radius);
    if (sdf > 0.0) { discard; }

    var bg = textureSample(bg_tex, bg_samp, in.uv);

    // Saturation + tint mix
    var color = apply_saturation(bg.rgb, u.saturation);
    color = mix(color, u.tint.rgb, u.tint.a);

    return vec4<f32>(color, 1.0);
}
```

- [ ] **Step 3: No commit yet** — shaders are validated by pipeline construction in Task 10.

---

## Task 9: Headless test harness

**Files:**
- Modify: `crates/ui-glass-engine/Cargo.toml` (add `headless` feature)
- Create: `crates/ui-glass-engine/src/headless.rs`
- Create: `crates/ui-glass-engine/tests/headless_render.rs`

- [ ] **Step 1: Add feature flag**

Edit `crates/ui-glass-engine/Cargo.toml`. Add:

```toml
[features]
default = []
headless = []
```

- [ ] **Step 2: Write failing test**

Create `crates/ui-glass-engine/tests/headless_render.rs`:

```rust
use ui_glass_engine::headless::TestHarness;

#[test]
fn harness_initializes_and_returns_device() {
    let h = pollster::block_on(TestHarness::new()).expect("device init");
    assert!(h.canvas_size().0 > 0);
}

#[test]
fn harness_clear_to_color_returns_solid_pixels() {
    let mut h = pollster::block_on(TestHarness::new()).expect("device init");
    let pixels = h.clear_and_read(64, 64, [0.0, 0.0, 1.0, 1.0]);
    // First pixel should be blue
    assert_eq!(pixels[0], 0);
    assert_eq!(pixels[1], 0);
    assert_eq!(pixels[2], 255);
    assert_eq!(pixels[3], 255);
}
```

- [ ] **Step 3: Run, expect failure**

Run: `cargo test -p ui-glass-engine --features headless --test headless_render`
Expected: FAIL — `TestHarness` unresolved.

- [ ] **Step 4: Implement `TestHarness`**

Create `crates/ui-glass-engine/src/headless.rs` with:

```rust
//! Test-only headless wgpu harness. Picks any available native backend
//! (Vulkan/Metal/DX12/GL) and renders to an offscreen RGBA8 texture, returning
//! the raw bytes for golden-image comparison.

use std::sync::Arc;
use wgpu::util::DeviceExt;

pub struct TestHarness {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    canvas_size: (u32, u32),
}

impl TestHarness {
    pub async fn new() -> Result<Self, String> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY | wgpu::Backends::GL,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| format!("no adapter: {e:?}"))?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("ui-glass-engine-test"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(|e| format!("no device: {e:?}"))?;

        Ok(Self {
            instance,
            adapter,
            device: Arc::new(device),
            queue: Arc::new(queue),
            canvas_size: (256, 256),
        })
    }

    pub fn device(&self) -> &Arc<wgpu::Device> { &self.device }
    pub fn queue(&self) -> &Arc<wgpu::Queue> { &self.queue }
    pub fn canvas_size(&self) -> (u32, u32) { self.canvas_size }

    /// Allocate an RGBA8 render target of the given size, clear it to `color`,
    /// then read back the pixels (row-major, top-down, premultiplied).
    pub fn clear_and_read(&mut self, w: u32, h: u32, color: [f64; 4]) -> Vec<u8> {
        self.canvas_size = (w, h);
        let target = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("test-target"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = target.create_view(&wgpu::TextureViewDescriptor::default());

        let bytes_per_row = align_up(w * 4, wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);
        let readback = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("readback"),
            size: (bytes_per_row * h) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("clear"),
        });
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: color[0], g: color[1], b: color[2], a: color[3],
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &target, mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(h),
                },
            },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        self.queue.submit(Some(encoder.finish()));

        let slice = readback.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| { tx.send(r).unwrap(); });
        let _ = self.device.poll(wgpu::PollType::Wait);
        rx.recv().unwrap().unwrap();
        let data = slice.get_mapped_range();

        let mut out = Vec::with_capacity((w * h * 4) as usize);
        for row in 0..h {
            let start = (row * bytes_per_row) as usize;
            out.extend_from_slice(&data[start..start + (w * 4) as usize]);
        }
        drop(data);
        readback.unmap();
        out
    }
}

fn align_up(n: u32, align: u32) -> u32 {
    ((n + align - 1) / align) * align
}
```

- [ ] **Step 5: Run, verify pass**

Run: `cargo test -p ui-glass-engine --features headless --test headless_render`
Expected: 2 tests pass. If the test runner has no GPU at all, both will fail with `"no adapter"` — install a software backend (e.g., Mesa Lavapipe on Linux) or skip on CI.

- [ ] **Step 6: Commit**

```bash
git add crates/ui-glass-engine/Cargo.toml crates/ui-glass-engine/src/lib.rs crates/ui-glass-engine/src/headless.rs crates/ui-glass-engine/tests/headless_render.rs
git commit -m "feat(ui-glass-engine): headless test harness"
```

---

## Task 10: Blur pipeline construction

**Files:**
- Modify: `crates/ui-glass-engine/src/pipeline.rs`
- Create: `crates/ui-glass-engine/tests/pipeline_compile.rs`

- [ ] **Step 1: Write failing test**

Create `crates/ui-glass-engine/tests/pipeline_compile.rs`:

```rust
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::pipeline::{build_blur_pipeline, BlurDirection};

#[test]
fn blur_horizontal_pipeline_compiles() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let _p = build_blur_pipeline(h.device(), BlurDirection::Horizontal, 13);
}

#[test]
fn blur_vertical_pipeline_compiles() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let _p = build_blur_pipeline(h.device(), BlurDirection::Vertical, 13);
}

#[test]
fn blur_pipeline_compiles_for_smaller_tap_count() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let _p = build_blur_pipeline(h.device(), BlurDirection::Horizontal, 5);
}
```

- [ ] **Step 2: Run, expect failure**

Run: `cargo test -p ui-glass-engine --features headless --test pipeline_compile`
Expected: FAIL — `build_blur_pipeline` not defined.

- [ ] **Step 3: Implement `build_blur_pipeline`**

Replace `crates/ui-glass-engine/src/pipeline.rs` with:

```rust
//! wgpu pipeline construction for the blur passes and composite pass.

use std::sync::Arc;

const BLUR_SRC: &str = include_str!("shaders/blur.wgsl");
const COMPOSE_SRC: &str = include_str!("shaders/compose.wgsl");

#[derive(Clone, Copy, Debug)]
pub enum BlurDirection {
    Horizontal,
    Vertical,
}

pub fn build_blur_pipeline(
    device: &Arc<wgpu::Device>,
    direction: BlurDirection,
    taps: u32,
) -> wgpu::RenderPipeline {
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("blur.wgsl"),
        source: wgpu::ShaderSource::Wgsl(BLUR_SRC.into()),
    });

    let (dx, dy) = match direction {
        BlurDirection::Horizontal => (1.0, 0.0),
        BlurDirection::Vertical => (0.0, 1.0),
    };

    let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("blur-bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("blur-layout"),
        bind_group_layouts: &[&bind_layout],
        push_constant_ranges: &[],
    });

    let mut constants = std::collections::HashMap::new();
    constants.insert("BLUR_DIRECTION_X".to_string(), dx as f64);
    constants.insert("BLUR_DIRECTION_Y".to_string(), dy as f64);
    constants.insert("BLUR_TAPS".to_string(), taps as f64);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("blur-pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &module,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                zero_initialize_workgroup_memory: false,
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                zero_initialize_workgroup_memory: false,
            },
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}

pub fn blur_bind_group_layout(device: &Arc<wgpu::Device>) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("blur-bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}
```

- [ ] **Step 4: Run, verify pass**

Run: `cargo test -p ui-glass-engine --features headless --test pipeline_compile`
Expected: 3 tests pass. If `blur.wgsl` fails naga validation, the test will panic with the error — fix the shader and re-run.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-glass-engine/src/pipeline.rs crates/ui-glass-engine/tests/pipeline_compile.rs
git commit -m "feat(ui-glass-engine): blur pipeline construction"
```

---

## Task 11: Composite pipeline construction + feature cache key

**Files:**
- Modify: `crates/ui-glass-engine/src/pipeline.rs`
- Modify: `crates/ui-glass-engine/tests/pipeline_compile.rs`

- [ ] **Step 1: Write failing tests**

Append to `crates/ui-glass-engine/tests/pipeline_compile.rs`:

```rust
use ui_glass::GlassFeatures;
use ui_glass_engine::pipeline::{build_compose_pipeline, ComposeKey};

#[test]
fn compose_pipeline_compiles_with_blur_only() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let key = ComposeKey { features: GlassFeatures::BLUR };
    let _p = build_compose_pipeline(h.device(), key);
}

#[test]
fn compose_pipeline_compiles_with_all_features_off() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let key = ComposeKey { features: GlassFeatures::empty() };
    let _p = build_compose_pipeline(h.device(), key);
}

#[test]
fn compose_key_is_hashable_for_cache() {
    let a = ComposeKey { features: GlassFeatures::BLUR };
    let b = ComposeKey { features: GlassFeatures::BLUR };
    use std::hash::{Hash, Hasher};
    let mut h1 = std::collections::hash_map::DefaultHasher::new();
    let mut h2 = std::collections::hash_map::DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}
```

- [ ] **Step 2: Run, expect failure**

Run: `cargo test -p ui-glass-engine --features headless --test pipeline_compile`
Expected: FAIL — `build_compose_pipeline` / `ComposeKey` missing.

- [ ] **Step 3: Implement `ComposeKey` + `build_compose_pipeline`**

Append to `crates/ui-glass-engine/src/pipeline.rs`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComposeKey {
    pub features: ui_glass::GlassFeatures,
}

pub fn compose_bind_group_layout(device: &Arc<wgpu::Device>) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("compose-bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}

pub fn build_compose_pipeline(
    device: &Arc<wgpu::Device>,
    key: ComposeKey,
) -> wgpu::RenderPipeline {
    use ui_glass::GlassFeatures as F;
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("compose.wgsl"),
        source: wgpu::ShaderSource::Wgsl(COMPOSE_SRC.into()),
    });

    let bgl = compose_bind_group_layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("compose-layout"),
        bind_group_layouts: &[&bgl],
        push_constant_ranges: &[],
    });

    let mut constants = std::collections::HashMap::new();
    let f = key.features;
    constants.insert("FEAT_BLUR".to_string(),         if f.contains(F::BLUR)         { 1.0 } else { 0.0 });
    constants.insert("FEAT_REFRACT".to_string(),      if f.contains(F::REFRACT)      { 1.0 } else { 0.0 });
    constants.insert("FEAT_DISPERSE".to_string(),     if f.contains(F::DISPERSE)     { 1.0 } else { 0.0 });
    constants.insert("FEAT_SPECULAR".to_string(),     if f.contains(F::SPECULAR)     { 1.0 } else { 0.0 });
    constants.insert("FEAT_INNER_SHADOW".to_string(), if f.contains(F::INNER_SHADOW) { 1.0 } else { 0.0 });
    constants.insert("FEAT_AMBIENT_MESH".to_string(), if f.contains(F::AMBIENT_MESH) { 1.0 } else { 0.0 });
    constants.insert("FEAT_POINTER".to_string(),      if f.contains(F::POINTER)      { 1.0 } else { 0.0 });
    constants.insert("FEAT_SCROLL".to_string(),       if f.contains(F::SCROLL)       { 1.0 } else { 0.0 });
    constants.insert("FEAT_TINT_ADAPT".to_string(),   if f.contains(F::TINT_ADAPT)   { 1.0 } else { 0.0 });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("compose-pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &module,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                zero_initialize_workgroup_memory: false,
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                zero_initialize_workgroup_memory: false,
            },
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}
```

- [ ] **Step 4: Run, verify pass**

Run: `cargo test -p ui-glass-engine --features headless --test pipeline_compile`
Expected: All pipeline_compile tests pass (6 total).

- [ ] **Step 5: Commit**

```bash
git add crates/ui-glass-engine/src/pipeline.rs crates/ui-glass-engine/tests/pipeline_compile.rs
git commit -m "feat(ui-glass-engine): composite pipeline construction"
```

---

## Task 12: Render graph — H blur → V blur → compose

**Files:**
- Modify: `crates/ui-glass-engine/src/render_graph.rs`
- Modify: `crates/ui-glass-engine/src/lib.rs`

- [ ] **Step 1: Replace `render_graph.rs`**

Replace `crates/ui-glass-engine/src/render_graph.rs` with:

```rust
//! Orders the render passes: bg → blur H → blur V → composite into the
//! output target.

use std::sync::Arc;
use wgpu::util::DeviceExt;

use crate::pipeline::{
    blur_bind_group_layout, build_blur_pipeline, build_compose_pipeline, compose_bind_group_layout,
    BlurDirection, ComposeKey,
};
use crate::uniforms::GlassUniforms;

/// One end-to-end pass: input bg texture → output RGBA8 texture, with the
/// material's blur radius applied via two separable passes and the composite
/// shader sampling the blurred result.
pub fn render_glass_to_texture(
    device: &Arc<wgpu::Device>,
    queue: &Arc<wgpu::Queue>,
    bg_view: &wgpu::TextureView,
    output_view: &wgpu::TextureView,
    uniforms: &GlassUniforms,
    compose_key: ComposeKey,
) {
    let (w, h) = (uniforms.canvas_size[0] as u32, uniforms.canvas_size[1] as u32);

    // Two scratch textures for separable blur.
    let make_scratch = |label: &str| {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
    };
    let scratch_h = make_scratch("blur-h");
    let scratch_v = make_scratch("blur-v");
    let scratch_h_view = scratch_h.create_view(&wgpu::TextureViewDescriptor::default());
    let scratch_v_view = scratch_v.create_view(&wgpu::TextureViewDescriptor::default());

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("linear-clamp"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    // Blur uniform buffer
    #[repr(C)]
    #[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
    struct BlurUniforms {
        canvas_size: [f32; 2],
        blur_radius_px: f32,
        _pad: f32,
    }
    let blur_u = BlurUniforms {
        canvas_size: uniforms.canvas_size,
        blur_radius_px: uniforms.blur_radius,
        _pad: 0.0,
    };
    let blur_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("blur-uniforms"),
        contents: bytemuck::bytes_of(&blur_u),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let blur_bgl = blur_bind_group_layout(device);
    let bg_h = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("blur-h-bg"),
        layout: &blur_bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: blur_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(bg_view) },
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
        ],
    });
    let bg_v = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("blur-v-bg"),
        layout: &blur_bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: blur_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&scratch_h_view) },
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
        ],
    });

    let blur_h_pipeline = build_blur_pipeline(device, BlurDirection::Horizontal, 13);
    let blur_v_pipeline = build_blur_pipeline(device, BlurDirection::Vertical, 13);

    // Compose uniform + bind
    let compose_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("compose-uniforms"),
        contents: bytemuck::bytes_of(uniforms),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let compose_bgl = compose_bind_group_layout(device);
    let compose_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("compose-bg"),
        layout: &compose_bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: compose_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&scratch_v_view) },
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
        ],
    });

    let compose_pipeline = build_compose_pipeline(device, compose_key);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("glass-frame"),
    });

    fn run_pass(
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        pipeline: &wgpu::RenderPipeline,
        bind: &wgpu::BindGroup,
        label: &str,
        clear: bool,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(label),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if clear {
                        wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
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
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, bind, &[]);
        pass.draw(0..3, 0..1);
    }

    run_pass(&mut encoder, &scratch_h_view, &blur_h_pipeline, &bg_h, "blur-h", true);
    run_pass(&mut encoder, &scratch_v_view, &blur_v_pipeline, &bg_v, "blur-v", true);
    run_pass(&mut encoder, output_view, &compose_pipeline, &compose_bg, "compose", true);

    queue.submit(Some(encoder.finish()));
}
```

- [ ] **Step 2: Update `lib.rs` to re-export the function**

Append to `crates/ui-glass-engine/src/lib.rs`:

```rust
pub use render_graph::render_glass_to_texture;
```

- [ ] **Step 3: Build to confirm compilation**

Run: `cargo build -p ui-glass-engine --features headless`
Expected: success.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-glass-engine/src/render_graph.rs crates/ui-glass-engine/src/lib.rs
git commit -m "feat(ui-glass-engine): render graph with separable blur and composite"
```

---

## Task 13: Compositor public API

**Files:**
- Modify: `crates/ui-glass-engine/src/compositor.rs`
- Create: `crates/ui-glass-engine/tests/compositor_api.rs`

- [ ] **Step 1: Write failing test**

Create `crates/ui-glass-engine/tests/compositor_api.rs`:

```rust
use ui_glass::LiquidMaterial;
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::{Compositor, GlassRegion};

#[test]
fn compositor_renders_single_region_without_panic() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());

    let bg = make_solid_bg(h.device(), h.queue(), 128, 128, [0, 0, 128, 255]);
    let out = make_output(h.device(), 128, 128);

    let region = GlassRegion {
        rect_px: [16.0, 16.0, 96.0, 96.0],
        material: LiquidMaterial::floating().blur(8.0).radius(12.0),
    };

    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [128.0, 128.0],
        &[region],
    );
}

fn make_solid_bg(
    device: &std::sync::Arc<wgpu::Device>,
    queue: &std::sync::Arc<wgpu::Queue>,
    w: u32, h: u32, rgba: [u8; 4],
) -> wgpu::Texture {
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
        wgpu::TexelCopyTextureInfo {
            texture: &t, mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h),
        },
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

- [ ] **Step 2: Run, expect failure**

Run: `cargo test -p ui-glass-engine --features headless --test compositor_api`
Expected: FAIL — `Compositor::new` / `render` not defined.

- [ ] **Step 3: Implement `Compositor`**

Replace `crates/ui-glass-engine/src/compositor.rs` with:

```rust
//! Public render entry point. Holds device/queue and exposes a single
//! `render()` call that does an end-to-end frame. Plan 1 creates pipelines
//! per render call; Plan 2 introduces the pipeline cache keyed by
//! `(GlassFeatures, BLUR_TAPS)`.

use std::sync::Arc;

use ui_glass::LiquidMaterial;

use crate::pipeline::ComposeKey;
use crate::render_graph::render_glass_to_texture;
use crate::uniforms::GlassUniforms;

#[derive(Clone, Copy, Debug)]
pub struct GlassRegion {
    pub rect_px: [f32; 4], // x, y, w, h
    pub material: LiquidMaterial,
}

pub struct Compositor {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

impl Compositor {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self { device, queue }
    }

    /// End-to-end render: bg → blur → compose → output.
    /// Plan 1 supports any number of regions; each is rendered in order with a
    /// fresh pipeline (no cache yet). Multi-region overlap with correct
    /// compositing lands in Plan 4 (background scene contract).
    pub fn render(
        &mut self,
        bg_view: &wgpu::TextureView,
        output_view: &wgpu::TextureView,
        canvas_size: [f32; 2],
        regions: &[GlassRegion],
    ) {
        for region in regions {
            let uniforms = GlassUniforms::from_material(
                &region.material,
                region.rect_px,
                canvas_size,
            );
            let key = ComposeKey { features: region.material.features };
            render_glass_to_texture(
                &self.device,
                &self.queue,
                bg_view,
                output_view,
                &uniforms,
                key,
            );
        }
    }
}
```

- [ ] **Step 4: Run, verify pass**

Run: `cargo test -p ui-glass-engine --features headless --test compositor_api`
Expected: 1 test passes.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-glass-engine/src/compositor.rs crates/ui-glass-engine/tests/compositor_api.rs
git commit -m "feat(ui-glass-engine): public Compositor::render entry point"
```

---

## Task 14: End-to-end smoke test — solid background through floating preset

**Files:**
- Create: `crates/ui-glass-engine/tests/end_to_end.rs`

- [ ] **Step 1: Write the test**

Create `crates/ui-glass-engine/tests/end_to_end.rs`:

```rust
use ui_glass::LiquidMaterial;
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::{Compositor, GlassRegion};

#[test]
fn floating_preset_over_blue_bg_writes_corner_pixels_outside_rect() {
    let mut h = pollster::block_on(TestHarness::new()).unwrap();
    let (w, hgt) = (128u32, 128u32);

    let bg = create_solid(h.device(), h.queue(), w, hgt, [0, 0, 200, 255]);
    let out = create_output(h.device(), w, hgt);

    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());
    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [w as f32, hgt as f32],
        &[GlassRegion {
            rect_px: [32.0, 32.0, 64.0, 64.0],
            material: LiquidMaterial::floating().tint(
                ui_tokens::Color::rgba(255, 255, 255, 1.0),
                0.4,
            ),
        }],
    );

    let pixels = read_back(h.device(), h.queue(), &out, w, hgt);

    // Pixel inside the rect (center of canvas) should be a tinted blue —
    // the blue background mixed with white tint at 40%.
    let center_idx = ((hgt / 2) * w + (w / 2)) as usize * 4;
    let r = pixels[center_idx];
    let g = pixels[center_idx + 1];
    let b = pixels[center_idx + 2];
    assert!(r > 90, "expected white-mixed red channel above 90, got {r}");
    assert!(g > 90, "expected white-mixed green channel above 90, got {g}");
    assert!(b < 255, "expected mixed blue, got pure white at center");

    // Pixel far outside the rect (top-left corner) should remain transparent
    // (compositor clears to TRANSPARENT before compose pass).
    let corner_idx = 0;
    assert_eq!(pixels[corner_idx + 3], 0, "corner alpha should be 0");
}

fn create_solid(
    device: &std::sync::Arc<wgpu::Device>,
    queue: &std::sync::Arc<wgpu::Queue>,
    w: u32, h: u32, rgba: [u8; 4],
) -> wgpu::Texture {
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
        wgpu::TexelCopyTextureInfo {
            texture: &t, mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h),
        },
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );
    t
}

fn create_output(device: &std::sync::Arc<wgpu::Device>, w: u32, h: u32) -> wgpu::Texture {
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

fn read_back(
    device: &std::sync::Arc<wgpu::Device>,
    queue: &std::sync::Arc<wgpu::Queue>,
    tex: &wgpu::Texture, w: u32, h: u32,
) -> Vec<u8> {
    let bpr = ((w * 4 + 255) / 256) * 256;
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size: (bpr * h) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc = device.create_command_encoder(&Default::default());
    enc.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: tex, mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
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

- [ ] **Step 2: Run the test**

Run: `cargo test -p ui-glass-engine --features headless --test end_to_end`
Expected: PASS. If it fails, the error message identifies which assertion broke — typically either the SDF mask is offset wrong, the blur radius is too large for the canvas, or the tint blend is inverted.

- [ ] **Step 3: Commit**

```bash
git add crates/ui-glass-engine/tests/end_to_end.rs
git commit -m "test(ui-glass-engine): end-to-end smoke test for floating preset"
```

---

## Task 15: Golden PNG comparison test

**Files:**
- Create: `crates/ui-glass-engine/tests/golden_floating.rs`
- Create: `crates/ui-glass-engine/tests/assets/.gitkeep`

- [ ] **Step 1: Create the gitkeep**

Create empty file `crates/ui-glass-engine/tests/assets/.gitkeep`.

- [ ] **Step 2: Write the test**

Create `crates/ui-glass-engine/tests/golden_floating.rs`:

```rust
//! Golden-image comparison. The first run writes the golden file (when env var
//! `UPDATE_GOLDEN=1` is set); subsequent runs compare against it within a
//! per-pixel tolerance.

use std::path::PathBuf;

use ui_glass::LiquidMaterial;
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::{Compositor, GlassRegion};

const GOLDEN: &str = "tests/assets/floating_neutral_128.png";
const TOLERANCE: u8 = 4;

#[test]
fn floating_neutral_matches_golden() {
    let pixels = render_test_scene();

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN);
    if std::env::var("UPDATE_GOLDEN").is_ok() || !path.exists() {
        write_png(&path, &pixels, 128, 128);
        if !std::env::var("UPDATE_GOLDEN").is_ok() {
            panic!(
                "golden missing at {}; wrote new reference. Re-run to verify.",
                path.display()
            );
        }
        return;
    }

    let expected = read_png(&path);
    assert_eq!(expected.len(), pixels.len(), "size mismatch");

    let mut diffs = 0usize;
    let mut worst = 0u8;
    for (a, b) in expected.iter().zip(pixels.iter()) {
        let d = a.abs_diff(*b);
        if d > TOLERANCE { diffs += 1; }
        if d > worst { worst = d; }
    }

    // Allow up to 0.5% of subpixels to exceed tolerance (compiler/driver jitter).
    let max_allowed = pixels.len() / 200;
    assert!(
        diffs <= max_allowed,
        "{diffs} subpixels exceeded tolerance {TOLERANCE} (max worst diff: {worst}); \
         set UPDATE_GOLDEN=1 to refresh",
    );
}

fn render_test_scene() -> Vec<u8> {
    let mut h = pollster::block_on(TestHarness::new()).unwrap();
    let (w, hgt) = (128u32, 128u32);
    let bg = create_gradient(h.device(), h.queue(), w, hgt);
    let out = create_output(h.device(), w, hgt);

    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());
    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [w as f32, hgt as f32],
        &[GlassRegion {
            rect_px: [24.0, 24.0, 80.0, 80.0],
            material: LiquidMaterial::floating().tint(
                ui_tokens::Color::rgba(255, 255, 255, 1.0),
                0.35,
            ),
        }],
    );

    read_back(h.device(), h.queue(), &out, w, hgt)
}

fn create_gradient(
    device: &std::sync::Arc<wgpu::Device>,
    queue: &std::sync::Arc<wgpu::Queue>,
    w: u32, h: u32,
) -> wgpu::Texture {
    let t = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("gradient-bg"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let mut px = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            let r = (x * 255 / w.max(1)) as u8;
            let g = ((x + y) * 255 / (w + h).max(1)) as u8;
            let b = (y * 255 / h.max(1)) as u8;
            px.extend_from_slice(&[r, g, b, 255]);
        }
    }
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &t, mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &px,
        wgpu::TexelCopyBufferLayout {
            offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h),
        },
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );
    t
}

fn create_output(device: &std::sync::Arc<wgpu::Device>, w: u32, h: u32) -> wgpu::Texture {
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

fn read_back(
    device: &std::sync::Arc<wgpu::Device>,
    queue: &std::sync::Arc<wgpu::Queue>,
    tex: &wgpu::Texture, w: u32, h: u32,
) -> Vec<u8> {
    let bpr = ((w * 4 + 255) / 256) * 256;
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size: (bpr * h) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc = device.create_command_encoder(&Default::default());
    enc.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: tex, mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
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

fn write_png(path: &std::path::Path, pixels: &[u8], w: u32, h: u32) {
    let img = image::RgbaImage::from_raw(w, h, pixels.to_vec())
        .expect("pixel buffer size mismatch");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    img.save(path).expect("png write");
}

fn read_png(path: &std::path::Path) -> Vec<u8> {
    let img = image::open(path).expect("png open").to_rgba8();
    img.into_raw()
}
```

- [ ] **Step 3: Generate the golden**

Run: `UPDATE_GOLDEN=1 cargo test -p ui-glass-engine --features headless --test golden_floating`
Expected: test passes after writing `tests/assets/floating_neutral_128.png`.

- [ ] **Step 4: Run again without the env var**

Run: `cargo test -p ui-glass-engine --features headless --test golden_floating`
Expected: PASS — the rendered output matches the golden within tolerance.

- [ ] **Step 5: Commit the golden + test**

```bash
git add crates/ui-glass-engine/tests/golden_floating.rs crates/ui-glass-engine/tests/assets/
git commit -m "test(ui-glass-engine): golden PNG comparison for floating preset"
```

---

## Task 16: Workspace-level integration check

**Files:**
- (no new files; verification only)

- [ ] **Step 1: Build the whole workspace**

Run: `cargo build --workspace`
Expected: success across all crates, including the existing component-gallery.

- [ ] **Step 2: Run the full test suite**

Run: `cargo test --workspace --features ui-glass-engine/headless`
Expected: all existing tests pass plus the new `ui-glass` and `ui-glass-engine` tests.

- [ ] **Step 3: Document the next plan handoff**

Edit `docs/superpowers/plans/2026-05-22-liquid-glass-plan-1-engine-scaffold.md` and append a short status line at the very bottom:

```markdown
---

## Status

Plan 1 complete. Next: Plan 2 — Full Tier 1 shader (refraction, dispersion,
specular, inner shadow, ambient mesh, tint adapt).
```

- [ ] **Step 4: Commit**

```bash
git add docs/superpowers/plans/2026-05-22-liquid-glass-plan-1-engine-scaffold.md
git commit -m "docs: mark Plan 1 status and next plan handoff"
```

---

## Plan 1 — Done. What's next

The engine compiles, renders, and produces a golden-matched PNG for the
floating preset against a gradient background. The pipeline cache, render
graph, and feature-toggle plumbing are wired in but Plan 1 only exercises
`BLUR`. Plan 2 will:

- Add the noise texture and bake the refraction sampling path.
- Light up `DISPERSE`, `SPECULAR`, `INNER_SHADOW`, `AMBIENT_MESH`, `TINT_ADAPT`
  in `compose.wgsl` with golden tests per feature.
- Extend `LiquidMaterial::from(MaterialRequest)` regression tests to cover the
  new visual paths.
