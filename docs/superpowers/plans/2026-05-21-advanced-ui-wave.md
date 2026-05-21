# Advanced UI Wave Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first advanced `dioxus-kinetics` wave: reusable library CSS, 12 controlled SaaS components, public facade exports, and a richer registry-driven component gallery.

**Architecture:** Add a new `ui-styles` crate for reusable CSS while keeping gallery-only layout CSS in the example app. Split `ui-dioxus` into focused modules for forms, navigation, overlays, and display components, then re-export through `unified_ui::prelude::*`. Upgrade the component gallery registry so the advanced components are marked `Ready` and render real SaaS examples.

**Tech Stack:** Rust 2021, Cargo workspace, Dioxus 0.7, Dioxus SSR tests, static CSS strings, controlled component props, workspace-level verification.

---

## Scope Check

The approved spec is broad, but it is one coherent implementation phase because every task contributes to the same deliverable: a reusable advanced UI library wave exposed by `unified_ui` and demonstrated in `examples/component-gallery`.

This plan intentionally does not implement global overlay stacks, focus trapping, runtime theme switching, runtime density switching, full keyboard engines, `DataTable`, visual regression screenshots, GSAP timelines, or HyperFrames demos. Those remain separate follow-up plans.

## File Structure

Create and modify these files:

```text
Cargo.toml
crates/
  ui-styles/
    Cargo.toml
    src/lib.rs
    tests/css.rs
  ui-dioxus/
    Cargo.toml
    src/lib.rs
    src/forms.rs
    src/navigation.rs
    src/overlays.rs
    src/display.rs
    tests/advanced_ssr.rs
  unified_ui/
    Cargo.toml
    src/lib.rs
    tests/prelude.rs
examples/
  component-gallery/
    Cargo.toml
    src/app.rs
    src/docs.rs
    src/styles.rs
    tests/gallery.rs
README.md
```

Responsibilities:

- `ui-styles`: reusable CSS variables and `.ui-*` component class styles.
- `ui-dioxus/src/forms.rs`: `TextField`, `Checkbox`, `Switch`.
- `ui-dioxus/src/navigation.rs`: `Tabs`, `Toolbar`, `Sidebar`.
- `ui-dioxus/src/overlays.rs`: `Dialog`, `Toast`, `CommandMenu`, `Tooltip`.
- `ui-dioxus/src/display.rs`: `MetricCard`, `EmptyState`.
- `ui-dioxus/src/lib.rs`: module wiring and existing primitive exports.
- `unified_ui`: single downstream facade and prelude.
- `component-gallery`: registry/workbench examples and gallery-only layout CSS.

## Shared Component API Shape

Use controlled props and simple owned data structs. Do not add Dioxus signals, hooks, or internal state. All new component structs derive `Clone`, `Debug`, and `PartialEq`; add `Eq` when every field supports it.

Shared enums:

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tone {
    #[default]
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}
```

Use the CSS class prefix `ui-` for all rendered elements. Public component names must remain semantic: no borrowed names from external UI libraries.

## Task 1: `ui-styles` Workspace Crate

**Files:**
- Modify: `Cargo.toml`
- Create: `crates/ui-styles/Cargo.toml`
- Create: `crates/ui-styles/src/lib.rs`
- Create: `crates/ui-styles/tests/css.rs`

- [ ] **Step 1: Write failing CSS tests**

Create `crates/ui-styles/tests/css.rs`:

```rust
use ui_styles::{library_css, BASE_CSS, COMPONENT_CSS};

#[test]
fn base_css_exposes_theme_density_and_preference_hooks() {
    let css = BASE_CSS;

    assert!(css.contains(":root"));
    assert!(css.contains("[data-ui-theme=\"dark\"]"));
    assert!(css.contains("[data-ui-density=\"compact\"]"));
    assert!(css.contains("[data-ui-density=\"spacious\"]"));
    assert!(css.contains("[data-ui-transparency=\"reduced\"]"));
    assert!(css.contains("@media (prefers-reduced-motion: reduce)"));
}

#[test]
fn component_css_covers_advanced_component_classes() {
    let css = COMPONENT_CSS;

    for selector in [
        ".ui-text-field",
        ".ui-checkbox",
        ".ui-switch",
        ".ui-tabs",
        ".ui-dialog",
        ".ui-toast",
        ".ui-command-menu",
        ".ui-tooltip",
        ".ui-toolbar",
        ".ui-sidebar",
        ".ui-metric-card",
        ".ui-empty-state",
        ".ui-glass-surface",
        ".ui-button:disabled",
        ".ui-field--invalid",
    ] {
        assert!(css.contains(selector), "missing selector {selector}");
    }
}

#[test]
fn library_css_concatenates_base_and_component_css() {
    let css = library_css();

    assert!(css.contains(":root"));
    assert!(css.contains(".ui-button"));
    assert!(css.contains(".ui-dialog"));
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```powershell
cargo test -p ui-styles
```

Expected: FAIL because package `ui-styles` does not exist.

- [ ] **Step 3: Add workspace member and dependency**

Modify root `Cargo.toml`:

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
ui-gsap = { path = "crates/ui-gsap" }
ui-hyperframes = { path = "crates/ui-hyperframes" }
ui-styles = { path = "crates/ui-styles" }
unified_ui = { path = "crates/unified_ui" }
```

- [ ] **Step 4: Create the style crate manifest**

Create `crates/ui-styles/Cargo.toml`:

```toml
[package]
name = "ui-styles"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[lib]
path = "src/lib.rs"
```

- [ ] **Step 5: Implement reusable CSS exports**

Create `crates/ui-styles/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

pub const BASE_CSS: &str = r#"
:root,
[data-ui-theme="light"] {
    color-scheme: light;
    --ui-font-sans: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    --ui-bg: #f6f8fb;
    --ui-surface: #ffffff;
    --ui-surface-muted: #f2f5f9;
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
}

[data-ui-theme="dark"] {
    color-scheme: dark;
    --ui-bg: #0d1117;
    --ui-surface: #151b23;
    --ui-surface-muted: #1c2430;
    --ui-glass: rgba(25, 32, 43, 0.72);
    --ui-glass-solid: #151b23;
    --ui-fg: #eef3f8;
    --ui-muted-fg: #aab4c2;
    --ui-border: rgba(205, 215, 228, 0.18);
    --ui-focus: #64b5ff;
}

[data-ui-density="compact"] {
    --ui-control-height: 32px;
    --ui-space-3: 10px;
    --ui-space-4: 12px;
}

[data-ui-density="comfortable"] {
    --ui-control-height: 36px;
}

[data-ui-density="spacious"] {
    --ui-control-height: 42px;
    --ui-space-3: 14px;
    --ui-space-4: 20px;
}

[data-ui-transparency="reduced"] {
    --ui-glass: var(--ui-glass-solid);
}

* {
    box-sizing: border-box;
}

body {
    margin: 0;
    font-family: var(--ui-font-sans);
    background: var(--ui-bg);
    color: var(--ui-fg);
}

button,
input,
textarea,
select {
    font: inherit;
}

@media (prefers-reduced-motion: reduce) {
    *,
    *::before,
    *::after {
        transition-duration: 0.01ms !important;
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        scroll-behavior: auto !important;
    }
}
"#;

pub const COMPONENT_CSS: &str = r#"
.ui-button,
.ui-field-control,
.ui-command-menu-input {
    min-height: var(--ui-control-height);
    border-radius: var(--ui-radius-md);
    transition: border-color var(--ui-motion-fast), box-shadow var(--ui-motion-fast), background var(--ui-motion-fast);
}

.ui-button {
    border: 1px solid transparent;
    padding: 0 14px;
    font-weight: 700;
    cursor: pointer;
}

.ui-button:disabled,
.ui-field-control:disabled,
.ui-checkbox-input:disabled,
.ui-switch-control[aria-disabled="true"] {
    cursor: not-allowed;
    opacity: 0.52;
}

.ui-button--primary {
    background: var(--ui-primary);
    color: #ffffff;
}

.ui-button--secondary {
    background: var(--ui-surface);
    color: var(--ui-fg);
    border-color: var(--ui-border);
}

.ui-button--ghost {
    background: transparent;
    color: var(--ui-fg);
}

.ui-button--danger {
    background: var(--ui-danger);
    color: #ffffff;
}

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
}

.ui-glass-surface,
.ui-dialog-panel,
.ui-command-menu-panel {
    background: var(--ui-glass);
    backdrop-filter: blur(18px) saturate(160%);
}

.ui-stack {
    display: flex;
    flex-direction: column;
}

.ui-stack--gap-sm { gap: var(--ui-space-2); }
.ui-stack--gap-md { gap: var(--ui-space-3); }

