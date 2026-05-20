# Unified UI Library Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first working MVP of a single Dioxus UI library for downstream SaaS apps, with semantic components, Apple-like glass tokens, motion primitives, layout math, Web/Desktop/Mobile DOM support, and a Native adapter contract.

**Architecture:** Create a Rust workspace with focused crates behind one public `unified_ui` facade. Keep semantic state and component contracts portable; renderer-specific crates convert the same tokens and contracts into DOM/WebView or Native behavior. Keep GSAP and HyperFrames as optional crates with explicit backend boundaries so they do not affect default runtime usage.

**Tech Stack:** Rust 2021, Cargo workspace, Dioxus 0.7, Dioxus SSR tests, pure Rust unit tests, feature-gated public facade, Git commits per task.

---

## Scope Check

The approved design is broad. This plan implements the first coherent MVP, not every mature backend feature. It creates the full workspace shape, stable public naming, token/glass/motion/layout cores, DOM/WebView support, Native capability planning, public prelude, docs, and optional backend boundaries. Rich GSAP timelines, HyperFrames video export rendering, Playwright visual tests, and native visual parity each need follow-up plans after this MVP compiles and has unit/SSR coverage.

This is still one testable implementation stream because each task contributes to one library facade: `unified_ui`.

## File Structure

Create this structure:

```text
Cargo.toml
.gitignore
README.md
crates/
  ui-core/
    Cargo.toml
    src/lib.rs
    tests/contracts.rs
  ui-tokens/
    Cargo.toml
    src/lib.rs
    tests/theme.rs
  ui-glass/
    Cargo.toml
    src/lib.rs
    tests/materials.rs
  ui-motion/
    Cargo.toml
    src/lib.rs
    tests/motion.rs
  ui-layout/
    Cargo.toml
    src/lib.rs
    tests/flip.rs
  ui-dom/
    Cargo.toml
    src/lib.rs
    tests/css.rs
  ui-native/
    Cargo.toml
    src/lib.rs
    tests/capabilities.rs
  ui-dioxus/
    Cargo.toml
    src/lib.rs
    tests/ssr.rs
  ui-gsap/
    Cargo.toml
    src/lib.rs
    tests/backend.rs
  ui-hyperframes/
    Cargo.toml
    src/lib.rs
    tests/export.rs
  unified_ui/
    Cargo.toml
    src/lib.rs
    tests/prelude.rs
```

Responsibility boundaries:

- `ui-tokens`: values and theme policy only.
- `ui-glass`: material recipes derived from tokens.
- `ui-core`: semantic contracts, roles, IDs, target sizing, accessibility policy.
- `ui-motion`: deterministic animation math and presence state.
- `ui-layout`: renderer-neutral box math.
- `ui-dom`: CSS/style serialization for Web, Desktop, and Mobile WebView targets.
- `ui-native`: capability planning for Dioxus Native/Blitz without web-only dependencies.
- `ui-dioxus`: semantic Dioxus components that use the core crates.
- `ui-gsap`: optional backend boundary for advanced web animation.
- `ui-hyperframes`: optional export boundary for deterministic scene output.
- `unified_ui`: the single downstream SaaS dependency and prelude.

## Task 1: Workspace Scaffold

**Files:**
- Create: `Cargo.toml`
- Create: `.gitignore`
- Create: `README.md`
- Create: each crate `Cargo.toml`
- Create: each crate `src/lib.rs`

- [ ] **Step 1: Initialize Git if the workspace is not already a repository**

Run:

```powershell
git rev-parse --is-inside-work-tree
```

Expected if no repository exists:

```text
fatal: not a git repository (or any of the parent directories): .git
```

Then run:

```powershell
git init
```

Expected:

```text
Initialized empty Git repository
```

- [ ] **Step 2: Create the workspace manifest**

Write `Cargo.toml`:

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
    "crates/ui-gsap",
    "crates/ui-hyperframes",
    "crates/unified_ui",
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
ui-gsap = { path = "crates/ui-gsap" }
ui-hyperframes = { path = "crates/ui-hyperframes" }
```

- [ ] **Step 3: Add repository ignore rules**

Write `.gitignore`:

```gitignore
/target/
/.dioxus/
/dist/
/node_modules/
Cargo.lock
*.pdb
*.log
```

- [ ] **Step 4: Add the initial README**

Write `README.md`:

```markdown
# Unified UI

Unified UI is a Dioxus-first UI library for downstream SaaS products.

The library exposes one public crate, `unified_ui`, while keeping tokens,
glass materials, motion, layout, and renderer adapters in focused internal
crates.

Default goals:

- semantic component names
- Apple-like glass materials with solid fallbacks
- Web, Desktop, Mobile, and Native adapter contracts
- accessibility and reduced-preference policies
- WCAG 2.2 AA target for default themes
- optional GSAP and HyperFrames integrations outside default features
```

- [ ] **Step 5: Create minimal crate manifests and library files**

Create each crate directory and write these manifests.

`crates/ui-tokens/Cargo.toml`:

```toml
[package]
name = "ui-tokens"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[lib]
path = "src/lib.rs"
```

`crates/ui-glass/Cargo.toml`:

```toml
[package]
name = "ui-glass"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
ui-tokens.workspace = true

[lib]
path = "src/lib.rs"
```

`crates/ui-core/Cargo.toml`:

```toml
[package]
name = "ui-core"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
ui-tokens.workspace = true

[lib]
path = "src/lib.rs"
```

`crates/ui-motion/Cargo.toml`:

```toml
[package]
name = "ui-motion"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[lib]
path = "src/lib.rs"
```

`crates/ui-layout/Cargo.toml`:

```toml
[package]
name = "ui-layout"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[lib]
path = "src/lib.rs"
```

`crates/ui-dom/Cargo.toml`:

```toml
[package]
name = "ui-dom"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
ui-glass.workspace = true
ui-tokens.workspace = true

[lib]
path = "src/lib.rs"
```

`crates/ui-native/Cargo.toml`:

```toml
[package]
name = "ui-native"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
ui-glass.workspace = true

[lib]
path = "src/lib.rs"
```

`crates/ui-dioxus/Cargo.toml`:

```toml
[package]
name = "ui-dioxus"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
dioxus.workspace = true
ui-core.workspace = true
ui-glass.workspace = true
ui-tokens.workspace = true

