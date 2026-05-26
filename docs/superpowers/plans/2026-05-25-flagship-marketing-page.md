# Flagship Marketing Page Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship `examples/flagship/`, a self-referential, single-page marketing site that hosts existing scenes at full bleed and exposes the workspace's Apple-quality primitives outside the gallery's documentation chrome.

**Architecture:** New `examples/flagship/` binary crate that path-deps on `component-gallery` (after making its `previews` module public). Five sections composed top-to-bottom in `src/app.rs`, each in its own `src/sections/*.rs` module. A single `FLAGSHIP_CSS` constant adds a display-tier type ramp, a half-step warmer `--ui-primary` accent, an ambient backdrop, and per-section layout primitives on top of `library_css()`. Playwright spec asserts zero gallery-shell markers and that the hero scene renders.

**Tech Stack:** Rust + Dioxus 0.7, kinetics workspace crates (`kinetics`, `ui-styles`, `ui-blocks`, `ui-glass-engine` via `GlassSurface`), Playwright + `http-server` for e2e, `chrome-devtools-mcp` for the manual hero-screenshot artifact.

**Spec:** `docs/superpowers/specs/2026-05-25-flagship-marketing-page-design.md` (commit `13a781a`).

---

## Task 1: Expose `previews` and `persistence` from component-gallery

The flagship reuses two pieces of gallery code: scene components from `previews/scenes/*` (`ProductIntroScene`, `ScrollPinnedStoryScene`, `MetricCounterScene`), and the OS-reduced-motion helper from `persistence` (the gallery already has a tested `match_media` reader with a native fallback). Both modules are currently private. Two-line change to make them reachable.

**Files:**
- Modify: `examples/component-gallery/src/lib.rs:9-10`

- [ ] **Step 1: Make both modules public**

Edit `examples/component-gallery/src/lib.rs`. The current file reads:

```rust
#![forbid(unsafe_code)]

mod app;
mod brand;
pub mod controls;
pub mod demo_frame;
mod docs;
mod persistence;
mod previews;
mod styles;

pub use app::App;
pub use docs::{categories, component_docs, ComponentCategory, ComponentDoc, ComponentStatus};
```

Change `mod persistence;` to `pub mod persistence;` and `mod previews;` to `pub mod previews;`. The file becomes:

```rust
#![forbid(unsafe_code)]

mod app;
mod brand;
pub mod controls;
pub mod demo_frame;
mod docs;
pub mod persistence;
pub mod previews;
mod styles;

pub use app::App;
pub use docs::{categories, component_docs, ComponentCategory, ComponentDoc, ComponentStatus};
```

- [ ] **Step 2: Verify the workspace still builds**

Run: `cargo check -p component-gallery`
Expected: clean build, no warnings about the visibility change.

- [ ] **Step 3: Verify the modules are reachable**

Run: `cargo check -p component-gallery --tests`
Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add examples/component-gallery/src/lib.rs
git commit -m "chore(gallery): make previews and persistence pub so the flagship can reuse them"
```

---

## Task 2: Scaffold the `examples/flagship/` package

Create a binary-only Dioxus crate with empty `App` so we can verify wiring and the workspace recognises it before adding any content.

**Files:**
- Create: `examples/flagship/Cargo.toml`
- Create: `examples/flagship/Dioxus.toml`
- Create: `examples/flagship/src/main.rs`
- Create: `examples/flagship/src/lib.rs`
- Create: `examples/flagship/src/app.rs`
- Create: `examples/flagship/src/styles.rs`
- Create: `examples/flagship/src/sections/mod.rs`
- Modify: `Cargo.toml` (workspace) — add `examples/flagship` to `members`.

- [ ] **Step 1: Create the package `Cargo.toml`**

Path: `examples/flagship/Cargo.toml`

```toml
[package]
name = "flagship"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
dioxus.workspace = true
kinetics.workspace = true
ui-styles.workspace = true
ui-glass.workspace = true
ui-glass-dioxus.workspace = true
ui-glass-engine.workspace = true
ui-tokens.workspace = true
ui-runtime.workspace = true
ui-composition.workspace = true
ui-motion.workspace = true
ui-timeline.workspace = true
ui-blocks = { path = "../../crates/ui-blocks" }
component-gallery = { path = "../component-gallery" }

[features]
web = ["dioxus/web"]

[target.'cfg(target_arch = "wasm32")'.dependencies]
web-sys = { version = "0.3", features = ["Window"] }
wasm-bindgen = "0.2"
js-sys = "0.3"

[lib]
path = "src/lib.rs"

[[bin]]
name = "flagship"
path = "src/main.rs"
```

- [ ] **Step 2: Create `Dioxus.toml`**

Path: `examples/flagship/Dioxus.toml`

```toml
[application]
out_dir = "dist"
asset_dir = "public"

[web.app]
title = "Kinetics — Composable motion for Rust apps"

[web.watcher]
index_on_404 = true
watch_path = ["src"]

[web.wasm_opt]
level = "4"
```

- [ ] **Step 3: Create `src/main.rs`**

Path: `examples/flagship/src/main.rs`

```rust
fn main() {
    dioxus::launch(flagship::App);
}
```

- [ ] **Step 4: Create `src/lib.rs`**

Path: `examples/flagship/src/lib.rs`

```rust
#![forbid(unsafe_code)]

mod app;
mod sections;
mod styles;

pub use app::App;
```

- [ ] **Step 5: Create `src/styles.rs` with an empty placeholder constant**

Path: `examples/flagship/src/styles.rs`

```rust
pub const FLAGSHIP_CSS: &str = "";
```

The real CSS lands in Task 8.

- [ ] **Step 6: Create `src/sections/mod.rs` with no sections wired yet**

Path: `examples/flagship/src/sections/mod.rs`

```rust
// Section modules land in later tasks. Each one renders a single
// full-bleed section of the flagship marketing page.
```

- [ ] **Step 7: Create `src/app.rs` with a placeholder `App`**

Path: `examples/flagship/src/app.rs`

```rust
use dioxus::prelude::*;
use ui_styles::library_css;

