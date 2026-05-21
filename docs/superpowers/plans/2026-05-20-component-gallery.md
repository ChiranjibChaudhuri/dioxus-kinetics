# Component Gallery Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a runnable Dioxus component-gallery example app that documents unified UI components by category with writeups, snippets, live rendered examples, and dynamic coming-soon entries.

**Architecture:** Add `examples/component-gallery` as a workspace crate with a small library plus binary. The library owns the documentation registry, Dioxus app components, live previews, and local CSS; the binary only launches the app. Tests compile and SSR-render the app so registry drift and invalid component usage are caught early.

**Tech Stack:** Rust 2021, Cargo workspace, Dioxus 0.7, Dioxus SSR tests, `kinetics::prelude::*`, local CSS embedded in the example app, frequent commits.

---

## Scope Check

The approved spec is one coherent feature: a documentation gallery example app. It does not implement new library components, generated Markdown, search, syntax highlighting, screenshot tests, or a full docs router. Coming-soon entries are visible registry data only; they must not call unavailable components.

## File Structure

Create this structure:

```text
Cargo.toml
README.md
examples/
  component-gallery/
    Cargo.toml
    src/
      app.rs
      docs.rs
      lib.rs
      main.rs
      styles.rs
    tests/
      gallery.rs
```

Responsibilities:

- `Cargo.toml`: add the example crate as a workspace member and expose `kinetics` as a workspace dependency.
- `README.md`: link to the gallery and show how to run it.
- `examples/component-gallery/Cargo.toml`: declare the runnable example crate.
- `examples/component-gallery/src/docs.rs`: registry data, categories, statuses, snippets, and live preview function pointers.
- `examples/component-gallery/src/app.rs`: Dioxus page components that render the registry.
- `examples/component-gallery/src/styles.rs`: local CSS string for the gallery and the MVP component classes.
- `examples/component-gallery/src/lib.rs`: public module exports for tests and the binary.
- `examples/component-gallery/src/main.rs`: `dioxus::launch(component_gallery::App)`.
- `examples/component-gallery/tests/gallery.rs`: registry and SSR tests.

## Task 1: Example Crate Scaffold

**Files:**
- Modify: `Cargo.toml`
- Create: `examples/component-gallery/Cargo.toml`
- Create: `examples/component-gallery/src/lib.rs`
- Create: `examples/component-gallery/src/main.rs`
- Create: `examples/component-gallery/src/app.rs`
- Create: `examples/component-gallery/src/docs.rs`
- Create: `examples/component-gallery/src/styles.rs`

- [ ] **Step 1: Add the example crate to the workspace**

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
    "crates/kinetics",
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
kinetics = { path = "crates/kinetics" }
```

- [ ] **Step 2: Create the example manifest**

Write `examples/component-gallery/Cargo.toml`:

```toml
[package]
name = "component-gallery"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
dioxus.workspace = true
kinetics.workspace = true

[dev-dependencies]
dioxus-ssr.workspace = true

[lib]
path = "src/lib.rs"

[[bin]]
name = "component-gallery"
path = "src/main.rs"
```

- [ ] **Step 3: Add initial source files**

Write `examples/component-gallery/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

mod app;
mod docs;
mod styles;

pub use app::App;
pub use docs::{component_docs, categories, ComponentCategory, ComponentDoc, ComponentStatus};
```

Write `examples/component-gallery/src/main.rs`:

```rust
fn main() {
    dioxus::launch(component_gallery::App);
}
```

Write `examples/component-gallery/src/app.rs`:

```rust
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        main { "Component Gallery" }
    }
}
```

Write `examples/component-gallery/src/docs.rs`:

```rust
use dioxus::prelude::Element;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentCategory {
    Actions,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentStatus {
    Ready,
    ComingSoon,
}

#[derive(Clone, Copy)]
pub struct ComponentDoc {
    pub name: &'static str,
    pub category: ComponentCategory,
    pub status: ComponentStatus,
    pub summary: &'static str,
    pub snippet: &'static str,
    pub render: Option<fn() -> Element>,
}

pub fn categories() -> &'static [ComponentCategory] {
    &[ComponentCategory::Actions]
}

