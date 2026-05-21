# Kinetics Rebrand And Showcase Polish Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rename the public facade crate from `kinetics` to `kinetics`, swap the component gallery's text-based brand for the inline `docs/assets/dioxus-kinetics-logo.svg` mark, and expand the showcase previews for the Ready primitives in Motion, Composition, Capture, and Foundations.

**Architecture:** Mechanical rename plus presentation polish. No new components, no motion math, no JS runtime work. The user picked full rename (option A): the old `kinetics` name disappears from active code AND from historical specs/plans. Previews change from single-instance demos to comparative `gallery-variant-grid` tiles. Coming-soon entries stay untouched for sub-projects 2-4.

**Tech Stack:** Rust 2021, Cargo workspace, Dioxus 0.7, Dioxus SSR tests, pure Rust unit tests, static CSS strings, PowerShell commands on Windows.

---

## Scope

This plan implements the rebrand and showcase polish described in `docs/superpowers/specs/2026-05-21-kinetics-rebrand-design.md`.

It includes:

- moving `crates/kinetics/` to `crates/kinetics/` and updating every consumer
- replacing all "Kinetics" brand strings with "Kinetics"
- updating historical specs and plans under `docs/superpowers/`
- inlining the `dioxus-kinetics-logo.svg` into the gallery rail
- replacing the existing single-instance previews in Motion, Composition, Capture, and Foundations with comparative variant grids
- promoting `KineticBox` and `PresenceGate` to their own Ready registry entries

It excludes:

- implementing the coming-soon Motion entries (Presence, Sequence, SharedLayout, SharedElement)
- implementing IconButton
- adding new categories
- changing component APIs or feature flags
- introducing a JS or wasm motion runtime

## Before You Start

Set up an isolated worktree for this work. If you are running this plan through `superpowers:subagent-driven-development`, that skill handles worktree setup for you. Otherwise, run:

```powershell
git worktree add .worktrees/kinetics-rebrand -b kinetics-rebrand main
```

Run every command in this plan from inside the worktree directory.

## File Map

- `Cargo.toml`: workspace members list (`kinetics` → `kinetics`).
- `crates/kinetics/`: replaces `crates/kinetics/`. Same source tree, renamed package.
- `crates/kinetics/Cargo.toml`: package name becomes `kinetics`.
- `crates/kinetics/tests/prelude.rs`: every `kinetics::` becomes `kinetics::`.
- Every `Cargo.toml` that lists `kinetics` as a dependency: rename it.
- Every `.rs` file with `use kinetics` or `kinetics::`: rename it.
- `examples/component-gallery/src/brand.rs`: new file. Holds the inlined logo SVG as a `&'static str`.
- `examples/component-gallery/src/lib.rs`: register the new `brand` module.
- `examples/component-gallery/src/app.rs`: swap the brand block, update header copy.
- `examples/component-gallery/src/docs.rs`: replace four preview functions, add two new ones, register two new doc entries.
- `examples/component-gallery/src/styles.rs`: append gallery-logo, gallery-variant-grid, gallery-variant-tile, gallery-variant-label, visually-hidden, and viewport-fit CSS.
- `examples/component-gallery/tests/gallery.rs`: replace brand assertions, add SVG and variant-grid assertions.
- `README.md`: every `kinetics` and "Kinetics" reference.
- `docs/component-naming.md`, `docs/platform-support.md`, `docs/glass-materials.md`: naming refs.
- `docs/superpowers/specs/2026-05-20-unified-ui-library-design.md`, `docs/superpowers/specs/2026-05-20-advanced-ui-wave-design.md`, `docs/superpowers/specs/2026-05-20-component-gallery-design.md`, `docs/superpowers/specs/2026-05-21-native-kinetics-systems-design.md`, `docs/superpowers/plans/2026-05-20-unified-ui-library.md`, `docs/superpowers/plans/2026-05-20-component-gallery.md`, `docs/superpowers/plans/2026-05-21-advanced-ui-wave.md`, `docs/superpowers/plans/2026-05-21-native-kinetics-systems.md`: content updates (filenames unchanged).

## Task 1: Move The Crate Directory And Update Every Consumer