.ui-text-field,
.ui-checkbox,
.ui-switch,
.ui-tabs,
.ui-toolbar,
.ui-sidebar,
.ui-metric-card,
.ui-empty-state,
.ui-toast,
.ui-command-menu,
.ui-tooltip {
    color: var(--ui-fg);
}

.ui-field {
    display: grid;
    gap: var(--ui-space-2);
}

.ui-field-label,
.ui-checkbox-label,
.ui-switch-label {
    font-weight: 700;
}

.ui-field-control,
.ui-command-menu-input {
    width: 100%;
    border: 1px solid var(--ui-border);
    background: var(--ui-surface);
    color: var(--ui-fg);
    padding: 0 12px;
}

.ui-field-control:focus-visible,
.ui-checkbox-input:focus-visible,
.ui-switch-control:focus-visible,
.ui-tab:focus-visible,
.ui-command-menu-input:focus-visible,
.ui-sidebar-link:focus-visible,
.ui-button:focus-visible {
    outline: 2px solid var(--ui-focus);
    outline-offset: 2px;
}

.ui-field--invalid .ui-field-control {
    border-color: var(--ui-danger);
}

.ui-field-help,
.ui-field-error,
.ui-checkbox-description,
.ui-switch-description,
.ui-empty-state-description,
.ui-metric-card-delta,
.ui-toast-description {
    color: var(--ui-muted-fg);
}

.ui-field-error {
    color: var(--ui-danger);
}

.ui-checkbox,
.ui-switch {
    display: flex;
    gap: var(--ui-space-3);
    align-items: flex-start;
}

.ui-checkbox-input {
    width: 18px;
    height: 18px;
    accent-color: var(--ui-primary);
}

.ui-switch-control {
    width: 42px;
    height: 24px;
    border: 1px solid var(--ui-border);
    border-radius: 999px;
    background: var(--ui-surface-muted);
}

.ui-switch-control[aria-checked="true"] {
    background: var(--ui-primary);
}

.ui-tabs-list,
.ui-toolbar,
.ui-command-menu-list {
    display: flex;
    gap: var(--ui-space-2);
}

.ui-tab {
    border: 0;
    border-radius: var(--ui-radius-md);
    background: transparent;
    color: var(--ui-muted-fg);
    padding: 8px 10px;
}

.ui-tab[aria-selected="true"] {
    background: var(--ui-surface);
    color: var(--ui-fg);
    box-shadow: inset 0 0 0 1px var(--ui-border);
}

.ui-dialog {
    position: fixed;
    inset: 0;
    display: grid;
    place-items: center;
    padding: var(--ui-space-5);
}

.ui-dialog-backdrop {
    position: absolute;
    inset: 0;
    background: rgba(10, 15, 24, 0.38);
}

.ui-dialog-panel,
.ui-command-menu-panel {
    position: relative;
    width: min(560px, 100%);
    padding: var(--ui-space-5);
    box-shadow: 0 24px 80px rgba(13, 20, 32, 0.24);
}

.ui-toast {
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-surface);
    padding: var(--ui-space-4);
}

.ui-toast--success { border-color: color-mix(in srgb, var(--ui-success), transparent 62%); }
.ui-toast--warning { border-color: color-mix(in srgb, var(--ui-warning), transparent 62%); }
.ui-toast--danger { border-color: color-mix(in srgb, var(--ui-danger), transparent 62%); }
.ui-toast--info { border-color: color-mix(in srgb, var(--ui-info), transparent 62%); }

.ui-command-menu {
    display: grid;
    gap: var(--ui-space-3);
}

.ui-command-menu-group {
    display: grid;
    gap: var(--ui-space-1);
}

.ui-command-menu-item[aria-selected="true"],
.ui-sidebar-link[aria-current="page"] {
    background: var(--ui-surface-muted);
    color: var(--ui-fg);
}

.ui-tooltip-content {
    border-radius: var(--ui-radius-md);
    background: var(--ui-fg);
    color: var(--ui-bg);
    padding: 6px 8px;
}

.ui-toolbar {
    align-items: center;
    justify-content: space-between;
}

.ui-toolbar-group {
    display: flex;
    gap: var(--ui-space-2);
    align-items: center;
}

.ui-sidebar {
    display: grid;
    gap: var(--ui-space-4);
    padding: var(--ui-space-4);
}

.ui-sidebar-section {
    display: grid;
    gap: var(--ui-space-2);
}

.ui-sidebar-link {
    border-radius: var(--ui-radius-md);
    color: var(--ui-muted-fg);
    padding: 8px 10px;
    text-decoration: none;
}

.ui-metric-card,
.ui-empty-state {
    display: grid;
    gap: var(--ui-space-3);
    padding: var(--ui-space-4);
}

.ui-metric-card-value {
    font-size: 28px;
    font-weight: 800;
}

.ui-empty-state {
    justify-items: start;
}
"#;

pub fn library_css() -> String {
    let mut css = String::with_capacity(BASE_CSS.len() + COMPONENT_CSS.len() + 1);
    css.push_str(BASE_CSS);
    css.push('\n');
    css.push_str(COMPONENT_CSS);
    css
}
```

- [ ] **Step 6: Run style tests**

Run:

```powershell
cargo test -p ui-styles
```

Expected: PASS, `3 passed`.

- [ ] **Step 7: Commit style crate**

Run:

```powershell
git add Cargo.toml crates/ui-styles
git commit -m "feat: add reusable ui styles"
```

## Task 2: Form Components

**Files:**
- Modify: `crates/ui-dioxus/src/lib.rs`
- Create: `crates/ui-dioxus/src/forms.rs`
- Modify: `crates/ui-dioxus/tests/advanced_ssr.rs`

- [ ] **Step 1: Write failing SSR tests for form components**

Create `crates/ui-dioxus/tests/advanced_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::{Checkbox, Switch, TextField};

#[test]
fn text_field_renders_label_value_help_and_invalid_state() {
    let html = dioxus_ssr::render(rsx! {
        TextField {
            id: "workspace-name",
            label: "Workspace name",
            value: "Acme Ops",
            placeholder: "Enter workspace",
            help_text: "Visible to teammates",
            error_text: "Use at least 3 characters",
            invalid: true,
        }
    });

    assert!(html.contains("ui-text-field"));
    assert!(html.contains("ui-field--invalid"));
    assert!(html.contains("for=\"workspace-name\""));
    assert!(html.contains("aria-invalid=\"true\""));
    assert!(html.contains("Visible to teammates"));
    assert!(html.contains("Use at least 3 characters"));
}

#[test]
fn checkbox_renders_checked_and_mixed_states() {
    let checked = dioxus_ssr::render(rsx! {
        Checkbox {
            id: "weekly-summary",
            label: "Send weekly summary",
            checked: true,
            description: "Every Monday morning",
        }
    });
    let mixed = dioxus_ssr::render(rsx! {
        Checkbox {
            id: "partial-selection",
            label: "Select visible rows",
            indeterminate: true,
        }
    });

    assert!(checked.contains("ui-checkbox"));
    assert!(checked.contains("checked"));
    assert!(checked.contains("Every Monday morning"));
    assert!(mixed.contains("aria-checked=\"mixed\""));
    assert!(mixed.contains("ui-checkbox--mixed"));
}

#[test]
fn switch_uses_switch_role_and_checked_state() {
    let html = dioxus_ssr::render(rsx! {
        Switch {
            id: "auto-renew",
            label: "Auto renew",
            checked: true,
            description: "Keep billing active",
        }
    });

    assert!(html.contains("ui-switch"));
    assert!(html.contains("role=\"switch\""));
    assert!(html.contains("aria-checked=\"true\""));
    assert!(html.contains("Keep billing active"));
}
```

- [ ] **Step 2: Run form tests to verify failure**

Run:

```powershell
cargo test -p ui-dioxus text_field_renders_label_value_help_and_invalid_state
```

Expected: FAIL with unresolved import `TextField`.

- [ ] **Step 3: Implement form components**

Create `crates/ui-dioxus/src/forms.rs`:

```rust
use dioxus::prelude::*;