pub fn component_docs() -> &'static [ComponentDoc] {
    &[]
}
```

Write `examples/component-gallery/src/styles.rs`:

```rust
pub const GALLERY_CSS: &str = "";
```

- [ ] **Step 4: Verify the scaffold compiles**

Run:

```powershell
cargo check -p component-gallery
```

Expected: command exits `0`.

- [ ] **Step 5: Commit the scaffold**

Run:

```powershell
git add Cargo.toml examples/component-gallery
git commit -m "chore: scaffold component gallery"
```

## Task 2: Registry Data And Coming-Soon Model

**Files:**
- Modify: `examples/component-gallery/src/docs.rs`
- Create: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Write failing registry tests**

Write `examples/component-gallery/tests/gallery.rs`:

```rust
use component_gallery::{categories, component_docs, ComponentCategory, ComponentStatus};

#[test]
fn registry_groups_components_by_product_category() {
    let categories = categories();

    assert_eq!(
        categories,
        &[
            ComponentCategory::Actions,
            ComponentCategory::Inputs,
            ComponentCategory::Layout,
            ComponentCategory::Surfaces,
            ComponentCategory::Feedback,
            ComponentCategory::Motion,
        ]
    );
}

#[test]
fn registry_contains_ready_and_coming_soon_components() {
    let docs = component_docs();

    assert!(docs.iter().any(|doc| doc.name == "Button" && doc.status == ComponentStatus::Ready));
    assert!(docs.iter().any(|doc| doc.name == "Surface" && doc.status == ComponentStatus::Ready));
    assert!(docs.iter().any(|doc| doc.name == "GlassSurface" && doc.status == ComponentStatus::Ready));
    assert!(docs.iter().any(|doc| doc.name == "Stack" && doc.status == ComponentStatus::Ready));
    assert!(docs.iter().any(|doc| doc.name == "TextField" && doc.status == ComponentStatus::ComingSoon));
    assert!(docs.iter().any(|doc| doc.name == "SharedElement" && doc.status == ComponentStatus::ComingSoon));
}

#[test]
fn coming_soon_entries_do_not_have_live_renderers() {
    for doc in component_docs() {
        if doc.status == ComponentStatus::ComingSoon {
            assert!(doc.render.is_none(), "{} should not render unavailable components", doc.name);
        }
    }
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```powershell
cargo test -p component-gallery
```

Expected: FAIL because `ComponentCategory` does not contain all variants and the registry is empty.

- [ ] **Step 3: Implement categories, statuses, snippets, and registry rows**

Replace `examples/component-gallery/src/docs.rs`:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentCategory {
    Actions,
    Inputs,
    Layout,
    Surfaces,
    Feedback,
    Motion,
}

impl ComponentCategory {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Actions => "Actions",
            Self::Inputs => "Inputs",
            Self::Layout => "Layout",
            Self::Surfaces => "Surfaces",
            Self::Feedback => "Feedback",
            Self::Motion => "Motion",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Actions => "Command controls that trigger a product action.",
            Self::Inputs => "Controls that collect user-entered data.",
            Self::Layout => "Structure primitives for arranging interface regions.",
            Self::Surfaces => "Containers that define visual layers and material treatment.",
            Self::Feedback => "Overlays and messages that respond to user or system state.",
            Self::Motion => "Lifecycle and layout motion primitives for continuity.",
        }
    }

    pub const fn slug(self) -> &'static str {
        match self {
            Self::Actions => "actions",
            Self::Inputs => "inputs",
            Self::Layout => "layout",
            Self::Surfaces => "surfaces",
            Self::Feedback => "feedback",
            Self::Motion => "motion",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentStatus {
    Ready,
    ComingSoon,
}

impl ComponentStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::ComingSoon => "Coming soon",
        }
    }
}