[dev-dependencies]
dioxus-ssr.workspace = true

[lib]
path = "src/lib.rs"
```

`crates/ui-gsap/Cargo.toml`:

```toml
[package]
name = "ui-gsap"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
ui-motion.workspace = true

[lib]
path = "src/lib.rs"
```

`crates/ui-hyperframes/Cargo.toml`:

```toml
[package]
name = "ui-hyperframes"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
ui-motion.workspace = true

[lib]
path = "src/lib.rs"
```

`crates/unified_ui/Cargo.toml`:

```toml
[package]
name = "unified_ui"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[features]
default = ["web", "desktop", "mobile", "tokens", "glass", "motion", "layout-motion", "a11y"]
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
gsap = ["dep:ui-gsap"]
hyperframes-export = ["dep:ui-hyperframes"]

[dependencies]
ui-core.workspace = true
ui-tokens.workspace = true
ui-glass.workspace = true
ui-motion.workspace = true
ui-layout.workspace = true
ui-dioxus.workspace = true
ui-dom = { workspace = true, optional = true }
ui-native = { workspace = true, optional = true }
ui-gsap = { workspace = true, optional = true }
ui-hyperframes = { workspace = true, optional = true }

[lib]
path = "src/lib.rs"
```

For every `src/lib.rs`, write:

```rust
#![forbid(unsafe_code)]
```

- [ ] **Step 6: Verify workspace discovery**

Run:

```powershell
cargo metadata --format-version 1 --no-deps
```

Expected: command exits `0` and output includes `"unified_ui"`.

- [ ] **Step 7: Commit scaffold**

Run:

```powershell
git add Cargo.toml .gitignore README.md crates
git commit -m "chore: scaffold unified ui workspace"
```

Expected:

```text
[main
```

## Task 2: Theme Tokens

**Files:**
- Modify: `crates/ui-tokens/src/lib.rs`
- Create: `crates/ui-tokens/tests/theme.rs`

- [ ] **Step 1: Write failing token tests**

Write `crates/ui-tokens/tests/theme.rs`:

```rust
use ui_tokens::{Color, Density, Theme, ThemeMode, TransparencyPreference};

#[test]
fn default_theme_has_apple_like_surface_bias() {
    let theme = Theme::default();

    assert_eq!(theme.mode, ThemeMode::Light);
    assert_eq!(theme.density, Density::Comfortable);
    assert_eq!(theme.radius.medium_px, 10.0);
    assert_eq!(theme.transparency, TransparencyPreference::Allow);
    assert_eq!(theme.semantic.background.css_rgba(), "rgba(246, 247, 249, 1.000)");
}

#[test]
fn colors_are_css_serializable() {
    let color = Color::rgba(12, 34, 56, 0.375);

    assert_eq!(color.css_rgba(), "rgba(12, 34, 56, 0.375)");
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```powershell
cargo test -p ui-tokens
```

Expected: FAIL with unresolved imports from `ui_tokens`.

- [ ] **Step 3: Implement theme tokens**

Replace `crates/ui-tokens/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn css_rgba(self) -> String {
        format!("rgba({}, {}, {}, {:.3})", self.r, self.g, self.b, self.a)
    }

    pub fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Density {
    Compact,
    Comfortable,
    Spacious,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionPreference {
    Allow,
    Reduce,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransparencyPreference {
    Allow,
    Reduce,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SemanticColors {
    pub background: Color,
    pub surface: Color,
    pub surface_solid: Color,
    pub foreground: Color,
    pub muted_foreground: Color,
    pub border: Color,
    pub primary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub info: Color,
    pub focus: Color,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RadiusScale {
    pub small_px: f32,
    pub medium_px: f32,
    pub large_px: f32,
    pub floating_px: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpacingScale {
    pub xs_px: f32,
    pub sm_px: f32,
    pub md_px: f32,
    pub lg_px: f32,
    pub xl_px: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionScale {
    pub fast_ms: u32,
    pub normal_ms: u32,
    pub slow_ms: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub mode: ThemeMode,
    pub density: Density,
    pub semantic: SemanticColors,
    pub radius: RadiusScale,
    pub spacing: SpacingScale,
    pub motion: MotionScale,
    pub transparency: TransparencyPreference,
    pub motion_preference: MotionPreference,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Light,
            density: Density::Comfortable,
            semantic: SemanticColors {
                background: Color::rgba(246, 247, 249, 1.0),
                surface: Color::rgba(255, 255, 255, 0.78),
                surface_solid: Color::rgba(255, 255, 255, 1.0),
                foreground: Color::rgba(20, 23, 28, 1.0),
                muted_foreground: Color::rgba(86, 94, 108, 1.0),
                border: Color::rgba(120, 132, 150, 0.24),
                primary: Color::rgba(0, 102, 204, 1.0),
                success: Color::rgba(36, 138, 61, 1.0),
                warning: Color::rgba(176, 105, 0, 1.0),
                danger: Color::rgba(196, 43, 43, 1.0),
                info: Color::rgba(20, 118, 191, 1.0),
                focus: Color::rgba(0, 122, 255, 1.0),
            },
            radius: RadiusScale {
                small_px: 6.0,
                medium_px: 10.0,
                large_px: 14.0,
                floating_px: 18.0,
            },
            spacing: SpacingScale {
                xs_px: 4.0,
                sm_px: 8.0,
                md_px: 12.0,
                lg_px: 16.0,
                xl_px: 24.0,
            },
            motion: MotionScale {
                fast_ms: 120,
                normal_ms: 180,
                slow_ms: 260,
            },
            transparency: TransparencyPreference::Allow,
            motion_preference: MotionPreference::Allow,
        }
    }
}
```

- [ ] **Step 4: Run token tests**

Run:

```powershell
cargo test -p ui-tokens
```

Expected: PASS, `2 passed`.

- [ ] **Step 5: Commit tokens**

Run:

```powershell
git add crates/ui-tokens
git commit -m "feat: add semantic theme tokens"
```

## Task 3: Glass Material Recipes

**Files:**
- Modify: `crates/ui-glass/src/lib.rs`
- Create: `crates/ui-glass/tests/materials.rs`

- [ ] **Step 1: Write failing glass tests**

Write `crates/ui-glass/tests/materials.rs`:

```rust
use ui_glass::{resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRequest, GlassTone};
use ui_tokens::{Theme, TransparencyPreference};

#[test]
fn floating_glass_uses_backdrop_blur_by_default() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(GlassLevel::Floating, GlassTone::Neutral, GlassDensity::Comfortable),
    );

    assert_eq!(recipe.backdrop_blur_px, 18.0);
    assert_eq!(recipe.saturate_percent, 160);
    assert!(!recipe.force_solid);
}

#[test]
fn reduced_transparency_forces_solid_recipe() {
    let mut theme = Theme::default();
    theme.transparency = TransparencyPreference::Reduce;

    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(GlassLevel::Overlay, GlassTone::Primary, GlassDensity::Comfortable),
    );

    assert_eq!(recipe.backdrop_blur_px, 0.0);
    assert!(recipe.force_solid);
    assert_eq!(recipe.background.css_rgba(), theme.semantic.surface_solid.css_rgba());
}

#[test]
fn explicit_solid_policy_overrides_blur() {
    let theme = Theme::default();
    let request = GlassRequest::new(GlassLevel::Chrome, GlassTone::Neutral, GlassDensity::Compact)
        .with_policy(GlassPolicy::SolidFallback);

    let recipe = resolve_glass(&theme, request);

    assert!(recipe.force_solid);
    assert_eq!(recipe.backdrop_blur_px, 0.0);
}
```

- [ ] **Step 2: Run glass tests to verify failure**

Run:

```powershell
cargo test -p ui-glass
```

Expected: FAIL with unresolved imports from `ui_glass`.

- [ ] **Step 3: Implement glass recipes**

Replace `crates/ui-glass/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

use ui_tokens::{Color, Theme, TransparencyPreference};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassLevel {
    Subtle,
    Floating,
    Overlay,
    Chrome,
}

impl Default for GlassLevel {
    fn default() -> Self {
        Self::Subtle
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassTone {
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

impl Default for GlassTone {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassDensity {
    Compact,
    Comfortable,
    Spacious,
}

impl Default for GlassDensity {
    fn default() -> Self {
        Self::Comfortable
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassPolicy {
    Auto,
    SolidFallback,
    HighContrast,
    ReducedTransparency,
}

impl Default for GlassPolicy {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassRequest {
    pub level: GlassLevel,
    pub tone: GlassTone,
    pub density: GlassDensity,
    pub policy: GlassPolicy,
}

impl GlassRequest {
    pub const fn new(level: GlassLevel, tone: GlassTone, density: GlassDensity) -> Self {
        Self {
            level,
            tone,
            density,
            policy: GlassPolicy::Auto,
        }
    }

    pub const fn with_policy(mut self, policy: GlassPolicy) -> Self {
        self.policy = policy;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassRecipe {
    pub background: Color,
    pub fallback_background: Color,
    pub foreground: Color,
    pub border: Color,
    pub focus_ring: Color,
    pub inner_highlight: Color,
    pub shadow_alpha: f32,
    pub backdrop_blur_px: f32,
    pub saturate_percent: u16,
    pub radius_px: f32,
    pub force_solid: bool,
}

pub fn resolve_glass(theme: &Theme, request: GlassRequest) -> GlassRecipe {
    let force_solid = matches!(
        request.policy,
        GlassPolicy::SolidFallback | GlassPolicy::HighContrast | GlassPolicy::ReducedTransparency
    ) || theme.transparency == TransparencyPreference::Reduce;

    let tone = tone_color(theme, request.tone);
    let (blur, alpha, shadow_alpha) = match request.level {
        GlassLevel::Subtle => (10.0, 0.64, 0.10),
        GlassLevel::Floating => (18.0, 0.72, 0.16),
        GlassLevel::Overlay => (24.0, 0.80, 0.22),
        GlassLevel::Chrome => (28.0, 0.68, 0.18),
    };
    let radius_px = match request.density {
        GlassDensity::Compact => theme.radius.small_px,
        GlassDensity::Comfortable => theme.radius.medium_px,
        GlassDensity::Spacious => theme.radius.large_px,
    };

    GlassRecipe {
        background: if force_solid {
            theme.semantic.surface_solid
        } else {
            tone.with_alpha(alpha)
        },
        fallback_background: theme.semantic.surface_solid,
        foreground: theme.semantic.foreground,
        border: theme.semantic.border,
        focus_ring: theme.semantic.focus,
        inner_highlight: Color::rgba(255, 255, 255, if force_solid { 0.0 } else { 0.38 }),
        shadow_alpha,
        backdrop_blur_px: if force_solid { 0.0 } else { blur },
        saturate_percent: if force_solid { 100 } else { 160 },
        radius_px,
        force_solid,
    }
}

fn tone_color(theme: &Theme, tone: GlassTone) -> Color {
    match tone {
        GlassTone::Neutral => theme.semantic.surface,
        GlassTone::Primary => theme.semantic.primary,
        GlassTone::Success => theme.semantic.success,
        GlassTone::Warning => theme.semantic.warning,
        GlassTone::Danger => theme.semantic.danger,
        GlassTone::Info => theme.semantic.info,
    }
}
```

- [ ] **Step 4: Run glass tests**

Run:

```powershell
cargo test -p ui-glass
```

Expected: PASS, `3 passed`.

- [ ] **Step 5: Commit glass recipes**

Run:

```powershell
git add crates/ui-glass
git commit -m "feat: add glass material recipes"
```

## Task 4: Semantic Core Contracts

**Files:**
- Modify: `crates/ui-core/src/lib.rs`
- Create: `crates/ui-core/tests/contracts.rs`

- [ ] **Step 1: Write failing contract tests**

Write `crates/ui-core/tests/contracts.rs`:

```rust
use ui_core::{A11yContract, ComponentContract, ComponentRole, FocusPolicy, TargetSize};

#[test]
fn button_contract_requires_action_role_and_touch_target() {
    let contract = ComponentContract::button("save-button");

    assert_eq!(contract.id.as_str(), "save-button");
    assert_eq!(contract.a11y.role, ComponentRole::Button);
    assert_eq!(contract.target_size, TargetSize::minimum_touch());
    assert!(contract.validate().is_ok());
}

#[test]
fn unlabeled_interactive_contract_is_invalid() {
    let contract = ComponentContract {
        a11y: A11yContract {
            role: ComponentRole::Button,
            label: None,
            focus_policy: FocusPolicy::Focusable,
            modal: false,
        },
        ..ComponentContract::button("icon-only")
    };

    assert_eq!(contract.validate().unwrap_err(), "interactive component needs an accessible label");
}
```

- [ ] **Step 2: Run core tests to verify failure**

Run:

```powershell
cargo test -p ui-core
```

Expected: FAIL with unresolved imports from `ui_core`.

- [ ] **Step 3: Implement core contracts**

Replace `crates/ui-core/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentId(String);

impl ComponentId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentRole {
    Button,
    TextField,
    Checkbox,
    RadioGroup,
    Switch,
    Select,
    Combobox,
    Tabs,
    Dialog,
    Drawer,
    Popover,
    Tooltip,
    Menu,
    Table,
    List,
    Tree,
    Surface,
    Status,
}

impl ComponentRole {
    pub fn is_interactive(self) -> bool {
        !matches!(self, ComponentRole::Surface | ComponentRole::Status)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusPolicy {
    NotFocusable,
    Focusable,
    FocusTrap,
    RestoreOnClose,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct A11yContract {
    pub role: ComponentRole,
    pub label: Option<String>,
    pub focus_policy: FocusPolicy,
    pub modal: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TargetSize {
    pub min_width_px: f32,
    pub min_height_px: f32,
}

impl TargetSize {
    pub const fn minimum_touch() -> Self {
        Self {
            min_width_px: 44.0,
            min_height_px: 44.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentContract {
    pub id: ComponentId,
    pub a11y: A11yContract,
    pub target_size: TargetSize,
}

impl ComponentContract {
    pub fn button(id: impl Into<String>) -> Self {
        Self {
            id: ComponentId::new(id),
            a11y: A11yContract {
                role: ComponentRole::Button,
                label: Some("Button".to_string()),
                focus_policy: FocusPolicy::Focusable,
                modal: false,
            },
            target_size: TargetSize::minimum_touch(),
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.a11y.role.is_interactive() && self.a11y.label.as_deref().unwrap_or("").is_empty() {
            return Err("interactive component needs an accessible label");
        }

        if self.target_size.min_width_px < 24.0 || self.target_size.min_height_px < 24.0 {
            return Err("target size is too small for pointer interaction");
        }

        Ok(())
    }
}
```

- [ ] **Step 4: Run core tests**

Run:

```powershell
cargo test -p ui-core
```

Expected: PASS, `2 passed`.

- [ ] **Step 5: Commit core contracts**

Run:

```powershell
git add crates/ui-core
git commit -m "feat: add semantic component contracts"
```

## Task 5: Motion Core

**Files:**
- Modify: `crates/ui-motion/src/lib.rs`
- Create: `crates/ui-motion/tests/motion.rs`

- [ ] **Step 1: Write failing motion tests**

Write `crates/ui-motion/tests/motion.rs`:

```rust
use ui_motion::{PresenceState, Spring, Transition};

#[test]
fn reduced_motion_collapses_transition_duration() {
    let transition = Transition::tween(180).reduced();

    assert_eq!(transition.duration_ms(), 0);
}

#[test]
fn spring_step_moves_toward_target() {
    let spring = Spring::snappy();
    let value = spring.step(0.0, 10.0, 0.0, 1.0 / 60.0).value;

    assert!(value > 0.0);
    assert!(value < 10.0);
}

#[test]
fn presence_state_keeps_exit_lifecycle_explicit() {
    assert_eq!(PresenceState::Present.request_exit(), PresenceState::Exiting);
    assert_eq!(PresenceState::Exiting.finish_exit(), PresenceState::Removed);
}
```

- [ ] **Step 2: Run motion tests to verify failure**

Run:

```powershell
cargo test -p ui-motion
```

Expected: FAIL with unresolved imports from `ui_motion`.

- [ ] **Step 3: Implement motion primitives**

Replace `crates/ui-motion/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spring {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
}

impl Spring {
    pub const fn snappy() -> Self {
        Self {
            stiffness: 420.0,
            damping: 34.0,
            mass: 1.0,
        }
    }

    pub fn step(self, value: f32, target: f32, velocity: f32, delta_seconds: f32) -> SpringStep {
        let displacement = value - target;
        let spring_force = -self.stiffness * displacement;
        let damping_force = -self.damping * velocity;
        let acceleration = (spring_force + damping_force) / self.mass;
        let next_velocity = velocity + acceleration * delta_seconds;
        let next_value = value + next_velocity * delta_seconds;

        SpringStep {
            value: next_value,
            velocity: next_velocity,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringStep {
    pub value: f32,
    pub velocity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ease {
    Linear,
    Standard,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Transition {
    Tween { duration_ms: u32, ease: Ease },
    Spring(Spring),
}

impl Transition {
    pub const fn tween(duration_ms: u32) -> Self {
        Self::Tween {
            duration_ms,
            ease: Ease::Standard,
        }
    }

    pub const fn spring(spring: Spring) -> Self {
        Self::Spring(spring)
    }

    pub const fn reduced(self) -> Self {
        match self {
            Self::Tween { ease, .. } => Self::Tween {
                duration_ms: 0,
                ease,
            },
            Self::Spring(_) => Self::Tween {
                duration_ms: 0,
                ease: Ease::Linear,
            },
        }
    }

    pub const fn duration_ms(self) -> u32 {
        match self {
            Self::Tween { duration_ms, .. } => duration_ms,
            Self::Spring(_) => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenceState {
    Present,
    Exiting,
    Removed,
}

impl PresenceState {
    pub const fn request_exit(self) -> Self {
        match self {
            Self::Present => Self::Exiting,
            Self::Exiting | Self::Removed => self,
        }
    }

    pub const fn finish_exit(self) -> Self {
        match self {
            Self::Exiting => Self::Removed,
            Self::Present | Self::Removed => self,
        }
    }
}
```

- [ ] **Step 4: Run motion tests**

Run:

```powershell
cargo test -p ui-motion
```

Expected: PASS, `3 passed`.

- [ ] **Step 5: Commit motion core**

Run:

```powershell
git add crates/ui-motion
git commit -m "feat: add portable motion primitives"
```

## Task 6: Layout FLIP Math

**Files:**
- Modify: `crates/ui-layout/src/lib.rs`
- Create: `crates/ui-layout/tests/flip.rs`

- [ ] **Step 1: Write failing layout tests**

Write `crates/ui-layout/tests/flip.rs`:

```rust
use ui_layout::{compute_flip, Rect};

#[test]
fn flip_delta_moves_from_last_box_back_to_first_box() {
    let first = Rect::new(10.0, 20.0, 100.0, 50.0);
    let last = Rect::new(30.0, 45.0, 200.0, 100.0);
    let delta = compute_flip(first, last);

    assert_eq!(delta.translate_x, -20.0);
    assert_eq!(delta.translate_y, -25.0);
    assert_eq!(delta.scale_x, 0.5);
    assert_eq!(delta.scale_y, 0.5);
}

#[test]
fn zero_sized_last_box_uses_identity_scale() {
    let first = Rect::new(0.0, 0.0, 100.0, 50.0);
    let last = Rect::new(0.0, 0.0, 0.0, 0.0);
    let delta = compute_flip(first, last);

    assert_eq!(delta.scale_x, 1.0);
    assert_eq!(delta.scale_y, 1.0);
}
```

- [ ] **Step 2: Run layout tests to verify failure**

Run:

```powershell
cargo test -p ui-layout
```

Expected: FAIL with unresolved imports from `ui_layout`.

- [ ] **Step 3: Implement layout math**

Replace `crates/ui-layout/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlipDelta {
    pub translate_x: f32,
    pub translate_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

pub fn compute_flip(first: Rect, last: Rect) -> FlipDelta {
    FlipDelta {
        translate_x: first.x - last.x,
        translate_y: first.y - last.y,
        scale_x: if last.width == 0.0 {
            1.0
        } else {
            first.width / last.width
        },
        scale_y: if last.height == 0.0 {
            1.0
        } else {
            first.height / last.height
        },
    }
}
```

- [ ] **Step 4: Run layout tests**

Run:

```powershell
cargo test -p ui-layout
```

Expected: PASS, `2 passed`.

- [ ] **Step 5: Commit layout core**

Run:

```powershell
git add crates/ui-layout
git commit -m "feat: add renderer neutral layout math"
```

## Task 7: DOM/WebView Style Adapter

**Files:**
- Modify: `crates/ui-dom/src/lib.rs`
- Create: `crates/ui-dom/tests/css.rs`

- [ ] **Step 1: Write failing DOM adapter tests**

Write `crates/ui-dom/tests/css.rs`:

```rust
use ui_dom::{glass_style, CssStyleWriter};
use ui_glass::{resolve_glass, GlassDensity, GlassLevel, GlassRequest, GlassTone};
use ui_tokens::Theme;

#[test]
fn style_writer_serializes_declarations() {
    let style = CssStyleWriter::new()
        .set("color", "red")
        .set("min-height", "44px")
        .to_inline_style();

    assert_eq!(style, "color:red;min-height:44px;");
}

#[test]
fn glass_style_uses_backdrop_filter_when_supported() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(GlassLevel::Floating, GlassTone::Neutral, GlassDensity::Comfortable),
    );
    let style = glass_style(&recipe, true);

    assert!(style.contains("backdrop-filter:blur(18px) saturate(160%);"));
    assert!(style.contains("background:rgba(255, 255, 255, 0.720);"));
}

#[test]
fn glass_style_uses_solid_background_without_backdrop_support() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(GlassLevel::Floating, GlassTone::Neutral, GlassDensity::Comfortable),
    );
    let style = glass_style(&recipe, false);

    assert!(!style.contains("backdrop-filter"));
    assert!(style.contains("background:rgba(255, 255, 255, 1.000);"));
}
```

- [ ] **Step 2: Run DOM tests to verify failure**

Run:

```powershell
cargo test -p ui-dom
```

Expected: FAIL with unresolved imports from `ui_dom`.

- [ ] **Step 3: Implement DOM style adapter**

Replace `crates/ui-dom/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

use ui_glass::GlassRecipe;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CssStyleWriter {
    declarations: Vec<(String, String)>,
}

impl CssStyleWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.declarations.push((name.into(), value.into()));
        self
    }

    pub fn to_inline_style(&self) -> String {
        self.declarations
            .iter()
            .map(|(name, value)| format!("{name}:{value};"))
            .collect()
    }
}

pub fn glass_style(recipe: &GlassRecipe, supports_backdrop_filter: bool) -> String {
    let mut writer = CssStyleWriter::new()
        .set(
            "background",
            if supports_backdrop_filter && !recipe.force_solid {
                recipe.background.css_rgba()
            } else {
                recipe.fallback_background.css_rgba()
            },
        )
        .set("border", format!("1px solid {}", recipe.border.css_rgba()))
        .set("color", recipe.foreground.css_rgba())
        .set("border-radius", format!("{}px", trim_float(recipe.radius_px)))
        .set(
            "box-shadow",
            format!("0 18px 42px rgba(20, 23, 28, {:.3})", recipe.shadow_alpha),
        );

    if supports_backdrop_filter && !recipe.force_solid && recipe.backdrop_blur_px > 0.0 {
        writer = writer.set(
            "backdrop-filter",
            format!(
                "blur({}px) saturate({}%)",
                trim_float(recipe.backdrop_blur_px),
                recipe.saturate_percent
            ),
        );
    }

    writer.to_inline_style()
}

fn trim_float(value: f32) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i32)
    } else {
        format!("{value:.2}")
    }
}
```

- [ ] **Step 4: Run DOM tests**

Run:

```powershell
cargo test -p ui-dom
```

Expected: PASS, `3 passed`.

- [ ] **Step 5: Commit DOM adapter**

Run:

```powershell
git add crates/ui-dom
git commit -m "feat: add dom glass style adapter"
```

## Task 8: Native Capability Adapter

**Files:**
- Modify: `crates/ui-native/src/lib.rs`
- Create: `crates/ui-native/tests/capabilities.rs`

- [ ] **Step 1: Write failing native adapter tests**

Write `crates/ui-native/tests/capabilities.rs`:

```rust
use ui_glass::{resolve_glass, GlassDensity, GlassLevel, GlassRequest, GlassTone};
use ui_native::{plan_native_glass, NativeCapabilities};
use ui_tokens::Theme;

#[test]
fn native_without_backdrop_sampling_uses_simulated_glass() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(GlassLevel::Chrome, GlassTone::Neutral, GlassDensity::Comfortable),
    );
    let plan = plan_native_glass(&recipe, NativeCapabilities::minimal());

    assert!(plan.uses_simulated_glass);
    assert!(!plan.uses_backdrop_blur);
    assert_eq!(plan.effective_blur_px, 0.0);
}

#[test]
fn native_with_filter_support_can_use_real_blur() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(GlassLevel::Chrome, GlassTone::Neutral, GlassDensity::Comfortable),
    );
    let plan = plan_native_glass(&recipe, NativeCapabilities::with_backdrop_filters());

    assert!(!plan.uses_simulated_glass);
    assert!(plan.uses_backdrop_blur);
    assert_eq!(plan.effective_blur_px, 28.0);
}
```

- [ ] **Step 2: Run native tests to verify failure**

Run:

```powershell
cargo test -p ui-native
```

Expected: FAIL with unresolved imports from `ui_native`.

- [ ] **Step 3: Implement native capability planning**

Replace `crates/ui-native/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

use ui_glass::GlassRecipe;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeCapabilities {
    pub backdrop_sampling: bool,
    pub filters: bool,
    pub elevation_shadows: bool,
}

impl NativeCapabilities {
    pub const fn minimal() -> Self {
        Self {
            backdrop_sampling: false,
            filters: false,
            elevation_shadows: true,
        }
    }

    pub const fn with_backdrop_filters() -> Self {
        Self {
            backdrop_sampling: true,
            filters: true,
            elevation_shadows: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativeGlassPlan {
    pub uses_backdrop_blur: bool,
    pub uses_simulated_glass: bool,
    pub effective_blur_px: f32,
}

pub fn plan_native_glass(
    recipe: &GlassRecipe,
    capabilities: NativeCapabilities,
) -> NativeGlassPlan {
    let can_blur = capabilities.backdrop_sampling && capabilities.filters && !recipe.force_solid;

    NativeGlassPlan {
        uses_backdrop_blur: can_blur,
        uses_simulated_glass: !can_blur,
        effective_blur_px: if can_blur {
            recipe.backdrop_blur_px
        } else {
            0.0
        },
    }
}
```

- [ ] **Step 4: Run native tests**

Run:

```powershell
cargo test -p ui-native
```

Expected: PASS, `2 passed`.

- [ ] **Step 5: Commit native adapter**

Run:

```powershell
git add crates/ui-native
git commit -m "feat: add native capability adapter"
```

## Task 9: Dioxus Semantic Components

**Files:**
- Modify: `crates/ui-dioxus/src/lib.rs`
- Create: `crates/ui-dioxus/tests/ssr.rs`

- [ ] **Step 1: Write failing SSR tests**

Write `crates/ui-dioxus/tests/ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::{Button, ButtonVariant, GlassSurface, Surface};
use ui_glass::{GlassLevel, GlassTone};

#[test]
fn button_renders_semantic_button() {
    let html = dioxus_ssr::render(rsx! {
        Button {
            variant: ButtonVariant::Primary,
            "Save"
        }
    });

    assert!(html.contains("<button"));
    assert!(html.contains("ui-button--primary"));
    assert!(html.contains("Save"));
}

#[test]
fn surface_renders_section_with_surface_class() {
    let html = dioxus_ssr::render(rsx! {
        Surface {
            "Panel"
        }
    });

    assert!(html.contains("<section"));
    assert!(html.contains("ui-surface"));
}

#[test]
fn glass_surface_uses_semantic_glass_attributes() {
    let html = dioxus_ssr::render(rsx! {
        GlassSurface {
            level: GlassLevel::Chrome,
            tone: GlassTone::Neutral,
            "Toolbar"
        }
    });

    assert!(html.contains("data-glass-level=\"chrome\""));
    assert!(html.contains("data-glass-tone=\"neutral\""));
}
```

- [ ] **Step 2: Run Dioxus tests to verify failure**

Run:

```powershell
cargo test -p ui-dioxus
```

Expected: FAIL with unresolved imports from `ui_dioxus`.

- [ ] **Step 3: Implement Dioxus component MVP**

Replace `crates/ui-dioxus/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

use dioxus::prelude::*;
use ui_glass::{GlassDensity, GlassLevel, GlassTone};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
}

impl ButtonVariant {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Primary => "ui-button ui-button--primary",
            Self::Secondary => "ui-button ui-button--secondary",
            Self::Ghost => "ui-button ui-button--ghost",
            Self::Danger => "ui-button ui-button--danger",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] disabled: bool,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: "{variant.class_name()}",
            disabled: disabled,
            type: "button",
            {children}
        }
    }
}

#[component]
pub fn Surface(children: Element) -> Element {
    rsx! {
        section {
            class: "ui-surface",
            {children}
        }
    }
}

#[component]
pub fn GlassSurface(
    #[props(default)] level: GlassLevel,
    #[props(default)] tone: GlassTone,
    #[props(default)] density: GlassDensity,
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

#[component]
pub fn Stack(
    #[props(default = "md".to_string())] gap: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "ui-stack ui-stack--gap-{gap}",
            {children}
        }
    }
}

pub const fn glass_level_name(level: GlassLevel) -> &'static str {
    match level {
        GlassLevel::Subtle => "subtle",
        GlassLevel::Floating => "floating",
        GlassLevel::Overlay => "overlay",
        GlassLevel::Chrome => "chrome",
    }
}

pub const fn glass_tone_name(tone: GlassTone) -> &'static str {
    match tone {
        GlassTone::Neutral => "neutral",
        GlassTone::Primary => "primary",
        GlassTone::Success => "success",
        GlassTone::Warning => "warning",
        GlassTone::Danger => "danger",
        GlassTone::Info => "info",
    }
}

pub const fn glass_density_name(density: GlassDensity) -> &'static str {
    match density {
        GlassDensity::Compact => "compact",
        GlassDensity::Comfortable => "comfortable",
        GlassDensity::Spacious => "spacious",
    }
}
```

- [ ] **Step 4: Run Dioxus tests**

Run:

```powershell
cargo test -p ui-dioxus
```

Expected: PASS, `3 passed`.

- [ ] **Step 5: Commit Dioxus components**

Run:

```powershell
git add crates/ui-dioxus
git commit -m "feat: add semantic dioxus components"
```

## Task 10: Optional Backend Boundaries

**Files:**
- Modify: `crates/ui-gsap/src/lib.rs`
- Create: `crates/ui-gsap/tests/backend.rs`
- Modify: `crates/ui-hyperframes/src/lib.rs`
- Create: `crates/ui-hyperframes/tests/export.rs`

- [ ] **Step 1: Write failing optional backend tests**

Write `crates/ui-gsap/tests/backend.rs`:

```rust
use ui_gsap::{GsapBackend, GsapCapability};

#[test]
fn gsap_backend_declares_web_only_capabilities() {
    let backend = GsapBackend::default();

    assert_eq!(backend.capabilities(), &[GsapCapability::Timeline, GsapCapability::Scroll, GsapCapability::Flip]);
    assert_eq!(backend.target(), "web");
}
```

Write `crates/ui-hyperframes/tests/export.rs`:

```rust
use ui_hyperframes::{Composition, RenderTrack};

#[test]
fn composition_exports_deterministic_metadata() {
    let composition = Composition::new("launch-demo", 1920, 1080, 240);
    let track = RenderTrack::from_composition(&composition);

    assert_eq!(track.composition_id, "launch-demo");
    assert_eq!(track.frame_count, 240);
    assert_eq!(track.aspect_ratio(), "16:9");
}
```

- [ ] **Step 2: Run optional backend tests to verify failure**

Run:

```powershell
cargo test -p ui-gsap -p ui-hyperframes
```

Expected: FAIL with unresolved imports from `ui_gsap` and `ui_hyperframes`.

- [ ] **Step 3: Implement GSAP backend boundary**

Replace `crates/ui-gsap/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GsapCapability {
    Timeline,
    Scroll,
    Flip,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GsapBackend;

impl GsapBackend {
    pub const fn target(&self) -> &'static str {
        "web"
    }

    pub const fn capabilities(&self) -> &'static [GsapCapability] {
        &[
            GsapCapability::Timeline,
            GsapCapability::Scroll,
            GsapCapability::Flip,
        ]
    }
}
```

- [ ] **Step 4: Implement HyperFrames export boundary**

Replace `crates/ui-hyperframes/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Composition {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
}

impl Composition {
    pub fn new(id: impl Into<String>, width: u32, height: u32, frame_count: u32) -> Self {
        Self {
            id: id.into(),
            width,
            height,
            frame_count,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderTrack {
    pub composition_id: String,
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
}

impl RenderTrack {
    pub fn from_composition(composition: &Composition) -> Self {
        Self {
            composition_id: composition.id.clone(),
            width: composition.width,
            height: composition.height,
            frame_count: composition.frame_count,
        }
    }

    pub fn aspect_ratio(&self) -> &'static str {
        match (self.width, self.height) {
            (1920, 1080) | (1280, 720) => "16:9",
            (1080, 1920) => "9:16",
            (1080, 1080) => "1:1",
            _ => "custom",
        }
    }
}
```

- [ ] **Step 5: Run optional backend tests**

Run:

```powershell
cargo test -p ui-gsap -p ui-hyperframes
```

Expected: PASS, `2 passed`.

- [ ] **Step 6: Commit backend boundaries**

Run:

```powershell
git add crates/ui-gsap crates/ui-hyperframes
git commit -m "feat: add optional backend boundaries"
```

## Task 11: Public Facade And Prelude

**Files:**
- Modify: `crates/unified_ui/src/lib.rs`
- Create: `crates/unified_ui/tests/prelude.rs`

- [ ] **Step 1: Write failing prelude tests**

Write `crates/unified_ui/tests/prelude.rs`:

```rust
use unified_ui::prelude::*;

#[test]
fn prelude_exposes_semantic_components_and_tokens() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(GlassLevel::Floating, GlassTone::Neutral, GlassDensity::Comfortable),
    );

    assert_eq!(ButtonVariant::Primary.class_name(), "ui-button ui-button--primary");
    assert_eq!(recipe.backdrop_blur_px, 18.0);
}

#[test]
fn default_features_do_not_expose_gsap_or_hyperframes_names() {
    let public_names = unified_ui::public_api_names();

    assert!(!public_names.iter().any(|name| name.contains("Gsap")));
    assert!(!public_names.iter().any(|name| name.contains("HyperFrames")));
}
```

- [ ] **Step 2: Run facade tests to verify failure**

Run:

```powershell
cargo test -p unified_ui
```

Expected: FAIL with unresolved imports or missing prelude exports.

- [ ] **Step 3: Implement public facade**

Replace `crates/unified_ui/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

pub mod prelude {
    pub use ui_core::{
        A11yContract, ComponentContract, ComponentId, ComponentRole, FocusPolicy, TargetSize,
    };
    pub use ui_dioxus::{Button, ButtonVariant, GlassSurface, Stack, Surface};
    pub use ui_glass::{
        resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRecipe, GlassRequest, GlassTone,
    };
    pub use ui_layout::{compute_flip, FlipDelta, Rect};
    pub use ui_motion::{Ease, PresenceState, Spring, SpringStep, Transition};
    pub use ui_tokens::{
        Color, Density, MotionPreference, MotionScale, RadiusScale, SemanticColors, SpacingScale,
        Theme, ThemeMode, TransparencyPreference,
    };

    #[cfg(any(feature = "web", feature = "desktop", feature = "mobile"))]
    pub use ui_dom::{glass_style, CssStyleWriter};

    #[cfg(feature = "native")]
    pub use ui_native::{plan_native_glass, NativeCapabilities, NativeGlassPlan};
}

pub fn public_api_names() -> &'static [&'static str] {
    &[
        "Button",
        "IconButton",
        "TextField",
        "Checkbox",
        "Tabs",
        "Dialog",
        "Toast",
        "Surface",
        "GlassSurface",
        "Presence",
        "Transition",
        "Sequence",
        "SharedLayout",
        "SharedElement",
    ]
}

#[cfg(feature = "gsap")]
pub mod gsap {
    pub use ui_gsap::{GsapBackend, GsapCapability};
}

#[cfg(feature = "hyperframes-export")]
pub mod hyperframes {
    pub use ui_hyperframes::{Composition, RenderTrack};
}
```

- [ ] **Step 4: Run facade tests**

Run:

```powershell
cargo test -p unified_ui
```

Expected: PASS, `2 passed`.

- [ ] **Step 5: Verify optional features compile**

Run:

```powershell
cargo test -p unified_ui --no-default-features --features "native gsap hyperframes-export"
```

Expected: PASS.

- [ ] **Step 6: Commit public facade**

Run:

```powershell
git add crates/unified_ui
git commit -m "feat: add unified public facade"
```

## Task 12: Documentation And MVP Verification

**Files:**
- Modify: `README.md`
- Create: `docs/component-naming.md`
- Create: `docs/glass-materials.md`
- Create: `docs/platform-support.md`

- [ ] **Step 1: Add component naming docs**

Write `docs/component-naming.md`:

```markdown
# Component Naming

Unified UI uses semantic component names.

Names describe the user-facing role or behavior:

- `Button`
- `TextField`
- `Tabs`
- `Dialog`
- `Toast`
- `Surface`
- `GlassSurface`
- `Presence`
- `SharedLayout`

Public names do not borrow library, framework, or platform brand names.
Backend names appear only behind optional backend modules.
```

- [ ] **Step 2: Add glass material docs**

Write `docs/glass-materials.md`:

```markdown
# Glass Materials

Glass is represented through tokenized material recipes.

Material axes:

- `GlassLevel`
- `GlassTone`
- `GlassDensity`
- `GlassPolicy`

Web, Desktop, and Mobile use `backdrop-filter` where supported. When
backdrop filtering is unavailable, the same recipe resolves to a solid
fallback. Native targets use the same recipe and map it to available
renderer capabilities.

Text and icons are validated against fallback surfaces, not only ideal
blurred surfaces.
```

- [ ] **Step 3: Add platform support docs**

Write `docs/platform-support.md`:

```markdown
# Platform Support

| Target | Status | Backend |
|---|---|---|
| Web | MVP | DOM style adapter |
| Desktop | MVP | WebView DOM style adapter |
| Mobile | MVP | WebView DOM style adapter |
| Native | MVP contract | Native capability adapter |

Native support begins with semantic parity, token rendering, glass fallback,
focus behavior, and basic motion planning. Native visual fidelity depends on
available Dioxus Native and Blitz renderer capabilities.

GSAP and HyperFrames are optional integrations and are not default runtime
features.
```

- [ ] **Step 4: Update README with usage**

Replace `README.md`:

```markdown
# Unified UI

Unified UI is a Dioxus-first UI library for downstream SaaS products.

The library exposes one public crate:

```rust
use unified_ui::prelude::*;
```

Design principles:

- semantic component names
- Apple-like glass materials with solid fallbacks
- Web, Desktop, Mobile, and Native adapter contracts
- accessibility and reduced-preference policies
- WCAG 2.2 AA target for default themes
- optional GSAP and HyperFrames integrations outside default features

## Features

Default features:

- `web`
- `desktop`
- `mobile`
- `tokens`
- `glass`
- `motion`
- `layout-motion`
- `a11y`

Optional features:

- `native`
- `gsap`
- `hyperframes-export`
- `a11y-tests`

## First Components

- `Button`
- `Surface`
- `GlassSurface`
- `Stack`

## Documentation

- `docs/component-naming.md`
- `docs/glass-materials.md`
- `docs/platform-support.md`
```

- [ ] **Step 5: Run full verification**

Run:

```powershell
cargo fmt --all -- --check
cargo test --workspace
cargo test -p unified_ui --no-default-features --features "native gsap hyperframes-export"
```

Expected: all commands exit `0`.

- [ ] **Step 6: Commit docs and verification support**

Run:

```powershell
git add README.md docs crates
git commit -m "docs: document unified ui mvp"
```

## Task 13: Final Acceptance Checklist

**Files:**
- Modify only if verification exposes a concrete defect in a previous task.

- [ ] **Step 1: Check public API naming**

Run:

```powershell
rg -n "Radix|Shadcn|Fluent|Material Design|Framer|AnimatePresence|LayoutGroup|motion::" crates README.md docs/component-naming.md docs/glass-materials.md docs/platform-support.md
```

Expected: no matches.

- [ ] **Step 2: Check default feature boundaries**

Run:

```powershell
cargo tree -p unified_ui --edges features
```

Expected: default feature output does not enable `ui-gsap` or `ui-hyperframes`.

- [ ] **Step 3: Check Native feature compiles separately**

Run:

```powershell
cargo test -p unified_ui --no-default-features --features native
```

Expected: PASS.

- [ ] **Step 4: Check Web/Desktop/Mobile DOM path compiles through defaults**

Run:

```powershell
cargo test -p unified_ui
```

Expected: PASS.

- [ ] **Step 5: Commit any acceptance fixes**

If no files changed, run:

```powershell
git status --short
```

Expected: no output.

If files changed to fix a concrete acceptance failure, run:

```powershell
git add crates docs README.md Cargo.toml
git commit -m "fix: satisfy unified ui mvp acceptance"
```

Expected:

```text
[main
```

## Post-MVP Follow-Up Plans

Write separate plans after this MVP lands:

1. Full component suite: text fields, choices, navigation, overlays, feedback, data display, and layout primitives.
2. Advanced motion: timelines, stagger orchestration, shared element transitions, mounted-element controllers.
3. Playwright and visual regression: Web/Desktop/Mobile preview routes, glass fallback checks, reduced preference checks.
4. Native fidelity: Blitz-specific rendering, focus parity, input handling, and visual comparison.
5. GSAP backend: local bridge, cleanup lifecycle, scroll choreography, path animation, and Flip.
6. HyperFrames exporter: composition generation, frame tracks, deterministic render commands, demo assets.