#[component]
pub fn TextField(
    id: String,
    label: String,
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default)] help_text: String,
    #[props(default)] error_text: String,
    #[props(default)] leading_text: String,
    #[props(default)] trailing_text: String,
    #[props(default)] disabled: bool,
    #[props(default)] invalid: bool,
) -> Element {
    let described_by = described_by(&id, !help_text.is_empty(), !error_text.is_empty());
    let field_class = if invalid {
        "ui-field ui-text-field ui-field--invalid"
    } else {
        "ui-field ui-text-field"
    };

    rsx! {
        div { class: "{field_class}",
            label { class: "ui-field-label", r#for: "{id}", "{label}" }
            div { class: "ui-field-row",
                if !leading_text.is_empty() {
                    span { class: "ui-field-adornment ui-field-adornment--leading", "{leading_text}" }
                }
                input {
                    id: "{id}",
                    class: "ui-field-control",
                    value: "{value}",
                    placeholder: "{placeholder}",
                    disabled,
                    "aria-invalid": if invalid { "true" } else { "false" },
                    "aria-describedby": "{described_by}",
                }
                if !trailing_text.is_empty() {
                    span { class: "ui-field-adornment ui-field-adornment--trailing", "{trailing_text}" }
                }
            }
            if !help_text.is_empty() {
                p { id: "{id}-help", class: "ui-field-help", "{help_text}" }
            }
            if !error_text.is_empty() {
                p { id: "{id}-error", class: "ui-field-error", "{error_text}" }
            }
        }
    }
}

#[component]
pub fn Checkbox(
    id: String,
    label: String,
    #[props(default)] description: String,
    #[props(default)] checked: bool,
    #[props(default)] indeterminate: bool,
    #[props(default)] disabled: bool,
) -> Element {
    let wrapper_class = if indeterminate {
        "ui-checkbox ui-checkbox--mixed"
    } else {
        "ui-checkbox"
    };
    let aria_checked = if indeterminate {
        "mixed"
    } else if checked {
        "true"
    } else {
        "false"
    };

    rsx! {
        div { class: "{wrapper_class}",
            input {
                id: "{id}",
                class: "ui-checkbox-input",
                r#type: "checkbox",
                checked,
                disabled,
                "aria-checked": "{aria_checked}",
            }
            div { class: "ui-checkbox-copy",
                label { class: "ui-checkbox-label", r#for: "{id}", "{label}" }
                if !description.is_empty() {
                    p { class: "ui-checkbox-description", "{description}" }
                }
            }
        }
    }
}

#[component]
pub fn Switch(
    id: String,
    label: String,
    #[props(default)] description: String,
    #[props(default)] checked: bool,
    #[props(default)] disabled: bool,
) -> Element {
    let aria_checked = if checked { "true" } else { "false" };
    let aria_disabled = if disabled { "true" } else { "false" };

    rsx! {
        div { class: "ui-switch",
            button {
                id: "{id}",
                class: "ui-switch-control",
                r#type: "button",
                role: "switch",
                disabled,
                "aria-checked": "{aria_checked}",
                "aria-disabled": "{aria_disabled}",
                span { class: "ui-switch-thumb" }
            }
            div { class: "ui-switch-copy",
                label { class: "ui-switch-label", r#for: "{id}", "{label}" }
                if !description.is_empty() {
                    p { class: "ui-switch-description", "{description}" }
                }
            }
        }
    }
}

fn described_by(id: &str, has_help: bool, has_error: bool) -> String {
    match (has_help, has_error) {
        (true, true) => format!("{id}-help {id}-error"),
        (true, false) => format!("{id}-help"),
        (false, true) => format!("{id}-error"),
        (false, false) => String::new(),
    }
}
```

- [ ] **Step 4: Export form components**

Modify `crates/ui-dioxus/src/lib.rs` by adding the module and exports near the top:

```rust
mod forms;

pub use forms::{Checkbox, Switch, TextField};
```

Keep the existing `Button`, `Surface`, `GlassSurface`, and `Stack` code unchanged.

- [ ] **Step 5: Run form component tests**

Run:

```powershell
cargo test -p ui-dioxus text_field_renders_label_value_help_and_invalid_state
cargo test -p ui-dioxus checkbox_renders_checked_and_mixed_states
cargo test -p ui-dioxus switch_uses_switch_role_and_checked_state
```

Expected: all three commands PASS.

- [ ] **Step 6: Commit form components**

Run:

```powershell
git add crates/ui-dioxus/src/lib.rs crates/ui-dioxus/src/forms.rs crates/ui-dioxus/tests/advanced_ssr.rs
git commit -m "feat: add advanced form components"
```

## Task 3: Navigation, Toolbar, Sidebar, And Display Components

**Files:**
- Modify: `crates/ui-dioxus/src/lib.rs`
- Create: `crates/ui-dioxus/src/navigation.rs`
- Create: `crates/ui-dioxus/src/display.rs`
- Modify: `crates/ui-dioxus/tests/advanced_ssr.rs`

- [ ] **Step 1: Add failing SSR tests**

Update the imports at the top of `crates/ui-dioxus/tests/advanced_ssr.rs` to include the navigation and display symbols:

```rust
use ui_dioxus::{
    Checkbox,
    EmptyState, MetricCard, MetricTone, Sidebar, SidebarItem, SidebarSection, TabItem, TabPanel,
    Tabs, TextField, Toolbar, Switch,
};
```

Append these tests to `crates/ui-dioxus/tests/advanced_ssr.rs`:

```rust

#[test]
fn tabs_render_selected_tab_and_panel() {
    let html = dioxus_ssr::render(rsx! {
        Tabs {
            selected: "billing",
            items: vec![
                TabItem::new("overview", "Overview"),
                TabItem::new("billing", "Billing"),
            ],
            panels: vec![
                TabPanel::new("overview", "Overview content"),
                TabPanel::new("billing", "Billing content"),
            ],
        }
    });

    assert!(html.contains("role=\"tablist\""));
    assert!(html.contains("aria-selected=\"true\""));
    assert!(html.contains("Billing content"));
    assert!(!html.contains("Overview content"));
}

#[test]
fn toolbar_sidebar_and_display_components_render_semantic_structure() {
    let toolbar = dioxus_ssr::render(rsx! {
        Toolbar {
            primary: vec!["New".to_string(), "Filter".to_string()],
            secondary: "Updated now",
        }
    });
    let sidebar = dioxus_ssr::render(rsx! {
        Sidebar {
            collapsed: false,
            selected: "settings",
            sections: vec![SidebarSection::new(
                "Workspace",
                vec![
                    SidebarItem::new("home", "Home", "#home"),
                    SidebarItem::new("settings", "Settings", "#settings"),
                ],
            )],
        }
    });
    let metric = dioxus_ssr::render(rsx! {
        MetricCard {
            label: "Net revenue",
            value: "$128.4k",
            delta: "+12.5%",
            tone: MetricTone::Success,
        }
    });
    let empty = dioxus_ssr::render(rsx! {
        EmptyState {
            title: "No reports yet",
            description: "Create a report to share performance with your team.",
            action_label: "Create report",
        }
    });

    assert!(toolbar.contains("role=\"toolbar\""));
    assert!(toolbar.contains("ui-toolbar-group"));
    assert!(sidebar.contains("ui-sidebar"));
    assert!(sidebar.contains("aria-current=\"page\""));
    assert!(metric.contains("ui-metric-card--success"));
    assert!(metric.contains("ui-metric-card-sparkline"));
    assert!(empty.contains("ui-empty-state"));
    assert!(empty.contains("Create report"));
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```powershell
cargo test -p ui-dioxus tabs_render_selected_tab_and_panel
```

Expected: FAIL with unresolved imports for `Tabs`, `TabItem`, or `TabPanel`.

- [ ] **Step 3: Implement navigation components**

Create `crates/ui-dioxus/src/navigation.rs`:

```rust
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabItem {
    pub value: String,
    pub label: String,
}

impl TabItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabPanel {
    pub value: String,
    pub content: String,
}

impl TabPanel {
    pub fn new(value: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            content: content.into(),
        }
    }
}

#[component]
pub fn Tabs(selected: String, items: Vec<TabItem>, panels: Vec<TabPanel>) -> Element {
    rsx! {
        div { class: "ui-tabs",
            div { class: "ui-tabs-list", role: "tablist",
                for item in items.iter() {
                    button {
                        class: "ui-tab",
                        r#type: "button",
                        role: "tab",
                        id: "tab-{item.value}",
                        "aria-controls": "panel-{item.value}",
                        "aria-selected": if item.value == selected { "true" } else { "false" },
                        "{item.label}"
                    }
                }
            }
            for panel in panels.iter().filter(|panel| panel.value == selected) {
                div {
                    class: "ui-tab-panel",
                    role: "tabpanel",
                    id: "panel-{panel.value}",
                    "aria-labelledby": "tab-{panel.value}",
                    "{panel.content}"
                }
            }
        }
    }
}

#[component]
pub fn Toolbar(primary: Vec<String>, #[props(default)] secondary: String) -> Element {
    rsx! {
        div { class: "ui-toolbar", role: "toolbar",
            div { class: "ui-toolbar-group ui-toolbar-group--primary",
                for command in primary {
                    button { class: "ui-button ui-button--secondary", r#type: "button", "{command}" }
                }
            }
            if !secondary.is_empty() {
                div { class: "ui-toolbar-secondary", "{secondary}" }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidebarItem {
    pub id: String,
    pub label: String,
    pub href: String,
}

impl SidebarItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            href: href.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidebarSection {
    pub label: String,
    pub items: Vec<SidebarItem>,
}

impl SidebarSection {
    pub fn new(label: impl Into<String>, items: Vec<SidebarItem>) -> Self {
        Self {
            label: label.into(),
            items,
        }
    }
}

#[component]
pub fn Sidebar(
    #[props(default)] collapsed: bool,
    #[props(default)] selected: String,
    sections: Vec<SidebarSection>,
) -> Element {
    let class_name = if collapsed {
        "ui-sidebar ui-sidebar--collapsed"
    } else {
        "ui-sidebar"
    };

    rsx! {
        nav { class: "{class_name}", "aria-label": "Application navigation",
            for section in sections {
                div { class: "ui-sidebar-section",
                    h3 { class: "ui-sidebar-section-label", "{section.label}" }
                    for item in section.items {
                        a {
                            class: "ui-sidebar-link",
                            href: "{item.href}",
                            "aria-current": if item.id == selected { "page" } else { "false" },
                            "{item.label}"
                        }
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 4: Implement display components**

Create `crates/ui-dioxus/src/display.rs`:

```rust
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MetricTone {
    #[default]
    Neutral,
    Success,
    Warning,
    Danger,
    Info,
}

impl MetricTone {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Neutral => "ui-metric-card ui-metric-card--neutral",
            Self::Success => "ui-metric-card ui-metric-card--success",
            Self::Warning => "ui-metric-card ui-metric-card--warning",
            Self::Danger => "ui-metric-card ui-metric-card--danger",
            Self::Info => "ui-metric-card ui-metric-card--info",
        }
    }
}

#[component]
pub fn MetricCard(
    label: String,
    value: String,
    #[props(default)] delta: String,
    #[props(default)] tone: MetricTone,
) -> Element {
    rsx! {
        article { class: "{tone.class_name()}",
            p { class: "ui-metric-card-label", "{label}" }
            strong { class: "ui-metric-card-value", "{value}" }
            if !delta.is_empty() {
                span { class: "ui-metric-card-delta", "{delta}" }
            }
            div { class: "ui-metric-card-sparkline", "aria-hidden": "true" }
        }
    }
}

#[component]
pub fn EmptyState(
    title: String,
    description: String,
    #[props(default)] action_label: String,
) -> Element {
    rsx! {
        section { class: "ui-empty-state",
            div { class: "ui-empty-state-visual", "aria-hidden": "true" }
            h3 { class: "ui-empty-state-title", "{title}" }
            p { class: "ui-empty-state-description", "{description}" }
            if !action_label.is_empty() {
                button { class: "ui-button ui-button--primary", r#type: "button", "{action_label}" }
            }
        }
    }
}
```

- [ ] **Step 5: Export navigation and display components**

Modify `crates/ui-dioxus/src/lib.rs`:

```rust
mod display;
mod forms;
mod navigation;

pub use display::{EmptyState, MetricCard, MetricTone};
pub use forms::{Checkbox, Switch, TextField};
pub use navigation::{Sidebar, SidebarItem, SidebarSection, TabItem, TabPanel, Tabs, Toolbar};
```

Keep the existing basic component code in the file.

- [ ] **Step 6: Run navigation and display tests**

Run:

```powershell
cargo test -p ui-dioxus tabs_render_selected_tab_and_panel
cargo test -p ui-dioxus toolbar_sidebar_and_display_components_render_semantic_structure
```

Expected: both commands PASS.

- [ ] **Step 7: Commit navigation and display components**

Run:

```powershell
git add crates/ui-dioxus/src/lib.rs crates/ui-dioxus/src/navigation.rs crates/ui-dioxus/src/display.rs crates/ui-dioxus/tests/advanced_ssr.rs
git commit -m "feat: add navigation and display components"
```

## Task 4: Overlay And Feedback Components

**Files:**
- Modify: `crates/ui-dioxus/src/lib.rs`
- Create: `crates/ui-dioxus/src/overlays.rs`
- Modify: `crates/ui-dioxus/tests/advanced_ssr.rs`

- [ ] **Step 1: Add failing SSR tests**

Update the imports at the top of `crates/ui-dioxus/tests/advanced_ssr.rs` to include the overlay symbols:

```rust
use ui_dioxus::{
    Checkbox, CommandGroup, CommandItem, CommandMenu, Dialog, EmptyState, MetricCard, MetricTone,
    Sidebar, SidebarItem, SidebarSection, Switch, TabItem, TabPanel, Tabs, TextField, Toast,
    ToastTone, Toolbar, Tooltip,
};
```

Append these tests to `crates/ui-dioxus/tests/advanced_ssr.rs`:

```rust

#[test]
fn dialog_toast_and_tooltip_render_overlay_semantics() {
    let dialog = dioxus_ssr::render(rsx! {
        Dialog {
            open: true,
            title: "Delete workspace",
            description: "This action cannot be undone.",
            body: "All reports and settings will be archived.",
            actions: vec!["Cancel".to_string(), "Delete".to_string()],
        }
    });
    let closed_dialog = dioxus_ssr::render(rsx! {
        Dialog {
            open: false,
            title: "Hidden",
        }
    });
    let toast = dioxus_ssr::render(rsx! {
        Toast {
            tone: ToastTone::Success,
            title: "Report exported",
            description: "The PDF is ready.",
            action_label: "Open",
            dismiss_label: "Dismiss",
        }
    });
    let tooltip = dioxus_ssr::render(rsx! {
        Tooltip {
            id: "revenue-tip",
            visible: true,
            trigger_label: "Net revenue",
            content: "Revenue after refunds and credits.",
        }
    });

    assert!(dialog.contains("role=\"dialog\""));
    assert!(dialog.contains("aria-modal=\"true\""));
    assert!(dialog.contains("ui-dialog-backdrop"));
    assert!(!closed_dialog.contains("ui-dialog-panel"));
    assert!(toast.contains("ui-toast--success"));
    assert!(toast.contains("role=\"status\""));
    assert!(tooltip.contains("role=\"tooltip\""));
    assert!(tooltip.contains("aria-describedby=\"revenue-tip\""));
}

#[test]
fn command_menu_renders_grouped_items_and_empty_state() {
    let menu = dioxus_ssr::render(rsx! {
        CommandMenu {
            open: true,
            query: "rep",
            selected_id: "reports",
            empty_text: "No commands",
            groups: vec![CommandGroup::new(
                "Navigation",
                vec![
                    CommandItem::new("dashboard", "Open dashboard", "Go to overview"),
                    CommandItem::new("reports", "Open reports", "Review exports"),
                ],
            )],
        }
    });
    let empty = dioxus_ssr::render(rsx! {
        CommandMenu {
            open: true,
            query: "zzz",
            empty_text: "No commands",
            groups: vec![],
        }
    });

    assert!(menu.contains("ui-command-menu"));
    assert!(menu.contains("role=\"dialog\""));
    assert!(menu.contains("role=\"listbox\""));
    assert!(menu.contains("aria-selected=\"true\""));
    assert!(menu.contains("Open reports"));
    assert!(empty.contains("No commands"));
}
```

- [ ] **Step 2: Run overlay tests to verify failure**

Run:

```powershell
cargo test -p ui-dioxus dialog_toast_and_tooltip_render_overlay_semantics
```

Expected: FAIL with unresolved imports for `Dialog`, `Toast`, or `Tooltip`.

- [ ] **Step 3: Implement overlay components**

Create `crates/ui-dioxus/src/overlays.rs`:

```rust
use dioxus::prelude::*;

#[component]
pub fn Dialog(
    #[props(default)] open: bool,
    title: String,
    #[props(default)] description: String,
    #[props(default)] body: String,
    #[props(default)] actions: Vec<String>,
) -> Element {
    if !open {
        return rsx! {};
    }

    rsx! {
        div { class: "ui-dialog", role: "dialog", "aria-modal": "true", "aria-labelledby": "ui-dialog-title",
            div { class: "ui-dialog-backdrop" }
            div { class: "ui-dialog-panel",
                h2 { id: "ui-dialog-title", class: "ui-dialog-title", "{title}" }
                if !description.is_empty() {
                    p { class: "ui-dialog-description", "{description}" }
                }
                if !body.is_empty() {
                    div { class: "ui-dialog-body", "{body}" }
                }
                if !actions.is_empty() {
                    div { class: "ui-dialog-actions",
                        for action in actions {
                            button { class: "ui-button ui-button--secondary", r#type: "button", "{action}" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastTone {
    #[default]
    Neutral,
    Success,
    Warning,
    Danger,
    Info,
}

impl ToastTone {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Neutral => "ui-toast ui-toast--neutral",
            Self::Success => "ui-toast ui-toast--success",
            Self::Warning => "ui-toast ui-toast--warning",
            Self::Danger => "ui-toast ui-toast--danger",
            Self::Info => "ui-toast ui-toast--info",
        }
    }

    pub const fn role(self) -> &'static str {
        match self {
            Self::Danger | Self::Warning => "alert",
            _ => "status",
        }
    }
}

#[component]
pub fn Toast(
    #[props(default)] tone: ToastTone,
    title: String,
    #[props(default)] description: String,
    #[props(default)] action_label: String,
    #[props(default)] dismiss_label: String,
) -> Element {
    rsx! {
        div { class: "{tone.class_name()}", role: "{tone.role()}",
            div { class: "ui-toast-content",
                strong { class: "ui-toast-title", "{title}" }
                if !description.is_empty() {
                    p { class: "ui-toast-description", "{description}" }
                }
            }
            if !action_label.is_empty() || !dismiss_label.is_empty() {
                div { class: "ui-toast-actions",
                    if !action_label.is_empty() {
                        button { class: "ui-button ui-button--secondary", r#type: "button", "{action_label}" }
                    }
                    if !dismiss_label.is_empty() {
                        button { class: "ui-button ui-button--ghost", r#type: "button", "{dismiss_label}" }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandItem {
    pub id: String,
    pub label: String,
    pub description: String,
}

impl CommandItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: description.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGroup {
    pub label: String,
    pub items: Vec<CommandItem>,
}

impl CommandGroup {
    pub fn new(label: impl Into<String>, items: Vec<CommandItem>) -> Self {
        Self {
            label: label.into(),
            items,
        }
    }
}

#[component]
pub fn CommandMenu(
    #[props(default)] open: bool,
    #[props(default)] query: String,
    #[props(default)] selected_id: String,
    #[props(default = "No commands found".to_string())] empty_text: String,
    #[props(default)] groups: Vec<CommandGroup>,
) -> Element {
    if !open {
        return rsx! {};
    }

    let has_items = groups.iter().any(|group| !group.items.is_empty());

    rsx! {
        div { class: "ui-command-menu", role: "dialog", "aria-modal": "true",
            div { class: "ui-command-menu-panel",
                input {
                    class: "ui-command-menu-input",
                    value: "{query}",
                    placeholder: "Search commands",
                    "aria-label": "Search commands",
                }
                if has_items {
                    div { class: "ui-command-menu-list", role: "listbox",
                        for group in groups {
                            div { class: "ui-command-menu-group",
                                p { class: "ui-command-menu-group-label", "{group.label}" }
                                for item in group.items {
                                    div {
                                        class: "ui-command-menu-item",
                                        role: "option",
                                        "aria-selected": if item.id == selected_id { "true" } else { "false" },
                                        strong { "{item.label}" }
                                        span { "{item.description}" }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    p { class: "ui-command-menu-empty", "{empty_text}" }
                }
            }
        }
    }
}

#[component]
pub fn Tooltip(id: String, visible: bool, trigger_label: String, content: String) -> Element {
    rsx! {
        span { class: "ui-tooltip",
            span {
                class: "ui-tooltip-trigger",
                "aria-describedby": if visible { "{id}" } else { "" },
                "{trigger_label}"
            }
            if visible {
                span { id: "{id}", class: "ui-tooltip-content", role: "tooltip", "{content}" }
            }
        }
    }
}
```

- [ ] **Step 4: Export overlay components**

Modify `crates/ui-dioxus/src/lib.rs`:

```rust
mod display;
mod forms;
mod navigation;
mod overlays;

pub use display::{EmptyState, MetricCard, MetricTone};
pub use forms::{Checkbox, Switch, TextField};
pub use navigation::{Sidebar, SidebarItem, SidebarSection, TabItem, TabPanel, Tabs, Toolbar};
pub use overlays::{
    CommandGroup, CommandItem, CommandMenu, Dialog, Toast, ToastTone, Tooltip,
};
```

- [ ] **Step 5: Run overlay tests**

Run:

```powershell
cargo test -p ui-dioxus dialog_toast_and_tooltip_render_overlay_semantics
cargo test -p ui-dioxus command_menu_renders_grouped_items_and_empty_state
```

Expected: both commands PASS.

- [ ] **Step 6: Commit overlay components**

Run:

```powershell
git add crates/ui-dioxus/src/lib.rs crates/ui-dioxus/src/overlays.rs crates/ui-dioxus/tests/advanced_ssr.rs
git commit -m "feat: add overlay and feedback components"
```

## Task 5: Unified Facade Exports

**Files:**
- Modify: `crates/unified_ui/Cargo.toml`
- Modify: `crates/unified_ui/src/lib.rs`
- Modify: `crates/unified_ui/tests/prelude.rs`

- [ ] **Step 1: Add failing prelude tests**

Append to `crates/unified_ui/tests/prelude.rs`:

```rust
use unified_ui::prelude::*;

#[test]
fn prelude_exposes_advanced_components_and_styles() {
    let css = library_css();

    assert!(css.contains(".ui-command-menu"));
    assert_eq!(MetricTone::Success.class_name(), "ui-metric-card ui-metric-card--success");
    assert_eq!(ToastTone::Warning.role(), "alert");

    let tabs = vec![TabItem::new("one", "One")];
    let panels = vec![TabPanel::new("one", "Panel")];
    assert_eq!(tabs[0].label, "One");
    assert_eq!(panels[0].content, "Panel");

    let commands = vec![CommandGroup::new(
        "Navigation",
        vec![CommandItem::new("home", "Home", "Open dashboard")],
    )];
    assert_eq!(commands[0].items[0].id, "home");
}

#[test]
fn public_api_names_include_advanced_wave_names() {
    let names = unified_ui::public_api_names();

    for expected in [
        "TextField",
        "Checkbox",
        "Switch",
        "Tabs",
        "Dialog",
        "Toast",
        "CommandMenu",
        "Tooltip",
        "Toolbar",
        "Sidebar",
        "MetricCard",
        "EmptyState",
    ] {
        assert!(names.contains(&expected), "missing public API name {expected}");
    }
}
```

- [ ] **Step 2: Run facade tests to verify failure**

Run:

```powershell
cargo test -p unified_ui prelude_exposes_advanced_components_and_styles
```

Expected: FAIL because `library_css` and new components are not exported from `unified_ui`.

- [ ] **Step 3: Add `ui-styles` dependency**

Modify `crates/unified_ui/Cargo.toml`:

```toml
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
ui-gsap = { workspace = true, optional = true }
ui-hyperframes = { workspace = true, optional = true }
```

- [ ] **Step 4: Export advanced components and styles**

Modify `crates/unified_ui/src/lib.rs` prelude exports:

```rust
pub mod prelude {
    pub use motion_core::{Ease, PresenceState, Spring, SpringStep, Transition};
    pub use ui_core::{
        A11yContract, ComponentContract, ComponentId, ComponentRole, FocusPolicy, TargetSize,
    };
    pub use ui_dioxus::{
        Button, ButtonVariant, Checkbox, CommandGroup, CommandItem, CommandMenu, Dialog,
        EmptyState, GlassSurface, MetricCard, MetricTone, Sidebar, SidebarItem, SidebarSection,
        Stack, Surface, Switch, TabItem, TabPanel, Tabs, TextField, Toast, ToastTone, Toolbar,
        Tooltip,
    };
    pub use ui_glass::{
        resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRecipe, GlassRequest, GlassTone,
    };
    pub use ui_layout::{compute_flip, FlipDelta, Rect};
    pub use ui_styles::{library_css, BASE_CSS, COMPONENT_CSS};
    pub use ui_tokens::{
        Color, Density, MotionPreference, MotionScale, RadiusScale, SemanticColors, SpacingScale,
        Theme, ThemeMode, TransparencyPreference,
    };

    #[cfg(any(feature = "web", feature = "desktop", feature = "mobile"))]
    pub use ui_dom::{glass_style, CssStyleWriter};

    #[cfg(feature = "native")]
    pub use ui_native::{plan_native_glass, NativeCapabilities, NativeGlassPlan};
}
```

Update `public_api_names()` so it returns:

```rust
pub fn public_api_names() -> &'static [&'static str] {
    &[
        "Button",
        "IconButton",
        "TextField",
        "Checkbox",
        "Switch",
        "Tabs",
        "Dialog",
        "Toast",
        "CommandMenu",
        "Tooltip",
        "Toolbar",
        "Sidebar",
        "MetricCard",
        "EmptyState",
        "Surface",
        "GlassSurface",
        "Presence",
        "Transition",
        "Sequence",
        "SharedLayout",
        "SharedElement",
    ]
}
```

- [ ] **Step 5: Run facade tests**

Run:

```powershell
cargo test -p unified_ui
```

Expected: PASS.

- [ ] **Step 6: Commit facade exports**

Run:

```powershell
git add crates/unified_ui/Cargo.toml crates/unified_ui/src/lib.rs crates/unified_ui/tests/prelude.rs
git commit -m "feat: expose advanced ui wave"
```

## Task 6: Gallery Registry Upgrade

**Files:**
- Modify: `examples/component-gallery/Cargo.toml`
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Add failing gallery registry tests**

Modify `examples/component-gallery/tests/gallery.rs` by adding:

```rust
#[test]
fn advanced_wave_components_are_ready_with_accessibility_notes() {
    let docs = component_docs();

    for name in [
        "TextField",
        "Checkbox",
        "Switch",
        "Tabs",
        "Dialog",
        "Toast",
        "CommandMenu",
        "Tooltip",
        "Toolbar",
        "Sidebar",
        "MetricCard",
        "EmptyState",
    ] {
        let doc = docs.iter().find(|doc| doc.name == name).expect("component doc exists");
        assert_eq!(doc.status, ComponentStatus::Ready, "{name} should be ready");
        assert!(doc.render.is_some(), "{name} should render a live example");
        assert!(!doc.accessibility.is_empty(), "{name} needs accessibility notes");
        assert!(!doc.snippet.is_empty(), "{name} needs a snippet");
    }
}
```

- [ ] **Step 2: Run gallery registry test to verify failure**

Run:

```powershell
cargo test -p component-gallery advanced_wave_components_are_ready_with_accessibility_notes
```

Expected: FAIL because `ComponentDoc` does not have `accessibility`, and planned components are still `ComingSoon`.

- [ ] **Step 3: Add `ui-styles` gallery dependency**

Modify `examples/component-gallery/Cargo.toml`:

```toml
[dependencies]
dioxus.workspace = true
unified_ui.workspace = true
ui-styles.workspace = true
```

- [ ] **Step 4: Add accessibility notes to `ComponentDoc`**

Modify `examples/component-gallery/src/docs.rs`:

```rust
#[derive(Clone, Copy)]
pub struct ComponentDoc {
    pub name: &'static str,
    pub category: ComponentCategory,
    pub status: ComponentStatus,
    pub summary: &'static str,
    pub snippet: &'static str,
    pub accessibility: &'static str,
    pub render: Option<fn() -> Element>,
}
```

Update every existing `ComponentDoc` entry to include an `accessibility` field. For existing components use these exact notes:

```rust
accessibility: "Renders native semantic elements and stable focusable controls.",
```

For every advanced component entry, use the component-specific accessibility notes in Step 5.

- [ ] **Step 5: Replace registry entries for advanced components**

In `examples/component-gallery/src/docs.rs`, update the 12 advanced components to `ComponentStatus::Ready`, real snippets, accessibility notes, and render functions. Use these entries:

```rust
ComponentDoc {
    name: "TextField",
    category: ComponentCategory::Inputs,
    status: ComponentStatus::Ready,
    summary: "A labeled text input with help, error, disabled, and adornment states.",
    snippet: TEXT_FIELD_SNIPPET,
    accessibility: "Associates label, input, help text, and error text with stable ids.",
    render: Some(text_field_preview),
},
ComponentDoc {
    name: "Checkbox",
    category: ComponentCategory::Inputs,
    status: ComponentStatus::Ready,
    summary: "A labeled binary or mixed selection control for settings and lists.",
    snippet: CHECKBOX_SNIPPET,
    accessibility: "Uses native checkbox behavior and aria-checked for mixed state.",
    render: Some(checkbox_preview),
},
ComponentDoc {
    name: "Switch",
    category: ComponentCategory::Inputs,
    status: ComponentStatus::Ready,
    summary: "A labeled on/off control for immediate settings.",
    snippet: SWITCH_SNIPPET,
    accessibility: "Uses role switch and aria-checked so assistive tech reads state.",
    render: Some(switch_preview),
},
ComponentDoc {
    name: "Tabs",
    category: ComponentCategory::Layout,
    status: ComponentStatus::Ready,
    summary: "A controlled tab interface for switching between related panels.",
    snippet: TABS_SNIPPET,
    accessibility: "Uses tablist, tab, and tabpanel roles with selected state.",
    render: Some(tabs_preview),
},
ComponentDoc {
    name: "Dialog",
    category: ComponentCategory::Feedback,
    status: ComponentStatus::Ready,
    summary: "A controlled modal surface for focused decisions and workflows.",
    snippet: DIALOG_SNIPPET,
    accessibility: "Uses role dialog and aria-modal; focus trapping is a later helper layer.",
    render: Some(dialog_preview),
},
ComponentDoc {
    name: "Toast",
    category: ComponentCategory::Feedback,
    status: ComponentStatus::Ready,
    summary: "A status notification with tone, description, action, and dismiss affordance.",
    snippet: TOAST_SNIPPET,
    accessibility: "Uses status or alert live-region roles based on tone.",
    render: Some(toast_preview),
},
ComponentDoc {
    name: "CommandMenu",
    category: ComponentCategory::Actions,
    status: ComponentStatus::Ready,
    summary: "A controlled command-search surface with grouped actions and empty state.",
    snippet: COMMAND_MENU_SNIPPET,
    accessibility: "Uses dialog and listbox-oriented semantics for command discovery.",
    render: Some(command_menu_preview),
},
ComponentDoc {
    name: "Tooltip",
    category: ComponentCategory::Feedback,
    status: ComponentStatus::Ready,
    summary: "A controlled explanatory layer connected to trigger text.",
    snippet: TOOLTIP_SNIPPET,
    accessibility: "Connects trigger and tooltip content with aria-describedby.",
    render: Some(tooltip_preview),
},
ComponentDoc {
    name: "Toolbar",
    category: ComponentCategory::Actions,
    status: ComponentStatus::Ready,
    summary: "A command grouping surface for page and workflow actions.",
    snippet: TOOLBAR_SNIPPET,
    accessibility: "Uses role toolbar and grouped command regions.",
    render: Some(toolbar_preview),
},
ComponentDoc {
    name: "Sidebar",
    category: ComponentCategory::Layout,
    status: ComponentStatus::Ready,
    summary: "A compact app navigation rail with sections and selected item state.",
    snippet: SIDEBAR_SNIPPET,
    accessibility: "Uses nav semantics and aria-current on the selected item.",
    render: Some(sidebar_preview),
},
ComponentDoc {
    name: "MetricCard",
    category: ComponentCategory::Surfaces,
    status: ComponentStatus::Ready,
    summary: "A dashboard metric surface with label, value, delta, tone, and sparkline region.",
    snippet: METRIC_CARD_SNIPPET,
    accessibility: "Keeps metric text readable and marks decorative sparkline region hidden.",
    render: Some(metric_card_preview),
},
ComponentDoc {
    name: "EmptyState",
    category: ComponentCategory::Feedback,
    status: ComponentStatus::Ready,
    summary: "A polished empty state for missing reports, records, or workflows.",
    snippet: EMPTY_STATE_SNIPPET,
    accessibility: "Uses semantic section content and a clear action button when present.",
    render: Some(empty_state_preview),
},
```

- [ ] **Step 6: Add advanced snippets and preview functions**

Append snippets and previews in `examples/component-gallery/src/docs.rs` using only public prelude APIs. Add these snippets:

```rust
const SWITCH_SNIPPET: &str = r#"Switch {
    id: "auto-renew",
    label: "Auto renew",
    checked: true,
    description: "Keep billing active",
}"#;

const COMMAND_MENU_SNIPPET: &str = r#"CommandMenu {
    open: true,
    query: "rep",
    selected_id: "reports",
    groups: command_groups,
}"#;

const TOOLTIP_SNIPPET: &str = r#"Tooltip {
    id: "net-revenue-tip",
    visible: true,
    trigger_label: "Net revenue",
    content: "Revenue after refunds and credits.",
}"#;

const TOOLBAR_SNIPPET: &str = r#"Toolbar {
    primary: vec!["New".to_string(), "Filter".to_string()],
    secondary: "Updated now",
}"#;

const SIDEBAR_SNIPPET: &str = r#"Sidebar {
    selected: "settings",
    sections: navigation_sections,
}"#;

const METRIC_CARD_SNIPPET: &str = r#"MetricCard {
    label: "Net revenue",
    value: "$128.4k",
    delta: "+12.5%",
    tone: MetricTone::Success,
}"#;

const EMPTY_STATE_SNIPPET: &str = r#"EmptyState {
    title: "No reports yet",
    description: "Create a report to share performance.",
    action_label: "Create report",
}"#;
```

Replace existing planned snippets for `TextField`, `Checkbox`, `Tabs`, `Dialog`, and `Toast` so they match implemented APIs:

```rust
const TEXT_FIELD_SNIPPET: &str = r#"TextField {
    id: "workspace-name",
    label: "Workspace name",
    value: "Acme Ops",
    help_text: "Visible to teammates",
}"#;

const CHECKBOX_SNIPPET: &str = r#"Checkbox {
    id: "weekly-summary",
    label: "Send weekly summary",
    checked: true,
    description: "Every Monday morning",
}"#;

const TABS_SNIPPET: &str = r#"Tabs {
    selected: "billing",
    items: tabs,
    panels: panels,
}"#;

const DIALOG_SNIPPET: &str = r#"Dialog {
    open: true,
    title: "Delete workspace",
    description: "This action cannot be undone.",
}"#;

const TOAST_SNIPPET: &str = r#"Toast {
    tone: ToastTone::Success,
    title: "Report exported",
    description: "The PDF is ready.",
}"#;
```

Add preview functions:

```rust
fn text_field_preview() -> Element {
    rsx! {
        TextField {
            id: "workspace-name",
            label: "Workspace name",
            value: "Acme Ops",
            help_text: "Visible to teammates",
            leading_text: "Org",
        }
    }
}

fn checkbox_preview() -> Element {
    rsx! {
        Checkbox {
            id: "weekly-summary",
            label: "Send weekly summary",
            checked: true,
            description: "Every Monday morning",
        }
    }
}

fn switch_preview() -> Element {
    rsx! {
        Switch {
            id: "auto-renew",
            label: "Auto renew",
            checked: true,
            description: "Keep billing active",
        }
    }
}

fn tabs_preview() -> Element {
    rsx! {
        Tabs {
            selected: "billing",
            items: vec![TabItem::new("overview", "Overview"), TabItem::new("billing", "Billing")],
            panels: vec![TabPanel::new("overview", "Account summary"), TabPanel::new("billing", "Payment method active")],
        }
    }
}

fn dialog_preview() -> Element {
    rsx! {
        Dialog {
            open: true,
            title: "Archive workspace",
            description: "Move this workspace out of active navigation.",
            body: "Team members can still request access later.",
            actions: vec!["Cancel".to_string(), "Archive".to_string()],
        }
    }
}

fn toast_preview() -> Element {
    rsx! {
        Toast {
            tone: ToastTone::Success,
            title: "Report exported",
            description: "The PDF is ready.",
            action_label: "Open",
            dismiss_label: "Dismiss",
        }
    }
}

fn command_menu_preview() -> Element {
    rsx! {
        CommandMenu {
            open: true,
            query: "rep",
            selected_id: "reports",
            groups: vec![CommandGroup::new(
                "Navigation",
                vec![
                    CommandItem::new("dashboard", "Open dashboard", "Go to overview"),
                    CommandItem::new("reports", "Open reports", "Review exports"),
                ],
            )],
        }
    }
}

fn tooltip_preview() -> Element {
    rsx! {
        Tooltip {
            id: "net-revenue-tip",
            visible: true,
            trigger_label: "Net revenue",
            content: "Revenue after refunds and credits.",
        }
    }
}

fn toolbar_preview() -> Element {
    rsx! {
        Toolbar {
            primary: vec!["New".to_string(), "Filter".to_string(), "Export".to_string()],
            secondary: "Updated now",
        }
    }
}

fn sidebar_preview() -> Element {
    rsx! {
        Sidebar {
            selected: "settings",
            sections: vec![SidebarSection::new(
                "Workspace",
                vec![
                    SidebarItem::new("home", "Home", "#home"),
                    SidebarItem::new("settings", "Settings", "#settings"),
                ],
            )],
        }
    }
}

fn metric_card_preview() -> Element {
    rsx! {
        MetricCard {
            label: "Net revenue",
            value: "$128.4k",
            delta: "+12.5%",
            tone: MetricTone::Success,
        }
    }
}

fn empty_state_preview() -> Element {
    rsx! {
        EmptyState {
            title: "No reports yet",
            description: "Create a report to share performance with your team.",
            action_label: "Create report",
        }
    }
}
```

- [ ] **Step 7: Run gallery registry tests**

Run:

```powershell
cargo test -p component-gallery advanced_wave_components_are_ready_with_accessibility_notes
cargo test -p component-gallery registry_status_matches_live_renderer_availability
```

Expected: both commands PASS.

- [ ] **Step 8: Commit gallery registry upgrade**

Run:

```powershell
git add examples/component-gallery/Cargo.toml examples/component-gallery/src/docs.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat: mark advanced components ready in gallery"
```

## Task 7: Gallery Workbench Layout And Shared Styles

**Files:**
- Modify: `examples/component-gallery/src/app.rs`
- Modify: `examples/component-gallery/src/styles.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Add failing workbench SSR tests**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_renders_advanced_workbench_controls_and_notes() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("Theme"));
    assert!(html.contains("Density"));
    assert!(html.contains("Light"));
    assert!(html.contains("Dark"));
    assert!(html.contains("Compact"));
    assert!(html.contains("Spacious"));
    assert!(html.contains("Accessibility"));
    assert!(html.contains(".ui-command-menu"));
    assert!(html.contains("[data-ui-theme=&quot;dark&quot;]") || html.contains("[data-ui-theme=\"dark\"]"));
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```powershell
cargo test -p component-gallery gallery_renders_advanced_workbench_controls_and_notes
```

Expected: FAIL because the current gallery does not render theme/density controls or accessibility notes.

- [ ] **Step 3: Import shared CSS into the gallery app**

Modify `examples/component-gallery/src/app.rs` imports:

```rust
use dioxus::prelude::*;
use ui_styles::library_css;

use crate::docs::{categories, component_docs, ComponentCategory, ComponentDoc, ComponentStatus};
use crate::styles::GALLERY_CSS;
```

In `App`, compute shared CSS and render both style blocks. Add `data-ui-theme` and `data-ui-density` attributes to the existing root `div { class: "gallery-shell", ... }` without changing its current children:

```rust
#[component]
pub fn App() -> Element {
    let shared_css = library_css();

    rsx! {
        style { "{shared_css}" }
        style { "{GALLERY_CSS}" }
        div {
            class: "gallery-shell",
            "data-ui-theme": "light",
            "data-ui-density": "comfortable",
        }
    }
}
```

- [ ] **Step 4: Add static theme and density controls**

Inside the `main { class: "gallery-main", ... }` element, after the header and before mobile tabs, add:

```rust
section { class: "gallery-controls", "aria-label": "Preview settings",
    div { class: "gallery-control-group",
        span { class: "gallery-control-label", "Theme" }
        button { class: "ui-button ui-button--primary", r#type: "button", "Light" }
        button { class: "ui-button ui-button--secondary", r#type: "button", "Dark" }
    }
    div { class: "gallery-control-group",
        span { class: "gallery-control-label", "Density" }
        button { class: "ui-button ui-button--secondary", r#type: "button", "Compact" }
        button { class: "ui-button ui-button--primary", r#type: "button", "Comfortable" }
        button { class: "ui-button ui-button--secondary", r#type: "button", "Spacious" }
    }
}
```

- [ ] **Step 5: Render accessibility notes in component entries**

In `component_entry`, after the summary paragraph, add:

```rust
div { class: "gallery-accessibility",
    strong { "Accessibility" }
    p { "{doc.accessibility}" }
}
```

- [ ] **Step 6: Replace gallery CSS with layout-only workbench CSS**

Modify `examples/component-gallery/src/styles.rs` so `GALLERY_CSS` no longer duplicates `.ui-button`, `.ui-surface`, `.ui-glass-surface`, `.ui-stack`, or other library component classes. It must include these gallery-only selectors:

```rust
pub const GALLERY_CSS: &str = r#"
body {
    min-width: 320px;
    background:
        linear-gradient(135deg, rgba(205, 231, 255, 0.72), rgba(255, 255, 255, 0.0) 34%),
        linear-gradient(180deg, var(--ui-bg) 0%, #eef2f7 100%);
}

.gallery-shell {
    display: grid;
    grid-template-columns: 280px minmax(0, 1fr);
    min-height: 100vh;
}

.gallery-rail {
    position: sticky;
    top: 0;
    align-self: start;
    height: 100vh;
    padding: 24px 18px;
    border-right: 1px solid var(--ui-border);
    background: var(--ui-glass);
    backdrop-filter: blur(22px) saturate(160%);
}

.gallery-brand {
    display: flex;
    gap: var(--ui-space-3);
    align-items: center;
    padding-bottom: var(--ui-space-5);
}

.gallery-mark {
    display: grid;
    width: 42px;
    height: 42px;
    place-items: center;
    border-radius: var(--ui-radius-lg);
    background: var(--ui-fg);
    color: var(--ui-bg);
    font-weight: 800;
}

.gallery-brand h1,
.gallery-brand p,
.gallery-header h2,
.gallery-header p,
.gallery-section-heading h3,
.gallery-section-heading p,
.gallery-entry h4,
.gallery-entry p {
    margin: 0;
}

.gallery-nav,
.gallery-mobile-tabs,
.gallery-controls,
.gallery-control-group {
    display: flex;
    gap: var(--ui-space-2);
}

.gallery-nav {
    flex-direction: column;
}

.gallery-nav a,
.gallery-mobile-tabs a {
    color: var(--ui-muted-fg);
    text-decoration: none;
    border-radius: var(--ui-radius-md);
    padding: 9px 10px;
}

.gallery-nav a:hover,
.gallery-mobile-tabs a:hover {
    background: var(--ui-surface-muted);
    color: var(--ui-fg);
}

.gallery-main {
    width: min(1220px, 100%);
    padding: 36px;
}

.gallery-header,
.gallery-section,
.gallery-entry-copy {
    display: grid;
    gap: var(--ui-space-3);
}

.gallery-eyebrow {
    color: var(--ui-primary);
    font-size: 13px;
    font-weight: 800;
    text-transform: uppercase;
}

.gallery-header h2 {
    font-size: 34px;
    line-height: 1.12;
}

.gallery-header p,
.gallery-section-heading p,
.gallery-entry p {
    color: var(--ui-muted-fg);
    line-height: 1.6;
}

.gallery-controls {
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    margin: 20px 0;
    padding: var(--ui-space-4);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-glass);
    backdrop-filter: blur(18px) saturate(160%);
}

.gallery-control-group {
    align-items: center;
    flex-wrap: wrap;
}

.gallery-control-label {
    font-weight: 800;
}

.gallery-mobile-tabs {
    display: none;
    overflow-x: auto;
    padding-bottom: var(--ui-space-4);
}

.gallery-section {
    padding: 24px 0 12px;
}

.gallery-grid {
    display: grid;
    gap: var(--ui-space-4);
}

.gallery-entry {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(300px, 0.9fr);
    gap: var(--ui-space-4);
    padding: var(--ui-space-4);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: color-mix(in srgb, var(--ui-surface), transparent 10%);
    box-shadow: 0 18px 46px rgba(27, 39, 61, 0.08);
}

.gallery-entry-title {
    display: flex;
    gap: var(--ui-space-3);
    align-items: center;
    justify-content: space-between;
}

.gallery-status {
    border-radius: 999px;
    padding: 4px 8px;
    font-size: 12px;
    font-weight: 800;
}

.gallery-status--ready {
    background: color-mix(in srgb, var(--ui-success), transparent 86%);
    color: var(--ui-success);
}

.gallery-status--soon {
    background: var(--ui-surface-muted);
    color: var(--ui-muted-fg);
}

.gallery-preview {
    min-height: 148px;
    display: grid;
    align-content: center;
    gap: var(--ui-space-3);
    padding: var(--ui-space-4);
    border-radius: var(--ui-radius-lg);
    border: 1px solid var(--ui-border);
    background: linear-gradient(135deg, var(--ui-surface), var(--ui-surface-muted));
    overflow: auto;
}

.gallery-preview--soon {
    color: var(--ui-muted-fg);
}

.gallery-inline {
    display: flex;
    flex-wrap: wrap;
    gap: var(--ui-space-2);
}

.gallery-accessibility {
    display: grid;
    gap: var(--ui-space-1);
    padding: var(--ui-space-3);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface-muted);
}

.gallery-code {
    grid-column: 1 / -1;
    overflow-x: auto;
    margin: 0;
    padding: 14px;
    border-radius: var(--ui-radius-md);
    background: #101722;
    color: #e8eef7;
    font-size: 13px;
    line-height: 1.55;
}

@media (max-width: 820px) {
    .gallery-shell {
        display: block;
    }

    .gallery-rail {
        position: static;
        height: auto;
        border-right: 0;
        border-bottom: 1px solid var(--ui-border);
    }

    .gallery-nav {
        display: none;
    }

    .gallery-mobile-tabs {
        display: flex;
    }

    .gallery-main {
        padding: 24px 16px;
    }

    .gallery-entry {
        grid-template-columns: 1fr;
    }
}
"#;
```

- [ ] **Step 7: Run gallery workbench tests**

Run:

```powershell
cargo test -p component-gallery gallery_renders_advanced_workbench_controls_and_notes
cargo test -p component-gallery gallery_embeds_styles_for_gallery_and_component_classes
```

Expected: both commands PASS.

- [ ] **Step 8: Commit gallery workbench upgrade**

Run:

```powershell
git add examples/component-gallery/src/app.rs examples/component-gallery/src/styles.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat: upgrade component gallery workbench"
```

## Task 8: README And Final Verification

**Files:**
- Modify: `README.md`
- Modify only if verification exposes a concrete defect.

- [ ] **Step 1: Update README with advanced wave status**

Modify `README.md` current status section so it includes:

```markdown
- reusable library CSS through `ui-styles`
- advanced controlled components for forms, overlays, navigation, dashboard display, and empty states
```

Modify the component list so it includes:

```markdown
- `TextField`
- `Checkbox`
- `Switch`
- `Tabs`
- `Dialog`
- `Toast`
- `CommandMenu`
- `Tooltip`
- `Toolbar`
- `Sidebar`
- `MetricCard`
- `EmptyState`
```

- [ ] **Step 2: Run final formatting check**

Run:

```powershell
cargo fmt --all -- --check
```

Expected: PASS.

- [ ] **Step 3: Run focused checks**

Run:

```powershell
cargo check -p component-gallery
cargo test -p ui-styles
cargo test -p ui-dioxus
cargo test -p unified_ui
cargo test -p component-gallery
```

Expected: all commands PASS.

- [ ] **Step 4: Run full workspace test**

Run:

```powershell
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 5: Check for forbidden borrowed public names**

Run:

```powershell
rg -n "Radix|Shadcn|Fluent|Material Design|Framer|AnimatePresence|LayoutGroup|motion::" crates examples README.md docs/component-naming.md docs/glass-materials.md docs/platform-support.md
```

Expected: no matches.

- [ ] **Step 6: Commit docs and verification fixes**

Run:

```powershell
git add README.md Cargo.toml crates examples
git commit -m "docs: document advanced ui wave"
```

If no files changed after verification except README, commit only README with the same message.

## Acceptance Checklist

- [ ] `ui-styles` is a workspace crate and exports `BASE_CSS`, `COMPONENT_CSS`, and `library_css()`.
- [ ] `ui-styles` tests verify theme, density, reduced preference hooks, glass, and advanced selectors.
- [ ] `ui-dioxus` exports `TextField`, `Checkbox`, `Switch`, `Tabs`, `Dialog`, `Toast`, `CommandMenu`, `Tooltip`, `Toolbar`, `Sidebar`, `MetricCard`, and `EmptyState`.
- [ ] New components render semantic HTML/ARIA in SSR tests.
- [ ] `unified_ui::prelude::*` exports all new components and style helpers.
- [ ] `component-gallery` imports `ui-styles` and no longer duplicates component styling in gallery CSS.
- [ ] Gallery registry marks all 12 advanced components `Ready`.
- [ ] Gallery entries include summary, snippet, accessibility note, and live renderer.
- [ ] Gallery renders static theme and density controls.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo check -p component-gallery` passes.
- [ ] `cargo test -p ui-styles` passes.
- [ ] `cargo test -p ui-dioxus` passes.
- [ ] `cargo test -p unified_ui` passes.
- [ ] `cargo test -p component-gallery` passes.
- [ ] `cargo test --workspace` passes.

## Follow-Up Plans

After this wave lands, write separate plans for:

1. Overlay manager, focus trap, and portal behavior.
2. Runtime theme and density switching in the gallery.
3. Full keyboard navigation helpers for tabs, command menu, toolbars, and sidebar.
4. `DataTable`, filter bar, row actions, and bulk action workflows.
5. Playwright visual regression for desktop and mobile gallery states.
6. Native renderer fidelity for the advanced component set.