#[derive(Clone, Copy)]
pub struct ComponentDoc {
    pub name: &'static str,
    pub category: ComponentCategory,
    pub status: ComponentStatus,
    pub summary: &'static str,
    pub snippet: &'static str,
    pub render: Option<fn() -> Element>,
}

pub fn categories() -> &'static [ComponentCategory] {
    &[
        ComponentCategory::Actions,
        ComponentCategory::Inputs,
        ComponentCategory::Layout,
        ComponentCategory::Surfaces,
        ComponentCategory::Feedback,
        ComponentCategory::Motion,
    ]
}

pub fn component_docs() -> &'static [ComponentDoc] {
    &COMPONENT_DOCS
}

const COMPONENT_DOCS: [ComponentDoc; 14] = [
    ComponentDoc {
        name: "Button",
        category: ComponentCategory::Actions,
        status: ComponentStatus::Ready,
        summary: "Triggers a user action with semantic variants for primary, secondary, quiet, and destructive commands.",
        snippet: BUTTON_SNIPPET,
        render: Some(button_preview),
    },
    ComponentDoc {
        name: "IconButton",
        category: ComponentCategory::Actions,
        status: ComponentStatus::ComingSoon,
        summary: "A compact icon-only command control with an accessible label.",
        snippet: ICON_BUTTON_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "TextField",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::ComingSoon,
        summary: "A labeled single-line text input for forms and filters.",
        snippet: TEXT_FIELD_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "Checkbox",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::ComingSoon,
        summary: "A binary choice control for settings, filters, and table selection.",
        snippet: CHECKBOX_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "Stack",
        category: ComponentCategory::Layout,
        status: ComponentStatus::Ready,
        summary: "Arranges children in a vertical rhythm with semantic spacing tokens.",
        snippet: STACK_SNIPPET,
        render: Some(stack_preview),
    },
    ComponentDoc {
        name: "Surface",
        category: ComponentCategory::Surfaces,
        status: ComponentStatus::Ready,
        summary: "Creates a solid content layer for panels, sections, and grouped SaaS workflows.",
        snippet: SURFACE_SNIPPET,
        render: Some(surface_preview),
    },
    ComponentDoc {
        name: "GlassSurface",
        category: ComponentCategory::Surfaces,
        status: ComponentStatus::Ready,
        summary: "Creates a translucent material layer with semantic level, tone, and density attributes.",
        snippet: GLASS_SURFACE_SNIPPET,
        render: Some(glass_surface_preview),
    },
    ComponentDoc {
        name: "Tabs",
        category: ComponentCategory::Layout,
        status: ComponentStatus::ComingSoon,
        summary: "Switches between related panels without leaving the current workflow.",
        snippet: TABS_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "Dialog",
        category: ComponentCategory::Feedback,
        status: ComponentStatus::ComingSoon,
        summary: "Presents blocking decisions and focused workflows above the page.",
        snippet: DIALOG_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "Toast",
        category: ComponentCategory::Feedback,
        status: ComponentStatus::ComingSoon,
        summary: "Shows short-lived status feedback after background or user-triggered events.",
        snippet: TOAST_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "Presence",
        category: ComponentCategory::Motion,
        status: ComponentStatus::ComingSoon,
        summary: "Coordinates mounted, exiting, and removed states for animated lifecycles.",
        snippet: PRESENCE_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "Sequence",
        category: ComponentCategory::Motion,
        status: ComponentStatus::ComingSoon,
        summary: "Orders small motion steps into predictable product transitions.",
        snippet: SEQUENCE_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "SharedLayout",
        category: ComponentCategory::Motion,
        status: ComponentStatus::ComingSoon,
        summary: "Coordinates layout continuity across related regions.",
        snippet: SHARED_LAYOUT_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "SharedElement",
        category: ComponentCategory::Motion,
        status: ComponentStatus::ComingSoon,
        summary: "Marks an element that can visually continue between layout states.",
        snippet: SHARED_ELEMENT_SNIPPET,
        render: None,
    },
];

const BUTTON_SNIPPET: &str = r#"Button {
    variant: ButtonVariant::Primary,
    "Save changes"
}"#;