use crate::styles::FLAGSHIP_CSS;

#[component]
pub fn App() -> Element {
    let shared = library_css();

    rsx! {
        style { "{shared}" }
        style { "{FLAGSHIP_CSS}" }
        main { class: "flagship-shell",
            p { "Flagship under construction." }
        }
    }
}
```

- [ ] **Step 8: Add the new package to the workspace**

Open `Cargo.toml` at the workspace root. Find the `members = [...]` array (starts at line 3). After the `"examples/component-gallery",` entry add `"examples/flagship",` on its own line. The block becomes:

```toml
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
    "crates/kinetics",
    "crates/kinetics-cli",
    "crates/kinetics-render",
    "crates/ui-runtime",
    "crates/ui-icons",
    "crates/ui-blocks",
    "examples/component-gallery",
    "examples/flagship",
    "crates/ui-glass-engine",
    "crates/ui-glass-dioxus",
]
```

- [ ] **Step 9: Build the empty package**

Run: `cargo check -p flagship`
Expected: clean build.

- [ ] **Step 10: Build the whole workspace**

Run: `cargo check --workspace`
Expected: clean build.

- [ ] **Step 11: Commit**

```bash
git add Cargo.toml examples/flagship
git commit -m "feat(flagship): scaffold examples/flagship package with empty App"
```

---

## Task 3: Hero section — full-bleed `ProductIntroScene`

Wrap the existing 10-second autoplay hero film in a full-viewport shell with no transport controls. This is the section the Hero-3-seconds binding check will evaluate.

**Files:**
- Create: `examples/flagship/src/sections/hero.rs`
- Modify: `examples/flagship/src/sections/mod.rs`
- Modify: `examples/flagship/src/app.rs`

- [ ] **Step 1: Create the hero section module**

Path: `examples/flagship/src/sections/hero.rs`

```rust
use dioxus::prelude::*;
use component_gallery::previews::scenes::product_intro::ProductIntroScene;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "flagship-hero", aria_label: "Kinetics product introduction",
            div { class: "flagship-hero-stage",
                ProductIntroScene {}
            }
        }
    }
}
```

- [ ] **Step 2: Re-export the section from the sections module**

Edit `examples/flagship/src/sections/mod.rs`. Replace the placeholder comment with:

```rust
pub mod hero;
```

- [ ] **Step 3: Mount the hero in `App`**

Edit `examples/flagship/src/app.rs`. Replace the placeholder `main` body so the file reads:

```rust
use dioxus::prelude::*;
use ui_styles::library_css;

use crate::sections::hero::Hero;
use crate::styles::FLAGSHIP_CSS;

#[component]
pub fn App() -> Element {
    let shared = library_css();

    rsx! {
        style { "{shared}" }
        style { "{FLAGSHIP_CSS}" }
        main { class: "flagship-shell",
            Hero {}
        }
    }
}
```

- [ ] **Step 4: Build the package**

Run: `cargo check -p flagship`
Expected: clean build.

- [ ] **Step 5: Commit**

```bash
git add examples/flagship
git commit -m "feat(flagship): mount ProductIntroScene as full-bleed hero section"
```

---

## Task 4: Scroll-driven product story section

Lift `ScrollPinnedStoryScene` (10-second sticky scroll-driven timeline) out of its 148 px gallery preview into a full-bleed slot. The scene already provides its own `200vh` trigger and inner `position: sticky` shell, so the flagship adds no extra scroll plumbing.

**Files:**
- Create: `examples/flagship/src/sections/story.rs`
- Modify: `examples/flagship/src/sections/mod.rs`
- Modify: `examples/flagship/src/app.rs`

- [ ] **Step 1: Create the story section module**

Path: `examples/flagship/src/sections/story.rs`

```rust
use dioxus::prelude::*;
use component_gallery::previews::scenes::scroll_story::ScrollPinnedStoryScene;

#[component]
pub fn Story() -> Element {
    rsx! {
        section { class: "flagship-story", aria_label: "Scroll-driven product story",
            ScrollPinnedStoryScene {}
        }
    }
}
```

- [ ] **Step 2: Re-export the section**

Edit `examples/flagship/src/sections/mod.rs`. Append after the existing `pub mod hero;`:

```rust
pub mod story;
```

The file now reads:

```rust
pub mod hero;
pub mod story;
```

- [ ] **Step 3: Mount the story section in `App`**

Edit `examples/flagship/src/app.rs`. The `use` block grows by one line and the `main` body gains a sibling. The file now reads:

```rust
use dioxus::prelude::*;
use ui_styles::library_css;

use crate::sections::hero::Hero;
use crate::sections::story::Story;
use crate::styles::FLAGSHIP_CSS;

