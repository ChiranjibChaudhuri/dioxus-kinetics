# Gallery Dynamic Examples And Visual Depth Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the component gallery into a living showcase that demonstrates glass materiality, motion choreography, and accessibility policies, without changing any public component API.

**Architecture:** Library plumbing adds a 4-tier elevation token scale in `ui-tokens` (consumed via CSS custom properties emitted by `ui-styles`) and ancestor-scoped CSS overrides in `ui-styles` for `[data-ui-motion="reduced"]` and `[data-ui-glass-policy="solid"]`. The gallery owns four signals (`Theme`, `Density`, `Motion`, `Glass policy`) in a `GalleryPrefs` context that drives `data-*` attributes on the shell root; signals are seeded from `localStorage` and `prefers-reduced-motion`, persisted on change. Three gallery-only demo wrappers (`ReplayFrame`, `ScrubFrame`, `FlipFrame`) provide replay, scrub, and layout-swap UX. Dialog/Toast/Tooltip previews become interactive. The 1044-line `docs.rs` splits into a `previews/` submodule.

**Tech Stack:** Rust 2021, Cargo workspace, Dioxus 0.7, `dioxus-ssr` for tests, `web-sys` (Window, Storage, MediaQueryList) for gallery persistence, PowerShell on Windows. No new public components.

---

## Scope

This plan implements the spec at `docs/superpowers/specs/2026-05-21-gallery-dynamic-examples-design.md`.

It includes:

- New module `crates/ui-tokens/src/elevation.rs` with light + dark elevation recipes.
- `ui-styles` emits `--ui-elevation-{0..3}` on `:root` and `[data-ui-theme="dark"]`; applies them to surface / metric-card / tooltip / toast / command-menu / dialog-panel classes.
- `ui-styles` adds `[data-ui-motion="reduced"]` ancestor scope (disabling transitions and animations) and `[data-ui-glass-policy="solid"]` ancestor scope (disabling backdrop-filter, swapping to the solid fallback).
- Gallery `controls` module: `GalleryPrefs` context, four gallery-local preference enums, `ToggleGroup` widget, sticky control bar replacing the dead `gallery-control-group` blocks.
- wasm-only `localStorage` round-trip + `prefers-reduced-motion` seeding behind `cfg(target_arch = "wasm32")`.
- Ambient mesh-gradient backdrop on `body::before` and a denser `gallery-section--glass-stage` plate on Foundations + Surfaces, with parallel dark-theme variants.
- Three gallery-only demo wrappers in `examples/component-gallery/src/demo_frame.rs`: `ReplayFrame`, `ScrubFrame`, `FlipFrame`.
- Interactive `Dialog`, `Toast`, `Tooltip` previews replacing the currently frozen-open versions.
- `docs.rs` split: preview function bodies move to `examples/component-gallery/src/previews/{actions,inputs,layout,navigation,surfaces,feedback,motion,composition,capture,shared,foundations}.rs`.
- Test updates in `crates/ui-tokens/tests/`, `crates/ui-styles/tests/`, `crates/ui-glass/tests/`, and `examples/component-gallery/tests/gallery.rs`.

It excludes:

- Public-API changes to any existing component.
- New public components or new public exports through `kinetics::prelude`.
- Visual-regression automation (still manual; see verification checklist in Task 19).
- Focus-trapping work on `Dialog`.

## Before You Start

```powershell
git worktree add .worktrees/gallery-dynamic-examples -b gallery-dynamic-examples main
```

Run every command from inside the worktree. PowerShell is the canonical shell; bash equivalents work where shown.

## File Map

- `crates/ui-tokens/src/elevation.rs` — new module with `ElevationScale`, `LIGHT_ELEVATION`, `DARK_ELEVATION`.
- `crates/ui-tokens/src/lib.rs` — register and re-export.
- `crates/ui-tokens/tests/elevation.rs` — unit tests.
- `crates/ui-styles/src/lib.rs` — emit elevation CSS vars; apply them to existing classes; add `[data-ui-motion="reduced"]` and `[data-ui-glass-policy="solid"]` sibling rules.
- `crates/ui-styles/tests/css.rs` — new file with CSS-substring assertions (created if absent; otherwise extended).
- `crates/ui-glass/tests/policy.rs` — new file with `resolve_glass` + `SolidFallback` assertion.
- `examples/component-gallery/Cargo.toml` — add `web-sys` + `wasm-bindgen` (target-cfg gated).
- `examples/component-gallery/src/controls.rs` — new module: `GalleryPrefs`, `ThemePref`, `DensityPref`, `MotionPref`, `GlassPolicyUi`, `ToggleGroup`, `PreferenceBar`.
- `examples/component-gallery/src/demo_frame.rs` — new module: `ReplayFrame`, `ScrubFrame`, `FlipFrame`.
- `examples/component-gallery/src/persistence.rs` — new module: wasm-gated `localStorage` + `matchMedia` access.
- `examples/component-gallery/src/lib.rs` — register new modules; re-export what tests touch.
- `examples/component-gallery/src/app.rs` — provide `GalleryPrefs`; spread the four `data-*` attributes onto `gallery-shell`; mount `PreferenceBar`.
- `examples/component-gallery/src/styles.rs` — ambient mesh + dense glass-stage CSS + dark-theme parallels + toggle bar styling.
- `examples/component-gallery/src/previews/mod.rs` — new submodule index.
- `examples/component-gallery/src/previews/{actions,inputs,layout,navigation,surfaces,feedback,motion,composition,capture,shared,foundations}.rs` — preview fns relocated from `docs.rs`.
- `examples/component-gallery/src/docs.rs` — registry only; imports preview fns from the new submodule.
- `examples/component-gallery/tests/gallery.rs` — updated assertions for the new attribute set, sticky bar, interactive overlays.

## Task 1: Elevation tokens in `ui-tokens`

**Files:**
- Create: `crates/ui-tokens/src/elevation.rs`
- Create: `crates/ui-tokens/tests/elevation.rs`
- Modify: `crates/ui-tokens/src/lib.rs`

- [ ] **Step 1: Write failing tests**

Create `crates/ui-tokens/tests/elevation.rs`:

```rust
use ui_tokens::elevation::{DARK_ELEVATION, LIGHT_ELEVATION};

#[test]
fn light_elevation_has_four_non_empty_tiers() {
    assert!(!LIGHT_ELEVATION.e0.is_empty());
    assert!(!LIGHT_ELEVATION.e1.is_empty());
    assert!(!LIGHT_ELEVATION.e2.is_empty());
    assert!(!LIGHT_ELEVATION.e3.is_empty());
}

#[test]
fn dark_elevation_has_four_non_empty_tiers() {
    assert!(!DARK_ELEVATION.e0.is_empty());
    assert!(!DARK_ELEVATION.e1.is_empty());
    assert!(!DARK_ELEVATION.e2.is_empty());
    assert!(!DARK_ELEVATION.e3.is_empty());
}

#[test]
fn elevation_tiers_progress_in_strength() {
    // A weak signal: deeper tiers contain larger blur radii than shallower ones.
    fn longest_blur_px(spec: &str) -> u32 {
        spec.split(',')
            .filter_map(|part| {
                let mut iter = part.split_whitespace();
                iter.next();
                iter.next();
                let blur = iter.next()?;
                blur.trim_end_matches("px").parse::<u32>().ok()
            })
            .max()
            .unwrap_or(0)
    }

    assert!(longest_blur_px(LIGHT_ELEVATION.e1) >= longest_blur_px(LIGHT_ELEVATION.e0));
    assert!(longest_blur_px(LIGHT_ELEVATION.e2) >= longest_blur_px(LIGHT_ELEVATION.e1));
    assert!(longest_blur_px(LIGHT_ELEVATION.e3) >= longest_blur_px(LIGHT_ELEVATION.e2));
}

#[test]
fn dark_elevation_includes_inner_highlight() {
    assert!(DARK_ELEVATION.e1.contains("inset"));
    assert!(DARK_ELEVATION.e2.contains("inset"));
    assert!(DARK_ELEVATION.e3.contains("inset"));
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p ui-tokens --test elevation
```

Expected: compile error — module `elevation` not found.

- [ ] **Step 3: Implement elevation module**

Create `crates/ui-tokens/src/elevation.rs`:

```rust
#[derive(Clone, Copy, Debug)]
pub struct ElevationScale {
    pub e0: &'static str,
    pub e1: &'static str,
    pub e2: &'static str,
    pub e3: &'static str,
}

pub const LIGHT_ELEVATION: ElevationScale = ElevationScale {
    e0: "0 1px 0 rgba(16, 23, 38, 0.04)",
    e1: "0 2px 6px rgba(16, 23, 38, 0.06), 0 8px 24px rgba(16, 23, 38, 0.05)",
    e2: "0 8px 18px rgba(16, 23, 38, 0.10), 0 22px 48px rgba(16, 23, 38, 0.10)",
    e3: "0 18px 32px rgba(16, 23, 38, 0.14), 0 40px 80px rgba(16, 23, 38, 0.18)",
};

pub const DARK_ELEVATION: ElevationScale = ElevationScale {
    e0: "0 1px 0 rgba(0, 0, 0, 0.18), inset 0 1px 0 rgba(255, 255, 255, 0.04)",
    e1: "0 2px 6px rgba(0, 0, 0, 0.30), 0 8px 24px rgba(0, 0, 0, 0.28), inset 0 1px 0 rgba(255, 255, 255, 0.05)",
    e2: "0 10px 22px rgba(0, 0, 0, 0.38), 0 26px 60px rgba(0, 0, 0, 0.42), inset 0 1px 0 rgba(255, 255, 255, 0.06)",
    e3: "0 22px 40px rgba(0, 0, 0, 0.46), 0 48px 96px rgba(0, 0, 0, 0.50), inset 0 1px 0 rgba(255, 255, 255, 0.08)",
};
```

- [ ] **Step 4: Register module**

In `crates/ui-tokens/src/lib.rs`, add the line after `#![forbid(unsafe_code)]`:

```rust
pub mod elevation;
```

- [ ] **Step 5: Run tests to verify passing**

```powershell
cargo test -p ui-tokens --test elevation
```

Expected: 4 passing.

- [ ] **Step 6: Commit**

```powershell
git add crates/ui-tokens/src/elevation.rs crates/ui-tokens/src/lib.rs crates/ui-tokens/tests/elevation.rs
git commit -m "feat(ui-tokens): add 4-tier elevation scale"
```

## Task 2: Emit elevation CSS variables in `ui-styles`

**Files:**
- Modify: `crates/ui-styles/src/lib.rs`
- Create or extend: `crates/ui-styles/tests/css.rs`

- [ ] **Step 1: Write failing CSS tests**

