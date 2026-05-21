# Native Kinetics Systems Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace bridge-oriented animation/export boundaries with native Rust and Dioxus timeline, composition, capture, glass, and semantic naming foundations.

**Architecture:** Keep one downstream facade in `unified_ui`, with focused internal crates for native timeline, frame composition, capture metadata, motion math, material recipes, and Dioxus rendering. The first implementation lands MVP contracts and SSR-safe examples, not pixel/video export.

**Tech Stack:** Rust 2021, Cargo workspace, Dioxus 0.7, Dioxus SSR tests, pure Rust unit tests, static CSS strings, PowerShell commands on Windows.

---

## Scope

This plan implements the native systems foundation described in `docs/superpowers/specs/2026-05-21-native-kinetics-systems-design.md`.

It includes:

- crate and feature rename from bridge terms to native terms
- `ui-timeline` native timeline MVP
- `ui-composition` frame composition MVP
- `ui-capture` capture manifest and frame seek MVP
- expanded material API in `ui-glass`
- semantic component names in `ui-dioxus` and `unified_ui`
- gallery categories and examples for native systems
- documentation updates

It excludes:

- PNG output
- MP4 output
- audio rendering
- arbitrary third-party DOM capture
- global visual authoring tools

Those outputs must build on the native contracts created here.

## File Map

- `Cargo.toml`: rename workspace members and dependencies.
- `crates/ui-timeline/`: replaces `crates/ui-gsap`; owns native timeline model and tests.
- `crates/ui-composition/`: new crate for frame-based scene composition.
- `crates/ui-capture/`: replaces `crates/ui-hyperframes`; owns capture stage descriptors, viewport profiles, marks, manifests, and validation.
- `crates/ui-motion/src/lib.rs`: add deterministic interpolation helpers used by timeline and composition.
- `crates/ui-glass/src/lib.rs`: add material-oriented names and extra axes while preserving existing glass behavior.
- `crates/ui-dom/src/lib.rs`: serialize expanded material CSS variables.
- `crates/ui-dioxus/src/`: add semantic name wrappers and native system preview components.
- `crates/ui-styles/src/lib.rs`: add native material, timeline, composition, and capture selectors.
- `crates/unified_ui/Cargo.toml`: replace `gsap` and `hyperframes-export` features with `timeline`, `composition`, and `capture`.
- `crates/unified_ui/src/lib.rs`: export native systems through the public prelude.
- `examples/component-gallery/src/docs.rs`: update registry categories, names, snippets, and native examples.
- `examples/component-gallery/src/app.rs`: keep registry rendering, with category list updated by `docs.rs`.
- `examples/component-gallery/src/styles.rs`: add preview styles for timeline, composition, and capture examples.
- `README.md`, `docs/component-naming.md`, `docs/platform-support.md`, `docs/glass-materials.md`: remove bridge language and document native systems.

## Task 1: Rename Bridge Crates To Native Boundaries

**Files:**
- Modify: `Cargo.toml`
- Modify: `crates/unified_ui/Cargo.toml`
- Modify: `crates/unified_ui/src/lib.rs`
- Move: `crates/ui-gsap` to `crates/ui-timeline`
- Move: `crates/ui-hyperframes` to `crates/ui-capture`
- Create: `crates/ui-composition/Cargo.toml`
- Create: `crates/ui-composition/src/lib.rs`
- Create: `crates/ui-composition/tests/composition.rs`
- Modify: `crates/unified_ui/tests/prelude.rs`

- [ ] **Step 1: Write failing facade test for native boundary names**

Add this test to `crates/unified_ui/tests/prelude.rs`:

```rust
#[test]
fn public_api_names_use_native_system_boundaries() {
    let names = unified_ui::public_api_names();

    for expected in ["Timeline", "Composition", "CaptureStage"] {
        assert!(names.contains(&expected), "missing native system name {expected}");
    }

    for rejected in ["Gsap", "GSAP", "HyperFrames", "Remotion"] {
        assert!(
            !names.iter().any(|name| name.contains(rejected)),
            "public names must not expose bridge term {rejected}"
        );
    }
}
```

- [ ] **Step 2: Run the test and verify it fails**

Run:

```powershell
cargo test -p unified_ui public_api_names_use_native_system_boundaries -- --exact
```

Expected: FAIL because `Timeline`, `Composition`, and `CaptureStage` are not in `public_api_names()`.

- [ ] **Step 3: Rename crate folders**

Run:

```powershell
git mv crates/ui-gsap crates/ui-timeline
git mv crates/ui-hyperframes crates/ui-capture
New-Item -ItemType Directory -Force crates/ui-composition/src, crates/ui-composition/tests
```

- [ ] **Step 4: Update the workspace manifest**

In root `Cargo.toml`, replace the old members and dependencies with:

```toml
[workspace]
resolver = "2"
members = [
    "crates/ui-core",
    "crates/ui-tokens",
    "crates/ui-glass",
    "crates/ui-motion",
    "crates/ui-layout",
    "crates/ui-dom",
    "crates/ui-native",
    "crates/ui-dioxus",
    "crates/ui-timeline",
    "crates/ui-composition",
    "crates/ui-capture",
    "crates/ui-styles",
    "crates/unified_ui",
    "examples/component-gallery",
]

[workspace.package]
edition = "2021"
license = "MIT OR Apache-2.0"
version = "0.1.0"
publish = false

[workspace.dependencies]
dioxus = "0.7"
dioxus-ssr = "0.7"
ui-core = { path = "crates/ui-core" }
ui-tokens = { path = "crates/ui-tokens" }
ui-glass = { path = "crates/ui-glass" }
ui-motion = { path = "crates/ui-motion" }
ui-layout = { path = "crates/ui-layout" }
ui-dom = { path = "crates/ui-dom" }
ui-native = { path = "crates/ui-native" }
ui-dioxus = { path = "crates/ui-dioxus" }
ui-timeline = { path = "crates/ui-timeline" }
ui-composition = { path = "crates/ui-composition" }
ui-capture = { path = "crates/ui-capture" }
ui-styles = { path = "crates/ui-styles" }
unified_ui = { path = "crates/unified_ui" }
```

- [ ] **Step 5: Update renamed crate manifests**

Replace `crates/ui-timeline/Cargo.toml` with:

```toml
[package]
name = "ui-timeline"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
ui-motion.workspace = true
ui-layout.workspace = true

[lib]
path = "src/lib.rs"
```

Replace `crates/ui-capture/Cargo.toml` with:

```toml
[package]
name = "ui-capture"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
ui-composition.workspace = true
ui-tokens.workspace = true

[lib]
path = "src/lib.rs"
```

Create `crates/ui-composition/Cargo.toml`:

```toml
[package]
name = "ui-composition"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
ui-motion.workspace = true
ui-glass.workspace = true

[lib]
path = "src/lib.rs"
```

- [ ] **Step 6: Replace renamed crate code**

Replace `crates/ui-timeline/src/lib.rs` with:

```rust
#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimelineCapability {
    Tracks,
    Labels,
    Stagger,
    SharedMove,
    ScrollProgress,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TimelineRuntime {
    _private: (),
}

impl TimelineRuntime {
    pub const fn target(&self) -> &'static str {
        "native"
    }

    pub const fn capabilities(&self) -> &'static [TimelineCapability] {
        &[
            TimelineCapability::Tracks,
            TimelineCapability::Labels,
            TimelineCapability::Stagger,
            TimelineCapability::SharedMove,
            TimelineCapability::ScrollProgress,
        ]
    }
}
```

Replace `crates/ui-capture/src/lib.rs` with:

```rust
#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaptureStageDescriptor {
    pub id: String,
}

impl CaptureStageDescriptor {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
```

Create `crates/ui-composition/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Composition {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub frame_count: u32,
}

impl Composition {
    pub fn new(
        id: impl Into<String>,
        width: u32,
        height: u32,
        fps: u32,
        frame_count: u32,
    ) -> Self {
        Self {
            id: id.into(),
            width,
            height,
            fps,
            frame_count,
        }
    }
}
```

- [ ] **Step 7: Update unified facade features and dependencies**

Replace `crates/unified_ui/Cargo.toml` with:

```toml
[package]
name = "unified_ui"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[features]
default = ["web", "desktop", "mobile", "tokens", "glass", "motion", "layout-motion", "a11y", "timeline", "composition", "capture"]
web = ["dep:ui-dom"]
desktop = ["dep:ui-dom"]
mobile = ["dep:ui-dom"]
native = ["dep:ui-native"]
tokens = []
glass = []
motion = []
layout-motion = []
a11y = []
a11y-tests = []
timeline = ["dep:ui-timeline"]
composition = ["dep:ui-composition"]
capture = ["dep:ui-capture"]

[dependencies]
ui-core.workspace = true
ui-tokens.workspace = true
ui-glass.workspace = true
motion-core = { package = "ui-motion", path = "../ui-motion" }
ui-layout.workspace = true
ui-dioxus.workspace = true
ui-styles.workspace = true
ui-dom = { workspace = true, optional = true }
ui-native = { workspace = true, optional = true }
ui-timeline = { workspace = true, optional = true }
ui-composition = { workspace = true, optional = true }
ui-capture = { workspace = true, optional = true }

[lib]
path = "src/lib.rs"
```

- [ ] **Step 8: Update the unified facade exports**

In `crates/unified_ui/src/lib.rs`, remove the `gsap` and `hyperframes` modules and add:

```rust
#[cfg(feature = "timeline")]
pub mod timeline {
    pub use ui_timeline::{TimelineCapability, TimelineRuntime};
}

#[cfg(feature = "composition")]
pub mod composition {
    pub use ui_composition::Composition;
}

#[cfg(feature = "capture")]
pub mod capture {
    pub use ui_capture::CaptureStageDescriptor;
}
```

Update `prelude` with:

```rust
#[cfg(feature = "timeline")]
pub use ui_timeline::{TimelineCapability, TimelineRuntime};

#[cfg(feature = "composition")]
pub use ui_composition::Composition;

#[cfg(feature = "capture")]
pub use ui_capture::CaptureStageDescriptor;
```

Update `public_api_names()` so the returned slice contains these native names and no bridge names:

```rust
"Timeline",
"TimelineScope",
"Composition",
"FrameStage",
"CaptureStage",
```

- [ ] **Step 9: Run the facade test and verify it passes**

Run:

```powershell
cargo test -p unified_ui public_api_names_use_native_system_boundaries -- --exact
```

Expected: PASS.

- [ ] **Step 10: Run workspace discovery**

Run:

```powershell
cargo check --workspace
```

Expected: PASS.

- [ ] **Step 11: Commit**

Run:

```powershell
git add Cargo.toml crates/ui-timeline crates/ui-capture crates/ui-composition crates/unified_ui
git commit -m "chore: rename native kinetics boundaries"
```

## Task 2: Add Deterministic Motion Sampling

**Files:**
- Modify: `crates/ui-motion/src/lib.rs`
- Modify: `crates/ui-motion/tests/motion.rs`

- [ ] **Step 1: Write failing interpolation tests**

Append to `crates/ui-motion/tests/motion.rs`:

```rust
use motion_core::{interpolate, sample_tween, Clamp, TweenSample};

#[test]
fn interpolate_clamps_progress_when_requested() {
    assert_eq!(interpolate(10.0, 20.0, -1.0, Clamp::Yes), 10.0);
    assert_eq!(interpolate(10.0, 20.0, 2.0, Clamp::Yes), 20.0);
    assert_eq!(interpolate(10.0, 20.0, 0.5, Clamp::Yes), 15.0);
}

#[test]
fn sample_tween_returns_deterministic_progress_and_value() {
    let sample = sample_tween(0.0, 100.0, 250.0, 1000.0, Ease::Linear);

    assert_eq!(
        sample,
        TweenSample {
            progress: 0.25,
            value: 25.0,
        }
    );
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p ui-motion interpolate_clamps_progress_when_requested -- --exact
cargo test -p ui-motion sample_tween_returns_deterministic_progress_and_value -- --exact
```

Expected: FAIL because `Clamp`, `interpolate`, `sample_tween`, and `TweenSample` do not exist.

- [ ] **Step 3: Implement deterministic helpers**

Add to `crates/ui-motion/src/lib.rs`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clamp {
    Yes,
    No,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TweenSample {
    pub progress: f32,
    pub value: f32,
}

pub fn interpolate(from: f32, to: f32, progress: f32, clamp: Clamp) -> f32 {
    let from = finite_or_zero(from);
    let to = finite_or_zero(to);
    let progress = finite_or_zero(progress);
    let progress = match clamp {
        Clamp::Yes => progress.clamp(0.0, 1.0),
        Clamp::No => progress,
    };

    from + (to - from) * progress
}

pub fn sample_tween(from: f32, to: f32, elapsed_ms: f32, duration_ms: f32, ease: Ease) -> TweenSample {
    let duration_ms = if duration_ms.is_finite() && duration_ms > 0.0 {
        duration_ms
    } else {
        1.0
    };
    let raw = finite_or_zero(elapsed_ms) / duration_ms;
    let progress = apply_ease(raw.clamp(0.0, 1.0), ease);

    TweenSample {
        progress,
        value: interpolate(from, to, progress, Clamp::Yes),
    }
}

pub fn apply_ease(progress: f32, ease: Ease) -> f32 {
    let progress = finite_or_zero(progress).clamp(0.0, 1.0);
    match ease {
        Ease::Linear => progress,
        Ease::Standard => progress * progress * (3.0 - 2.0 * progress),
    }
}

fn finite_or_zero(value: f32) -> f32 {
    if value.is_finite() {
        value
    } else {
        0.0
    }
}
```

- [ ] **Step 4: Run motion tests**

Run:

```powershell
cargo test -p ui-motion
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add crates/ui-motion
git commit -m "feat: add deterministic motion sampling"
```

## Task 3: Expand Material And Glass Recipes

**Files:**
- Modify: `crates/ui-glass/src/lib.rs`
- Modify: `crates/ui-glass/tests/materials.rs`
- Modify: `crates/ui-dom/src/lib.rs`
- Modify: `crates/ui-dom/tests/css.rs`

- [ ] **Step 1: Write failing material recipe tests**

Append to `crates/ui-glass/tests/materials.rs`:

```rust
use ui_glass::{
    resolve_material, GlassDepth, MaterialDensity, MaterialEdge, MaterialPolicy, MaterialRequest,
    MaterialTone, MaterialVibrancy,
};

#[test]
fn material_request_resolves_depth_edge_and_vibrancy() {
    let theme = Theme::default();
    let recipe = resolve_material(
        &theme,
        MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral)
            .with_density(MaterialDensity::Comfortable)
            .with_edge(MaterialEdge::Hairline)
            .with_vibrancy(MaterialVibrancy::Vivid),
    );

    assert_eq!(recipe.backdrop_blur_px, 18.0);
    assert_eq!(recipe.saturate_percent, 180);
    assert_eq!(recipe.radius_px, theme.radius.medium_px);
    assert!(!recipe.force_solid);
}