#[component]
pub fn App() -> Element {
    let shared = library_css();

    rsx! {
        style { "{shared}" }
        style { "{FLAGSHIP_CSS}" }
        main { class: "flagship-shell",
            Hero {}
            Story {}
        }
    }
}
```

- [ ] **Step 4: Build the package**

Run: `cargo check -p flagship`
Expected: clean build.

- [ ] **Step 5: Commit**

```bash
git add examples/flagship
git commit -m "feat(flagship): add scroll-driven product story section"
```

---

## Task 5: Glass feature triplet section

Three `GlassSurface` cards in a 3-up grid on desktop, vertical stack on mobile. Glass tones: `Info` / `Primary` / `Success`. Level: `Floating`. This is the section that demands the WebGPU `ui-glass-engine` path; the colorful ambient gradient gives the material something to refract.

**Files:**
- Create: `examples/flagship/src/sections/features.rs`
- Modify: `examples/flagship/src/sections/mod.rs`
- Modify: `examples/flagship/src/app.rs`

- [ ] **Step 1: Create the features section module**

Path: `examples/flagship/src/sections/features.rs`

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn Features() -> Element {
    rsx! {
        section { class: "flagship-features", aria_labelledby: "flagship-features-heading",
            div { class: "flagship-features-inner",
                p { class: "flagship-eyebrow", "Three pillars" }
                h2 { id: "flagship-features-heading", class: "flagship-display-2",
                    "Glass. Scenes. Render."
                }
                div { class: "flagship-features-grid",
                    GlassSurface {
                        level: GlassLevel::Floating,
                        tone: GlassTone::Info,
                        density: GlassDensity::Comfortable,
                        h3 { class: "flagship-card-title", "Liquid glass. Honestly rendered." }
                        p { class: "flagship-card-body",
                            "WebGPU when it's available. SVG filter fallback. Solid fallback when accessibility says so."
                        }
                    }
                    GlassSurface {
                        level: GlassLevel::Floating,
                        tone: GlassTone::Primary,
                        density: GlassDensity::Comfortable,
                        h3 { class: "flagship-card-title", "One clock. Every runtime." }
                        p { class: "flagship-card-body",
                            "Scene owns the time. Clip, SplitText, MotionPath, presence, and shared-element layout all read from it."
                        }
                    }
                    GlassSurface {
                        level: GlassLevel::Floating,
                        tone: GlassTone::Success,
                        density: GlassDensity::Comfortable,
                        h3 { class: "flagship-card-title", "Frame-perfect render." }
                        p { class: "flagship-card-body",
                            "kinetics render walks any scene with SceneDriver::Manual, writes per-frame HTML, ships a manifest, and optionally encodes PNG or MP4."
                        }
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 2: Re-export the section**

Edit `examples/flagship/src/sections/mod.rs`. Append:

```rust
pub mod features;
```

The file now reads:

```rust
pub mod hero;
pub mod story;
pub mod features;
```

- [ ] **Step 3: Mount the features section in `App`**

Edit `examples/flagship/src/app.rs`. Add the import and the sibling render:

```rust
use dioxus::prelude::*;
use ui_styles::library_css;

use crate::sections::features::Features;
use crate::sections::hero::Hero;
use crate::sections::story::Story;
use crate::styles::FLAGSHIP_CSS;

#[component]
pub fn App() -> Element {
    let shared = library_css();

    rsx! {
        style { "{shared}" }
        style { "{FLAGSHIP_CSS}" }
        main { class: "flagship-shell",
            Hero {}
            Story {}
            Features {}
        }
    }
}
```

- [ ] **Step 4: Build the package**

Run: `cargo check -p flagship`
Expected: clean build.

- [ ] **Step 5: Commit**

```bash
git add examples/flagship
git commit -m "feat(flagship): add glass feature triplet section"
```

---

## Task 6: Live metric strip section

Four `MetricCounter`s in a row, revealing on scroll-entry via the existing `TimelineScope` stagger that `MetricCounter` already uses internally. Honest numbers pulled from the README's public-API surface and platform-support table.

**Files:**
- Create: `examples/flagship/src/sections/metrics.rs`
- Modify: `examples/flagship/src/sections/mod.rs`
- Modify: `examples/flagship/src/app.rs`

- [ ] **Step 1: Create the metrics section module**

Path: `examples/flagship/src/sections/metrics.rs`

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn Metrics() -> Element {
    rsx! {
        section { class: "flagship-metrics", aria_labelledby: "flagship-metrics-heading",
            div { class: "flagship-metrics-inner",
                p { class: "flagship-eyebrow", "Honest numbers" }
                h2 { id: "flagship-metrics-heading", class: "flagship-display-2",
                    "Built to ship."
                }
                div { class: "flagship-metrics-grid",
                    MetricCounter {
                        label: "Components ready".to_string(),
                        value: "33".to_string(),
                        delta_text: Some("from the public prelude".to_string()),
                    }
                    MetricCounter {
                        label: "Frame target".to_string(),
                        value: "60 fps".to_string(),
                        delta_text: Some("scene clock + frame scheduler".to_string()),
                    }
                    MetricCounter {
                        label: "Platform adapters".to_string(),
                        value: "4".to_string(),
                        delta_text: Some("Web · Desktop · Mobile · Native".to_string()),
                    }
                    MetricCounter {
                        label: "Glass engine".to_string(),
                        value: "WebGPU".to_string(),
                        delta_text: Some("SVG and solid fallbacks built in".to_string()),
                    }
                }
            }
        }
    }
}
```

> Note. `33` matches the count of entries in the README "Ready rendered components" list (see `README.md` lines 23–57). If that list changes before this plan executes, update the value at write time — it's a number, not a design fork.

- [ ] **Step 2: Re-export the section**

Edit `examples/flagship/src/sections/mod.rs`. Append:

```rust
pub mod metrics;
```

The file now reads:

```rust
pub mod hero;
pub mod story;
pub mod features;
pub mod metrics;
```

- [ ] **Step 3: Mount the metrics section in `App`**

Edit `examples/flagship/src/app.rs`. Add the import and sibling render:

```rust
use dioxus::prelude::*;
use ui_styles::library_css;

use crate::sections::features::Features;
use crate::sections::hero::Hero;
use crate::sections::metrics::Metrics;
use crate::sections::story::Story;
use crate::styles::FLAGSHIP_CSS;

#[component]
pub fn App() -> Element {
    let shared = library_css();

    rsx! {
        style { "{shared}" }
        style { "{FLAGSHIP_CSS}" }
        main { class: "flagship-shell",
            Hero {}
            Story {}
            Features {}
            Metrics {}
        }
    }
}
```