const ICON_BUTTON_SNIPPET: &str = r#"IconButton {
    label: "Archive",
    icon: ArchiveIcon,
}"#;

const TEXT_FIELD_SNIPPET: &str = r#"TextField {
    label: "Workspace name",
    value: workspace_name,
}"#;

const CHECKBOX_SNIPPET: &str = r#"Checkbox {
    label: "Send weekly summary",
    checked: true,
}"#;

const STACK_SNIPPET: &str = r#"Stack {
    gap: "sm".to_string(),
    Button { "Create" }
    Button {
        variant: ButtonVariant::Secondary,
        "Cancel"
    }
}"#;

const SURFACE_SNIPPET: &str = r#"Surface {
    h3 { "Pipeline health" }
    p { "12 workflows running" }
}"#;

const GLASS_SURFACE_SNIPPET: &str = r#"GlassSurface {
    level: GlassLevel::Floating,
    tone: GlassTone::Neutral,
    density: GlassDensity::Comfortable,
    "Revenue operations"
}"#;

const TABS_SNIPPET: &str = r#"Tabs {
    value: "overview",
    items: tabs,
}"#;

const DIALOG_SNIPPET: &str = r#"Dialog {
    title: "Delete workspace",
    open: true,
}"#;

const TOAST_SNIPPET: &str = r#"Toast {
    tone: ToastTone::Success,
    "Report exported"
}"#;

const PRESENCE_SNIPPET: &str = r#"Presence {
    visible: is_open,
    Dialog { title: "Invite member" }
}"#;

const SEQUENCE_SNIPPET: &str = r#"Sequence {
    steps: onboarding_steps,
}"#;

const SHARED_LAYOUT_SNIPPET: &str = r#"SharedLayout {
    layout_id: "billing-plan",
    children
}"#;

const SHARED_ELEMENT_SNIPPET: &str = r#"SharedElement {
    element_id: "customer-row-42",
    children
}"#;

fn button_preview() -> Element {
    rsx! {
        div { class: "gallery-inline",
            Button { variant: ButtonVariant::Primary, "Save changes" }
            Button { variant: ButtonVariant::Secondary, "Review" }
            Button { variant: ButtonVariant::Ghost, "Dismiss" }
            Button { variant: ButtonVariant::Danger, "Delete" }
        }
    }
}

fn stack_preview() -> Element {
    rsx! {
        Stack { gap: "sm".to_string(),
            Button { "Create workspace" }
            Button { variant: ButtonVariant::Secondary, "Import data" }
        }
    }
}

fn surface_preview() -> Element {
    rsx! {
        Surface {
            h4 { "Pipeline health" }
            p { "12 workflows running" }
        }
    }
}

fn glass_surface_preview() -> Element {
    rsx! {
        GlassSurface {
            level: GlassLevel::Floating,
            tone: GlassTone::Neutral,
            density: GlassDensity::Comfortable,
            h4 { "Revenue operations" }
            p { "Forecast updated 4 minutes ago" }
        }
    }
}
```

- [ ] **Step 4: Run registry tests**

Run:

```powershell
cargo test -p component-gallery
```

Expected: PASS for the three registry tests.

- [ ] **Step 5: Commit the registry**

Run:

```powershell
git add examples/component-gallery/src/docs.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat: add component gallery registry"
```

## Task 3: Gallery Page Rendering

**Files:**
- Modify: `examples/component-gallery/src/app.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Add failing SSR tests for page content**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
use dioxus::prelude::*;

#[test]
fn gallery_renders_ready_examples_and_coming_soon_entries() {
    let html = dioxus_ssr::render(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("Kinetics Component Gallery"));
    assert!(html.contains("Actions"));
    assert!(html.contains("Button"));
    assert!(html.contains("Save changes"));
    assert!(html.contains("GlassSurface"));
    assert!(html.contains("Coming soon"));
    assert!(html.contains("TextField"));
    assert!(html.contains("SharedElement"));
}