**Files:**
- Move: `crates/kinetics/` to `crates/kinetics/`
- Modify: `Cargo.toml`
- Modify: `crates/kinetics/Cargo.toml`
- Modify: `crates/kinetics/tests/prelude.rs`
- Modify: `examples/component-gallery/Cargo.toml`
- Modify: every `.rs` file under `crates/` and `examples/` that contains `kinetics`

- [ ] **Step 1: Confirm baseline tests pass**

Run:

```powershell
cargo test --workspace
```

Expected: PASS (this is the green starting state).

- [ ] **Step 2: Move the crate directory**

Run:

```powershell
git mv crates/kinetics crates/kinetics
```

- [ ] **Step 3: Update the workspace `Cargo.toml`**

In root `Cargo.toml`, change the workspace members entry that reads `crates/kinetics` to `crates/kinetics`. Leave every other entry untouched.

- [ ] **Step 4: Update the kinetics package manifest**

In `crates/kinetics/Cargo.toml`, change the `[package]` `name` field from `kinetics` to `kinetics`. Leave version, edition, license, publish, and dependencies untouched.

- [ ] **Step 5: Update the gallery dependency**

In `examples/component-gallery/Cargo.toml`, change the dependency line that reads `kinetics = { ... }` so the dependency name on the left is `kinetics`. Keep the rest of the line identical. If the dependency uses a `package = "..."` rename, remove that field (the package name is now `kinetics`).

- [ ] **Step 6: Update every `use kinetics` import**

In every `.rs` file under `crates/` and `examples/`, replace the string `kinetics` with `kinetics`. This includes `use kinetics::prelude::*;`, `use kinetics::...`, doc comments that reference `kinetics`, and the test file `crates/kinetics/tests/prelude.rs` which still says `kinetics::public_api_names()` etc.

Use this PowerShell loop to do it consistently:

```powershell
$files = git ls-files crates examples | Where-Object { $_ -like '*.rs' }
foreach ($file in $files) {
    $content = Get-Content $file -Raw
    $updated = $content -replace 'kinetics', 'kinetics'
    if ($content -ne $updated) {
        Set-Content -Path $file -Value $updated -NoNewline
    }
}
```

- [ ] **Step 7: Run the workspace tests**

Run:

```powershell
cargo test --workspace
```

Expected: PASS. If any test fails, the most likely cause is a `Cargo.toml` `dependencies` entry that still names `kinetics` somewhere — fix it and re-run.

- [ ] **Step 8: Confirm format check**

Run:

```powershell
cargo fmt --all -- --check
```

Expected: PASS.

- [ ] **Step 9: Commit**

Run:

```powershell
git add Cargo.toml crates examples
git commit -m "refactor: rename kinetics crate to kinetics"
```

## Task 2: Update Active Brand Strings In Docs

**Files:**
- Modify: `README.md`
- Modify: `docs/component-naming.md`
- Modify: `docs/platform-support.md`
- Modify: `docs/glass-materials.md`

The gallery's `app.rs` is NOT touched in this task. Changing the header text here would break the existing `gallery_renders_ready_examples_and_coming_soon_entries` assertion until Task 5 lands. All gallery-code rename work happens atomically in Task 5.

- [ ] **Step 1: Update README**

In `README.md`, replace every occurrence of:

- `Kinetics` → `Kinetics`
- `kinetics` → `kinetics`

Keep the repository name `dioxus-kinetics` unchanged. Keep the workspace layout block correct: the directory listing line now reads `  kinetics/         public facade and prelude`.

Update the README code examples so `use kinetics::prelude::*;` becomes `use kinetics::prelude::*;` and any `cargo test -p kinetics` becomes `cargo test -p kinetics`.

- [ ] **Step 2: Update naming docs**

In `docs/component-naming.md`, replace `Kinetics` with `Kinetics` in any prose. Component names stay unchanged.

In `docs/platform-support.md` and `docs/glass-materials.md`, replace `Kinetics` with `Kinetics` and `kinetics` with `kinetics` wherever they appear.

- [ ] **Step 3: Run tests**

Run:

```powershell
cargo test --workspace
```

Expected: PASS. README and docs changes do not affect any current test (the README naming test for `kinetics` is added in Task 12).

- [ ] **Step 4: Commit**

Run:

```powershell
git add README.md docs/component-naming.md docs/platform-support.md docs/glass-materials.md
git commit -m "docs: update active brand strings to kinetics"
```

## Task 3: Update Historical Spec And Plan Docs