- [ ] **Step 4: Build the package**

Run: `cargo check -p flagship`
Expected: clean build.

- [ ] **Step 5: Commit**

```bash
git add examples/flagship
git commit -m "feat(flagship): add live metric strip section"
```

---

## Task 7: CTA band + footer section

The flagship composes its own CTA band from `Button` primitives rather than reusing `CtaPulseScene` (which only ships one button). Two buttons side by side: primary "View on GitHub", ghost "Open the gallery". Below them, a single caption line. Minimal footer underneath with the kinetics brand mark and license note.

**Files:**
- Create: `examples/flagship/src/sections/cta.rs`
- Modify: `examples/flagship/src/sections/mod.rs`
- Modify: `examples/flagship/src/app.rs`

- [ ] **Step 1: Create the CTA section module**

Path: `examples/flagship/src/sections/cta.rs`

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn CallToAction() -> Element {
    rsx! {
        section { class: "flagship-cta", aria_labelledby: "flagship-cta-heading",
            div { class: "flagship-cta-inner",
                p { class: "flagship-eyebrow", "Start moving" }
                h2 { id: "flagship-cta-heading", class: "flagship-display-2",
                    "Drop kinetics into your next Dioxus app."
                }
                p { class: "flagship-cta-caption",
                    "Built in Rust. MIT licensed. Web, desktop, mobile, and native."
                }
                div { class: "flagship-cta-actions",
                    Button { variant: ButtonVariant::Primary, "View on GitHub" }
                    Button { variant: ButtonVariant::Ghost, "Open the gallery" }
                }
            }
            footer { class: "flagship-footer",
                p { class: "flagship-footer-brand", "dioxus-kinetics" }
                p { class: "flagship-footer-meta",
                    "MIT · v"
                    "{env!(\"CARGO_PKG_VERSION\")}"
                }
            }
        }
    }
}
```

- [ ] **Step 2: Re-export the section**

Edit `examples/flagship/src/sections/mod.rs`. Append:

```rust
pub mod cta;
```

The file now reads:

```rust
pub mod hero;
pub mod story;
pub mod features;
pub mod metrics;
pub mod cta;
```

- [ ] **Step 3: Mount the CTA section in `App`**

Edit `examples/flagship/src/app.rs`. Add the import and sibling render:

```rust
use dioxus::prelude::*;
use ui_styles::library_css;

use crate::sections::cta::CallToAction;
use crate::sections::features::Features;
use crate::sections::hero::Hero;
use crate::sections::metrics::Metrics;
use crate::sections::story::Story;
use crate::styles::FLAGSHIP_CSS;

#[component]
pub fn App() -> Element {
    let shared = library_css();

    rsx! {
        style { "{shared}" }
        style { "{FLAGSHIP_CSS}" }
        main { class: "flagship-shell",
            Hero {}
            Story {}
            Features {}
            Metrics {}
            CallToAction {}
        }
    }
}
```

- [ ] **Step 4: Build the package**

Run: `cargo check -p flagship`
Expected: clean build.

- [ ] **Step 5: Commit**

```bash
git add examples/flagship
git commit -m "feat(flagship): add CTA band and minimal footer"
```

---

## Task 8: Identity CSS — type ramp, accent inflection, ambient backdrop, section layout

Replace the empty `FLAGSHIP_CSS` placeholder with the real identity CSS. The CSS file contains:
1. A scoped `--ui-primary` override (`#0a7aff`, a half-step warmer than the library default `#0066cc`).
2. Three display-tier type variables (`--flagship-display-1/2`, `--flagship-eyebrow`) on top of the existing token ramp.
3. The ambient radial-mesh backdrop, reused from the gallery's `body::before` pattern.
4. Section-level layout primitives: hero (100 vh), story (no override, the scene provides its own), features (3-up grid + 1-col on mobile), metrics (4-up row + 2-col on mobile), CTA + footer (centered, max-width inner).
5. Reduced-motion suppression for the ambient drift.

**Files:**
- Modify: `examples/flagship/src/styles.rs`

- [ ] **Step 1: Replace the placeholder with the full CSS**

Path: `examples/flagship/src/styles.rs`. Replace the entire file with:

```rust
pub const FLAGSHIP_CSS: &str = r#"
.flagship-shell {
    --ui-primary: #0a7aff;
    --flagship-display-1: clamp(56px, 8vw, 96px);
    --flagship-display-2: clamp(40px, 5vw, 64px);
    --flagship-eyebrow: 13px;
    --flagship-section-pad-y: clamp(72px, 10vh, 144px);
    --flagship-section-pad-x: clamp(20px, 5vw, 88px);
    --flagship-content-max: 1180px;

    display: block;
    position: relative;
    isolation: isolate;
}

.flagship-shell::before {
    content: "";
    position: fixed;
    inset: -10vmax;
    z-index: -1;
    background:
        radial-gradient(closest-side at 18% 28%, color-mix(in srgb, var(--ui-primary), transparent 60%), transparent 70%),
        radial-gradient(closest-side at 78% 22%, color-mix(in srgb, var(--ui-info), transparent 62%), transparent 70%),
        radial-gradient(closest-side at 50% 82%, color-mix(in srgb, var(--ui-success), transparent 70%), transparent 70%),
        var(--ui-bg);
    filter: saturate(112%);
    animation: flagship-mesh-drift 48s linear infinite;
}

@keyframes flagship-mesh-drift {
    0%   { transform: translate3d(0, 0, 0); }
    50%  { transform: translate3d(-4%, -3%, 0); }
    100% { transform: translate3d(0, 0, 0); }
}

[data-ui-motion="reduced"] .flagship-shell::before,
[data-ui-theme="dark"] .flagship-shell::before {
    /* Dark theme uses its own backdrop drift via the existing GALLERY token
       set; the flagship inherits ui-bg and re-tints. Animation suppression
       under reduced motion mirrors the gallery contract. */
}

@media (prefers-reduced-motion: reduce) {
    .flagship-shell::before { animation: none !important; }
}

[data-ui-motion="reduced"] .flagship-shell::before {
    animation: none !important;
}

.flagship-display-2 {
    margin: 0;
    font-size: var(--flagship-display-2);
    font-weight: 800;
    line-height: 1.08;
    letter-spacing: -0.01em;
}

.flagship-eyebrow {
    margin: 0;
    color: var(--ui-primary);
    font-size: var(--flagship-eyebrow);
    font-weight: 800;
    letter-spacing: 0.08em;
    text-transform: uppercase;
}

/* Hero: full viewport, no chrome, the scene fills it edge-to-edge. */
.flagship-hero {
    position: relative;
    width: 100vw;
    height: 100vh;
    min-height: 640px;
    display: grid;
    place-items: center;
    overflow: hidden;
    padding: 0;
}

.flagship-hero-stage {
    width: min(100vw, 1920px);
    height: min(100vh, 1080px);
    display: grid;
    place-items: center;
    transform: translateZ(0);
}

.flagship-hero-stage > * {
    width: 100%;
    height: 100%;
}

/* Story: the embedded scene already provides a 200vh trigger and a sticky
   inner shell, so we only widen the outer slot. */
.flagship-story {
    width: 100vw;
}

.flagship-story .scene-scroll-trigger {
    width: 100vw;
}

.flagship-story .scene-scroll-sticky > * {
    width: min(100vw, 1280px);
    margin-inline: auto;
}

/* Glass feature triplet. */
.flagship-features {
    width: 100vw;
    padding: var(--flagship-section-pad-y) var(--flagship-section-pad-x);
}

.flagship-features-inner {
    width: min(100%, var(--flagship-content-max));
    margin-inline: auto;
    display: grid;
    gap: var(--ui-space-5);
}

.flagship-features-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: var(--ui-space-4);
}

.flagship-features-grid .ui-glass-surface {
    min-height: 220px;
    display: grid;
    gap: var(--ui-space-2);
    transition: transform var(--ui-motion-fast), box-shadow var(--ui-motion-fast);
}

.flagship-features-grid .ui-glass-surface:hover {
    transform: translateY(-2px);
    box-shadow: var(--ui-elevation-3);
}

.flagship-card-title {
    margin: 0;
    font-size: 22px;
    font-weight: 700;
    line-height: 1.18;
}

.flagship-card-body {
    margin: 0;
    color: var(--ui-muted-fg);
    line-height: 1.5;
}

/* Live metric strip. */
.flagship-metrics {
    width: 100vw;
    padding: var(--flagship-section-pad-y) var(--flagship-section-pad-x);
    background: color-mix(in srgb, var(--ui-surface), transparent 18%);
    backdrop-filter: blur(16px) saturate(150%);
    -webkit-backdrop-filter: blur(16px) saturate(150%);
}

.flagship-metrics-inner {
    width: min(100%, var(--flagship-content-max));
    margin-inline: auto;
    display: grid;
    gap: var(--ui-space-5);
}

.flagship-metrics-grid {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: var(--ui-space-4);
}

.flagship-metrics-grid .ui-block-metric-counter {
    padding: var(--ui-space-4);
    border-radius: var(--ui-radius-lg);
    border: 1px solid var(--ui-border);
    background: var(--ui-surface);
    box-shadow: var(--ui-elevation-1);
}

/* CTA band + footer. */
.flagship-cta {
    width: 100vw;
    padding: var(--flagship-section-pad-y) var(--flagship-section-pad-x) 0;
}

.flagship-cta-inner {
    width: min(100%, var(--flagship-content-max));
    margin-inline: auto;
    text-align: center;
    display: grid;
    justify-items: center;
    gap: var(--ui-space-4);
}

.flagship-cta-inner .flagship-display-2 {
    max-width: 18ch;
}

.flagship-cta-caption {
    margin: 0;
    color: var(--ui-muted-fg);
    font-size: 16px;
    max-width: 56ch;
}

.flagship-cta-actions {
    display: flex;
    gap: var(--ui-space-3);
    flex-wrap: wrap;
    justify-content: center;
    padding-top: var(--ui-space-2);
}

.flagship-footer {
    margin-top: var(--flagship-section-pad-y);
    padding: var(--ui-space-4) var(--flagship-section-pad-x);
    border-top: 1px solid var(--ui-border);
    display: flex;
    flex-wrap: wrap;
    gap: var(--ui-space-3);
    align-items: center;
    justify-content: space-between;
    color: var(--ui-muted-fg);
    font-size: 13px;
}

.flagship-footer p {
    margin: 0;
}

.flagship-footer-brand {
    font-weight: 800;
    color: var(--ui-fg);
    letter-spacing: -0.01em;
}

/* Mobile: collapse multi-column grids. */
@media (max-width: 820px) {
    .flagship-features-grid {
        grid-template-columns: 1fr;
    }
    .flagship-metrics-grid {
        grid-template-columns: repeat(2, minmax(0, 1fr));
    }
}

@media (max-width: 540px) {
    .flagship-metrics-grid {
        grid-template-columns: 1fr;
    }
    .flagship-cta-actions {
        flex-direction: column;
        width: 100%;
        align-items: stretch;
    }
}
"#;
```

- [ ] **Step 2: Build the package**

Run: `cargo check -p flagship`
Expected: clean build.

- [ ] **Step 3: Build the whole workspace**