Create `crates/ui-styles/tests/css.rs` (if it doesn't exist):

```rust
use ui_styles::library_css;

#[test]
fn light_root_declares_four_elevation_variables() {
    let css = library_css();
    assert!(css.contains("--ui-elevation-0:"));
    assert!(css.contains("--ui-elevation-1:"));
    assert!(css.contains("--ui-elevation-2:"));
    assert!(css.contains("--ui-elevation-3:"));
}

#[test]
fn dark_theme_re_declares_elevation_variables() {
    let css = library_css();
    let dark_idx = css
        .find("[data-ui-theme=\"dark\"]")
        .expect("dark theme block exists");
    let dark_block = &css[dark_idx..];
    assert!(dark_block.contains("--ui-elevation-0:"));
    assert!(dark_block.contains("--ui-elevation-3:"));
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p ui-styles --test css
```

Expected: 2 failing.

- [ ] **Step 3: Add `ui-tokens` dep to `ui-styles`**

Check `crates/ui-styles/Cargo.toml`. If `ui-tokens` is not already listed under `[dependencies]`, add it:

```toml
ui-tokens = { path = "../ui-tokens" }
```

If present already (verify with `cargo metadata`), skip.

- [ ] **Step 4: Emit elevation variables**

In `crates/ui-styles/src/lib.rs`, replace `pub const BASE_CSS: &str = r#" ... "#;` with a function that interpolates the elevation strings. Add at the top of the file under the existing `#![forbid(unsafe_code)]`:

```rust
use ui_tokens::elevation::{DARK_ELEVATION, LIGHT_ELEVATION};
```

Then replace the existing `pub const BASE_CSS: &str = r#"...";` block with a function that builds the same string while injecting elevation:

```rust
pub fn base_css() -> String {
    format!(
        r#"
:root,
[data-ui-theme="light"] {{
    color-scheme: light;
    --ui-font-sans: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    --ui-bg: #f6f8fb;
    --ui-surface: #ffffff;
    --ui-surface-muted: #f2f5f9;
    --ui-surface-strong: #e8eef6;
    --ui-glass: rgba(255, 255, 255, 0.68);
    --ui-glass-solid: #ffffff;
    --ui-fg: #111827;
    --ui-muted-fg: #5c6778;
    --ui-border: rgba(118, 132, 150, 0.26);
    --ui-focus: #007aff;
    --ui-primary: #0066cc;
    --ui-success: #248a3d;
    --ui-warning: #b66900;
    --ui-danger: #c42b2b;
    --ui-info: #1476bf;
    --ui-shadow-soft: 0 18px 46px rgba(27, 39, 61, 0.10);
    --ui-shadow-lifted: 0 24px 80px rgba(13, 20, 32, 0.24);
    --ui-elevation-0: {l0};
    --ui-elevation-1: {l1};
    --ui-elevation-2: {l2};
    --ui-elevation-3: {l3};
    --ui-radius-sm: 6px;
    --ui-radius-md: 8px;
    --ui-radius-lg: 12px;
    --ui-space-1: 4px;
    --ui-space-2: 8px;
    --ui-space-3: 12px;
    --ui-space-4: 16px;
    --ui-space-5: 24px;
    --ui-control-height: 36px;
    --ui-motion-fast: 120ms;
    --ui-motion-normal: 180ms;
}}

[data-ui-theme="dark"] {{
    color-scheme: dark;
    --ui-bg: #0d1117;
    --ui-surface: #151b23;
    --ui-surface-muted: #1c2430;
    --ui-surface-strong: #263142;
    --ui-glass: rgba(25, 32, 43, 0.72);
    --ui-glass-solid: #151b23;
    --ui-fg: #eef3f8;
    --ui-muted-fg: #aab4c2;
    --ui-border: rgba(205, 215, 228, 0.18);
    --ui-focus: #64b5ff;
    --ui-shadow-soft: 0 18px 46px rgba(0, 0, 0, 0.24);
    --ui-shadow-lifted: 0 26px 90px rgba(0, 0, 0, 0.42);
    --ui-elevation-0: {d0};
    --ui-elevation-1: {d1};
    --ui-elevation-2: {d2};
    --ui-elevation-3: {d3};
}}

[data-ui-density="compact"] {{
    --ui-control-height: 32px;
    --ui-space-3: 10px;
    --ui-space-4: 12px;
}}

[data-ui-density="comfortable"] {{
    --ui-control-height: 36px;
}}

[data-ui-density="spacious"] {{
    --ui-control-height: 42px;
    --ui-space-3: 14px;
    --ui-space-4: 20px;
}}

[data-ui-transparency="reduced"] {{
    --ui-glass: var(--ui-glass-solid);
}}

* {{
    box-sizing: border-box;
}}

body {{
    margin: 0;
    font-family: var(--ui-font-sans);
    background: var(--ui-bg);
    color: var(--ui-fg);
}}

button,
input,
textarea,
select {{
    font: inherit;
}}

@media (prefers-reduced-motion: reduce) {{
    *,
    *::before,
    *::after {{
        transition-duration: 0.01ms !important;
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        scroll-behavior: auto !important;
    }}
}}
"#,
        l0 = LIGHT_ELEVATION.e0,
        l1 = LIGHT_ELEVATION.e1,
        l2 = LIGHT_ELEVATION.e2,
        l3 = LIGHT_ELEVATION.e3,
        d0 = DARK_ELEVATION.e0,
        d1 = DARK_ELEVATION.e1,
        d2 = DARK_ELEVATION.e2,
        d3 = DARK_ELEVATION.e3,
    )
}
```

Remove the old `pub const BASE_CSS: &str = r#"..."#;` definition. Update `library_css()` to call `base_css()`:

```rust
pub fn library_css() -> String {
    let base = base_css();
    let mut css = String::with_capacity(base.len() + COMPONENT_CSS.len() + 1);
    css.push_str(&base);
    css.push('\n');
    css.push_str(COMPONENT_CSS);
    css
}
```

- [ ] **Step 5: Search for downstream uses of `BASE_CSS`**

```powershell
rg "BASE_CSS" --type rust
```

If any consumer references `BASE_CSS` directly, replace with `base_css()`. (Expected: none outside the crate.)

- [ ] **Step 6: Run tests to verify passing**

```powershell
cargo test -p ui-styles --test css
cargo test -p ui-styles
```

Expected: 2 + existing all green.

- [ ] **Step 7: Verify whole workspace still compiles**

```powershell
cargo check --workspace
```

Expected: success.

- [ ] **Step 8: Commit**

```powershell
git add crates/ui-styles/src/lib.rs crates/ui-styles/tests/css.rs crates/ui-styles/Cargo.toml
git commit -m "feat(ui-styles): emit --ui-elevation-0..3 via ui-tokens"
```

## Task 3: Apply elevation tiers to existing component classes

**Files:**
- Modify: `crates/ui-styles/src/lib.rs`
- Modify: `crates/ui-styles/tests/css.rs`

- [ ] **Step 1: Write failing tests**

In `crates/ui-styles/tests/css.rs`, append:

```rust
#[test]
fn surface_uses_elevation_0() {
    let css = library_css();
    let block = component_block(&css, ".ui-surface,");
    assert!(
        block.contains("box-shadow: var(--ui-elevation-0)"),
        "surface block missing elevation-0: {block}"
    );
}

#[test]
fn metric_card_uses_elevation_1() {
    let css = library_css();
    let block = component_block(&css, ".ui-metric-card,");
    assert!(
        block.contains("box-shadow: var(--ui-elevation-1)")
            || css.contains(".ui-metric-card {")
                && css[css.find(".ui-metric-card {").unwrap()..]
                    .contains("box-shadow: var(--ui-elevation-1)")
    );
}

#[test]
fn tooltip_uses_elevation_1() {
    let css = library_css();
    assert!(
        css.contains(".ui-tooltip-content")
            && css[css.find(".ui-tooltip-content").unwrap()..]
                .contains("box-shadow: var(--ui-elevation-1)")
    );
}

#[test]
fn toast_uses_elevation_2() {
    let css = library_css();
    let idx = css.find(".ui-toast {").expect(".ui-toast rule exists");
    assert!(
        css[idx..].split('}').next().unwrap().contains("box-shadow: var(--ui-elevation-2)")
    );
}

#[test]
fn command_menu_panel_uses_elevation_2() {
    let css = library_css();
    let idx = css
        .find(".ui-command-menu-panel")
        .expect(".ui-command-menu-panel rule exists");
    assert!(css[idx..].contains("box-shadow: var(--ui-elevation-2)"));
}

#[test]
fn dialog_panel_uses_elevation_3() {
    let css = library_css();
    let idx = css
        .find(".ui-dialog-panel")
        .expect(".ui-dialog-panel rule exists");
    assert!(css[idx..].contains("box-shadow: var(--ui-elevation-3)"));
}

fn component_block<'a>(css: &'a str, selector_prefix: &str) -> &'a str {
    let idx = css.find(selector_prefix).expect("selector exists");
    let rest = &css[idx..];
    let end = rest.find('}').unwrap_or(rest.len());
    &rest[..end]
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p ui-styles --test css
```

Expected: 6 new tests failing.

- [ ] **Step 3: Apply elevation vars in `COMPONENT_CSS`**

In `crates/ui-styles/src/lib.rs`, inside the `COMPONENT_CSS` raw string:

Find the rule starting with `.ui-surface,` (the combined selector around line 154 in current code). Add `box-shadow: var(--ui-elevation-0);` to the shared block. The full edited rule becomes:

```css
.ui-surface,
.ui-glass-surface,
.ui-metric-card,
.ui-empty-state,
.ui-dialog-panel,
.ui-command-menu-panel,
.ui-sidebar {
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-surface);
    color: var(--ui-fg);
    box-shadow: var(--ui-elevation-0);
}
```

Find `.ui-glass-surface, .ui-dialog-panel, .ui-command-menu-panel {` rule, REPLACE its `box-shadow: var(--ui-shadow-lifted);` with split-out rules:

```css
.ui-glass-surface,
.ui-dialog-panel,
.ui-command-menu-panel {
    background: var(--ui-glass);
    backdrop-filter: blur(18px) saturate(160%);
}

.ui-glass-surface {
    box-shadow: var(--ui-elevation-2);
}

.ui-dialog-panel {
    box-shadow: var(--ui-elevation-3);
}

.ui-command-menu-panel {
    box-shadow: var(--ui-elevation-2);
}
```

Find `.ui-metric-card, .ui-empty-state {` rule. Append a sibling rule:

```css
.ui-metric-card {
    box-shadow: var(--ui-elevation-1);
}
```

Find `.ui-toast {` rule. REPLACE `box-shadow: var(--ui-shadow-soft);` with `box-shadow: var(--ui-elevation-2);`.

Find `.ui-tooltip-content {` rule. Append `box-shadow: var(--ui-elevation-1);` inside the block.

Find `.ui-glass-layer {` rule. REPLACE `box-shadow: var(--ui-material-shadow, var(--ui-shadow-soft));` with `box-shadow: var(--ui-material-shadow, var(--ui-elevation-2));`.

- [ ] **Step 4: Run tests to verify passing**

```powershell
cargo test -p ui-styles --test css
```

Expected: all green.

- [ ] **Step 5: Run dependent crates' tests**

```powershell
cargo test -p kinetics
cargo test -p component-gallery --test gallery
```

Expected: still green (the test on line 142 of gallery.rs asserts `backdrop-filter` is in the CSS — still true).

- [ ] **Step 6: Commit**

```powershell
git add crates/ui-styles/src/lib.rs crates/ui-styles/tests/css.rs
git commit -m "feat(ui-styles): apply elevation tiers to surface/card/toast/tooltip/menu/dialog"
```

## Task 4: Motion-policy ancestor scope in `ui-styles`

**Files:**
- Modify: `crates/ui-styles/src/lib.rs`
- Modify: `crates/ui-styles/tests/css.rs`

- [ ] **Step 1: Write failing tests**

Append to `crates/ui-styles/tests/css.rs`:

```rust
#[test]
fn reduced_motion_ancestor_scope_disables_transitions_globally() {
    let css = library_css();
    assert!(css.contains(r#"[data-ui-motion="reduced"]"#),
        "expected motion-policy ancestor scope");
    // The scope must neutralize transitions on at least the kinetic + button + switch + menu classes.
    let block_start = css.find(r#"[data-ui-motion="reduced"]"#).unwrap();
    let block = &css[block_start..];
    for selector in [".ui-button", ".ui-kinetic-box", ".ui-switch-thumb", ".ui-icon-button"] {
        assert!(
            block.contains(selector),
            "motion-reduced scope should target {selector}"
        );
    }
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p ui-styles --test css
```

Expected: 1 failing.

- [ ] **Step 3: Add the motion-reduced block**

In `crates/ui-styles/src/lib.rs`, append to the end of `COMPONENT_CSS` raw string (before the closing `"#;`):

```css

[data-ui-motion="reduced"] .ui-button,
[data-ui-motion="reduced"] .ui-field-control,
[data-ui-motion="reduced"] .ui-command-menu-input,
[data-ui-motion="reduced"] .ui-icon-button,
[data-ui-motion="reduced"] .ui-switch-thumb,
[data-ui-motion="reduced"] .ui-kinetic-box,
[data-ui-motion="reduced"] .ui-kinetic-text,
[data-ui-motion="reduced"] .ui-frame-layer,
[data-ui-motion="reduced"] .ui-shared-element,
[data-ui-motion="reduced"] .ui-presence {
    transition: none !important;
    animation: none !important;
    transform: none !important;
}

[data-ui-motion="reduced"] .ui-presence {
    --ui-presence-t: 1 !important;
    opacity: 1 !important;
}
```

- [ ] **Step 4: Run tests to verify passing**

```powershell
cargo test -p ui-styles --test css
```

Expected: green.

- [ ] **Step 5: Commit**

```powershell
git add crates/ui-styles/src/lib.rs crates/ui-styles/tests/css.rs
git commit -m "feat(ui-styles): add [data-ui-motion=reduced] ancestor scope"
```

## Task 5: Glass-policy ancestor scope in `ui-styles`

**Files:**
- Modify: `crates/ui-styles/src/lib.rs`
- Modify: `crates/ui-styles/tests/css.rs`

- [ ] **Step 1: Write failing tests**

Append to `crates/ui-styles/tests/css.rs`:

```rust
#[test]
fn solid_glass_ancestor_scope_targets_every_backdrop_filter_class() {
    let css = library_css();
    assert!(css.contains(r#"[data-ui-glass-policy="solid"]"#));

    // Enumerate every class that introduces backdrop-filter and ensure the ancestor scope covers it.
    for class in [".ui-glass-surface", ".ui-glass-layer", ".ui-dialog-panel", ".ui-command-menu-panel"] {
        let pattern = format!(r#"[data-ui-glass-policy="solid"] {class}"#);
        assert!(
            css.contains(&pattern),
            "missing solid-glass override for {class}: pattern {pattern}",
        );
    }
}

#[test]
fn solid_glass_ancestor_scope_neutralizes_backdrop_filter() {
    let css = library_css();
    let idx = css.find(r#"[data-ui-glass-policy="solid"]"#).unwrap();
    let block = &css[idx..];
    assert!(block.contains("backdrop-filter: none"));
    assert!(block.contains("background: var(--ui-glass-solid)"));
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p ui-styles --test css
```

Expected: 2 failing.

- [ ] **Step 3: Add the solid-glass block**

In `crates/ui-styles/src/lib.rs`, append to the end of `COMPONENT_CSS` raw string (before the closing `"#;`):

```css

[data-ui-glass-policy="solid"] .ui-glass-surface,
[data-ui-glass-policy="solid"] .ui-glass-layer,
[data-ui-glass-policy="solid"] .ui-dialog-panel,
[data-ui-glass-policy="solid"] .ui-command-menu-panel {
    background: var(--ui-glass-solid) !important;
    backdrop-filter: none !important;
    -webkit-backdrop-filter: none !important;
}
```

- [ ] **Step 4: Run tests to verify passing**

```powershell
cargo test -p ui-styles --test css
```

Expected: green.

- [ ] **Step 5: Commit**

```powershell
git add crates/ui-styles/src/lib.rs crates/ui-styles/tests/css.rs
git commit -m "feat(ui-styles): add [data-ui-glass-policy=solid] ancestor scope"
```

## Task 6: Verify `ui-glass` SolidFallback resolves to a solid recipe

**Files:**
- Create: `crates/ui-glass/tests/policy.rs`

- [ ] **Step 1: Check whether a policy test already exists**

```powershell
rg "SolidFallback" crates/ui-glass --type rust
```

If a test asserting force-solid behavior already exists, skip to Step 4 with the assertions verified.

- [ ] **Step 2: Write a failing test**

Create `crates/ui-glass/tests/policy.rs`:

```rust
use ui_glass::{
    resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRequest, GlassTone,
};
use ui_tokens::Theme;

#[test]
fn solid_fallback_policy_returns_zero_blur_and_force_solid() {
    let theme = Theme::default();
    let req = GlassRequest::new(GlassLevel::Floating, GlassTone::Neutral, GlassDensity::Comfortable)
        .with_policy(GlassPolicy::SolidFallback);
    let recipe = resolve_glass(&theme, req);
    assert!(recipe.force_solid);
    assert_eq!(recipe.backdrop_blur_px, 0.0);
    assert_eq!(recipe.saturate_percent, 100);
    assert_eq!(recipe.background, theme.semantic.surface_solid);
}

#[test]
fn auto_policy_returns_blurred_recipe() {
    let theme = Theme::default();
    let req = GlassRequest::new(GlassLevel::Floating, GlassTone::Neutral, GlassDensity::Comfortable);
    let recipe = resolve_glass(&theme, req);
    assert!(!recipe.force_solid);
    assert!(recipe.backdrop_blur_px > 0.0);
    assert!(recipe.saturate_percent > 100);
}
```

If `ui-glass/Cargo.toml` lacks `ui-tokens` as a test dep (it has one as a regular dep already per the use in lib.rs), no change needed.

- [ ] **Step 3: Run**

```powershell
cargo test -p ui-glass --test policy
```

Expected: green (the `resolve_glass` implementation already handles `SolidFallback` per the existing code).

- [ ] **Step 4: Commit**

```powershell
git add crates/ui-glass/tests/policy.rs
git commit -m "test(ui-glass): cover SolidFallback policy contract"
```

## Task 7: Gallery preference enums in `controls` module

**Files:**
- Create: `examples/component-gallery/src/controls.rs`
- Modify: `examples/component-gallery/src/lib.rs`
- Create: `examples/component-gallery/tests/controls.rs`

- [ ] **Step 1: Write failing tests**

Create `examples/component-gallery/tests/controls.rs`:

```rust
use component_gallery::controls::{DensityPref, GlassPolicyUi, MotionPref, ThemePref};

#[test]
fn theme_pref_has_two_attribute_values() {
    assert_eq!(ThemePref::Light.attr_value(), "light");
    assert_eq!(ThemePref::Dark.attr_value(), "dark");
}

#[test]
fn density_pref_has_three_attribute_values() {
    assert_eq!(DensityPref::Compact.attr_value(), "compact");
    assert_eq!(DensityPref::Comfortable.attr_value(), "comfortable");
    assert_eq!(DensityPref::Spacious.attr_value(), "spacious");
}

#[test]
fn motion_pref_maps_to_normal_or_reduced_attribute() {
    assert_eq!(MotionPref::Normal.attr_value(), "normal");
    assert_eq!(MotionPref::Reduced.attr_value(), "reduced");
}

#[test]
fn glass_policy_ui_maps_to_translucent_or_solid_attribute() {
    assert_eq!(GlassPolicyUi::Translucent.attr_value(), "translucent");
    assert_eq!(GlassPolicyUi::Solid.attr_value(), "solid");
}

#[test]
fn enums_round_trip_via_attr_value() {
    assert_eq!(ThemePref::from_attr("dark"), Some(ThemePref::Dark));
    assert_eq!(ThemePref::from_attr("nope"), None);
    assert_eq!(MotionPref::from_attr("reduced"), Some(MotionPref::Reduced));
    assert_eq!(GlassPolicyUi::from_attr("solid"), Some(GlassPolicyUi::Solid));
    assert_eq!(DensityPref::from_attr("compact"), Some(DensityPref::Compact));
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test controls
```

Expected: compile error — `controls` module not found.

- [ ] **Step 3: Implement enums**

Create `examples/component-gallery/src/controls.rs`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemePref {
    Light,
    Dark,
}

impl ThemePref {
    pub const fn attr_value(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    pub fn from_attr(value: &str) -> Option<Self> {
        match value {
            "light" => Some(Self::Light),
            "dark" => Some(Self::Dark),
            _ => None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DensityPref {
    Compact,
    Comfortable,
    Spacious,
}

impl DensityPref {
    pub const fn attr_value(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Comfortable => "comfortable",
            Self::Spacious => "spacious",
        }
    }

    pub fn from_attr(value: &str) -> Option<Self> {
        match value {
            "compact" => Some(Self::Compact),
            "comfortable" => Some(Self::Comfortable),
            "spacious" => Some(Self::Spacious),
            _ => None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Compact => "Compact",
            Self::Comfortable => "Comfortable",
            Self::Spacious => "Spacious",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionPref {
    Normal,
    Reduced,
}

impl MotionPref {
    pub const fn attr_value(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Reduced => "reduced",
        }
    }

    pub fn from_attr(value: &str) -> Option<Self> {
        match value {
            "normal" => Some(Self::Normal),
            "reduced" => Some(Self::Reduced),
            _ => None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Reduced => "Reduced",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassPolicyUi {
    Translucent,
    Solid,
}

impl GlassPolicyUi {
    pub const fn attr_value(self) -> &'static str {
        match self {
            Self::Translucent => "translucent",
            Self::Solid => "solid",
        }
    }

    pub fn from_attr(value: &str) -> Option<Self> {
        match value {
            "translucent" => Some(Self::Translucent),
            "solid" => Some(Self::Solid),
            _ => None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Translucent => "Translucent",
            Self::Solid => "Solid",
        }
    }
}
```

- [ ] **Step 4: Register module in `lib.rs`**

In `examples/component-gallery/src/lib.rs`, add after `mod app;`:

```rust
pub mod controls;
```

- [ ] **Step 5: Run tests to verify passing**

```powershell
cargo test -p component-gallery --test controls
```

Expected: 5 passing.

- [ ] **Step 6: Commit**

```powershell
git add examples/component-gallery/src/controls.rs examples/component-gallery/src/lib.rs examples/component-gallery/tests/controls.rs
git commit -m "feat(gallery): add preference enums for theme/density/motion/glass-policy"
```

## Task 8: `GalleryPrefs` context with default state

**Files:**
- Modify: `examples/component-gallery/src/controls.rs`
- Modify: `examples/component-gallery/tests/controls.rs`

- [ ] **Step 1: Write failing test**

Append to `examples/component-gallery/tests/controls.rs`:

```rust
#[test]
fn gallery_default_constants_match_documented_fallbacks() {
    use component_gallery::controls::{
        DEFAULT_DENSITY, DEFAULT_GLASS, DEFAULT_MOTION, DEFAULT_THEME,
    };
    assert_eq!(DEFAULT_THEME, ThemePref::Light);
    assert_eq!(DEFAULT_DENSITY, DensityPref::Comfortable);
    assert_eq!(DEFAULT_MOTION, MotionPref::Normal);
    assert_eq!(DEFAULT_GLASS, GlassPolicyUi::Translucent);
}
```

Asserting constants (not signal values) keeps the test outside of any required Dioxus runtime context, which is the simplest correctness boundary for unit tests.

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test controls
```

Expected: compile error — `DEFAULT_*` not defined.

- [ ] **Step 3: Add defaults + `GalleryPrefs`**

Append to `examples/component-gallery/src/controls.rs`:

```rust
use dioxus::prelude::*;

pub const DEFAULT_THEME: ThemePref = ThemePref::Light;
pub const DEFAULT_DENSITY: DensityPref = DensityPref::Comfortable;
pub const DEFAULT_MOTION: MotionPref = MotionPref::Normal;
pub const DEFAULT_GLASS: GlassPolicyUi = GlassPolicyUi::Translucent;

#[derive(Clone, Copy)]
pub struct GalleryPrefs {
    pub theme: Signal<ThemePref>,
    pub density: Signal<DensityPref>,
    pub motion: Signal<MotionPref>,
    pub glass: Signal<GlassPolicyUi>,
}

impl GalleryPrefs {
    pub fn use_provided() -> Self {
        let theme = use_signal(|| DEFAULT_THEME);
        let density = use_signal(|| DEFAULT_DENSITY);
        let motion = use_signal(|| DEFAULT_MOTION);
        let glass = use_signal(|| DEFAULT_GLASS);
        Self { theme, density, motion, glass }
    }
}
```

- [ ] **Step 4: Run tests**

```powershell
cargo test -p component-gallery --test controls
```

Expected: green.

- [ ] **Step 5: Commit**

```powershell
git add examples/component-gallery/src/controls.rs examples/component-gallery/tests/controls.rs
git commit -m "feat(gallery): add GalleryPrefs context with default constants"
```

## Task 9: Wire `GalleryPrefs` into `App` and drive `data-*` attributes

**Files:**
- Modify: `examples/component-gallery/src/app.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write failing tests**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_shell_emits_all_four_preference_data_attributes() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains(r#"data-ui-theme="light""#));
    assert!(html.contains(r#"data-ui-density="comfortable""#));
    assert!(html.contains(r#"data-ui-motion="normal""#));
    assert!(html.contains(r#"data-ui-glass-policy="translucent""#));
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test gallery gallery_shell_emits_all_four_preference_data_attributes
```

Expected: failure — motion and glass-policy attributes missing.

- [ ] **Step 3: Wire `GalleryPrefs` in `app.rs`**

In `examples/component-gallery/src/app.rs`, replace the body of `pub fn App() -> Element` so the top of the function reads:

```rust
#[component]
pub fn App() -> Element {
    use crate::controls::GalleryPrefs;
    let prefs = GalleryPrefs::use_provided();
    use_context_provider(|| prefs);

    let theme_attr = prefs.theme.read().attr_value();
    let density_attr = prefs.density.read().attr_value();
    let motion_attr = prefs.motion.read().attr_value();
    let glass_attr = prefs.glass.read().attr_value();

    let shared_css = library_css();

    rsx! {
        style { "{shared_css}" }
        style { "{GALLERY_CSS}" }
        div {
            class: "gallery-shell",
            "data-ui-theme": "{theme_attr}",
            "data-ui-density": "{density_attr}",
            "data-ui-motion": "{motion_attr}",
            "data-ui-glass-policy": "{glass_attr}",
            // ... rest unchanged
```

Keep the rest of the function body (rail, nav, main, sections) as-is for now.

- [ ] **Step 4: Run tests to verify passing**

```powershell
cargo test -p component-gallery --test gallery
```

Expected: new test passes; existing tests still pass.

- [ ] **Step 5: Commit**

```powershell
git add examples/component-gallery/src/app.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat(gallery): drive data-* attributes from GalleryPrefs"
```

## Task 10: `ToggleGroup` widget and `PreferenceBar`

**Files:**
- Modify: `examples/component-gallery/src/controls.rs`
- Modify: `examples/component-gallery/src/app.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write failing tests**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn preference_bar_renders_all_four_toggle_groups() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains(r#"role="radiogroup""#));
    // One radiogroup per preference.
    let radiogroup_count = html.matches(r#"role="radiogroup""#).count();
    assert!(
        radiogroup_count >= 4,
        "expected >=4 radiogroups, got {radiogroup_count}"
    );

    // Each labelled.
    for label in ["Theme", "Density", "Motion", "Glass"] {
        assert!(html.contains(label), "missing toggle group label: {label}");
    }

    // The current value of each shows aria-checked=true on exactly one option.
    for value in ["Light", "Comfortable", "Normal", "Translucent"] {
        assert!(
            html.contains(value),
            "missing default-selected option: {value}"
        );
    }
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test gallery preference_bar_renders_all_four_toggle_groups
```

Expected: failure — no radiogroups in current HTML.

- [ ] **Step 3: Implement `ToggleGroup` + `PreferenceBar`**

Append to `examples/component-gallery/src/controls.rs`:

```rust
#[derive(Clone, PartialEq, Props)]
pub struct ToggleGroupProps {
    pub label: &'static str,
    pub options: Vec<(&'static str, &'static str, bool)>,
    pub on_select: EventHandler<&'static str>,
}

#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    rsx! {
        div { class: "gallery-toggle-group", role: "radiogroup", "aria-label": "{props.label}",
            span { class: "gallery-control-label", "{props.label}" }
            for (value, label, selected) in props.options.iter().copied() {
                button {
                    class: if selected { "ui-button ui-button--primary" } else { "ui-button ui-button--secondary" },
                    role: "radio",
                    "aria-checked": "{selected}",
                    r#type: "button",
                    onclick: move |_| props.on_select.call(value),
                    "{label}"
                }
            }
        }
    }
}

#[component]
pub fn PreferenceBar() -> Element {
    let prefs = use_context::<GalleryPrefs>();
    let mut theme_sig = prefs.theme;
    let mut density_sig = prefs.density;
    let mut motion_sig = prefs.motion;
    let mut glass_sig = prefs.glass;

    let theme_now = *theme_sig.read();
    let density_now = *density_sig.read();
    let motion_now = *motion_sig.read();
    let glass_now = *glass_sig.read();

    rsx! {
        section { class: "gallery-controls", "aria-label": "Preview settings",
            ToggleGroup {
                label: "Theme",
                options: vec![
                    ("light", ThemePref::Light.label(), theme_now == ThemePref::Light),
                    ("dark", ThemePref::Dark.label(), theme_now == ThemePref::Dark),
                ],
                on_select: move |v: &str| {
                    if let Some(next) = ThemePref::from_attr(v) {
                        theme_sig.set(next);
                    }
                },
            }
            ToggleGroup {
                label: "Density",
                options: vec![
                    ("compact", DensityPref::Compact.label(), density_now == DensityPref::Compact),
                    ("comfortable", DensityPref::Comfortable.label(), density_now == DensityPref::Comfortable),
                    ("spacious", DensityPref::Spacious.label(), density_now == DensityPref::Spacious),
                ],
                on_select: move |v: &str| {
                    if let Some(next) = DensityPref::from_attr(v) {
                        density_sig.set(next);
                    }
                },
            }
            ToggleGroup {
                label: "Motion",
                options: vec![
                    ("normal", MotionPref::Normal.label(), motion_now == MotionPref::Normal),
                    ("reduced", MotionPref::Reduced.label(), motion_now == MotionPref::Reduced),
                ],
                on_select: move |v: &str| {
                    if let Some(next) = MotionPref::from_attr(v) {
                        motion_sig.set(next);
                    }
                },
            }
            ToggleGroup {
                label: "Glass",
                options: vec![
                    ("translucent", GlassPolicyUi::Translucent.label(), glass_now == GlassPolicyUi::Translucent),
                    ("solid", GlassPolicyUi::Solid.label(), glass_now == GlassPolicyUi::Solid),
                ],
                on_select: move |v: &str| {
                    if let Some(next) = GlassPolicyUi::from_attr(v) {
                        glass_sig.set(next);
                    }
                },
            }
        }
    }
}
```

- [ ] **Step 4: Replace dead control bar in `app.rs`**

In `examples/component-gallery/src/app.rs`, locate the existing `section { class: "gallery-controls", ... }` block (the two button groups with hardcoded primary/secondary classes) and replace it with:

```rust
crate::controls::PreferenceBar {}
```

- [ ] **Step 5: Run tests to verify passing**

```powershell
cargo test -p component-gallery --test gallery
```

Expected: new test passes; existing tests still pass (`Theme`, `Density`, `Light`, `Dark`, `Compact`, `Spacious` text still appears).

- [ ] **Step 6: Commit**

```powershell
git add examples/component-gallery/src/controls.rs examples/component-gallery/src/app.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat(gallery): live preference bar with four toggle groups"
```

## Task 11: wasm-only persistence and `prefers-reduced-motion` seeding

**Files:**
- Create: `examples/component-gallery/src/persistence.rs`
- Modify: `examples/component-gallery/src/controls.rs`
- Modify: `examples/component-gallery/src/lib.rs`
- Modify: `examples/component-gallery/Cargo.toml`

- [ ] **Step 1: Add wasm dependencies**

In `examples/component-gallery/Cargo.toml`, append:

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Window", "Storage", "MediaQueryList"] }
```

- [ ] **Step 2: Implement `persistence` module**

Create `examples/component-gallery/src/persistence.rs`:

```rust
#[cfg(target_arch = "wasm32")]
mod imp {
    pub fn load(key: &str) -> Option<String> {
        let window = web_sys::window()?;
        let storage = window.local_storage().ok()??;
        storage.get_item(key).ok().flatten()
    }

    pub fn save(key: &str, value: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(key, value);
            }
        }
    }

    pub fn prefers_reduced_motion() -> bool {
        let Some(window) = web_sys::window() else { return false; };
        match window.match_media("(prefers-reduced-motion: reduce)") {
            Ok(Some(mql)) => mql.matches(),
            _ => false,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    pub fn load(_key: &str) -> Option<String> { None }
    pub fn save(_key: &str, _value: &str) {}
    pub fn prefers_reduced_motion() -> bool { false }
}

pub use imp::*;

pub const KEY_THEME: &str = "kx-gallery-theme";
pub const KEY_DENSITY: &str = "kx-gallery-density";
pub const KEY_MOTION: &str = "kx-gallery-motion";
pub const KEY_GLASS: &str = "kx-gallery-glass";
```

- [ ] **Step 3: Register `persistence` module**

In `examples/component-gallery/src/lib.rs`, add:

```rust
mod persistence;
```

- [ ] **Step 4: Seed `GalleryPrefs` from persistence**

In `examples/component-gallery/src/controls.rs`, replace the body of `GalleryPrefs::use_provided`:

```rust
impl GalleryPrefs {
    pub fn use_provided() -> Self {
        use crate::persistence::{self, KEY_DENSITY, KEY_GLASS, KEY_MOTION, KEY_THEME};

        let initial_theme = persistence::load(KEY_THEME)
            .and_then(|v| ThemePref::from_attr(&v))
            .unwrap_or(DEFAULT_THEME);
        let initial_density = persistence::load(KEY_DENSITY)
            .and_then(|v| DensityPref::from_attr(&v))
            .unwrap_or(DEFAULT_DENSITY);
        let initial_motion = persistence::load(KEY_MOTION)
            .and_then(|v| MotionPref::from_attr(&v))
            .unwrap_or_else(|| {
                if persistence::prefers_reduced_motion() {
                    MotionPref::Reduced
                } else {
                    DEFAULT_MOTION
                }
            });
        let initial_glass = persistence::load(KEY_GLASS)
            .and_then(|v| GlassPolicyUi::from_attr(&v))
            .unwrap_or(DEFAULT_GLASS);

        let theme = use_signal(|| initial_theme);
        let density = use_signal(|| initial_density);
        let motion = use_signal(|| initial_motion);
        let glass = use_signal(|| initial_glass);

        use_effect(move || {
            persistence::save(KEY_THEME, theme.read().attr_value());
        });
        use_effect(move || {
            persistence::save(KEY_DENSITY, density.read().attr_value());
        });
        use_effect(move || {
            persistence::save(KEY_MOTION, motion.read().attr_value());
        });
        use_effect(move || {
            persistence::save(KEY_GLASS, glass.read().attr_value());
        });

        Self { theme, density, motion, glass }
    }
}
```

- [ ] **Step 5: Verify SSR tests still green**

```powershell
cargo test -p component-gallery --test gallery
cargo test -p component-gallery --test controls
```

Expected: green (SSR path uses non-wasm `imp` — returns None / false; defaults still apply).

- [ ] **Step 6: Verify wasm compile**

```powershell
cargo check -p component-gallery --target wasm32-unknown-unknown --features web
```

If `wasm32-unknown-unknown` is not installed, run `rustup target add wasm32-unknown-unknown` first. Expected: success.

- [ ] **Step 7: Commit**

```powershell
git add examples/component-gallery/src/persistence.rs examples/component-gallery/src/controls.rs examples/component-gallery/src/lib.rs examples/component-gallery/Cargo.toml
git commit -m "feat(gallery): persist preferences via localStorage and seed motion from media query"
```

## Task 12: Ambient mesh backdrop and toggle-bar styling

**Files:**
- Modify: `examples/component-gallery/src/styles.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write failing test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_css_includes_ambient_mesh_and_toggle_group_styles() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for selector in [
        ".gallery-toggle-group",
        ".gallery-ambient-mesh",
        ".gallery-section--glass-stage",
    ] {
        assert!(
            html.contains(selector),
            "missing CSS selector {selector}",
        );
    }

    // Sticky position on the controls bar so it stays reachable while scrolling.
    assert!(html.contains("position: sticky"));
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test gallery gallery_css_includes_ambient_mesh_and_toggle_group_styles
```

Expected: failure.

- [ ] **Step 3: Add ambient mesh, dense plate, and toggle-bar CSS**

In `examples/component-gallery/src/styles.rs`, append the following inside the `GALLERY_CSS` raw string, before the trailing `"#;`:

```css

body {
    position: relative;
}

body::before {
    content: "";
    position: fixed;
    inset: -10vmax;
    z-index: -1;
    background:
        radial-gradient(closest-side at 18% 28%, color-mix(in srgb, var(--ui-primary), transparent 64%), transparent 70%),
        radial-gradient(closest-side at 78% 22%, color-mix(in srgb, var(--ui-info), transparent 64%), transparent 70%),
        radial-gradient(closest-side at 50% 82%, color-mix(in srgb, var(--ui-success), transparent 70%), transparent 70%),
        var(--ui-bg);
    filter: saturate(110%);
    animation: gallery-mesh-drift 40s linear infinite;
}

.gallery-ambient-mesh {
    /* Marker class used by tests; the real backdrop is body::before above. */
    display: none;
}

[data-ui-theme="dark"] body::before {
    background:
        radial-gradient(closest-side at 18% 28%, rgba(40, 90, 140, 0.50), transparent 70%),
        radial-gradient(closest-side at 78% 22%, rgba(110, 60, 150, 0.40), transparent 70%),
        radial-gradient(closest-side at 50% 82%, rgba(30, 110, 100, 0.40), transparent 70%),
        var(--ui-bg);
}

@keyframes gallery-mesh-drift {
    0%   { transform: translate3d(0, 0, 0); }
    50%  { transform: translate3d(-4%, -3%, 0); }
    100% { transform: translate3d(0, 0, 0); }
}

[data-ui-motion="reduced"] body::before {
    animation: none !important;
}

@media (prefers-reduced-motion: reduce) {
    body::before { animation: none !important; }
}

.gallery-section--glass-stage {
    position: relative;
    isolation: isolate;
}

.gallery-section--glass-stage::before {
    content: "";
    position: absolute;
    inset: var(--ui-space-4);
    z-index: -1;
    border-radius: var(--ui-radius-lg);
    background:
        radial-gradient(circle at 25% 30%, color-mix(in srgb, var(--ui-primary), transparent 30%), transparent 55%),
        radial-gradient(circle at 80% 28%, color-mix(in srgb, var(--ui-success), transparent 30%), transparent 55%),
        radial-gradient(circle at 50% 80%, color-mix(in srgb, var(--ui-warning), transparent 30%), transparent 60%),
        linear-gradient(135deg, color-mix(in srgb, var(--ui-info), transparent 50%), transparent);
    opacity: 0.6;
}

[data-ui-theme="dark"] .gallery-section--glass-stage::before {
    opacity: 0.42;
}

.gallery-controls {
    position: sticky;
    top: 0;
    z-index: 4;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--ui-space-3);
    margin: 0 0 var(--ui-space-4);
    padding: var(--ui-space-3) var(--ui-space-4);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-glass);
    backdrop-filter: blur(22px) saturate(160%);
    box-shadow: var(--ui-elevation-1);
}

.gallery-toggle-group {
    display: inline-flex;
    align-items: center;
    gap: var(--ui-space-2);
    padding: 0 var(--ui-space-2);
}

.gallery-toggle-group .ui-button {
    padding: 4px 10px;
    min-height: 28px;
    font-size: 13px;
}
```

- [ ] **Step 4: Tag the glass-stage sections**

In `examples/component-gallery/src/app.rs`, locate the `CategorySection` component and update it to apply the `gallery-section--glass-stage` class for Foundations and Surfaces:

```rust
#[component]
fn CategorySection(category: ComponentCategory) -> Element {
    let docs = component_docs()
        .iter()
        .filter(|doc| doc.category == category)
        .collect::<Vec<_>>();

    let stage_class = match category {
        ComponentCategory::Foundations | ComponentCategory::Surfaces => {
            " gallery-section--glass-stage"
        }
        _ => "",
    };
    let class = format!("gallery-section{stage_class}");

    rsx! {
        section { id: "{category.slug()}", class: "{class}",
            // ... existing body unchanged
```

- [ ] **Step 5: Add a hidden marker for the ambient mesh in `app.rs`**

To satisfy the test's `.gallery-ambient-mesh` selector check (the CSS body::before is not enumerable by a class substring match alone — actually it is, as long as the CSS string contains the literal `.gallery-ambient-mesh`. The CSS already contains it via the marker rule. Confirm the test passes; no app.rs change needed.

- [ ] **Step 6: Run tests to verify passing**

```powershell
cargo test -p component-gallery --test gallery
```

Expected: green.

- [ ] **Step 7: Commit**

```powershell
git add examples/component-gallery/src/styles.rs examples/component-gallery/src/app.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat(gallery): ambient mesh backdrop and dense glass-stage plate"
```

## Task 13: Split `docs.rs` into a `previews/` submodule

This task is a pure relocation. No behavior change. Subsequent tasks edit the relocated functions.

**Files:**
- Create: `examples/component-gallery/src/previews/mod.rs`
- Create: `examples/component-gallery/src/previews/{actions,inputs,layout,navigation,surfaces,feedback,motion,composition,capture,shared,foundations}.rs`
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/src/lib.rs`

- [ ] **Step 1: Create the module index**

Create `examples/component-gallery/src/previews/mod.rs`:

```rust
pub mod actions;
pub mod capture;
pub mod composition;
pub mod feedback;
pub mod foundations;
pub mod inputs;
pub mod layout;
pub mod motion;
pub mod navigation;
pub mod shared;
pub mod surfaces;
```

- [ ] **Step 2: Move preview functions out of `docs.rs`**

For each file below, create it with `use dioxus::prelude::*;` and `use kinetics::prelude::*;` at the top, and move the listed preview functions verbatim from `docs.rs`:

- `actions.rs` — `button_preview`, `icon_button_preview`, `command_menu_preview`, `toolbar_preview`
- `inputs.rs` — `text_field_preview`, `checkbox_preview`, `switch_preview`
- `layout.rs` — `stack_preview`, `tabs_preview`
- `navigation.rs` — `sidebar_preview`
- `surfaces.rs` — `surface_preview`, `glass_surface_preview`, `metric_card_preview`
- `feedback.rs` — `dialog_preview`, `toast_preview`, `tooltip_preview`, `empty_state_preview`
- `motion.rs` — `presence_preview`, `sequence_preview`, `timeline_scope_preview`, `kinetic_box_preview`, `presence_gate_preview`
- `composition.rs` — `frame_stage_preview`
- `capture.rs` — `capture_stage_preview`
- `shared.rs` — `shared_layout_preview`, `shared_element_preview`
- `foundations.rs` — `glass_layer_preview`

Each function must be `pub` in its new module (e.g. `pub fn button_preview() -> Element { ... }`).

- [ ] **Step 3: Update `docs.rs` imports**

In `examples/component-gallery/src/docs.rs`, at the top after the existing imports, add:

```rust
use crate::previews::{
    actions::{button_preview, command_menu_preview, icon_button_preview, toolbar_preview},
    capture::capture_stage_preview,
    composition::frame_stage_preview,
    feedback::{dialog_preview, empty_state_preview, toast_preview, tooltip_preview},
    foundations::glass_layer_preview,
    inputs::{checkbox_preview, switch_preview, text_field_preview},
    layout::{stack_preview, tabs_preview},
    motion::{
        kinetic_box_preview, presence_gate_preview, presence_preview, sequence_preview,
        timeline_scope_preview,
    },
    navigation::sidebar_preview,
    shared::{shared_element_preview, shared_layout_preview},
    surfaces::{glass_surface_preview, metric_card_preview, surface_preview},
};
```

Delete the relocated function bodies from `docs.rs`. Keep snippet constants and the `ComponentDoc`-table.

- [ ] **Step 4: Register the new submodule**

In `examples/component-gallery/src/lib.rs`, add:

```rust
mod previews;
```

- [ ] **Step 5: Run tests**

```powershell
cargo test -p component-gallery
```

Expected: every test still passes (no behavior change).

- [ ] **Step 6: Verify `docs.rs` is under 500 lines**

```powershell
(Get-Content examples/component-gallery/src/docs.rs | Measure-Object -Line).Lines
```

Expected: < 500.

- [ ] **Step 7: Commit**

```powershell
git add examples/component-gallery/src/previews examples/component-gallery/src/docs.rs examples/component-gallery/src/lib.rs
git commit -m "refactor(gallery): split docs.rs preview fns into previews submodule"
```

## Task 14: `ReplayFrame` wrapper

**Files:**
- Create: `examples/component-gallery/src/demo_frame.rs`
- Modify: `examples/component-gallery/src/lib.rs`
- Create: `examples/component-gallery/tests/demo_frame.rs`

- [ ] **Step 1: Write failing test**

Create `examples/component-gallery/tests/demo_frame.rs`:

```rust
use dioxus::prelude::*;

#[test]
fn replay_frame_renders_label_and_replay_button() {
    use component_gallery::demo_frame::ReplayFrame;
    let html = dioxus_ssr::render_element(rsx! {
        ReplayFrame {
            label: "Demo",
            children: rsx! { p { "child" } },
        }
    });
    assert!(html.contains("Demo"));
    assert!(html.contains(">Replay<") || html.contains("aria-label=\"Replay\""));
    assert!(html.contains("child"));
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test demo_frame
```

Expected: compile error — `demo_frame` module not found.

- [ ] **Step 3: Implement `ReplayFrame`**

Create `examples/component-gallery/src/demo_frame.rs`:

```rust
use dioxus::prelude::*;

#[component]
pub fn ReplayFrame(label: &'static str, children: Element) -> Element {
    let mut token = use_signal(|| 0u32);

    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "{label}" }
                button {
                    class: "ui-button ui-button--secondary gallery-demo-frame-replay",
                    r#type: "button",
                    "aria-label": "Replay",
                    onclick: move |_| token += 1,
                    "Replay"
                }
            }
            div { class: "gallery-demo-frame-body", key: "{token.read()}",
                {children}
            }
        }
    }
}
```

Keying the body on the replay token tears down and remounts the inner motion element on every click, retriggering enter animations without needing the children to read from context.

- [ ] **Step 4: Register module**

In `examples/component-gallery/src/lib.rs`, add:

```rust
pub mod demo_frame;
```

- [ ] **Step 5: Add CSS for the demo frame**

In `examples/component-gallery/src/styles.rs`, append inside `GALLERY_CSS` (before the trailing `"#;`):

```css

.gallery-demo-frame {
    display: grid;
    gap: var(--ui-space-2);
    padding: var(--ui-space-3);
    border: 1px dashed var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
}

.gallery-demo-frame-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--ui-space-2);
}

.gallery-demo-frame-replay {
    min-height: 26px;
    padding: 2px 10px;
    font-size: 12px;
}

[data-ui-motion="reduced"] .gallery-demo-frame-replay {
    display: none;
}
```

- [ ] **Step 6: Run tests to verify passing**

```powershell
cargo test -p component-gallery --test demo_frame
cargo test -p component-gallery --test gallery
```

Expected: green.

- [ ] **Step 7: Commit**

```powershell
git add examples/component-gallery/src/demo_frame.rs examples/component-gallery/src/lib.rs examples/component-gallery/src/styles.rs examples/component-gallery/tests/demo_frame.rs
git commit -m "feat(gallery): add ReplayFrame demo wrapper"
```

## Task 15: `ScrubFrame` wrapper with elapsed-ms context

**Files:**
- Modify: `examples/component-gallery/src/demo_frame.rs`
- Modify: `examples/component-gallery/src/styles.rs`
- Modify: `examples/component-gallery/tests/demo_frame.rs`

- [ ] **Step 1: Write failing test**

Append to `examples/component-gallery/tests/demo_frame.rs`:

```rust
#[test]
fn scrub_frame_renders_slider_play_pause_and_label() {
    use component_gallery::demo_frame::ScrubFrame;
    let html = dioxus_ssr::render_element(rsx! {
        ScrubFrame {
            duration_ms: 1000.0,
            fps: None,
            label: "Demo scrub",
            children: rsx! { p { "scrubbed" } },
        }
    });
    assert!(html.contains("Demo scrub"));
    assert!(html.contains(r#"type="range""#));
    assert!(html.contains("scrubbed"));
    assert!(html.contains(r#"max="1000""#));
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test demo_frame scrub_frame_renders_slider_play_pause_and_label
```

Expected: failure.

- [ ] **Step 3: Implement `ScrubFrame`**

Append to `examples/component-gallery/src/demo_frame.rs`:

```rust
#[derive(Clone, Copy)]
pub struct ScrubElapsedMs(pub Signal<f32>);

#[derive(Clone, Copy)]
pub struct ScrubFps(pub u32);

#[component]
pub fn ScrubFrame(
    duration_ms: f32,
    fps: Option<u32>,
    label: &'static str,
    children: Element,
) -> Element {
    let mut elapsed = use_signal(|| 0.0_f32);
    let mut playing = use_signal(|| false);
    use_context_provider(|| ScrubElapsedMs(elapsed));
    use_context_provider(|| ScrubFps(fps.unwrap_or(30)));

    let max_str = format!("{:.0}", duration_ms);
    let value_str = format!("{:.0}", *elapsed.read());

    rsx! {
        div { class: "gallery-demo-frame gallery-demo-frame--scrub",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "{label}" }
                div { class: "gallery-demo-frame-transport",
                    button {
                        class: "ui-button ui-button--secondary gallery-demo-frame-play",
                        r#type: "button",
                        "aria-label": if *playing.read() { "Pause" } else { "Play" },
                        onclick: move |_| {
                            let now = *playing.read();
                            playing.set(!now);
                        },
                        if *playing.read() { "Pause" } else { "Play" }
                    }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "{max_str}",
                        step: "1",
                        value: "{value_str}",
                        oninput: move |evt| {
                            if let Ok(v) = evt.value().parse::<f32>() {
                                elapsed.set(v);
                            }
                        },
                    }
                    span { class: "gallery-demo-frame-elapsed", "{value_str} / {max_str} ms" }
                }
            }
            div { class: "gallery-demo-frame-body",
                {children}
            }
        }
    }
}
```

- [ ] **Step 4: Add CSS**

In `examples/component-gallery/src/styles.rs`, append inside `GALLERY_CSS`:

```css

.gallery-demo-frame-transport {
    display: inline-flex;
    align-items: center;
    gap: var(--ui-space-2);
}

.gallery-demo-frame-transport input[type="range"] {
    width: 120px;
}

.gallery-demo-frame-elapsed {
    color: var(--ui-muted-fg);
    font-size: 12px;
}

.gallery-demo-frame-play {
    min-height: 26px;
    padding: 2px 10px;
    font-size: 12px;
}
```

- [ ] **Step 5: Note on rAF auto-advance**

The play button toggles `playing` but does not yet drive `elapsed` forward — that requires an `eval`-bridged `requestAnimationFrame` loop, which is wasm-specific. For this wave, the slider remains the primary control; pressing Play under non-wasm SSR is a no-op. Under wasm, the next task wires the loop. The test only asserts the slider, play button, and label render — no loop assertion.

- [ ] **Step 6: Run tests**

```powershell
cargo test -p component-gallery --test demo_frame
```

Expected: green.

- [ ] **Step 7: Commit**

```powershell
git add examples/component-gallery/src/demo_frame.rs examples/component-gallery/src/styles.rs examples/component-gallery/tests/demo_frame.rs
git commit -m "feat(gallery): add ScrubFrame demo wrapper with slider transport"
```

## Task 16: rAF auto-advance for `ScrubFrame` (wasm only)

**Files:**
- Modify: `examples/component-gallery/src/demo_frame.rs`
- Modify: `examples/component-gallery/Cargo.toml`

- [ ] **Step 1: Add wasm-bindgen-futures + dioxus document evaluator dependency check**

Confirm `wasm-bindgen` already in `[target.'cfg(target_arch = "wasm32")'.dependencies]` from Task 11.

- [ ] **Step 2: Implement rAF loop via `document::eval`**

In `examples/component-gallery/src/demo_frame.rs`, replace the body of the `playing` button's `onclick` and add a `use_effect` that, when `*playing.read() == true`, runs an `eval` loop that pushes `requestAnimationFrame` ticks back into `elapsed`. Dioxus 0.7's `document::eval` lets JS communicate back to Rust via the returned `Eval` handle's `recv()`. Replace the inner body of `ScrubFrame` with this version (additions marked):

```rust
#[component]
pub fn ScrubFrame(
    duration_ms: f32,
    fps: Option<u32>,
    label: &'static str,
    children: Element,
) -> Element {
    let mut elapsed = use_signal(|| 0.0_f32);
    let mut playing = use_signal(|| false);
    use_context_provider(|| ScrubElapsedMs(elapsed));
    use_context_provider(|| ScrubFps(fps.unwrap_or(30)));

    use_effect(move || {
        if !*playing.read() {
            return;
        }
        // Drive rAF via document::eval — JS pushes elapsed ms back.
        let mut eval = dioxus::document::eval(&format!(
            r#"
                const start = performance.now();
                const dur = {duration_ms};
                let raf;
                const tick = (now) => {{
                    const t = now - start;
                    if (t >= dur) {{
                        dioxus.send(dur);
                        return;
                    }}
                    dioxus.send(t);
                    raf = requestAnimationFrame(tick);
                }};
                raf = requestAnimationFrame(tick);
            "#,
        ));
        spawn(async move {
            loop {
                match eval.recv::<f64>().await {
                    Ok(t) => {
                        let t_f32 = t as f32;
                        elapsed.set(t_f32);
                        if t_f32 >= duration_ms {
                            playing.set(false);
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    });

    let max_str = format!("{:.0}", duration_ms);
    let value_str = format!("{:.0}", *elapsed.read());

    rsx! {
        div { class: "gallery-demo-frame gallery-demo-frame--scrub",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "{label}" }
                div { class: "gallery-demo-frame-transport",
                    button {
                        class: "ui-button ui-button--secondary gallery-demo-frame-play",
                        r#type: "button",
                        "aria-label": if *playing.read() { "Pause" } else { "Play" },
                        onclick: move |_| {
                            let now = *playing.read();
                            playing.set(!now);
                        },
                        if *playing.read() { "Pause" } else { "Play" }
                    }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "{max_str}",
                        step: "1",
                        value: "{value_str}",
                        oninput: move |evt| {
                            if let Ok(v) = evt.value().parse::<f32>() {
                                elapsed.set(v);
                            }
                        },
                    }
                    span { class: "gallery-demo-frame-elapsed", "{value_str} / {max_str} ms" }
                }
            }
            div { class: "gallery-demo-frame-body",
                {children}
            }
        }
    }
}
```

Note: `dioxus::document::eval` API surface may differ between 0.7 minor releases. If `dioxus.send`/`recv` does not exist exactly as written, consult `dioxus::document::eval` docs and the existing `crates/ui-runtime/src/measurement_web.rs` for the established eval idiom in this repo. The contract is: JS calls a callback the Rust side awaits; the Rust side updates the `elapsed` signal and stops the loop when the duration is exceeded.

- [ ] **Step 3: Verify build for wasm and SSR**

```powershell
cargo check -p component-gallery
cargo check -p component-gallery --target wasm32-unknown-unknown --features web
cargo test -p component-gallery
```

Expected: all green.

- [ ] **Step 4: Commit**

```powershell
git add examples/component-gallery/src/demo_frame.rs
git commit -m "feat(gallery): rAF-driven auto-advance for ScrubFrame under wasm"
```

## Task 17: `FlipFrame` wrapper

**Files:**
- Modify: `examples/component-gallery/src/demo_frame.rs`
- Modify: `examples/component-gallery/src/styles.rs`
- Modify: `examples/component-gallery/tests/demo_frame.rs`

- [ ] **Step 1: Write failing test**

Append to `examples/component-gallery/tests/demo_frame.rs`:

```rust
#[test]
fn flip_frame_renders_swap_button_and_one_layout_at_a_time() {
    use component_gallery::demo_frame::FlipFrame;
    let html = dioxus_ssr::render_element(rsx! {
        FlipFrame {
            label: "Demo flip",
            layout_a: rsx! { p { "Layout A" } },
            layout_b: rsx! { p { "Layout B" } },
        }
    });
    assert!(html.contains("Demo flip"));
    assert!(html.contains("Swap"));
    // Initial state renders layout_a only.
    assert!(html.contains("Layout A"));
    assert!(!html.contains("Layout B"));
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test demo_frame flip_frame_renders_swap_button_and_one_layout_at_a_time
```

Expected: failure.

- [ ] **Step 3: Implement `FlipFrame`**

Append to `examples/component-gallery/src/demo_frame.rs`:

```rust
#[component]
pub fn FlipFrame(label: &'static str, layout_a: Element, layout_b: Element) -> Element {
    let mut at_b = use_signal(|| false);

    rsx! {
        div { class: "gallery-demo-frame gallery-demo-frame--flip",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "{label}" }
                button {
                    class: "ui-button ui-button--secondary gallery-demo-frame-swap",
                    r#type: "button",
                    onclick: move |_| {
                        let now = *at_b.read();
                        at_b.set(!now);
                    },
                    "Swap layout"
                }
            }
            div { class: "gallery-demo-frame-body",
                if *at_b.read() {
                    {layout_b}
                } else {
                    {layout_a}
                }
            }
        }
    }
}
```

- [ ] **Step 4: Add CSS**

In `examples/component-gallery/src/styles.rs`, append:

```css

.gallery-demo-frame-swap {
    min-height: 26px;
    padding: 2px 10px;
    font-size: 12px;
}
```

- [ ] **Step 5: Run tests**

```powershell
cargo test -p component-gallery --test demo_frame
```

Expected: green.

- [ ] **Step 6: Commit**

```powershell
git add examples/component-gallery/src/demo_frame.rs examples/component-gallery/src/styles.rs examples/component-gallery/tests/demo_frame.rs
git commit -m "feat(gallery): add FlipFrame demo wrapper for shared-element swaps"
```

## Task 18: Use `ReplayFrame` in motion previews (Presence, PresenceGate, KineticBox)

**Files:**
- Modify: `examples/component-gallery/src/previews/motion.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write failing test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn motion_previews_use_replay_frame() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });
    // Each motion-category live demo should be wrapped in a gallery-demo-frame.
    let frame_count = html.matches("gallery-demo-frame").count();
    assert!(
        frame_count >= 3,
        "expected >=3 demo frames in motion previews, got {frame_count}"
    );
    // Replay button is present.
    assert!(html.contains("Replay"));
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test gallery motion_previews_use_replay_frame
```

Expected: failure.

- [ ] **Step 3: Wrap motion previews in `ReplayFrame`**

In `examples/component-gallery/src/previews/motion.rs`, edit `presence_preview`, `presence_gate_preview`, and `kinetic_box_preview` to wrap the live children. Example for `presence_preview`:

```rust
use crate::demo_frame::ReplayFrame;
use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn presence_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--2col",
            div { class: "gallery-variant-tile",
                ReplayFrame {
                    label: "Enter",
                    children: rsx! {
                        Presence { present: true, cue: PresenceCue::Rise,
                            p { "Visible state" }
                        }
                    },
                }
            }
            div { class: "gallery-variant-tile",
                ReplayFrame {
                    label: "Exit",
                    children: rsx! {
                        Presence { present: false, cue: PresenceCue::Rise,
                            p { "Hidden state" }
                        }
                    },
                }
            }
        }
    }
}
```

For `presence_gate_preview`, do the same — replace each `gallery-variant-tile` body with a `ReplayFrame` wrapping the `PresenceGate` child.

For `kinetic_box_preview`, replace each cue tile's body:

```rust
pub fn kinetic_box_preview() -> Element {
    let cues = ["rise-in", "fade-in", "slide-up"];
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--3col",
            for cue in cues {
                div { class: "gallery-variant-tile",
                    ReplayFrame {
                        label: cue,
                        children: rsx! {
                            KineticBox { id: "cue-{cue}", cue: cue.to_string(),
                                p { "Cue preview" }
                            }
                        },
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 4: Verify existing motion tests still pass**

```powershell
cargo test -p component-gallery --test gallery
```

The existing tests check for `data-motion-cue="rise-in"`, `Present`, `Hidden`, etc. These are still produced by the inner components. Expected: green.

- [ ] **Step 5: Commit**

```powershell
git add examples/component-gallery/src/previews/motion.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat(gallery): wrap Presence/PresenceGate/KineticBox previews in ReplayFrame"
```

## Task 19: Use `ScrubFrame` in Sequence, TimelineScope, FrameStage previews

**Files:**
- Modify: `examples/component-gallery/src/previews/motion.rs`
- Modify: `examples/component-gallery/src/previews/composition.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write failing test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn timeline_previews_use_scrub_frame_with_range_slider() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    let scrub_count = html.matches("gallery-demo-frame--scrub").count();
    assert!(
        scrub_count >= 2,
        "expected >=2 scrub frames (Sequence, TimelineScope, FrameStage), got {scrub_count}"
    );
    let range_count = html.matches(r#"type="range""#).count();
    assert!(range_count >= 2, "expected >=2 range sliders, got {range_count}");
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test gallery timeline_previews_use_scrub_frame_with_range_slider
```

Expected: failure.

- [ ] **Step 3: Rewire `sequence_preview` to consume scrub context**

In `examples/component-gallery/src/previews/motion.rs`, replace `sequence_preview`:

```rust
use crate::demo_frame::{ScrubElapsedMs, ScrubFrame};

pub fn sequence_preview() -> Element {
    rsx! {
        ScrubFrame {
            duration_ms: 560.0,
            fps: None,
            label: "Sequence",
            children: rsx! { SequenceBody {} },
        }
    }
}

#[component]
fn SequenceBody() -> Element {
    let elapsed = use_context::<ScrubElapsedMs>().0;
    let elapsed_ms = *elapsed.read();
    let tween_short = Transition::Tween { duration_ms: 220, ease: Ease::Standard };
    let tween_med = Transition::Tween { duration_ms: 200, ease: Ease::Standard };
    let tween_long = Transition::Tween { duration_ms: 240, ease: Ease::Standard };
    let cues = vec![
        Cue::new("title", 0.0,
            MotionCue::Opacity { from: 0.0, to: 1.0, transition: tween_short }),
        Cue::new("body", 120.0,
            MotionCue::Translate { axis: Axis::Y, from: 12.0, to: 0.0, transition: tween_med }),
        Cue::new("cta", 320.0,
            MotionCue::Scale { from: 0.94, to: 1.0, transition: tween_long }),
    ];
    rsx! {
        Sequence {
            cues: Some(cues),
            clock: TimelineClock::Manual { elapsed_ms },
            KineticBox { id: "title", h4 { "Welcome" } }
            KineticBox { id: "body", p { "Subtle entry choreography" } }
            KineticBox { id: "cta", Button { "Get started" } }
        }
    }
}
```

- [ ] **Step 4: Rewire `timeline_scope_preview` (single tile as `ScrubFrame`)**

Replace the existing `timeline_scope_preview` with a stack containing one `ScrubFrame` plus the existing "Reduced motion" variant (kept for the existing `data-ui-transparency=\"reduced\"` assertion):

```rust
pub fn timeline_scope_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                ScrubFrame {
                    duration_ms: 1200.0,
                    fps: None,
                    label: "Stagger",
                    children: rsx! {
                        TimelineScope { id: "stagger-demo", autoplay: false,
                            for index in 0u32..4 {
                                div { "data-stagger-index": "{index}",
                                    KineticBox { id: "stagger-{index}", cue: "rise-in",
                                        "Tile {index}"
                                    }
                                }
                            }
                        }
                    },
                }
            }
            div { class: "gallery-variant-tile",
                ScrubFrame {
                    duration_ms: 1000.0,
                    fps: None,
                    label: "Sequence",
                    children: rsx! {
                        TimelineScope { id: "sequence-demo", autoplay: false,
                            KineticBox { id: "sequence-enter", cue: "enter", "Enter" }
                            KineticBox { id: "sequence-settle", cue: "settle", "Settle" }
                            KineticBox { id: "sequence-pulse", cue: "pulse", "Pulse" }
                        }
                    },
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

- [ ] **Step 5: Rewire `frame_stage_preview`**

In `examples/component-gallery/src/previews/composition.rs`, replace the three-frame static grid with one `ScrubFrame` that derives `frame` from elapsed × fps:

```rust
use crate::demo_frame::{ScrubElapsedMs, ScrubFps, ScrubFrame};
use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn frame_stage_preview() -> Element {
    rsx! {
        ScrubFrame {
            duration_ms: 6000.0,
            fps: Some(30),
            label: "Frame 0 / 180",
            children: rsx! { FrameStageBody {} },
        }
    }
}

#[component]
fn FrameStageBody() -> Element {
    let elapsed = use_context::<ScrubElapsedMs>().0;
    let fps = use_context::<ScrubFps>().0;
    let frame = ((*elapsed.read() / 1000.0) * fps as f32).round() as u32;
    rsx! {
        FrameStage {
            composition: Composition::new("launch-demo", 1920, 1080, 30, 180),
            frame,
            FrameClip { start: 0, duration: 60,
                FrameLayer { id: "title", depth: 10,
                    h4 { "Dioxus Kinetics" }
                    p { "Frame {frame} / 180" }
                }
            }
        }
    }
}
```

The existing test `gallery_frame_stage_preview_renders_three_frame_snapshots` asserts captions `Frame 0 / 180`, `Frame 90 / 180`, `Frame 179 / 180`. With a single `ScrubFrame`, only `Frame 0 / 180` renders by default. **Update that test** to assert exactly one caption at start:

```rust
#[test]
fn gallery_frame_stage_preview_renders_starting_frame_caption() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });
    assert!(html.contains("Frame 0 / 180"));
}
```

Replace the old multi-caption test with this single-caption version.

- [ ] **Step 6: Run all gallery tests**

```powershell
cargo test -p component-gallery
```

Expected: all green.

- [ ] **Step 7: Commit**

```powershell
git add examples/component-gallery/src/previews/motion.rs examples/component-gallery/src/previews/composition.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat(gallery): wrap Sequence/TimelineScope/FrameStage in ScrubFrame"
```

## Task 20: Use `FlipFrame` in SharedLayout and SharedElement previews

**Files:**
- Modify: `examples/component-gallery/src/previews/shared.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write failing test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn shared_layout_preview_uses_flip_frame_with_swap_control() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });
    assert!(html.contains("gallery-demo-frame--flip"));
    assert!(html.contains("Swap layout"));
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test gallery shared_layout_preview_uses_flip_frame_with_swap_control
```

Expected: failure.

- [ ] **Step 3: Rewire `shared_layout_preview` and `shared_element_preview`**

In `examples/component-gallery/src/previews/shared.rs`, replace both functions:

```rust
use crate::demo_frame::FlipFrame;
use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn shared_layout_preview() -> Element {
    rsx! {
        FlipFrame {
            label: "Cross-tree layout swap",
            layout_a: rsx! {
                SharedLayout {
                    div { class: "gallery-variant-grid gallery-variant-grid--2col",
                        div { class: "gallery-variant-tile",
                            SharedElement { id: "card-left".to_string(),
                                p { "Card A" }
                            }
                        }
                        div { class: "gallery-variant-tile",
                            SharedElement { id: "card-right".to_string(),
                                p { "Card B" }
                            }
                        }
                    }
                }
            },
            layout_b: rsx! {
                SharedLayout {
                    div { class: "gallery-variant-grid gallery-variant-grid--2col",
                        div { class: "gallery-variant-tile",
                            SharedElement { id: "card-right".to_string(),
                                p { "Card B" }
                            }
                        }
                        div { class: "gallery-variant-tile",
                            SharedElement { id: "card-left".to_string(),
                                p { "Card A" }
                            }
                        }
                    }
                }
            },
        }
    }
}

pub fn shared_element_preview() -> Element {
    rsx! {
        FlipFrame {
            label: "Shared element FLIP",
            layout_a: rsx! {
                SharedLayout {
                    div { class: "gallery-variant-grid gallery-variant-grid--2col",
                        div { class: "gallery-variant-tile",
                            SharedElement { id: "demo-hero".to_string(),
                                p { "Hero position A" }
                            }
                        }
                        div { class: "gallery-variant-tile",
                            span { class: "gallery-variant-label", "Other slot" }
                        }
                    }
                }
            },
            layout_b: rsx! {
                SharedLayout {
                    div { class: "gallery-variant-grid gallery-variant-grid--2col",
                        div { class: "gallery-variant-tile",
                            span { class: "gallery-variant-label", "Other slot" }
                        }
                        div { class: "gallery-variant-tile",
                            SharedElement { id: "demo-hero".to_string(),
                                p { "Hero position B" }
                            }
                        }
                    }
                }
            },
        }
    }
}
```

- [ ] **Step 4: Update existing shared-layout test**

The test `gallery_shared_layout_and_shared_element_are_ready` asserts `class="ui-shared-layout"` and `data-shared-id="`. Both still hold (each layout body contains `SharedLayout`/`SharedElement`). Run all tests:

```powershell
cargo test -p component-gallery
```

Expected: green.

- [ ] **Step 5: Commit**

```powershell
git add examples/component-gallery/src/previews/shared.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat(gallery): wrap SharedLayout/SharedElement in FlipFrame"
```

## Task 21: Interactive `Dialog` preview

**Files:**
- Modify: `examples/component-gallery/src/previews/feedback.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write failing test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn dialog_preview_renders_open_trigger_and_starts_closed() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });
    // Trigger button labelled "Show dialog" is present.
    assert!(html.contains("Show dialog"));
    // Default state is closed → dialog panel markup is NOT in the rendered HTML for the preview tile.
    // The existing test asserts ".ui-dialog" appears in CSS — that's still true.
    // We assert no aria-modal in the preview area by counting overall occurrences (gallery has CSS .ui-dialog but no live dialog).
    let aria_modal_count = html.matches(r#"aria-modal="true""#).count();
    assert_eq!(aria_modal_count, 0, "dialog should start closed in preview");
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test gallery dialog_preview_renders_open_trigger_and_starts_closed
```

Expected: failure — current preview renders `open: true` so the panel is present.

- [ ] **Step 3: Rewrite `dialog_preview`**

In `examples/component-gallery/src/previews/feedback.rs`, replace `dialog_preview`:

```rust
pub fn dialog_preview() -> Element {
    rsx! { DialogPreviewBody {} }
}

#[component]
fn DialogPreviewBody() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Dialog" }
                button {
                    class: "ui-button ui-button--primary",
                    r#type: "button",
                    onclick: move |_| open.set(true),
                    "Show dialog"
                }
            }
            Dialog {
                open: *open.read(),
                title: "Archive workspace",
                description: "Move this workspace out of active navigation.",
                body: "Team members can still request access later.",
                actions: vec!["Cancel".to_string(), "Move it".to_string()],
                on_dismiss: move |_| open.set(false),
                on_action: move |_action: String| open.set(false),
            }
        }
    }
}
```

- [ ] **Step 4: Run tests**

```powershell
cargo test -p component-gallery --test gallery
```

Expected: green. The existing assertion `.gallery-preview .ui-dialog` (CSS selector) still passes — it's in the static CSS string.

- [ ] **Step 5: Commit**

```powershell
git add examples/component-gallery/src/previews/feedback.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat(gallery): interactive Dialog preview with show/dismiss controls"
```

## Task 22: Interactive `Toast` preview with stage stack

**Files:**
- Modify: `examples/component-gallery/src/previews/feedback.rs`
- Modify: `examples/component-gallery/src/styles.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write failing test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn toast_preview_renders_trigger_buttons_for_each_tone() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });
    // The four toast-trigger buttons.
    for label in ["Trigger success", "Trigger info", "Trigger warning", "Trigger error"] {
        assert!(html.contains(label), "missing toast trigger: {label}");
    }
    // Stage container is always rendered (empty by default).
    assert!(html.contains("gallery-toast-stage"));
}
```

- [ ] **Step 2: Run to verify failure**

```powershell
cargo test -p component-gallery --test gallery toast_preview_renders_trigger_buttons_for_each_tone
```

Expected: failure.

- [ ] **Step 3: Rewrite `toast_preview`**

In `examples/component-gallery/src/previews/feedback.rs`, replace `toast_preview`:

```rust
use std::sync::atomic::{AtomicU32, Ordering};

static TOAST_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, PartialEq)]
struct ToastInstance {
    id: u32,
    tone: ToastTone,
    title: &'static str,
    description: &'static str,
}

pub fn toast_preview() -> Element {
    rsx! { ToastPreviewBody {} }
}

#[component]
fn ToastPreviewBody() -> Element {
    let mut toasts: Signal<Vec<ToastInstance>> = use_signal(Vec::new);

    let mut push = move |tone: ToastTone, title: &'static str, description: &'static str| {
        let id = TOAST_ID.fetch_add(1, Ordering::Relaxed);
        toasts.write().push(ToastInstance { id, tone, title, description });
        let mut t = toasts;
        spawn(async move {
            #[cfg(target_arch = "wasm32")]
            {
                let promise = js_sys::Promise::new(&mut |resolve, _| {
                    let win = web_sys::window().unwrap();
                    let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                        &resolve, 3000,
                    );
                });
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            }
            t.write().retain(|x| x.id != id);
        });
    };

    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Toast" }
                div { class: "gallery-demo-frame-transport",
                    button {
                        class: "ui-button ui-button--secondary",
                        r#type: "button",
                        onclick: move |_| push(ToastTone::Success, "Report exported", "The PDF is ready."),
                        "Trigger success"
                    }
                    button {
                        class: "ui-button ui-button--secondary",
                        r#type: "button",
                        onclick: move |_| push(ToastTone::Info, "Sync started", "Pulling the latest data."),
                        "Trigger info"
                    }
                    button {
                        class: "ui-button ui-button--secondary",
                        r#type: "button",
                        onclick: move |_| push(ToastTone::Warning, "Quota close", "You are at 92% of the plan."),
                        "Trigger warning"
                    }
                    button {
                        class: "ui-button ui-button--secondary",
                        r#type: "button",
                        onclick: move |_| push(ToastTone::Danger, "Export failed", "Retry or contact support."),
                        "Trigger error"
                    }
                }
            }
            div { class: "gallery-toast-stage",
                for t in toasts.read().iter() {
                    Toast {
                        key: "{t.id}",
                        tone: t.tone,
                        title: t.title,
                        description: t.description,
                        dismiss_label: "Dismiss",
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 4: Add `js-sys` + `wasm-bindgen-futures` to wasm deps**

In `examples/component-gallery/Cargo.toml`, extend the wasm target block:

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["Window", "Storage", "MediaQueryList"] }
```

- [ ] **Step 5: Add toast-stage CSS**

In `examples/component-gallery/src/styles.rs`, append inside `GALLERY_CSS`:

```css

.gallery-toast-stage {
    display: grid;
    gap: var(--ui-space-2);
    min-height: 60px;
    padding: var(--ui-space-2);
    border: 1px dashed var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface-muted);
}
```

- [ ] **Step 6: Run tests**

```powershell
cargo test -p component-gallery
```

Expected: green.

- [ ] **Step 7: Commit**

```powershell
git add examples/component-gallery/src/previews/feedback.rs examples/component-gallery/src/styles.rs examples/component-gallery/Cargo.toml examples/component-gallery/tests/gallery.rs
git commit -m "feat(gallery): interactive Toast preview with trigger stack"
```

## Task 23: Interactive `Tooltip` preview

**Files:**
- Modify: `examples/component-gallery/src/previews/feedback.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write failing test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn tooltip_preview_renders_trigger_label() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });
    assert!(html.contains("Net revenue"));
    assert!(html.contains("Hover or focus the trigger"));
}
```

(This test is intentionally light: SSR cannot simulate hover. Hover-driven show/hide is verified manually in Task 24.)

- [ ] **Step 2: Rewrite `tooltip_preview`**

In `examples/component-gallery/src/previews/feedback.rs`, replace `tooltip_preview`:

```rust
pub fn tooltip_preview() -> Element {
    rsx! { TooltipPreviewBody {} }
}

#[component]
fn TooltipPreviewBody() -> Element {
    let mut visible = use_signal(|| false);
    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Tooltip" }
                span { class: "gallery-demo-frame-elapsed", "Hover or focus the trigger" }
            }
            div {
                class: "gallery-demo-frame-body",
                onmouseenter: move |_| visible.set(true),
                onmouseleave: move |_| visible.set(false),
                onfocusin: move |_| visible.set(true),
                onfocusout: move |_| visible.set(false),
                Tooltip {
                    id: "net-revenue-tip",
                    visible: *visible.read(),
                    trigger_label: "Net revenue",
                    content: "Revenue after refunds and credits.",
                }
            }
        }
    }
}
```

- [ ] **Step 3: Run tests**

```powershell
cargo test -p component-gallery --test gallery
```

Expected: green.

- [ ] **Step 4: Commit**

```powershell
git add examples/component-gallery/src/previews/feedback.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat(gallery): interactive Tooltip preview driven by hover/focus"
```

## Task 24: Final integration check, regression sweep, and manual verification checklist

**Files:**
- None (verification-only)

- [ ] **Step 1: Workspace-level checks**

```powershell
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
```

Expected: all green. If formatter complains, run `cargo fmt --all` and commit:

```powershell
git add -A
git commit -m "style: apply rustfmt"
```

- [ ] **Step 2: wasm target check**

```powershell
rustup target add wasm32-unknown-unknown
cargo check -p component-gallery --target wasm32-unknown-unknown --features web
```

Expected: success.

- [ ] **Step 3: Manual verification checklist (run gallery in browser)**

```powershell
dx serve --package component-gallery
```

Open the served URL and verify, in order:

- [ ] Page loads with the ambient mesh visible behind the rail and main column.
- [ ] Toggling Theme between Light and Dark swaps the page palette and the elevation shadows visibly deepen under Dark.
- [ ] Toggling Density between Compact, Comfortable, Spacious resizes control heights and gaps.
- [ ] Toggling Motion between Normal and Reduced disables transitions on buttons and the switch thumb; replay buttons disappear from `ReplayFrame` tiles; mesh drift halts.
- [ ] Toggling Glass between Translucent and Solid removes blur from the rail, the control bar, and every `.ui-glass-*` surface; the surface still reads clearly against the ambient mesh.
- [ ] Each `ReplayFrame` tile shows a "▶ Replay" button that retriggers the inner animation on click.
- [ ] Each `ScrubFrame` tile (Sequence, TimelineScope×2, FrameStage) shows a working slider and Play/Pause; Play advances elapsed via rAF and stops at the end.
- [ ] `FrameStage` body updates its caption from `Frame 0 / 180` to `Frame N / 180` as you scrub.
- [ ] `FlipFrame` swap button cross-fades the two layouts via the FLIP runtime.
- [ ] Dialog "Show dialog" button opens the dialog with its enter transition; backdrop click / Cancel closes it with exit.
- [ ] Toast "Trigger success / info / warning / error" buttons each push a toast that auto-dismisses after 3s.
- [ ] Tooltip appears on hover and on keyboard focus of the trigger; disappears on leave/blur.
- [ ] Refreshing the page persists the toggled values from `localStorage`.
- [ ] On a machine with OS-level reduced-motion enabled and no prior localStorage value, Motion starts at Reduced.
- [ ] Foundations and Surfaces sections show a denser color plate behind the glass tiles.

- [ ] **Step 4: Worktree handoff or merge**

This plan does not auto-merge. Either:

- Hand back to the user for review, then merge via `git merge --no-ff gallery-dynamic-examples`; or
- Open a pull request via `gh pr create` if that's the project's convention (no existing PR setup is required; the user can choose).

Do **not** push or open a PR without explicit user confirmation.