**Files:**
- Modify: `docs/superpowers/specs/2026-05-20-unified-ui-library-design.md`
- Modify: `docs/superpowers/specs/2026-05-20-advanced-ui-wave-design.md`
- Modify: `docs/superpowers/specs/2026-05-20-component-gallery-design.md`
- Modify: `docs/superpowers/specs/2026-05-21-native-kinetics-systems-design.md`
- Modify: `docs/superpowers/plans/2026-05-20-unified-ui-library.md`
- Modify: `docs/superpowers/plans/2026-05-20-component-gallery.md`
- Modify: `docs/superpowers/plans/2026-05-21-advanced-ui-wave.md`
- Modify: `docs/superpowers/plans/2026-05-21-native-kinetics-systems.md`

- [ ] **Step 1: Run a search-and-replace across closed docs**

Run:

```powershell
$files = git ls-files docs/superpowers | Where-Object { $_ -like '*.md' }
foreach ($file in $files) {
    $content = Get-Content $file -Raw
    $updated = $content -replace 'kinetics', 'kinetics' -replace 'Kinetics', 'Kinetics'
    if ($content -ne $updated) {
        Set-Content -Path $file -Value $updated -NoNewline
    }
}
```

This intentionally rewrites historical specs and plans. The user picked the full-rename option, so the closed phase docs stop referring to the old names.

- [ ] **Step 2: Verify no other references remain**

Run:

```powershell
rg -n "kinetics|Kinetics" README.md docs crates examples Cargo.toml
```

Expected: zero matches.

If any matches appear, fix them by hand. Then re-run the scan.

- [ ] **Step 3: Commit**

Run:

```powershell
git add docs/superpowers
git commit -m "docs: rename historical references to kinetics"
```

## Task 4: Add The Brand Module With Inlined Logo SVG

**Files:**
- Create: `examples/component-gallery/src/brand.rs`
- Modify: `examples/component-gallery/src/lib.rs`

- [ ] **Step 1: Read the SVG file content**

Run:

```powershell
Get-Content docs/assets/dioxus-kinetics-logo.svg -Raw
```

Copy the entire SVG content (including the `<svg ...>` opening tag, all child elements, and the closing `</svg>`) to your clipboard. You will paste it into a Rust raw string in the next step.

- [ ] **Step 2: Create the brand module**

Create `examples/component-gallery/src/brand.rs` with this content (replace the placeholder line `<!-- paste SVG content here -->` with the full SVG markup from Step 1):

```rust
//! Brand assets for the Kinetics component gallery.

pub const KINETICS_LOGO_SVG: &str = r#"<!-- paste SVG content here -->"#;
```

The SVG already contains a `<title id="title">dioxus-kinetics logo</title>` element, which the brand tests rely on. Do not edit the SVG content.

- [ ] **Step 3: Register the module**

In `examples/component-gallery/src/lib.rs`, find the existing `mod` declarations and add `mod brand;` next to them. Re-export the constant by adding `pub use brand::KINETICS_LOGO_SVG;` near the other public re-exports (or, if there are none, add it as the only public re-export).

If `lib.rs` does not yet exist, examine the existing module structure (`main.rs`, `docs.rs`, `app.rs`, `styles.rs`) to determine the right place. The `App` component lives in `app.rs`, which already imports siblings via `use crate::*` patterns; place `mod brand;` next to those.

- [ ] **Step 4: Verify the crate still compiles**

Run:

```powershell
cargo check -p component-gallery
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add examples/component-gallery/src/brand.rs examples/component-gallery/src/lib.rs
git commit -m "feat: add kinetics logo brand module"
```

## Task 5: Swap The Gallery Brand Block

**Files:**
- Modify: `examples/component-gallery/src/app.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write the brand-swap test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_brand_uses_kinetics_logo_and_name() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("Kinetics"));
    assert!(!html.contains("Kinetics"));
    assert!(html.contains("<svg"));
    assert!(html.contains("dioxus-kinetics logo"));
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p component-gallery gallery_brand_uses_kinetics_logo_and_name -- --exact
```

Expected: FAIL because the gallery still renders the "Kinetics" wordmark and does not inline the SVG.

- [ ] **Step 3: Replace the brand block AND header copy atomically**

In `examples/component-gallery/src/app.rs`, make all three edits before re-running tests:

**3a. Brand block.** Locate the existing block:

```rust
aside { class: "gallery-rail",
    div { class: "gallery-brand",
        span { class: "gallery-mark", "UI" }
        div {
            h1 { "Kinetics" }
            p { "Component reference" }
        }
    }
```

Replace it with:

```rust
aside { class: "gallery-rail",
    div { class: "gallery-brand",
        div {
            class: "gallery-logo",
            aria_label: "Kinetics",
            dangerous_inner_html: crate::brand::KINETICS_LOGO_SVG,
        }
        span { class: "visually-hidden", "Kinetics component gallery" }
    }
```

Keep the rest of the `aside` element unchanged (the `nav.gallery-nav` block remains untouched).

**3b. Main header h2.** Locate `h2 { "Kinetics Component Gallery" }` and replace the literal text with `"Kinetics Component Gallery"`.

**3c. Eyebrow text.** Locate `p { class: "gallery-eyebrow", "Dioxus SaaS library" }` and replace the literal text with `"Dioxus Kinetics library"`.

- [ ] **Step 4: Run the brand-swap test**

Run:

```powershell
cargo test -p component-gallery gallery_brand_uses_kinetics_logo_and_name -- --exact
```

Expected: PASS.

- [ ] **Step 5: Update the existing header-text assertion**

In `examples/component-gallery/tests/gallery.rs`, find this line inside `gallery_renders_ready_examples_and_coming_soon_entries`:

```rust
assert!(html.contains("Kinetics Component Gallery"));
```

Replace it with:

```rust
assert!(html.contains("Kinetics Component Gallery"));
```

The header text was just changed in Step 3b; without this assertion update the test would fail.

- [ ] **Step 6: Run the full gallery test suite**

Run:

```powershell
cargo test -p component-gallery
```

Expected: PASS.

- [ ] **Step 7: Commit**

Run:

```powershell
git add examples/component-gallery/src/app.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat: render kinetics logo in gallery brand area"
```

## Task 6: Add Gallery Logo And Variant-Grid CSS

**Files:**
- Modify: `examples/component-gallery/src/styles.rs`

- [ ] **Step 1: Write a CSS presence test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_css_includes_logo_and_variant_grid_styles() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for selector in [
        ".gallery-logo",
        ".visually-hidden",
        ".gallery-variant-grid",
        ".gallery-variant-grid--3x3",
        ".gallery-variant-grid--3col",
        ".gallery-variant-grid--2col",
        ".gallery-variant-grid--stack",
        ".gallery-variant-tile",
        ".gallery-variant-label",
    ] {
        assert!(html.contains(selector), "missing CSS selector {selector}");
    }
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p component-gallery gallery_css_includes_logo_and_variant_grid_styles -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Append the CSS block**

In `examples/component-gallery/src/styles.rs`, locate the `@media (max-width: 820px)` block inside `GALLERY_CSS` and insert this CSS just before it:

```css
.gallery-logo {
    display: block;
    width: 100%;
    max-width: 260px;
    margin-bottom: var(--ui-space-4);
}

.gallery-logo svg {
    width: 100%;
    height: auto;
    border-radius: var(--ui-radius-lg);
    display: block;
}

.visually-hidden {
    position: absolute;
    width: 1px;
    height: 1px;
    margin: -1px;
    padding: 0;
    overflow: hidden;
    clip: rect(0 0 0 0);
    border: 0;
}

.gallery-variant-grid {
    display: grid;
    gap: var(--ui-space-2);
    width: 100%;
}

.gallery-variant-grid--3x3 {
    grid-template-columns: repeat(3, minmax(0, 1fr));
}

.gallery-variant-grid--3col {
    grid-template-columns: repeat(3, minmax(0, 1fr));
}

.gallery-variant-grid--2col {
    grid-template-columns: repeat(2, minmax(0, 1fr));
}

.gallery-variant-grid--stack {
    grid-template-columns: 1fr;
}

.gallery-variant-tile {
    display: grid;
    gap: var(--ui-space-1);
    padding: var(--ui-space-3);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    min-height: 96px;
}

.gallery-variant-label {
    font-size: 11px;
    color: var(--ui-muted-fg);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
}
```

Then, inside the existing `@media (max-width: 820px)` block, append these rules just before the closing `}` of the media query:

```css
    .gallery-variant-grid--3x3,
    .gallery-variant-grid--3col,
    .gallery-variant-grid--2col {
        grid-template-columns: 1fr;
    }
```

- [ ] **Step 4: Run the CSS test**

Run:

```powershell
cargo test -p component-gallery gallery_css_includes_logo_and_variant_grid_styles -- --exact
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add examples/component-gallery/src/styles.rs examples/component-gallery/tests/gallery.rs
git commit -m "style: add gallery logo and variant grid CSS"
```

## Task 7: Replace `glass_layer_preview` With A 3×3 Variant Grid

**Files:**
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write the variant-grid assertions for GlassLayer**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_glass_layer_preview_renders_tone_level_matrix() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("gallery-variant-grid--3x3"));
    for level in ["Floating", "Raised", "Sunken"] {
        for tone in ["Neutral", "Warm", "Cool"] {
            assert!(
                html.contains(&format!("{level} · {tone}")),
                "missing GlassLayer tile {level} · {tone}",
            );
        }
    }
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p component-gallery gallery_glass_layer_preview_renders_tone_level_matrix -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Replace `glass_layer_preview`**

In `examples/component-gallery/src/docs.rs`, locate the function:

```rust
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

Replace it with:

```rust
fn glass_layer_preview() -> Element {
    let levels = [
        (GlassLevel::Floating, "Floating"),
        (GlassLevel::Raised, "Raised"),
        (GlassLevel::Sunken, "Sunken"),
    ];
    let tones = [
        (GlassTone::Neutral, "Neutral"),
        (GlassTone::Warm, "Warm"),
        (GlassTone::Cool, "Cool"),
    ];

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--3x3",
            for (level, level_label) in levels {
                for (tone, tone_label) in tones {
                    div { class: "gallery-variant-tile",
                        span { class: "gallery-variant-label", "{level_label} · {tone_label}" }
                        GlassLayer {
                            level: level,
                            tone: tone,
                            density: GlassDensity::Comfortable,
                            "Material preview"
                        }
                    }
                }
            }
        }
    }
}
```

If `GlassLevel`, `GlassTone`, or `GlassDensity` are not `Copy`, capture them by reference inside the closures (use `&level`, `&tone` and adjust the rsx accordingly). Run the next step to learn which.

- [ ] **Step 4: Run the variant-grid test**

Run:

```powershell
cargo test -p component-gallery gallery_glass_layer_preview_renders_tone_level_matrix -- --exact
```

Expected: PASS.

If the compile fails because the enum types are not `Copy`, derive `Copy` on them only if the source crate is in this workspace and the existing API already treats them as values. If a `Copy` derive is not appropriate, change the iteration to consume the arrays by value with `.into_iter()` and clone the labels.

- [ ] **Step 5: Run the full gallery test suite**

Run:

```powershell
cargo test -p component-gallery
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```powershell
git add examples/component-gallery/src/docs.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat: render GlassLayer tone-level variant grid"
```

## Task 8: Replace `timeline_scope_preview` With A 3-Variant Stack

**Files:**
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write the variant-grid assertions for TimelineScope**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_timeline_scope_preview_renders_three_variants() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("gallery-variant-grid--stack"));
    assert!(html.contains("\"data-stagger-index\": \"0\"")
        || html.contains("data-stagger-index=\"0\""));
    for cue in ["rise-in", "enter", "settle", "pulse"] {
        assert!(
            html.contains(&format!("data-motion-cue=\"{cue}\"")),
            "missing TimelineScope cue {cue}",
        );
    }
    assert!(html.contains("data-ui-transparency=\"reduced\""));
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p component-gallery gallery_timeline_scope_preview_renders_three_variants -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Replace `timeline_scope_preview`**

In `examples/component-gallery/src/docs.rs`, locate and replace the existing function with:

```rust
fn timeline_scope_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Stagger" }
                TimelineScope { id: "stagger-demo", autoplay: true,
                    for index in 0u32..4 {
                        div { "data-stagger-index": "{index}",
                            KineticBox { id: "stagger-{index}", cue: "rise-in",
                                "Tile {index}"
                            }
                        }
                    }
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Sequence" }
                TimelineScope { id: "sequence-demo", autoplay: true,
                    KineticBox { id: "sequence-enter", cue: "enter", "Enter" }
                    KineticBox { id: "sequence-settle", cue: "settle", "Settle" }
                    KineticBox { id: "sequence-pulse", cue: "pulse", "Pulse" }
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Reduced motion" }
                div { "data-ui-transparency": "reduced",
                    TimelineScope { id: "reduced-demo", autoplay: true,
                        for index in 0u32..4 {
                            div { "data-stagger-index": "{index}",
                                KineticBox { id: "reduced-{index}", cue: "rise-in",
                                    "Tile {index}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 4: Run the variant test**

Run:

```powershell
cargo test -p component-gallery gallery_timeline_scope_preview_renders_three_variants -- --exact
```

Expected: PASS.

- [ ] **Step 5: Run the full gallery test suite**

Run:

```powershell
cargo test -p component-gallery
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```powershell
git add examples/component-gallery/src/docs.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat: render TimelineScope stagger sequence reduced variants"
```

## Task 9: Replace `frame_stage_preview` With A 3-Frame Row

**Files:**
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write the variant-grid assertions for FrameStage**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_frame_stage_preview_renders_three_frame_snapshots() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for caption in ["Frame 0 / 180", "Frame 90 / 180", "Frame 179 / 180"] {
        assert!(
            html.contains(caption),
            "missing FrameStage caption {caption}",
        );
    }
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p component-gallery gallery_frame_stage_preview_renders_three_frame_snapshots -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Replace `frame_stage_preview`**

In `examples/component-gallery/src/docs.rs`, locate and replace the existing function with:

```rust
fn frame_stage_preview() -> Element {
    let frames: [u32; 3] = [0, 90, 179];

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--3col",
            for frame in frames {
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "Frame {frame} / 180" }
                    FrameStage {
                        composition: Composition::new("launch-demo", 1920, 1080, 30, 180),
                        frame: frame,
                        FrameClip { start: 0, duration: 60,
                            FrameLayer { id: "title", depth: 10,
                                h4 { "Dioxus Kinetics" }
                                p { "Frame {frame} / 180" }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 4: Run the variant test**

Run:

```powershell
cargo test -p component-gallery gallery_frame_stage_preview_renders_three_frame_snapshots -- --exact
```

Expected: PASS.

- [ ] **Step 5: Run the full gallery test suite**

Run:

```powershell
cargo test -p component-gallery
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```powershell
git add examples/component-gallery/src/docs.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat: render FrameStage multi-frame variant grid"
```

## Task 10: Replace `capture_stage_preview` With A 3-Viewport Row

**Files:**
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write the variant-grid assertions for CaptureStage**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_capture_stage_preview_renders_three_viewport_profiles() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for caption in [
        "Mobile · 360 × 640",
        "Tablet · 768 × 1024",
        "Desktop · 1440 × 900",
    ] {
        assert!(
            html.contains(caption),
            "missing CaptureStage caption {caption}",
        );
    }

    for viewport in ["mobile", "tablet", "desktop"] {
        assert!(
            html.contains(&format!("data-viewport=\"{viewport}\""))
                || html.contains(&format!("viewport=\"{viewport}\""))
                || html.contains(&format!(">{viewport}<")),
            "missing CaptureStage viewport prop {viewport}",
        );
    }
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p component-gallery gallery_capture_stage_preview_renders_three_viewport_profiles -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Replace `capture_stage_preview`**

In `examples/component-gallery/src/docs.rs`, locate and replace the existing function with:

```rust
fn capture_stage_preview() -> Element {
    let profiles: [(&str, &str, u32); 3] = [
        ("mobile", "Mobile · 360 × 640", 24),
        ("tablet", "Tablet · 768 × 1024", 48),
        ("desktop", "Desktop · 1440 × 900", 72),
    ];

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--3col",
            for (viewport, caption, frame) in profiles {
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "{caption}" }
                    CaptureStage {
                        id: "capture-{viewport}",
                        viewport: viewport.to_string(),
                        frame: frame,
                        p { "Frame {frame}" }
                    }
                }
            }
        }
    }
}
```

The current `CaptureStage` already serializes `viewport` to a `data-viewport` attribute (verified in `crates/ui-dioxus/src/capture.rs`). If it does not, the test's alternate assertions still match the inline body text. Inspect the rendered HTML if the test fails.

- [ ] **Step 4: Run the variant test**

Run:

```powershell
cargo test -p component-gallery gallery_capture_stage_preview_renders_three_viewport_profiles -- --exact
```

Expected: PASS.

- [ ] **Step 5: Run the full gallery test suite**

Run:

```powershell
cargo test -p component-gallery
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```powershell
git add examples/component-gallery/src/docs.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat: render CaptureStage viewport variant grid"
```

## Task 11: Add `KineticBox` And `PresenceGate` Registry Entries

**Files:**
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write the assertions for the new entries**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_includes_kinetic_box_and_presence_gate_entries() {
    let docs = component_gallery::component_docs();

    let kb = docs
        .iter()
        .find(|doc| doc.name == "KineticBox")
        .expect("KineticBox doc exists");
    assert_eq!(kb.status, component_gallery::ComponentStatus::Ready);
    assert!(kb.render.is_some());

    let pg = docs
        .iter()
        .find(|doc| doc.name == "PresenceGate")
        .expect("PresenceGate doc exists");
    assert_eq!(pg.status, component_gallery::ComponentStatus::Ready);
    assert!(pg.render.is_some());
}

#[test]
fn gallery_kinetic_box_preview_renders_three_cues() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for cue in ["rise-in", "fade-in", "slide-up"] {
        assert!(
            html.contains(&format!("data-motion-cue=\"{cue}\"")),
            "missing KineticBox cue {cue}",
        );
    }
}

#[test]
fn gallery_presence_gate_preview_renders_present_and_hidden_tiles() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("Visible state"));
    assert!(html.contains("Hidden state"));
    assert!(html.contains("gallery-variant-grid--2col"));
}
```

- [ ] **Step 2: Run and verify failure**

Run:

```powershell
cargo test -p component-gallery gallery_includes_kinetic_box_and_presence_gate_entries -- --exact
cargo test -p component-gallery gallery_kinetic_box_preview_renders_three_cues -- --exact
cargo test -p component-gallery gallery_presence_gate_preview_renders_present_and_hidden_tiles -- --exact
```

Expected: all three FAIL — registry has no `KineticBox` or `PresenceGate` entries.

- [ ] **Step 3: Increase the COMPONENT_DOCS array size**

In `examples/component-gallery/src/docs.rs`, find the declaration:

```rust
const COMPONENT_DOCS: [ComponentDoc; 25] = [
```

Change `25` to `27`.

- [ ] **Step 4: Add the new doc entries**

In the same array, immediately after the `TimelineScope` `ComponentDoc { ... },` entry, insert these two entries:

```rust
ComponentDoc {
    name: "KineticBox",
    category: ComponentCategory::Motion,
    status: ComponentStatus::Ready,
    summary: "Tags a region with a motion cue and stable kinetic id so timeline cues can target it.",
    snippet: KINETIC_BOX_SNIPPET,
    accessibility: "Motion cue is exposed via data attributes; reduced-motion policies replace cues with stable presentation.",
    render: Some(kinetic_box_preview),
},
ComponentDoc {
    name: "PresenceGate",
    category: ComponentCategory::Motion,
    status: ComponentStatus::Ready,
    summary: "Renders children only when the presence flag is set; gallery preview compares present and hidden states.",
    snippet: PRESENCE_GATE_SNIPPET,
    accessibility: "Hidden state renders no children; assistive tech does not encounter stale content.",
    render: Some(presence_gate_preview),
},
```

- [ ] **Step 5: Add the snippet constants**

In `examples/component-gallery/src/docs.rs`, near the other `const ..._SNIPPET: &str = r#"..."#;` declarations, add:

```rust
const KINETIC_BOX_SNIPPET: &str = r#"KineticBox {
    id: "metric-card",
    cue: "rise-in",
    "Tile body"
}"#;

const PRESENCE_GATE_SNIPPET: &str = r#"PresenceGate {
    present: is_visible,
    p { "Visible state" }
}"#;
```

- [ ] **Step 6: Add the preview functions**

In `examples/component-gallery/src/docs.rs`, near the other preview functions, add:

```rust
fn kinetic_box_preview() -> Element {
    let cues = ["rise-in", "fade-in", "slide-up"];

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--3col",
            for cue in cues {
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "{cue}" }
                    KineticBox { id: "cue-{cue}", cue: cue.to_string(),
                        p { "Cue preview" }
                    }
                }
            }
        }
    }
}

fn presence_gate_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--2col",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Present" }
                PresenceGate { present: true,
                    p { "Visible state" }
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Hidden" }
                PresenceGate { present: false }
                p { "Hidden state" }
            }
        }
    }
}
```