Run: `cargo check --workspace`
Expected: clean build.

- [ ] **Step 4: Commit**

```bash
git add examples/flagship/src/styles.rs
git commit -m "feat(flagship): land identity CSS — type ramp, accent, ambient mesh, section layout"
```

---

## Task 9: Provide `ReducedMotion` context, honour OS pref

The hero `Scene` reads `ReducedMotion` from the Dioxus context to decide whether to autoplay; under reduced motion it renders the hold-end frame as a static composition. The gallery reads OS preference via `persistence::prefers_reduced_motion()` and feeds it into the context. The flagship has no runtime UI toggle, so it reads OS preference once at mount and provides the context value directly.

**Files:**
- Modify: `examples/flagship/src/app.rs`

- [ ] **Step 1: Wire `ReducedMotion` from OS preference**

Edit `examples/flagship/src/app.rs`. Add the OS-pref read and the context provider. The file becomes:

```rust
use component_gallery::persistence::prefers_reduced_motion;
use dioxus::prelude::*;
use ui_runtime::ReducedMotion;
use ui_styles::library_css;

use crate::sections::cta::CallToAction;
use crate::sections::features::Features;
use crate::sections::hero::Hero;
use crate::sections::metrics::Metrics;
use crate::sections::story::Story;
use crate::styles::FLAGSHIP_CSS;

#[component]
pub fn App() -> Element {
    let shared = library_css();

    // Read OS-level prefers-reduced-motion once at mount. The flagship has
    // no runtime motion-pref toggle (unlike the gallery's PreferenceBar), so
    // changes during a session require a refresh. That's acceptable for a
    // marketing page; the cost of subscribing to changes (forget'd Closure +
    // MediaQueryList plumbing) is not justified for this surface.
    let reduced = prefers_reduced_motion();
    use_context_provider(|| ReducedMotion(reduced));

    let motion_attr = if reduced { "reduced" } else { "normal" };

    rsx! {
        style { "{shared}" }
        style { "{FLAGSHIP_CSS}" }
        main {
            class: "flagship-shell",
            "data-ui-motion": "{motion_attr}",
            Hero {}
            Story {}
            Features {}
            Metrics {}
            CallToAction {}
        }
    }
}
```

> The `data-ui-motion` attribute on `<main>` is also what the existing `[data-ui-motion="reduced"] ...` rules in `library_css()` and `FLAGSHIP_CSS` key off, so CSS-level motion suppression stays consistent with the JS-level context.

- [ ] **Step 2: Build the package**

Run: `cargo check -p flagship`
Expected: clean build.

- [ ] **Step 3: Commit**

```bash
git add examples/flagship/src/app.rs
git commit -m "feat(flagship): honour OS prefers-reduced-motion via ReducedMotion context"
```

---

## Task 10: Playwright spec — marker-free assertion + hero mount

A minimal Playwright project under `examples/flagship/e2e/`. One spec asserts:
- Page loads at `/`.
- Zero `.gallery-rail`, `.gallery-controls`, `.gallery-entry`, `.gallery-code` markers exist on the page.
- The hero scene root (`[data-scene-id="product-intro"]`) is in the DOM.
- No console errors during the first 2 seconds.

We do not run snapshot diffs — the Hero-3-seconds check is a manual screenshot inspection done in Task 11.

**Files:**
- Create: `examples/flagship/e2e/package.json`
- Create: `examples/flagship/e2e/tsconfig.json`
- Create: `examples/flagship/e2e/playwright.config.ts`
- Create: `examples/flagship/e2e/tests/flagship.spec.ts`
- Create: `examples/flagship/e2e/.gitignore`

- [ ] **Step 1: Verify the scene root selector**

The flagship's e2e assertion targets `[data-scene-id="product-intro"]`. Open `crates/ui-dioxus/src/scene_player.rs` and grep for `data-scene-id` to confirm the attribute name and the value emitted by `Scene { id: "product-intro", ... }`.

Run: `grep -n 'data-scene-id\|data-scene' crates/ui-dioxus/src/scene_player.rs`
Expected: at least one match using the `id` prop verbatim.

If the attribute is named differently (e.g., `data-scene` or `data-scene-root`), update Step 4's selector to match. The principle stays the same: assert the hero scene root exists.

- [ ] **Step 2: Create `package.json`**

Path: `examples/flagship/e2e/package.json`

```json
{
  "name": "flagship-e2e",
  "private": true,
  "type": "module",
  "scripts": {
    "test": "playwright test",
    "test:static": "playwright test --project=static",
    "test:dev-loop": "KINETICS_E2E_MODE=dev-loop playwright test --project=dev-loop"
  },
  "devDependencies": {
    "@playwright/test": "^1.48.0",
    "http-server": "^14.1.1",
    "typescript": "^5.5.0"
  }
}
```

- [ ] **Step 3: Create `tsconfig.json`**

Path: `examples/flagship/e2e/tsconfig.json`

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "strict": true,
    "esModuleInterop": true,
    "resolveJsonModule": true,
    "skipLibCheck": true,
    "allowJs": false,
    "noEmit": true
  },
  "include": ["tests/**/*.ts", "playwright.config.ts"]
}
```

- [ ] **Step 4: Create `playwright.config.ts`**

Path: `examples/flagship/e2e/playwright.config.ts`

```ts
import { defineConfig, devices } from "@playwright/test";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = resolve(__filename, "..");

const projectArg = process.argv.find((arg) => arg.startsWith("--project="));
if (projectArg) {
  const name = projectArg.slice("--project=".length);
  process.env.KINETICS_E2E_MODE = name === "dev-loop" ? "dev-loop" : "static";
} else if (!process.env.KINETICS_E2E_MODE) {
  process.env.KINETICS_E2E_MODE = "static";
}