#[test]
fn high_contrast_material_policy_forces_solid_surface() {
    let theme = Theme::default();
    let recipe = resolve_material(
        &theme,
        MaterialRequest::new(GlassDepth::Overlay, MaterialTone::Primary)
            .with_policy(MaterialPolicy::HighContrast),
    );

    assert!(recipe.force_solid);
    assert_eq!(recipe.backdrop_blur_px, 0.0);
    assert_eq!(recipe.saturate_percent, 100);
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p ui-glass material_request_resolves_depth_edge_and_vibrancy -- --exact
cargo test -p ui-glass high_contrast_material_policy_forces_solid_surface -- --exact
```

Expected: FAIL because material types and `resolve_material` do not exist.

- [ ] **Step 3: Add material aliases and resolver**

Add these public types to `crates/ui-glass/src/lib.rs`:

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GlassDepth {
    Inline,
    Raised,
    #[default]
    Floating,
    Chrome,
    Overlay,
    Modal,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialTone {
    #[default]
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialDensity {
    Compact,
    #[default]
    Comfortable,
    Spacious,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialEdge {
    None,
    #[default]
    Hairline,
    Standard,
    Emphasized,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialVibrancy {
    Muted,
    #[default]
    Standard,
    Vivid,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialPolicy {
    #[default]
    Auto,
    SolidFallback,
    ReducedTransparency,
    HighContrast,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialRequest {
    pub depth: GlassDepth,
    pub tone: MaterialTone,
    pub density: MaterialDensity,
    pub edge: MaterialEdge,
    pub vibrancy: MaterialVibrancy,
    pub policy: MaterialPolicy,
}

impl MaterialRequest {
    pub const fn new(depth: GlassDepth, tone: MaterialTone) -> Self {
        Self {
            depth,
            tone,
            density: MaterialDensity::Comfortable,
            edge: MaterialEdge::Hairline,
            vibrancy: MaterialVibrancy::Standard,
            policy: MaterialPolicy::Auto,
        }
    }

    pub const fn with_density(mut self, density: MaterialDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_edge(mut self, edge: MaterialEdge) -> Self {
        self.edge = edge;
        self
    }

    pub const fn with_vibrancy(mut self, vibrancy: MaterialVibrancy) -> Self {
        self.vibrancy = vibrancy;
        self
    }

    pub const fn with_policy(mut self, policy: MaterialPolicy) -> Self {
        self.policy = policy;
        self
    }
}
```

Add this resolver:

```rust
pub fn resolve_material(theme: &Theme, request: MaterialRequest) -> GlassRecipe {
    let level = match request.depth {
        GlassDepth::Inline | GlassDepth::Raised => GlassLevel::Subtle,
        GlassDepth::Floating => GlassLevel::Floating,
        GlassDepth::Chrome => GlassLevel::Chrome,
        GlassDepth::Overlay | GlassDepth::Modal => GlassLevel::Overlay,
    };
    let tone = match request.tone {
        MaterialTone::Neutral => GlassTone::Neutral,
        MaterialTone::Primary => GlassTone::Primary,
        MaterialTone::Success => GlassTone::Success,
        MaterialTone::Warning => GlassTone::Warning,
        MaterialTone::Danger => GlassTone::Danger,
        MaterialTone::Info => GlassTone::Info,
    };
    let density = match request.density {
        MaterialDensity::Compact => GlassDensity::Compact,
        MaterialDensity::Comfortable => GlassDensity::Comfortable,
        MaterialDensity::Spacious => GlassDensity::Spacious,
    };
    let policy = match request.policy {
        MaterialPolicy::Auto => GlassPolicy::Auto,
        MaterialPolicy::SolidFallback => GlassPolicy::SolidFallback,
        MaterialPolicy::ReducedTransparency => GlassPolicy::ReducedTransparency,
        MaterialPolicy::HighContrast => GlassPolicy::HighContrast,
    };

    let mut recipe = resolve_glass(theme, GlassRequest::new(level, tone, density).with_policy(policy));
    recipe.saturate_percent = match (recipe.force_solid, request.vibrancy) {
        (true, _) => 100,
        (false, MaterialVibrancy::Muted) => 130,
        (false, MaterialVibrancy::Standard) => recipe.saturate_percent,
        (false, MaterialVibrancy::Vivid) => 180,
    };
    recipe
}
```

- [ ] **Step 4: Run glass tests**

Run:

```powershell
cargo test -p ui-glass
```

Expected: PASS.

- [ ] **Step 5: Write failing DOM material CSS test**

Append to `crates/ui-dom/tests/css.rs`:

```rust
use ui_glass::{resolve_material, GlassDepth, MaterialRequest, MaterialTone};

#[test]
fn material_style_writes_css_variables_for_native_material_recipe() {
    let theme = ui_tokens::Theme::default();
    let recipe = resolve_material(
        &theme,
        MaterialRequest::new(GlassDepth::Floating, MaterialTone::Neutral),
    );

    let css = ui_dom::material_style(&recipe);

    assert!(css.contains("--ui-material-blur"));
    assert!(css.contains("--ui-material-saturate"));
    assert!(css.contains("--ui-material-bg"));
    assert!(css.contains("-webkit-backdrop-filter"));
}
```

- [ ] **Step 6: Run and verify failure**

Run:

```powershell
cargo test -p ui-dom material_style_writes_css_variables_for_native_material_recipe -- --exact
```

Expected: FAIL because `material_style` does not exist.

- [ ] **Step 7: Implement DOM material style writer**

Add to `crates/ui-dom/src/lib.rs`:

```rust
pub fn material_style(recipe: &ui_glass::GlassRecipe) -> String {
    CssStyleWriter::new()
        .set("--ui-material-bg", recipe.background.css_rgba())
        .set("--ui-material-solid-bg", recipe.fallback_background.css_rgba())
        .set("--ui-material-border", recipe.border.css_rgba())
        .set("--ui-material-blur", format!("{}px", recipe.backdrop_blur_px))
        .set("--ui-material-saturate", format!("{}%", recipe.saturate_percent))
        .set("background", "var(--ui-material-bg)")
        .set("border-color", "var(--ui-material-border)")
        .set(
        "backdrop-filter",
        "blur(var(--ui-material-blur)) saturate(var(--ui-material-saturate))",
        )
        .set(
        "-webkit-backdrop-filter",
        "blur(var(--ui-material-blur)) saturate(var(--ui-material-saturate))",
        )
        .to_inline_style()
}
```

- [ ] **Step 8: Run DOM tests**

Run:

```powershell
cargo test -p ui-dom
```

Expected: PASS.

- [ ] **Step 9: Commit**

Run:

```powershell
git add crates/ui-glass crates/ui-dom
git commit -m "feat: add native material recipes"
```

## Task 4: Implement Native Timeline Core

**Files:**
- Replace: `crates/ui-timeline/src/lib.rs`
- Create: `crates/ui-timeline/tests/timeline.rs`

- [ ] **Step 1: Write failing timeline tests**

Create `crates/ui-timeline/tests/timeline.rs`:

```rust
use ui_motion::{Ease, Transition};
use ui_timeline::{
    FillMode, MotionCue, MotionSegment, MotionTarget, RepeatMode, StaggerFlow, Timeline,
    TimelineClock, TimelineLabel, TimelineTrack,
};

#[test]
fn timeline_resolves_labels_and_samples_track_values() {
    let timeline = Timeline::new("panel", 500.0)
        .with_label(TimelineLabel::new("enter", 100.0))
        .with_track(TimelineTrack::new(
            MotionTarget::node("panel-card"),
            vec![MotionSegment::new(
                100.0,
                300.0,
                MotionCue::opacity(0.0, 1.0, Transition::tween(300)),
            )],
        ));

    assert_eq!(timeline.label_offset("enter"), Some(100.0));

    let sample = timeline.sample(TimelineClock::Playback { elapsed_ms: 250.0 });
    let value = sample
        .states
        .iter()
        .find(|state| state.target == MotionTarget::node("panel-card"))
        .expect("target state exists")
        .opacity;

    assert!(value > 0.0);
    assert!(value < 1.0);
}

#[test]
fn stagger_flow_produces_deterministic_offsets() {
    let offsets = StaggerFlow::ByIndex { step_ms: 24.0 }.offsets(4);

    assert_eq!(offsets, vec![0.0, 24.0, 48.0, 72.0]);
}

#[test]
fn reduced_motion_collapses_timeline_segments() {
    let timeline = Timeline::new("notice", 180.0).with_track(TimelineTrack::new(
        MotionTarget::self_node(),
        vec![MotionSegment::new(
            0.0,
            180.0,
            MotionCue::opacity(0.0, 1.0, Transition::tween(180)),
        )],
    ));

    let reduced = timeline.reduced_motion();
    let sample = reduced.sample(TimelineClock::Playback { elapsed_ms: 0.0 });

    assert_eq!(reduced.duration_ms, 0.0);
    assert_eq!(sample.states[0].opacity, 1.0);
}

#[test]
fn repeat_yoyo_maps_clock_into_reverse_progress() {
    let timeline = Timeline::new("pulse", 100.0)
        .with_repeat(RepeatMode::Count { count: 2, yoyo: true })
        .with_fill(FillMode::Both)
        .with_track(TimelineTrack::new(
            MotionTarget::self_node(),
            vec![MotionSegment::new(
                0.0,
                100.0,
                MotionCue::opacity(0.0, 1.0, Transition::Tween {
                    duration_ms: 100,
                    ease: Ease::Linear,
                }),
            )],
        ));

    let sample = timeline.sample(TimelineClock::Playback { elapsed_ms: 150.0 });

    assert!(sample.states[0].opacity < 1.0);
    assert!(sample.states[0].opacity > 0.0);
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p ui-timeline
```

Expected: FAIL because timeline model types do not exist.

- [ ] **Step 3: Implement the native timeline model**

Replace `crates/ui-timeline/src/lib.rs` with this MVP:

```rust
#![forbid(unsafe_code)]

use ui_motion::{interpolate, Clamp, Transition};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineId(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KineticId(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MotionTarget {
    SelfNode,
    Node(KineticId),
}

impl MotionTarget {
    pub fn self_node() -> Self {
        Self::SelfNode
    }

    pub fn node(id: impl Into<String>) -> Self {
        Self::Node(KineticId(id.into()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TimelineClock {
    Playback { elapsed_ms: f32 },
    Manual { progress: f32 },
    Scroll { progress: f32 },
    Frame { frame: u32, fps: u32 },
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimelineLabel {
    pub name: String,
    pub offset_ms: f32,
}

impl TimelineLabel {
    pub fn new(name: impl Into<String>, offset_ms: f32) -> Self {
        Self {
            name: name.into(),
            offset_ms,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepeatMode {
    None,
    Count { count: u32, yoyo: bool },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Timeline {
    pub id: TimelineId,
    pub duration_ms: f32,
    pub labels: Vec<TimelineLabel>,
    pub tracks: Vec<TimelineTrack>,
    pub repeat: RepeatMode,
    pub fill: FillMode,
}

impl Timeline {
    pub fn new(id: impl Into<String>, duration_ms: f32) -> Self {
        Self {
            id: TimelineId(id.into()),
            duration_ms: finite_non_negative(duration_ms),
            labels: Vec::new(),
            tracks: Vec::new(),
            repeat: RepeatMode::None,
            fill: FillMode::Both,
        }
    }

    pub fn with_label(mut self, label: TimelineLabel) -> Self {
        self.labels.push(label);
        self
    }

    pub fn with_track(mut self, track: TimelineTrack) -> Self {
        self.tracks.push(track);
        self
    }

    pub fn with_repeat(mut self, repeat: RepeatMode) -> Self {
        self.repeat = repeat;
        self
    }

    pub fn with_fill(mut self, fill: FillMode) -> Self {
        self.fill = fill;
        self
    }

    pub fn label_offset(&self, name: &str) -> Option<f32> {
        self.labels
            .iter()
            .find(|label| label.name == name)
            .map(|label| label.offset_ms)
    }

    pub fn reduced_motion(&self) -> Self {
        let mut reduced = self.clone();
        reduced.duration_ms = 0.0;
        for track in &mut reduced.tracks {
            for segment in &mut track.segments {
                segment.start_ms = 0.0;
                segment.duration_ms = 0.0;
                segment.cue = segment.cue.reduced();
            }
        }
        reduced
    }

    pub fn sample(&self, clock: TimelineClock) -> TimelineSample {
        let elapsed_ms = self.clock_to_elapsed(clock);
        let elapsed_ms = self.map_repeat(elapsed_ms);
        let states = self
            .tracks
            .iter()
            .map(|track| track.sample(elapsed_ms))
            .collect();

        TimelineSample { elapsed_ms, states }
    }

    fn clock_to_elapsed(&self, clock: TimelineClock) -> f32 {
        match clock {
            TimelineClock::Playback { elapsed_ms } => finite_non_negative(elapsed_ms),
            TimelineClock::Manual { progress } | TimelineClock::Scroll { progress } => {
                finite_non_negative(progress).clamp(0.0, 1.0) * self.duration_ms
            }
            TimelineClock::Frame { frame, fps } => {
                if fps == 0 {
                    0.0
                } else {
                    frame as f32 / fps as f32 * 1000.0
                }
            }
        }
    }

    fn map_repeat(&self, elapsed_ms: f32) -> f32 {
        if self.duration_ms == 0.0 {
            return 0.0;
        }

        match self.repeat {
            RepeatMode::None => elapsed_ms.clamp(0.0, self.duration_ms),
            RepeatMode::Count { count, yoyo } => {
                let total = self.duration_ms * count.max(1) as f32;
                let elapsed = elapsed_ms.clamp(0.0, total);
                let iteration = (elapsed / self.duration_ms).floor() as u32;
                let local = elapsed % self.duration_ms;
                if yoyo && iteration % 2 == 1 {
                    self.duration_ms - local
                } else {
                    local
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimelineTrack {
    pub target: MotionTarget,
    pub segments: Vec<MotionSegment>,
}

impl TimelineTrack {
    pub fn new(target: MotionTarget, segments: Vec<MotionSegment>) -> Self {
        Self { target, segments }
    }

    fn sample(&self, elapsed_ms: f32) -> ResolvedMotionState {
        let mut state = ResolvedMotionState {
            target: self.target.clone(),
            opacity: 1.0,
        };

        for segment in &self.segments {
            if segment.contains(elapsed_ms) || elapsed_ms >= segment.start_ms {
                state.opacity = segment.cue.sample_opacity(elapsed_ms, segment);
            }
        }

        state
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MotionSegment {
    pub start_ms: f32,
    pub duration_ms: f32,
    pub cue: MotionCue,
}

impl MotionSegment {
    pub fn new(start_ms: f32, duration_ms: f32, cue: MotionCue) -> Self {
        Self {
            start_ms: finite_non_negative(start_ms),
            duration_ms: finite_non_negative(duration_ms),
            cue,
        }
    }

    fn contains(&self, elapsed_ms: f32) -> bool {
        elapsed_ms >= self.start_ms && elapsed_ms <= self.start_ms + self.duration_ms
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionCue {
    pub opacity_from: f32,
    pub opacity_to: f32,
    pub transition: Transition,
}

impl MotionCue {
    pub fn opacity(from: f32, to: f32, transition: Transition) -> Self {
        Self {
            opacity_from: from,
            opacity_to: to,
            transition,
        }
    }

    pub fn fade_in() -> Self {
        Self::opacity(0.0, 1.0, Transition::tween(180))
    }

    pub fn rise_in() -> Self {
        Self::opacity(0.0, 1.0, Transition::tween(220))
    }

    fn reduced(self) -> Self {
        Self {
            opacity_from: self.opacity_to,
            opacity_to: self.opacity_to,
            transition: self.transition.reduced(),
        }
    }

    fn sample_opacity(&self, elapsed_ms: f32, segment: &MotionSegment) -> f32 {
        if segment.duration_ms == 0.0 {
            return self.opacity_to;
        }

        let progress = ((elapsed_ms - segment.start_ms) / segment.duration_ms).clamp(0.0, 1.0);
        interpolate(self.opacity_from, self.opacity_to, progress, Clamp::Yes)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimelineSample {
    pub elapsed_ms: f32,
    pub states: Vec<ResolvedMotionState>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedMotionState {
    pub target: MotionTarget,
    pub opacity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StaggerFlow {
    ByIndex { step_ms: f32 },
    FromEnd { step_ms: f32 },
}

impl StaggerFlow {
    pub fn offsets(self, count: usize) -> Vec<f32> {
        match self {
            Self::ByIndex { step_ms } => (0..count).map(|index| index as f32 * step_ms).collect(),
            Self::FromEnd { step_ms } => (0..count)
                .map(|index| (count.saturating_sub(index + 1)) as f32 * step_ms)
                .collect(),
        }
    }
}

fn finite_non_negative(value: f32) -> f32 {
    if value.is_finite() && value >= 0.0 {
        value
    } else {
        0.0
    }
}
```

- [ ] **Step 4: Run timeline tests**

Run:

```powershell
cargo test -p ui-timeline
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add crates/ui-timeline
git commit -m "feat: add native timeline core"
```

## Task 5: Add Dioxus Timeline Components

**Files:**
- Create: `crates/ui-dioxus/src/kinetics.rs`
- Modify: `crates/ui-dioxus/src/lib.rs`
- Modify: `crates/ui-dioxus/Cargo.toml`
- Create: `crates/ui-dioxus/tests/kinetics_ssr.rs`

- [ ] **Step 1: Write failing SSR tests**

Create `crates/ui-dioxus/tests/kinetics_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::{KineticBox, KineticText, PresenceGate, TimelineScope};

#[test]
fn timeline_scope_and_kinetic_box_render_stable_attributes() {
    let html = dioxus_ssr::render_element(rsx! {
        TimelineScope { id: "dashboard-enter", autoplay: true,
            KineticBox { id: "metric-card", cue: "rise-in",
                "Revenue"
            }
        }
    });

    assert!(html.contains("ui-timeline-scope"));
    assert!(html.contains("data-timeline-id=\"dashboard-enter\""));
    assert!(html.contains("ui-kinetic-box"));
    assert!(html.contains("data-kinetic-id=\"metric-card\""));
    assert!(html.contains("data-motion-cue=\"rise-in\""));
}

#[test]
fn presence_gate_does_not_render_removed_children() {
    let html = dioxus_ssr::render_element(rsx! {
        PresenceGate { present: false,
            KineticText { id: "toast-copy", text: "Saved" }
        }
    });

    assert!(!html.contains("Saved"));
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p ui-dioxus --test kinetics_ssr
```

Expected: FAIL because the components do not exist.

- [ ] **Step 3: Add dependency**

Add to `crates/ui-dioxus/Cargo.toml`:

```toml
ui-timeline.workspace = true
```

- [ ] **Step 4: Implement Dioxus kinetics components**

Create `crates/ui-dioxus/src/kinetics.rs`:

```rust
use dioxus::prelude::*;

#[component]
pub fn TimelineScope(id: String, #[props(default)] autoplay: bool, children: Element) -> Element {
    rsx! {
        section {
            class: "ui-timeline-scope",
            "data-timeline-id": "{id}",
            "data-autoplay": if autoplay { "true" } else { "false" },
            {children}
        }
    }
}

#[component]
pub fn KineticBox(id: String, #[props(default = "fade-in".to_string())] cue: String, children: Element) -> Element {
    rsx! {
        div {
            class: "ui-kinetic-box",
            "data-kinetic-id": "{id}",
            "data-motion-cue": "{cue}",
            {children}
        }
    }
}

#[component]
pub fn KineticText(id: String, text: String, #[props(default = "text-flow".to_string())] cue: String) -> Element {
    rsx! {
        span {
            class: "ui-kinetic-text",
            "data-kinetic-id": "{id}",
            "data-motion-cue": "{cue}",
            aria_label: "{text}",
            "{text}"
        }
    }
}

#[component]
pub fn PresenceGate(#[props(default = true)] present: bool, children: Element) -> Element {
    if !present {
        return rsx! {};
    }

    rsx! {
        div {
            class: "ui-presence-gate",
            "data-presence": "present",
            {children}
        }
    }
}
```

Modify `crates/ui-dioxus/src/lib.rs`:

```rust
mod kinetics;
pub use kinetics::{KineticBox, KineticText, PresenceGate, TimelineScope};
```

- [ ] **Step 5: Run Dioxus SSR tests**

Run:

```powershell
cargo test -p ui-dioxus
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```powershell
git add crates/ui-dioxus
git commit -m "feat: add dioxus kinetic components"
```

## Task 6: Implement Native Frame Composition Core

**Files:**
- Replace: `crates/ui-composition/src/lib.rs`
- Create: `crates/ui-composition/tests/composition.rs`

- [ ] **Step 1: Write failing composition tests**

Create `crates/ui-composition/tests/composition.rs`:

```rust
use ui_composition::{
    ClipFill, Composition, CompositionError, FrameClock, FrameClip, FrameCue, FrameEase,
    FrameLayer,
};

#[test]
fn composition_validation_rejects_zero_dimensions() {
    let composition = Composition::new("bad", 0, 1080, 30, 120);

    assert_eq!(
        composition.validate(),
        Err(CompositionError::InvalidDimensions)
    );
}

#[test]
fn frame_clock_reports_seconds_and_clamped_progress() {
    let clock = FrameClock { frame: 15, fps: 30 };

    assert_eq!(clock.seconds(), 0.5);
    assert_eq!(clock.progress(0, 30), 0.5);
    assert_eq!(clock.progress(20, 0), 1.0);
}

#[test]
fn frame_clip_activation_respects_fill_mode() {
    let clip = FrameClip::new(10, 20, ClipFill::None);

    assert!(!clip.active_at(9));
    assert!(clip.active_at(10));
    assert!(clip.active_at(29));
    assert!(!clip.active_at(30));
}

#[test]
fn frame_cue_samples_opacity_deterministically() {
    let cue = FrameCue::opacity(0, 30, 0.0, 1.0, FrameEase::Linear);

    assert_eq!(cue.sample_opacity(FrameClock { frame: 15, fps: 30 }), 0.5);
}

#[test]
fn frame_layers_sort_by_depth_then_id() {
    let mut layers = vec![
        FrameLayer::new("b", 10),
        FrameLayer::new("a", 10),
        FrameLayer::new("back", 0),
    ];

    layers.sort();

    assert_eq!(layers[0].id, "back");
    assert_eq!(layers[1].id, "a");
    assert_eq!(layers[2].id, "b");
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p ui-composition
```

Expected: FAIL because composition types are incomplete.

- [ ] **Step 3: Implement composition MVP**

Replace `crates/ui-composition/src/lib.rs` with:

```rust
#![forbid(unsafe_code)]

use std::cmp::Ordering;

use ui_motion::{interpolate, Clamp};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Composition {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub frame_count: u32,
}

impl Composition {
    pub fn new(
        id: impl Into<String>,
        width: u32,
        height: u32,
        fps: u32,
        frame_count: u32,
    ) -> Self {
        Self {
            id: id.into(),
            width,
            height,
            fps,
            frame_count,
        }
    }

    pub fn validate(&self) -> Result<(), CompositionError> {
        if self.width == 0 || self.height == 0 {
            return Err(CompositionError::InvalidDimensions);
        }
        if self.fps == 0 {
            return Err(CompositionError::InvalidFps);
        }
        if self.frame_count == 0 {
            return Err(CompositionError::InvalidFrameCount);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompositionError {
    InvalidDimensions,
    InvalidFps,
    InvalidFrameCount,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameClock {
    pub frame: u32,
    pub fps: u32,
}

impl FrameClock {
    pub fn seconds(&self) -> f32 {
        if self.fps == 0 {
            0.0
        } else {
            self.frame as f32 / self.fps as f32
        }
    }

    pub fn progress(&self, start: u32, duration: u32) -> f32 {
        if duration == 0 {
            return 1.0;
        }
        let elapsed = self.frame.saturating_sub(start) as f32;
        (elapsed / duration as f32).clamp(0.0, 1.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClipFill {
    None,
    HoldStart,
    HoldEnd,
    HoldBoth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameClip {
    pub start: u32,
    pub duration: u32,
    pub fill: ClipFill,
}

impl FrameClip {
    pub fn new(start: u32, duration: u32, fill: ClipFill) -> Self {
        Self {
            start,
            duration,
            fill,
        }
    }

    pub fn active_at(&self, frame: u32) -> bool {
        let end = self.start.saturating_add(self.duration);
        match self.fill {
            ClipFill::None => frame >= self.start && frame < end,
            ClipFill::HoldStart => frame <= end,
            ClipFill::HoldEnd => frame >= self.start,
            ClipFill::HoldBoth => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameEase {
    Linear,
    Standard,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameCue {
    pub start: u32,
    pub duration: u32,
    pub opacity_from: f32,
    pub opacity_to: f32,
    pub ease: FrameEase,
}

impl FrameCue {
    pub fn opacity(start: u32, duration: u32, from: f32, to: f32, ease: FrameEase) -> Self {
        Self {
            start,
            duration,
            opacity_from: from,
            opacity_to: to,
            ease,
        }
    }

    pub fn fade_in(start: u32, duration: u32) -> Self {
        Self::opacity(start, duration, 0.0, 1.0, FrameEase::Standard)
    }

    pub fn sample_opacity(&self, clock: FrameClock) -> f32 {
        let progress = clock.progress(self.start, self.duration);
        let progress = match self.ease {
            FrameEase::Linear => progress,
            FrameEase::Standard => progress * progress * (3.0 - 2.0 * progress),
        };
        interpolate(self.opacity_from, self.opacity_to, progress, Clamp::Yes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameLayer {
    pub id: String,
    pub depth: i32,
}

impl FrameLayer {
    pub fn new(id: impl Into<String>, depth: i32) -> Self {
        Self {
            id: id.into(),
            depth,
        }
    }
}

impl Ord for FrameLayer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.depth
            .cmp(&other.depth)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for FrameLayer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

- [ ] **Step 4: Run composition tests**

Run:

```powershell
cargo test -p ui-composition
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add crates/ui-composition
git commit -m "feat: add native frame composition core"
```

## Task 7: Implement Native Capture Core

**Files:**
- Replace: `crates/ui-capture/src/lib.rs`
- Create: `crates/ui-capture/tests/capture.rs`

- [ ] **Step 1: Write failing capture tests**

Create `crates/ui-capture/tests/capture.rs`:

```rust
use ui_capture::{
    CaptureError, CaptureMark, CaptureStageDescriptor, ExportManifest, ViewportProfile,
};
use ui_composition::Composition;

#[test]
fn viewport_profile_rejects_zero_size() {
    let viewport = ViewportProfile::new("bad", 0, 844);

    assert_eq!(viewport.validate(), Err(CaptureError::InvalidViewport));
}

#[test]
fn capture_marks_resolve_by_name() {
    let manifest = ExportManifest::new("0.1.0")
        .with_mark(CaptureMark::new("modal-open", 24))
        .with_mark(CaptureMark::new("settled", 90));

    assert_eq!(manifest.mark_frame("settled"), Some(90));
    assert_eq!(manifest.mark_frame("missing"), None);
}

#[test]
fn manifest_validates_stage_composition_and_viewport() {
    let manifest = ExportManifest::new("0.1.0")
        .with_composition(Composition::new("demo", 1920, 1080, 30, 120))
        .with_stage(CaptureStageDescriptor::new("stage", "demo"))
        .with_viewport(ViewportProfile::desktop());

    assert_eq!(manifest.validate(), Ok(()));
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p ui-capture
```

Expected: FAIL because capture types are incomplete.

- [ ] **Step 3: Implement capture MVP**

Replace `crates/ui-capture/src/lib.rs` with:

```rust
#![forbid(unsafe_code)]

use ui_composition::Composition;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewportProfile {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

impl ViewportProfile {
    pub fn new(name: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            name: name.into(),
            width,
            height,
        }
    }

    pub fn desktop() -> Self {
        Self::new("desktop", 1440, 960)
    }

    pub fn tablet() -> Self {
        Self::new("tablet", 1024, 768)
    }

    pub fn mobile() -> Self {
        Self::new("mobile", 390, 844)
    }

    pub fn validate(&self) -> Result<(), CaptureError> {
        if self.width == 0 || self.height == 0 {
            Err(CaptureError::InvalidViewport)
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaptureMark {
    pub name: String,
    pub frame: u32,
}

impl CaptureMark {
    pub fn new(name: impl Into<String>, frame: u32) -> Self {
        Self {
            name: name.into(),
            frame,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaptureStageDescriptor {
    pub id: String,
    pub composition_id: String,
}

impl CaptureStageDescriptor {
    pub fn new(id: impl Into<String>, composition_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            composition_id: composition_id.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportManifest {
    pub schema_version: u32,
    pub library_version: String,
    pub compositions: Vec<Composition>,
    pub stages: Vec<CaptureStageDescriptor>,
    pub viewports: Vec<ViewportProfile>,
    pub marks: Vec<CaptureMark>,
}

impl ExportManifest {
    pub fn new(library_version: impl Into<String>) -> Self {
        Self {
            schema_version: 1,
            library_version: library_version.into(),
            compositions: Vec::new(),
            stages: Vec::new(),
            viewports: Vec::new(),
            marks: Vec::new(),
        }
    }

    pub fn with_composition(mut self, composition: Composition) -> Self {
        self.compositions.push(composition);
        self
    }

    pub fn with_stage(mut self, stage: CaptureStageDescriptor) -> Self {
        self.stages.push(stage);
        self
    }

    pub fn with_viewport(mut self, viewport: ViewportProfile) -> Self {
        self.viewports.push(viewport);
        self
    }

    pub fn with_mark(mut self, mark: CaptureMark) -> Self {
        self.marks.push(mark);
        self
    }

    pub fn mark_frame(&self, name: &str) -> Option<u32> {
        self.marks
            .iter()
            .find(|mark| mark.name == name)
            .map(|mark| mark.frame)
    }

    pub fn validate(&self) -> Result<(), CaptureError> {
        for viewport in &self.viewports {
            viewport.validate()?;
        }
        for composition in &self.compositions {
            composition.validate().map_err(|_| CaptureError::InvalidComposition)?;
        }
        for stage in &self.stages {
            if !self
                .compositions
                .iter()
                .any(|composition| composition.id == stage.composition_id)
            {
                return Err(CaptureError::MissingComposition);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureError {
    InvalidViewport,
    InvalidComposition,
    MissingComposition,
}
```

- [ ] **Step 4: Run capture tests**

Run:

```powershell
cargo test -p ui-capture
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add crates/ui-capture
git commit -m "feat: add native capture manifest core"
```

## Task 8: Add Dioxus Composition And Capture Components

**Files:**
- Modify: `crates/ui-dioxus/Cargo.toml`
- Create: `crates/ui-dioxus/src/composition.rs`
- Create: `crates/ui-dioxus/src/capture.rs`
- Modify: `crates/ui-dioxus/src/lib.rs`
- Create: `crates/ui-dioxus/tests/composition_capture_ssr.rs`

- [ ] **Step 1: Write failing SSR tests**

Create `crates/ui-dioxus/tests/composition_capture_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_composition::Composition;
use ui_dioxus::{CaptureStage, FrameClip, FrameLayer, FrameStage};

#[test]
fn frame_stage_clip_and_layer_render_deterministic_frame_attributes() {
    let composition = Composition::new("launch-demo", 1920, 1080, 30, 180);
    let html = dioxus_ssr::render_element(rsx! {
        FrameStage { composition, frame: 42,
            FrameClip { start: 0, duration: 60,
                FrameLayer { id: "title", depth: 10,
                    "Dioxus Kinetics"
                }
            }
        }
    });

    assert!(html.contains("ui-frame-stage"));
    assert!(html.contains("data-composition-id=\"launch-demo\""));
    assert!(html.contains("data-frame=\"42\""));
    assert!(html.contains("ui-frame-layer"));
    assert!(html.contains("data-depth=\"10\""));
}

#[test]
fn capture_stage_renders_viewport_and_frame_metadata() {
    let html = dioxus_ssr::render_element(rsx! {
        CaptureStage { id: "component-showcase", viewport: "desktop", frame: 72,
            "Preview"
        }
    });

    assert!(html.contains("ui-capture-stage"));
    assert!(html.contains("data-capture-id=\"component-showcase\""));
    assert!(html.contains("data-viewport=\"desktop\""));
    assert!(html.contains("data-frame=\"72\""));
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p ui-dioxus --test composition_capture_ssr
```

Expected: FAIL because components do not exist.

- [ ] **Step 3: Add dependencies**

Add to `crates/ui-dioxus/Cargo.toml`:

```toml
ui-composition.workspace = true
ui-capture.workspace = true
```

- [ ] **Step 4: Implement frame components**

Create `crates/ui-dioxus/src/composition.rs`:

```rust
use dioxus::prelude::*;
use ui_composition::Composition;

#[component]
pub fn FrameStage(composition: Composition, frame: u32, children: Element) -> Element {
    rsx! {
        section {
            class: "ui-frame-stage",
            "data-composition-id": "{composition.id}",
            "data-width": "{composition.width}",
            "data-height": "{composition.height}",
            "data-fps": "{composition.fps}",
            "data-frame": "{frame}",
            {children}
        }
    }
}

#[component]
pub fn FrameClip(start: u32, duration: u32, children: Element) -> Element {
    rsx! {
        div {
            class: "ui-frame-clip",
            "data-start": "{start}",
            "data-duration": "{duration}",
            {children}
        }
    }
}

#[component]
pub fn FrameLayer(id: String, depth: i32, children: Element) -> Element {
    rsx! {
        div {
            class: "ui-frame-layer",
            "data-layer-id": "{id}",
            "data-depth": "{depth}",
            {children}
        }
    }
}
```

- [ ] **Step 5: Implement capture component**

Create `crates/ui-dioxus/src/capture.rs`:

```rust
use dioxus::prelude::*;

#[component]
pub fn CaptureStage(id: String, viewport: String, frame: u32, children: Element) -> Element {
    rsx! {
        section {
            class: "ui-capture-stage",
            "data-capture-id": "{id}",
            "data-viewport": "{viewport}",
            "data-frame": "{frame}",
            {children}
        }
    }
}
```

Modify `crates/ui-dioxus/src/lib.rs`:

```rust
mod capture;
mod composition;

pub use capture::CaptureStage;
pub use composition::{FrameClip, FrameLayer, FrameStage};
```

- [ ] **Step 6: Run SSR tests**

Run:

```powershell
cargo test -p ui-dioxus
```

Expected: PASS.

- [ ] **Step 7: Commit**

Run:

```powershell
git add crates/ui-dioxus
git commit -m "feat: add dioxus composition capture components"
```

## Task 9: Add Functional Component Names

**Files:**
- Modify: `crates/ui-dioxus/src/lib.rs`
- Modify: `crates/unified_ui/src/lib.rs`
- Modify: `crates/unified_ui/tests/prelude.rs`
- Modify: `docs/component-naming.md`

- [ ] **Step 1: Write failing prelude test**

Append to `crates/unified_ui/tests/prelude.rs`:

```rust
#[test]
fn prelude_exposes_functional_component_names() {
    let names = unified_ui::public_api_names();

    for expected in [
        "ActionControl",
        "TextEntry",
        "ChoiceMark",
        "StateSwitch",
        "ViewSwitcher",
        "ActionBar",
        "NavigationRail",
        "MetricReadout",
        "BlankState",
        "ModalLayer",
        "NoticeStack",
        "CommandFinder",
        "ContextHint",
        "ContentPlane",
        "GlassLayer",
    ] {
        assert!(names.contains(&expected), "missing functional name {expected}");
    }
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p unified_ui prelude_exposes_functional_component_names -- --exact
```

Expected: FAIL because `public_api_names()` does not contain the functional names.

- [ ] **Step 3: Add functional re-exports in `ui-dioxus`**

In `crates/ui-dioxus/src/lib.rs`, add these public aliases after existing `pub use` lines:

```rust
pub use display::{EmptyState as BlankState, MetricCard as MetricReadout};
pub use forms::{Checkbox as ChoiceMark, Switch as StateSwitch, TextField as TextEntry};
pub use navigation::{
    Sidebar as NavigationRail, Tabs as ViewSwitcher, Toolbar as ActionBar,
};
pub use overlays::{
    CommandMenu as CommandFinder, Dialog as ModalLayer, Toast as NoticeStack,
    Tooltip as ContextHint,
};

pub use Button as ActionControl;
pub use GlassSurface as GlassLayer;
pub use Surface as ContentPlane;
```

- [ ] **Step 4: Add functional exports in facade prelude**

In `crates/unified_ui/src/lib.rs`, add the functional names to the `ui_dioxus` prelude export list:

```rust
ActionBar, ActionControl, BlankState, ChoiceMark, CommandFinder, ContentPlane, ContextHint,
GlassLayer, MetricReadout, ModalLayer, NavigationRail, NoticeStack, StateSwitch, TextEntry,
ViewSwitcher,
```

Update `public_api_names()` to include the same strings.

- [ ] **Step 5: Update naming docs**

Replace `docs/component-naming.md` with:

```markdown
# Component Naming

Unified UI uses functional component names.

Names describe the user-facing role or behavior:

- `ActionControl`
- `TextEntry`
- `ChoiceMark`
- `StateSwitch`
- `ViewSwitcher`
- `ActionBar`
- `NavigationRail`
- `MetricReadout`
- `BlankState`
- `ModalLayer`
- `NoticeStack`
- `CommandFinder`
- `ContextHint`
- `ContentPlane`
- `GlassLayer`
- `TimelineScope`
- `FrameStage`
- `CaptureStage`

Public names do not borrow library, framework, platform, animation, or video product names.
The previous MVP names can remain available during the `0.1.x` transition, but the default
documentation and examples should prefer the functional names.
```

- [ ] **Step 6: Run facade tests**

Run:

```powershell
cargo test -p unified_ui
```

Expected: PASS.

- [ ] **Step 7: Commit**

Run:

```powershell
git add crates/ui-dioxus crates/unified_ui docs/component-naming.md
git commit -m "feat: expose functional component names"
```

## Task 10: Add Native System CSS

**Files:**
- Modify: `crates/ui-styles/src/lib.rs`
- Modify: `crates/ui-styles/tests/css.rs`

- [ ] **Step 1: Write failing CSS test**

Append to `crates/ui-styles/tests/css.rs`:

```rust
#[test]
fn component_css_covers_native_kinetics_systems() {
    let css = COMPONENT_CSS;

    for selector in [
        ".ui-glass-layer",
        ".ui-timeline-scope",
        ".ui-kinetic-box",
        ".ui-kinetic-text",
        ".ui-presence-gate",
        ".ui-frame-stage",
        ".ui-frame-clip",
        ".ui-frame-layer",
        ".ui-capture-stage",
    ] {
        assert!(css.contains(selector), "missing selector {selector}");
    }
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p ui-styles component_css_covers_native_kinetics_systems -- --exact
```

Expected: FAIL because the selectors are missing.

- [ ] **Step 3: Add CSS selectors**

Append this block inside `COMPONENT_CSS` in `crates/ui-styles/src/lib.rs`:

```css
.ui-glass-layer {
    background: var(--ui-material-bg, var(--ui-glass));
    border: 1px solid var(--ui-material-border, var(--ui-border));
    border-radius: var(--ui-radius-lg);
    box-shadow: var(--ui-material-shadow, var(--ui-shadow-soft));
    backdrop-filter: blur(var(--ui-material-blur, 18px)) saturate(var(--ui-material-saturate, 160%));
    -webkit-backdrop-filter: blur(var(--ui-material-blur, 18px)) saturate(var(--ui-material-saturate, 160%));
}

.ui-timeline-scope,
.ui-presence-gate {
    display: grid;
    gap: var(--ui-space-3);
}

.ui-kinetic-box,
.ui-kinetic-text,
.ui-frame-layer {
    transition: opacity var(--ui-motion-normal), transform var(--ui-motion-normal);
}

.ui-frame-stage,
.ui-capture-stage {
    position: relative;
    overflow: hidden;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-surface);
}

.ui-frame-clip {
    display: contents;
}
```

- [ ] **Step 4: Run style tests**

Run:

```powershell
cargo test -p ui-styles
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add crates/ui-styles
git commit -m "feat: style native kinetics systems"
```

## Task 11: Update Component Gallery For Native Systems

**Files:**
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/src/styles.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write failing gallery category tests**

Update `examples/component-gallery/tests/gallery.rs` category expectation:

```rust
assert_eq!(
    categories,
    &[
        ComponentCategory::Foundations,
        ComponentCategory::Actions,
        ComponentCategory::Inputs,
        ComponentCategory::Navigation,
        ComponentCategory::Layout,
        ComponentCategory::Surfaces,
        ComponentCategory::Feedback,
        ComponentCategory::DataWorkflows,
        ComponentCategory::Motion,
        ComponentCategory::Composition,
        ComponentCategory::Capture,
    ]
);
```

Append this test:

```rust
#[test]
fn gallery_renders_native_kinetics_examples_without_bridge_copy() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for expected in ["TimelineScope", "FrameStage", "CaptureStage", "GlassLayer"] {
        assert!(html.contains(expected), "missing gallery entry {expected}");
    }

    for rejected in ["GSAP", "Remotion", "HyperFrames"] {
        assert!(!html.contains(rejected), "gallery must not show bridge copy {rejected}");
    }
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p component-gallery gallery_renders_native_kinetics_examples_without_bridge_copy -- --exact
cargo test -p component-gallery registry_groups_components_by_product_category -- --exact
```

Expected: FAIL because categories and entries are not updated.

- [ ] **Step 3: Expand `ComponentCategory`**

In `examples/component-gallery/src/docs.rs`, replace `ComponentCategory` with:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentCategory {
    Foundations,
    Actions,
    Inputs,
    Navigation,
    Layout,
    Surfaces,
    Feedback,
    DataWorkflows,
    Motion,
    Composition,
    Capture,
}
```

Update `label`, `description`, `slug`, and `categories()` with exact values from the test.

- [ ] **Step 4: Add native system docs entries**

Increase `COMPONENT_DOCS` length and add these entries:

```rust
ComponentDoc {
    name: "TimelineScope",
    category: ComponentCategory::Motion,
    status: ComponentStatus::Ready,
    summary: "Coordinates native Rust timeline cues for Dioxus UI motion.",
    snippet: TIMELINE_SCOPE_SNIPPET,
    accessibility: "Reduced motion policies collapse timeline cues to stable states.",
    render: Some(timeline_scope_preview),
},
ComponentDoc {
    name: "FrameStage",
    category: ComponentCategory::Composition,
    status: ComponentStatus::Ready,
    summary: "Renders a deterministic frame-addressable scene for previews and export-safe compositions.",
    snippet: FRAME_STAGE_SNIPPET,
    accessibility: "Frame content remains readable at the selected frame and does not depend on wall-clock animation.",
    render: Some(frame_stage_preview),
},
ComponentDoc {
    name: "CaptureStage",
    category: ComponentCategory::Capture,
    status: ComponentStatus::Ready,
    summary: "Declares a viewport and frame target for documentation, tests, and future capture runners.",
    snippet: CAPTURE_STAGE_SNIPPET,
    accessibility: "Capture previews preserve semantic text and expose stable frame metadata.",
    render: Some(capture_stage_preview),
},
ComponentDoc {
    name: "GlassLayer",
    category: ComponentCategory::Foundations,
    status: ComponentStatus::Ready,
    summary: "Functional material name for translucent glass surfaces with solid fallback behavior.",
    snippet: GLASS_LAYER_SNIPPET,
    accessibility: "Text contrast is validated against solid fallback surfaces.",
    render: Some(glass_layer_preview),
},
```

Add snippets:

```rust
const TIMELINE_SCOPE_SNIPPET: &str = r#"TimelineScope {
    id: "dashboard-enter",
    autoplay: true,
    KineticBox {
        id: "metric-card",
        cue: "rise-in",
        "Revenue"
    }
}"#;

const FRAME_STAGE_SNIPPET: &str = r#"FrameStage {
    composition: Composition::new("launch-demo", 1920, 1080, 30, 180),
    frame: 42,
    FrameClip {
        start: 0,
        duration: 60,
        FrameLayer {
            id: "title",
            depth: 10,
            "Dioxus Kinetics"
        }
    }
}"#;

const CAPTURE_STAGE_SNIPPET: &str = r#"CaptureStage {
    id: "component-showcase",
    viewport: "desktop",
    frame: 72,
    "Preview"
}"#;

const GLASS_LAYER_SNIPPET: &str = r#"GlassLayer {
    level: GlassLevel::Floating,
    tone: GlassTone::Neutral,
    density: GlassDensity::Comfortable,
    "Revenue operations"
}"#;
```

- [ ] **Step 5: Add native preview functions**

Add to `examples/component-gallery/src/docs.rs`:

```rust
fn timeline_scope_preview() -> Element {
    rsx! {
        TimelineScope { id: "dashboard-enter", autoplay: true,
            KineticBox { id: "metric-card", cue: "rise-in",
                MetricReadout {
                    label: "Pipeline",
                    value: "$418k",
                    delta: "+8.2%",
                    tone: MetricTone::Info,
                }
            }
        }
    }
}

fn frame_stage_preview() -> Element {
    rsx! {
        FrameStage {
            composition: Composition::new("launch-demo", 1920, 1080, 30, 180),
            frame: 42,
            FrameClip { start: 0, duration: 60,
                FrameLayer { id: "title", depth: 10,
                    h4 { "Dioxus Kinetics" }
                    p { "Frame 42 / 180" }
                }
            }
        }
    }
}

fn capture_stage_preview() -> Element {
    rsx! {
        CaptureStage { id: "component-showcase", viewport: "desktop", frame: 72,
            p { "Desktop viewport at frame 72" }
        }
    }
}

fn glass_layer_preview() -> Element {
    rsx! {
        GlassLayer {
            level: GlassLevel::Floating,
            tone: GlassTone::Neutral,
            density: GlassDensity::Comfortable,
            h4 { "Revenue operations" }
            p { "Native material contract" }
        }
    }
}
```

- [ ] **Step 6: Add gallery preview CSS**

Append to `examples/component-gallery/src/styles.rs`:

```css
.gallery-preview .ui-frame-stage,
.gallery-preview .ui-capture-stage {
    min-height: 180px;
    display: grid;
    place-items: center;
    padding: var(--ui-space-4);
}
```

- [ ] **Step 7: Run gallery tests**

Run:

```powershell
cargo test -p component-gallery
```

Expected: PASS.

- [ ] **Step 8: Commit**

Run:

```powershell
git add examples/component-gallery
git commit -m "feat: document native kinetics systems"
```

## Task 12: Update README And Platform Docs

**Files:**
- Modify: `README.md`
- Modify: `docs/platform-support.md`
- Modify: `docs/glass-materials.md`

- [ ] **Step 1: Write failing README test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn root_readme_describes_native_systems_without_bridge_language() {
    let readme_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../README.md");
    let readme = std::fs::read_to_string(readme_path).expect("README.md should be readable");

    for expected in ["ui-timeline", "ui-composition", "ui-capture"] {
        assert!(readme.contains(expected), "README missing {expected}");
    }

    for rejected in ["GSAP", "Remotion", "HyperFrames"] {
        assert!(!readme.contains(rejected), "README still contains bridge term {rejected}");
    }
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p component-gallery root_readme_describes_native_systems_without_bridge_language -- --exact
```

Expected: FAIL because README still mentions bridge language.

- [ ] **Step 3: Update README workspace layout**

In `README.md`, replace the bridge crate lines with:

```text
  ui-timeline/      native timeline, stagger, presence, scroll, and shared movement contracts
  ui-composition/   native frame composition and deterministic frame sampling
  ui-capture/       native capture stages, viewport profiles, marks, and export manifests
```

Replace optional features with:

```text
- `native`
- `timeline`
- `composition`
- `capture`
- `a11y-tests`
```

Replace current status bridge bullets with:

```text
- native timeline boundary
- native frame composition boundary
- native capture manifest boundary
```

- [ ] **Step 4: Update platform support docs**

Replace `docs/platform-support.md` with:

```markdown
# Platform Support

| Target | Status | Backend |
|---|---|---|
| Web | MVP | DOM style adapter |
| Desktop | MVP | WebView DOM style adapter |
| Mobile | MVP | WebView DOM style adapter |
| Native | MVP contract | Native capability adapter |

Native support begins with semantic parity, token rendering, glass fallback,
focus behavior, motion snapshots, composition metadata, and capture manifests.

Timeline, composition, and capture are native Rust/Dioxus systems. They do not
depend on third-party animation, video, or capture runtimes.
```

- [ ] **Step 5: Update glass docs**

Append to `docs/glass-materials.md`:

```markdown
## Native Material Names

The public material API uses functional names:

- `GlassLayer`
- `MaterialRequest`
- `MaterialTone`
- `MaterialDensity`
- `MaterialPolicy`
- `GlassDepth`

Existing glass recipe types remain available during the `0.1.x` transition,
but docs and examples prefer the functional material vocabulary.
```

- [ ] **Step 6: Run README test**

Run:

```powershell
cargo test -p component-gallery root_readme_describes_native_systems_without_bridge_language -- --exact
```

Expected: PASS.

- [ ] **Step 7: Commit**

Run:

```powershell
git add README.md docs/platform-support.md docs/glass-materials.md examples/component-gallery/tests/gallery.rs
git commit -m "docs: describe native kinetics architecture"
```

## Task 13: Full Verification

**Files:**
- No planned source edits.

- [ ] **Step 1: Run format check**

Run:

```powershell
cargo fmt --all -- --check
```

Expected: PASS.

- [ ] **Step 2: Run focused crate tests**

Run:

```powershell
cargo test -p ui-motion
cargo test -p ui-layout
cargo test -p ui-glass
cargo test -p ui-dom
cargo test -p ui-timeline
cargo test -p ui-composition
cargo test -p ui-capture
cargo test -p ui-dioxus
cargo test -p ui-styles
cargo test -p unified_ui
cargo test -p component-gallery
```

Expected: every command exits `0`.

- [ ] **Step 3: Run component gallery check**

Run:

```powershell
cargo check -p component-gallery
```

Expected: PASS.

- [ ] **Step 4: Run full workspace tests**

Run:

```powershell
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 5: Run bridge-name scan**

Run:

```powershell
rg -n "GSAP|Remotion|HyperFrames|ui-gsap|ui-hyperframes|hyperframes-export" README.md docs/component-naming.md docs/platform-support.md docs/glass-materials.md crates examples
```

Expected: no matches. If matches appear in public docs, source crates, or gallery files, remove those bridge references and rerun this scan.

- [ ] **Step 6: Commit verification-only doc corrections if needed**

If Step 5 required source or public doc edits, run:

```powershell
git add README.md docs crates examples
git commit -m "docs: remove bridge naming remnants"
```

If Step 5 required no edits, do not create an empty commit.

- [ ] **Step 7: Push**

Run:

```powershell
git status --short --branch
git push origin main
```

Expected: `main` is pushed. `Reading_material/` remains untracked unless the user explicitly asks to add it.

## Acceptance Checklist

- [ ] `ui-gsap` is replaced by `ui-timeline`.
- [ ] `ui-hyperframes` is replaced by `ui-capture`.
- [ ] `ui-composition` exists.
- [ ] Facade features are `timeline`, `composition`, and `capture`.
- [ ] Default public docs do not present bridge integrations.
- [ ] `ui-motion` has deterministic sampling helpers.
- [ ] `ui-glass` has material names and expanded axes.
- [ ] `ui-timeline` can sample labels, segments, repeat, yoyo, stagger, and reduced motion.
- [ ] `ui-composition` can validate compositions, sample frame cues, and sort layers.
- [ ] `ui-capture` can validate viewports, resolve marks, and validate manifests.
- [ ] `ui-dioxus` exports SSR-safe timeline, frame, and capture components.
- [ ] `unified_ui::prelude::*` exports native names.
- [ ] Gallery has category-wise native systems documentation.
- [ ] `cargo test --workspace` passes.