- [ ] **Step 7: Run the new tests**

Run:

```powershell
cargo test -p component-gallery gallery_includes_kinetic_box_and_presence_gate_entries -- --exact
cargo test -p component-gallery gallery_kinetic_box_preview_renders_three_cues -- --exact
cargo test -p component-gallery gallery_presence_gate_preview_renders_present_and_hidden_tiles -- --exact
```

Expected: all PASS.

- [ ] **Step 8: Run the full gallery test suite**

Run:

```powershell
cargo test -p component-gallery
```

Expected: PASS.

- [ ] **Step 9: Commit**

Run:

```powershell
git add examples/component-gallery/src/docs.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat: document KineticBox and PresenceGate variants"
```

## Task 12: Update The README Naming Test

**Files:**
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Append the kinetics naming test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn root_readme_uses_kinetics_crate_name() {
    let readme_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../README.md");
    let readme = std::fs::read_to_string(readme_path).expect("README.md should be readable");

    assert!(readme.contains("use kinetics::prelude::*"));
    assert!(readme.contains("crates/kinetics"));
    assert!(!readme.contains("kinetics"));
    assert!(!readme.contains("Kinetics"));
}
```

- [ ] **Step 2: Run the new test**

Run:

```powershell
cargo test -p component-gallery root_readme_uses_kinetics_crate_name -- --exact
```

Expected: PASS (README was already updated in Task 2).

- [ ] **Step 3: Commit**

Run:

```powershell
git add examples/component-gallery/tests/gallery.rs
git commit -m "test: assert README uses kinetics crate name"
```

## Task 13: Full Verification

**Files:**
- No planned source edits.

- [ ] **Step 1: Format check**

Run:

```powershell
cargo fmt --all -- --check
```

Expected: PASS.

If the check fails, run `cargo fmt --all` and commit with `style: apply rustfmt`.

- [ ] **Step 2: Full workspace tests**

Run:

```powershell
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 3: Gallery compile check**

Run:

```powershell
cargo check -p component-gallery
```

Expected: PASS.

- [ ] **Step 4: Naming-leakage scan**

Run:

```powershell
rg -n "kinetics|Kinetics" README.md docs crates examples Cargo.toml
```

Expected: zero matches.

If matches appear in source files, public docs, or test source (other than negative-assertion test strings — which are not present in this codebase for `kinetics`), remove them and re-run.

- [ ] **Step 5: Acceptance checklist verification**

Manually confirm each item from the spec's Acceptance Checklist (in `docs/superpowers/specs/2026-05-21-kinetics-rebrand-design.md`).

If every item is satisfied, this plan is complete. Hand off to `superpowers:finishing-a-development-branch`.

## Acceptance Checklist

- [ ] `crates/kinetics` directory no longer exists; `crates/kinetics` directory contains the same source tree.
- [ ] Workspace `Cargo.toml` lists `crates/kinetics`.
- [ ] `crates/kinetics/Cargo.toml` package name is `kinetics`.
- [ ] Every `use kinetics::*` is replaced with `use kinetics::*`.
- [ ] Component gallery rail renders the inlined SVG logo and a visually-hidden "Kinetics" label.
- [ ] Component gallery main header reads "Kinetics Component Gallery".
- [ ] `COMPONENT_DOCS` length is 27 and includes `KineticBox` and `PresenceGate` as `Ready` entries with previews.
- [ ] `GlassLayer`, `TimelineScope`, `FrameStage`, `CaptureStage`, `KineticBox`, `PresenceGate` previews use `.gallery-variant-grid` containers.
- [ ] README, naming docs, platform docs, glass docs, and all docs under `docs/superpowers/` reference `kinetics` and "Kinetics".
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo test --workspace` passes.
- [ ] `cargo check -p component-gallery` passes.
- [ ] `rg "kinetics|Kinetics" crates examples docs README.md Cargo.toml` returns zero matches.
- [ ] Coming-soon entries (Presence, Sequence, SharedLayout, SharedElement, IconButton) remain `ComponentStatus::ComingSoon` and untouched.