const PROJECT_ROOT = resolve(__dirname, "..");
const WORKSPACE_ROOT = resolve(PROJECT_ROOT, "..", "..");
const DIST_DIR = resolve(
  WORKSPACE_ROOT,
  "target",
  "dx",
  "flagship",
  "release",
  "web",
  "public"
);
const STATIC_PORT = 4174;
const DEV_LOOP_URL = process.env.KINETICS_FLAGSHIP_URL ?? "http://localhost:9174";

export default defineConfig({
  testDir: "./tests",
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [["list"], ["html", { open: "never", outputFolder: "playwright-report" }]],
  expect: {
    toHaveScreenshot: { maxDiffPixelRatio: 0.05, animations: "disabled" },
  },
  use: {
    actionTimeout: 10_000,
    navigationTimeout: 30_000,
    trace: "retain-on-failure",
    screenshot: "only-on-failure",
  },
  projects: [
    {
      name: "static",
      use: {
        ...devices["Desktop Chrome"],
        baseURL: `http://localhost:${STATIC_PORT}`,
      },
    },
    {
      name: "dev-loop",
      use: {
        ...devices["Desktop Chrome"],
        baseURL: DEV_LOOP_URL,
      },
    },
  ],
  webServer:
    process.env.KINETICS_E2E_MODE === "dev-loop"
      ? undefined
      : {
          command: `npx http-server "${DIST_DIR}" -p ${STATIC_PORT} --silent`,
          port: STATIC_PORT,
          reuseExistingServer: !process.env.CI,
          timeout: 60_000,
        },
});
```

- [ ] **Step 5: Create the smoke spec**

Path: `examples/flagship/e2e/tests/flagship.spec.ts`

```ts
import { test, expect } from "@playwright/test";

test.describe("flagship marketing page", () => {
  test("has zero gallery-shell markers", async ({ page }) => {
    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    await page.goto("/");

    await expect(page.locator(".flagship-shell")).toBeVisible();

    expect(await page.locator(".gallery-rail").count()).toBe(0);
    expect(await page.locator(".gallery-controls").count()).toBe(0);
    expect(await page.locator(".gallery-entry").count()).toBe(0);
    expect(await page.locator(".gallery-code").count()).toBe(0);

    await page.waitForTimeout(500);
    expect(consoleErrors, `console errors: ${consoleErrors.join("\n")}`).toHaveLength(0);
  });

  test("hero scene root mounts", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator(".flagship-hero")).toBeVisible();
    // The selector below is the hero Scene's DOM root. If the
    // ui-dioxus scene_player emits a different attribute, update it
    // to match what the Scene component renders (see Task 10 Step 1).
    await expect(page.locator('[data-scene-id="product-intro"]').first()).toBeAttached();
  });

  test("five sections are present", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator(".flagship-hero")).toBeVisible();
    await expect(page.locator(".flagship-story")).toBeAttached();
    await expect(page.locator(".flagship-features")).toBeAttached();
    await expect(page.locator(".flagship-metrics")).toBeAttached();
    await expect(page.locator(".flagship-cta")).toBeAttached();
  });
});
```

- [ ] **Step 6: Create `.gitignore` for the e2e directory**

Path: `examples/flagship/e2e/.gitignore`

```
node_modules/
playwright-report/
test-results/
```

- [ ] **Step 7: Install Playwright deps**

Run from `examples/flagship/e2e/`:
```bash
npm install
npx playwright install chromium
```

Expected: dependencies installed, chromium browser cached.

- [ ] **Step 8: Build the flagship for static serving**

Run from the workspace root:
```bash
dx build --package flagship --release
```

Expected: artifacts under `target/dx/flagship/release/web/public/`.

- [ ] **Step 9: Run the Playwright tests**

Run from `examples/flagship/e2e/`:
```bash
npm test
```

Expected: 3 tests pass. If `data-scene-id="product-intro"` is wrong, the hero-mount test fails — fix the selector and re-run.

- [ ] **Step 10: Commit**

```bash
git add examples/flagship/e2e
git commit -m "test(flagship): playwright spec — no gallery markers, hero scene mounts, five sections present"
```

---

## Task 11: README pointer + manual hero screenshot

Add a one-paragraph subsection to the repository README explaining how to run the flagship, and capture the binding `hero-screenshot.png` via chrome-devtools-mcp.

**Files:**
- Modify: `README.md`
- Create: `examples/flagship/docs/hero-screenshot.png` (binary artifact, generated)
- Create: `examples/flagship/README.md`

- [ ] **Step 1: Add a README subsection**

Open `README.md`. Find the existing `## Component Gallery` section (~line 130). After the gallery section ends and before `## Render & CLI`, insert:

```markdown
## Flagship Marketing Page

A self-referential marketing page for `dioxus-kinetics` lives in
`examples/flagship`. It composes existing scenes
(`ProductIntroScene`, `ScrollPinnedStoryScene`, the glass triplet,
`MetricCounter` strip, and a CTA band) at full bleed, with no
documentation chrome. Use it as a reference for what shipping with
kinetics actually looks like, and as the binding visual check for
the workspace's Apple-quality story.

```powershell
dx serve --package flagship --port 9174
```

Open `http://localhost:9174` in a browser that supports WebGPU (the
glass triplet section reveals the WebGPU `ui-glass-engine` path; on
non-WebGPU browsers it falls back through SVG filter to solid).

The binding visual check is documented in
`docs/superpowers/specs/2026-05-25-flagship-marketing-page-design.md`
(the "Hero-3-seconds" check). The reference screenshot lives at
`examples/flagship/docs/hero-screenshot.png`.
```

- [ ] **Step 2: Create the example's own README**

Path: `examples/flagship/README.md`

```markdown
# Flagship