#[test]
fn gallery_renders_snippets_as_rust_code_blocks() {
    let html = dioxus_ssr::render(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains("language-rust"));
    assert!(html.contains("ButtonVariant::Primary"));
    assert!(html.contains("GlassLevel::Floating"));
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```powershell
cargo test -p component-gallery
```

Expected: FAIL because `App` only renders `Component Gallery`.

- [ ] **Step 3: Implement the registry-driven Dioxus page**

Replace `examples/component-gallery/src/app.rs`:

```rust
use dioxus::prelude::*;

use crate::docs::{categories, component_docs, ComponentCategory, ComponentDoc, ComponentStatus};
use crate::styles::GALLERY_CSS;

#[component]
pub fn App() -> Element {
    rsx! {
        style { "{GALLERY_CSS}" }
        div { class: "gallery-shell",
            aside { class: "gallery-rail",
                div { class: "gallery-brand",
                    span { class: "gallery-mark", "UI" }
                    div {
                        h1 { "Kinetics" }
                        p { "Component reference" }
                    }
                }
                nav { class: "gallery-nav", aria_label: "Component categories",
                    for category in categories() {
                        a { href: "#{category.slug()}", "{category.label()}" }
                    }
                }
            }
            main { class: "gallery-main",
                header { class: "gallery-header",
                    p { class: "gallery-eyebrow", "Dioxus SaaS library" }
                    h2 { "Kinetics Component Gallery" }
                    p {
                        "Semantic components grouped by product function, with live rendered examples for available primitives and disabled coming-soon entries for the next phase."
                    }
                }
                div { class: "gallery-mobile-tabs", aria_label: "Component categories",
                    for category in categories() {
                        a { href: "#{category.slug()}", "{category.label()}" }
                    }
                }
                for category in categories() {
                    CategorySection { category: *category }
                }
            }
        }
    }
}

#[component]
fn CategorySection(category: ComponentCategory) -> Element {
    let docs = component_docs()
        .iter()
        .filter(|doc| doc.category == category)
        .collect::<Vec<_>>();

    rsx! {
        section { id: "{category.slug()}", class: "gallery-section",
            div { class: "gallery-section-heading",
                h3 { "{category.label()}" }
                p { "{category.description()}" }
            }
            div { class: "gallery-grid",
                for doc in docs {
                    ComponentEntry { doc }
                }
            }
        }
    }
}

#[component]
fn ComponentEntry(doc: &'static ComponentDoc) -> Element {
    let status_class = match doc.status {
        ComponentStatus::Ready => "gallery-status gallery-status--ready",
        ComponentStatus::ComingSoon => "gallery-status gallery-status--soon",
    };

    rsx! {
        article { class: "gallery-entry",
            div { class: "gallery-entry-copy",
                div { class: "gallery-entry-title",
                    h4 { "{doc.name}" }
                    span { class: "{status_class}", "{doc.status.label()}" }
                }
                p { "{doc.summary}" }
            }
            div { class: "gallery-example",
                RenderedExample { doc }
            }
            pre { class: "gallery-code",
                code { class: "language-rust", "{doc.snippet}" }
            }
        }
    }
}

#[component]
fn RenderedExample(doc: &'static ComponentDoc) -> Element {
    match (doc.status, doc.render) {
        (ComponentStatus::Ready, Some(render)) => rsx! {
            div { class: "gallery-preview gallery-preview--ready", {render()} }
        },
        _ => rsx! {
            div { class: "gallery-preview gallery-preview--soon", aria_disabled: "true",
                span { "Coming soon" }
                p { "{doc.name} will render here when the component lands." }
            }
        },
    }
}
```

- [ ] **Step 4: Run gallery tests**

Run:

```powershell
cargo test -p component-gallery
```

Expected: PASS for registry and SSR tests.

- [ ] **Step 5: Commit page rendering**

Run:

```powershell
git add examples/component-gallery/src/app.rs examples/component-gallery/tests/gallery.rs
git commit -m "feat: render component gallery page"
```

## Task 4: Gallery Styling And Library Component Classes

**Files:**
- Modify: `examples/component-gallery/src/styles.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Add failing CSS coverage test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_embeds_styles_for_gallery_and_component_classes() {
    let html = dioxus_ssr::render(rsx! {
        component_gallery::App {}
    });

    assert!(html.contains(".gallery-shell"));
    assert!(html.contains(".ui-button--primary"));
    assert!(html.contains(".ui-glass-surface"));
    assert!(html.contains("backdrop-filter"));
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```powershell
cargo test -p component-gallery gallery_embeds_styles_for_gallery_and_component_classes
```

Expected: FAIL because `GALLERY_CSS` is empty.

- [ ] **Step 3: Add local gallery CSS**

Replace `examples/component-gallery/src/styles.rs`:

```rust
pub const GALLERY_CSS: &str = r#"
:root {
    color-scheme: light;
    font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    background: #f5f7fa;
    color: #151922;
}

* {
    box-sizing: border-box;
}

body {
    margin: 0;
    min-width: 320px;
    background:
        linear-gradient(135deg, rgba(205, 231, 255, 0.72), rgba(255, 255, 255, 0.0) 34%),
        linear-gradient(180deg, #f7f9fc 0%, #eef2f7 100%);
}

button,
input,
textarea,
select {
    font: inherit;
}

.gallery-shell {
    display: grid;
    grid-template-columns: 260px minmax(0, 1fr);
    min-height: 100vh;
}

.gallery-rail {
    position: sticky;
    top: 0;
    align-self: start;
    height: 100vh;
    padding: 24px 18px;
    border-right: 1px solid rgba(118, 132, 150, 0.24);
    background: rgba(255, 255, 255, 0.64);
    backdrop-filter: blur(22px) saturate(160%);
}

.gallery-brand {
    display: flex;
    gap: 12px;
    align-items: center;
    padding-bottom: 24px;
}

.gallery-mark {
    display: grid;
    width: 42px;
    height: 42px;
    place-items: center;
    border-radius: 12px;
    background: #111827;
    color: #ffffff;
    font-weight: 700;
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

.gallery-brand h1 {
    font-size: 15px;
}

.gallery-brand p,
.gallery-section-heading p,
.gallery-entry p {
    color: #5d6676;
}

.gallery-nav,
.gallery-mobile-tabs {
    display: flex;
    gap: 8px;
}

.gallery-nav {
    flex-direction: column;
}

.gallery-nav a,
.gallery-mobile-tabs a {
    color: #303846;
    text-decoration: none;
    border-radius: 8px;
    padding: 9px 10px;
    font-size: 14px;
}

.gallery-nav a:hover,
.gallery-mobile-tabs a:hover {
    background: rgba(17, 24, 39, 0.06);
}

.gallery-main {
    width: min(1180px, 100%);
    padding: 36px;
}

.gallery-header {
    display: grid;
    gap: 8px;
    padding-bottom: 28px;
}

.gallery-eyebrow {
    color: #0066cc;
    font-size: 13px;
    font-weight: 700;
    text-transform: uppercase;
}

.gallery-header h2 {
    font-size: 34px;
    line-height: 1.12;
}

.gallery-header p {
    max-width: 760px;
    color: #566174;
    line-height: 1.6;
}

.gallery-mobile-tabs {
    display: none;
    overflow-x: auto;
    padding-bottom: 18px;
}

.gallery-section {
    display: grid;
    gap: 16px;
    padding: 24px 0 12px;
}

.gallery-section-heading {
    display: grid;
    gap: 6px;
}

.gallery-section-heading h3 {
    font-size: 22px;
}

.gallery-grid {
    display: grid;
    gap: 16px;
}

.gallery-entry {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(260px, 0.9fr);
    gap: 16px;
    padding: 16px;
    border: 1px solid rgba(118, 132, 150, 0.20);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.78);
    box-shadow: 0 18px 46px rgba(27, 39, 61, 0.08);
}

.gallery-entry-copy {
    display: grid;
    gap: 10px;
}

.gallery-entry-title {
    display: flex;
    gap: 10px;
    align-items: center;
    justify-content: space-between;
}

.gallery-entry-title h4 {
    font-size: 18px;
}

.gallery-status {
    border-radius: 999px;
    padding: 4px 8px;
    font-size: 12px;
    font-weight: 700;
}

.gallery-status--ready {
    background: rgba(36, 138, 61, 0.12);
    color: #1f7a3a;
}

.gallery-status--soon {
    background: rgba(86, 94, 108, 0.12);
    color: #566174;
}

.gallery-example {
    min-width: 0;
}

.gallery-preview {
    min-height: 132px;
    display: grid;
    align-content: center;
    gap: 10px;
    padding: 16px;
    border-radius: 8px;
    border: 1px solid rgba(118, 132, 150, 0.22);
    background:
        linear-gradient(135deg, rgba(255, 255, 255, 0.86), rgba(242, 247, 255, 0.66)),
        #ffffff;
}

.gallery-preview--soon {
    color: #647084;
    background: repeating-linear-gradient(
        135deg,
        rgba(100, 112, 132, 0.08),
        rgba(100, 112, 132, 0.08) 8px,
        rgba(255, 255, 255, 0.72) 8px,
        rgba(255, 255, 255, 0.72) 16px
    );
}

.gallery-preview--soon span {
    font-weight: 700;
}

.gallery-inline {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
}

.gallery-code {
    grid-column: 1 / -1;
    overflow-x: auto;
    margin: 0;
    padding: 14px;
    border-radius: 8px;
    background: #101722;
    color: #e8eef7;
    font-size: 13px;
    line-height: 1.55;
}

.ui-button {
    min-height: 36px;
    border: 1px solid transparent;
    border-radius: 8px;
    padding: 0 14px;
    font-weight: 700;
    cursor: pointer;
}

.ui-button--primary {
    background: #0066cc;
    color: #ffffff;
    box-shadow: 0 10px 22px rgba(0, 102, 204, 0.20);
}

.ui-button--secondary {
    background: #ffffff;
    color: #182230;
    border-color: rgba(118, 132, 150, 0.28);
}

.ui-button--ghost {
    background: transparent;
    color: #2f3a4b;
}

.ui-button--danger {
    background: #c42b2b;
    color: #ffffff;
}

.ui-surface,
.ui-glass-surface {
    display: grid;
    gap: 6px;
    border-radius: 8px;
    padding: 16px;
}

.ui-surface {
    border: 1px solid rgba(118, 132, 150, 0.22);
    background: #ffffff;
    color: #151922;
}

.ui-glass-surface {
    border: 1px solid rgba(255, 255, 255, 0.58);
    background: rgba(255, 255, 255, 0.66);
    backdrop-filter: blur(18px) saturate(160%);
    box-shadow: 0 18px 42px rgba(27, 39, 61, 0.16);
}

.ui-stack {
    display: flex;
    flex-direction: column;
}

.ui-stack--gap-sm {
    gap: 8px;
}

.ui-stack--gap-md {
    gap: 12px;
}

@media (max-width: 820px) {
    .gallery-shell {
        display: block;
    }

    .gallery-rail {
        position: static;
        height: auto;
        border-right: 0;
        border-bottom: 1px solid rgba(118, 132, 150, 0.24);
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

- [ ] **Step 4: Run CSS test**

Run:

```powershell
cargo test -p component-gallery gallery_embeds_styles_for_gallery_and_component_classes
```

Expected: PASS.

- [ ] **Step 5: Run all gallery tests**

Run:

```powershell
cargo test -p component-gallery
```

Expected: PASS.

- [ ] **Step 6: Commit styling**

Run:

```powershell
git add examples/component-gallery/src/styles.rs examples/component-gallery/tests/gallery.rs
git commit -m "style: add component gallery glass styling"
```

## Task 5: README And Run Instructions

**Files:**
- Modify: `README.md`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Add a failing README coverage test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn root_readme_mentions_component_gallery() {
    let readme = std::fs::read_to_string("README.md").expect("README.md should be readable");

    assert!(readme.contains("Component Gallery"));
    assert!(readme.contains("cargo check -p component-gallery"));
    assert!(readme.contains("dx serve --package component-gallery"));
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```powershell
cargo test -p component-gallery root_readme_mentions_component_gallery
```

Expected: FAIL because the README does not mention the gallery yet.

- [ ] **Step 3: Update README with gallery docs**

Replace `README.md`:

```markdown
# Kinetics

Kinetics is a Dioxus-first UI library for downstream SaaS products.

The library exposes one public crate:

```rust
use kinetics::prelude::*;
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

## Component Gallery

The runnable documentation app lives in `examples/component-gallery`.

Check it with:

```powershell
cargo check -p component-gallery
```

Run it with the Dioxus CLI when available:

```powershell
dx serve --package component-gallery
```

The gallery is registry-driven. Ready components render live examples, and planned components appear as disabled coming-soon entries until their implementation lands.

## Documentation

- `docs/component-naming.md`
- `docs/glass-materials.md`
- `docs/platform-support.md`
- `docs/superpowers/specs/2026-05-20-component-gallery-design.md`
```

- [ ] **Step 4: Run README test**

Run:

```powershell
cargo test -p component-gallery root_readme_mentions_component_gallery
```

Expected: PASS.

- [ ] **Step 5: Commit README update**

Run:

```powershell
git add README.md examples/component-gallery/tests/gallery.rs
git commit -m "docs: document component gallery"
```

## Task 6: Final Verification And Optional Local Serve

**Files:**
- Modify only if verification exposes a concrete defect.

- [ ] **Step 1: Run formatting check**

Run:

```powershell
cargo fmt --all -- --check
```

Expected: command exits `0`.

- [ ] **Step 2: Run focused gallery check**

Run:

```powershell
cargo check -p component-gallery
```

Expected: command exits `0`.

- [ ] **Step 3: Run focused gallery tests**

Run:

```powershell
cargo test -p component-gallery
```

Expected: all gallery tests pass.

- [ ] **Step 4: Run full workspace tests**

Run:

```powershell
cargo test --workspace
```

Expected: all workspace tests pass, including existing library tests and the new gallery tests.

- [ ] **Step 5: Check Dioxus CLI availability**

Run:

```powershell
Get-Command dx -ErrorAction SilentlyContinue
```

Expected if installed: output includes a `dx` command path.

Expected if not installed: no output. In that case, do not claim the local server was started.

- [ ] **Step 6: Start the gallery only if `dx` exists**

If Step 5 found `dx`, run:

```powershell
dx serve --package component-gallery
```

Expected: the command reports a local URL. Keep the server running only if the user wants to inspect it now; otherwise stop it after confirming startup.

- [ ] **Step 7: Commit any verification fixes**

If verification changed files, run:

```powershell
git add Cargo.toml README.md examples/component-gallery
git commit -m "fix: satisfy component gallery verification"
```

If no files changed, run:

```powershell
git status --short
```

Expected: only the existing untracked `Reading_material/` path may appear.

## Acceptance Checklist

- [ ] `examples/component-gallery` is a workspace crate.
- [ ] The binary launches `component_gallery::App`.
- [ ] The app uses `kinetics::prelude::*` for live examples.
- [ ] Ready entries render `Button`, `Surface`, `GlassSurface`, and `Stack`.
- [ ] Coming-soon entries exist for `IconButton`, `TextField`, `Checkbox`, `Tabs`, `Dialog`, `Toast`, `Presence`, `Sequence`, `SharedLayout`, and `SharedElement`.
- [ ] Coming-soon entries do not call unavailable components.
- [ ] Each entry has a category, status, summary, snippet, and rendered or disabled preview.
- [ ] Styling includes Apple-like glass and CSS for the current `.ui-*` component classes.
- [ ] `cargo fmt --all -- --check` exits `0`.
- [ ] `cargo check -p component-gallery` exits `0`.
- [ ] `cargo test -p component-gallery` exits `0`.
- [ ] `cargo test --workspace` exits `0`.