Single-page, self-referential marketing site for `dioxus-kinetics`.
Composed entirely from existing scenes and components — no new
primitives. The page exists to make the library look like it
deserves the showcase, and to expose any primitive-level gaps the
gallery hides behind 148 px preview tiles.

## Run

```powershell
dx serve --package flagship --port 9174
```

## Sections (top to bottom)

1. **Hero** — `ProductIntroScene` at full viewport, autoplay-once.
2. **Story** — `ScrollPinnedStoryScene` pinned full-bleed, scroll-driven.
3. **Features** — three `GlassSurface` cards (Info / Primary / Success).
4. **Metrics** — four `MetricCounter`s in a row.
5. **CTA + footer** — two `Button`s and a minimal one-row footer.

## E2E

Playwright spec at `e2e/tests/flagship.spec.ts` asserts zero
gallery-shell markers, hero scene mount, and five-section presence.

```powershell
cd e2e
npm install
npx playwright install chromium
npm test
```

## Binding visual check

See `docs/superpowers/specs/2026-05-25-flagship-marketing-page-design.md`
section "Pass / fail check". The reference screenshot lives at
`docs/hero-screenshot.png`.
```

- [ ] **Step 3: Capture the hero screenshot via chrome-devtools-mcp**

Boot the flagship dev server:
```bash
dx serve --package flagship --port 9174
```

Use chrome-devtools-mcp to drive the capture. Required calls (in order):
1. `mcp__plugin_chrome-devtools-mcp_chrome-devtools__new_page` with `http://localhost:9174`.
2. `mcp__plugin_chrome-devtools-mcp_chrome-devtools__resize_page` to `1440 x 900`.
3. `mcp__plugin_chrome-devtools-mcp_chrome-devtools__wait_for` with a 3-second wait so the hero film reaches the hold-end of its title clip (~2.4 s, with margin).
4. `mcp__plugin_chrome-devtools-mcp_chrome-devtools__take_screenshot` and save the binary to `examples/flagship/docs/hero-screenshot.png`.

Create the docs directory first if it does not exist:
```bash
mkdir -p examples/flagship/docs
```

If chrome-devtools-mcp is unavailable in the executing environment, replace step 3 above with a manual capture: open `http://localhost:9174` in Chrome at viewport 1440×900, wait ~3 s, take a viewport screenshot via OS tooling, and save it to the same path. The artifact is the source of truth either way.

- [ ] **Step 4: Inspect the screenshot against the binding check**

Open `examples/flagship/docs/hero-screenshot.png`. Apply the Hero-3-seconds check (does the first paint read as apple.com-grade, not SaaS-dashboard?). If it fails, file follow-up tickets against the primitives that fell short (per the spec, this is the intended outcome of the forcing function); do **not** block this plan on a perfect first capture.

- [ ] **Step 5: Commit**

```bash
git add README.md examples/flagship/README.md examples/flagship/docs/hero-screenshot.png
git commit -m "docs(flagship): README pointer + binding hero screenshot artifact"
```

---

## Self-review notes

Worked through the spec one last time against this plan:

- **Scope coverage.** All eight in-scope items map to tasks: new package (Task 2), single-route App with five sections (Tasks 3–7), zero re-implementation of scenes (Tasks 3–7 reuse component-gallery exports, enabled by Task 1), flagship CSS only on top of `library_css()` (Task 8), `screenshot.png` artifact (Task 11), Playwright spec (Task 10), README pointer (Task 11).
- **Out-of-scope items** stay out of scope. No new primitives, no scene #2, no real GitHub URLs (the CTA buttons have no `onclick`, just labels), no raster assets (footer brand is text, not an SVG file), no i18n.
- **Identity moves** match the spec: type ramp (`--flagship-display-1/2`, `--flagship-eyebrow`), accent inflection (`--ui-primary: #0a7aff` scoped to `.flagship-shell`), ambient mesh, zero raster imagery — all in Task 8.
- **Motion principles** — no new primitives introduced; the CSS uses only existing tokens (`--ui-motion-fast`, `--ui-elevation-*`).
- **Reduced motion** — Task 9 provides `ReducedMotion(false)` so Scene autoplay reads the context cleanly, the CSS suppresses backdrop drift under `[data-ui-motion="reduced"]` and `@media (prefers-reduced-motion: reduce)`.
- **Accessibility** — `aria-label` / `aria-labelledby` on each section, real `<h2>` per section, decorative backdrop in `::before` (non-focusable by construction).
- **Open risks** — the `MetricCounter` fan-out works because the component takes `label`/`value`/`delta_text` and we instantiate four. The `CtaPulseScene` reuse risk is resolved by composing our own band (Task 7). The scene-reuse and OS-reduced-motion paths are both resolved by making `previews` and `persistence` public on the gallery (Task 1) rather than promoting new crates. The `MetricCounter`s use `TimelineScope { autoplay: false }` internally, which means they render static (un-animated) text without a surrounding `Scene` clock — that is acceptable for v1; if the section reads as flat, a follow-up wraps the four counters in a single autoplay `Scene` with a 200 ms stagger.
- **Type consistency** — `Hero`, `Story`, `Features`, `Metrics`, `CallToAction` names match across all tasks. `ProductIntroScene`, `ScrollPinnedStoryScene`, `MetricCounter`, `GlassSurface`, `Button`, `ButtonVariant`, `GlassLevel`, `GlassTone`, `GlassDensity`, `ReducedMotion` all match their actual API surfaces from `kinetics::prelude`, `ui_runtime`, and `component_gallery::previews::scenes::*`.
- **No placeholders** — every step has either a complete file body, a complete edit diff, an exact command, or a chrome-devtools-mcp call sequence. The single `33` for "Components ready" has a Note allowing write-time correction; that's a number, not a design hole.
