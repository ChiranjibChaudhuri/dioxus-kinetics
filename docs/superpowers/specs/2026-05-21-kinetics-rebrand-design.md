# Kinetics Rebrand And Showcase Polish Design

## Goal

Rename the public facade crate from `kinetics` to `kinetics`, swap the
component gallery's "UI" mark and "Kinetics" wordmark for the existing
`docs/assets/dioxus-kinetics-logo.svg`, and expand the showcase previews for
the already-Ready primitives in the Motion, Composition, Capture, and
Foundations categories.

This is sub-project 1 of a 4-part rebrand and animation effort. It is purely
mechanical and presentational: no new components, no motion math, no JS or
wasm runtime work. Animated implementations of Presence, Sequence,
SharedLayout, SharedElement, and IconButton land in later sub-projects.

## Non-Goals

- Implementing Presence, Sequence, SharedLayout, SharedElement, or IconButton.
- Introducing a JS or wasm motion runtime.
- FLIP layout animation.
- Changing component APIs, props, or feature flags.
- Adding new categories or removing existing ones.
- Touching the workspace token, glass, motion, layout, or DOM math crates
  beyond what the rename forces.

## Tech Stack

- Rust 2021, Cargo workspace.
- Dioxus 0.7, Dioxus SSR for tests.
- Static CSS strings in `ui-styles` and `examples/component-gallery/src/styles.rs`.
- Inline SVG as a `&'static str` constant for SSR safety.
- PowerShell on Windows for verification commands.

## Architecture

The change has three independent concerns that land in one spec because they
share the same files:

### Concern 1: Workspace rename

The `crates/kinetics` directory becomes `crates/kinetics`. The package
name in its `Cargo.toml` becomes `kinetics`. Every consumer that named
`kinetics` in a dependency, `use` statement, or doc comment is updated to
`kinetics`. Brand strings "Kinetics" become "Kinetics". The user chose the
full rename option, so historical specs and plans under
`docs/superpowers/specs/` and `docs/superpowers/plans/` are updated too,
even though they describe past phases.

### Concern 2: Gallery logo

The current rail brand block uses two stacked elements:

```rust
span { class: "gallery-mark", "UI" }
div {
    h1 { "Kinetics" }
    p { "Component reference" }
}
```

It is replaced with a single inline SVG container:

```rust
div {
    class: "gallery-logo",
    aria_label: "Kinetics",
    dangerous_inner_html: KINETICS_LOGO_SVG,
}
```

The SVG content is stored as a `pub const KINETICS_LOGO_SVG: &str` in a new
`examples/component-gallery/src/brand.rs` module. The constant is the exact
contents of `docs/assets/dioxus-kinetics-logo.svg`. Inlining keeps the
gallery SSR-pure and avoids depending on an asset pipeline.

The SVG already includes the wordmark and a light-tile background, so the
rail no longer needs a separate text wordmark. A visually-hidden
`<span class="visually-hidden">Kinetics component gallery</span>` is added
next to the logo so the SSR HTML still contains the literal word "Kinetics"
for branding tests and screen readers.

### Concern 3: Showcase variant grids

Each Ready primitive in Motion, Composition, Capture, and Foundations keeps
its registry entry. Its `render` function returns a `.gallery-variant-grid`
of tiles. The `snippet` field stays a single representative usage. The
`summary` and `accessibility` text are updated to mention the variant axes
shown in the preview.

Coming-soon entries (Presence, Sequence, SharedLayout, SharedElement,
IconButton) are not touched.

## Variant Grid Specifications

### GlassLayer

A 3×3 grid of `(level, tone)` pairs at Comfortable density.

| level / tone | Neutral | Warm | Cool |
| --- | --- | --- | --- |
| Floating | tile | tile | tile |
| Raised | tile | tile | tile |
| Sunken | tile | tile | tile |

Each tile labels its `level · tone` in a small uppercase caption and
contains a single line of body text ("Material preview"). Grid container
uses class `.gallery-variant-grid--3x3`.

### TimelineScope

Three stacked variants in a single column grid (`.gallery-variant-grid--stack`):

- **Stagger** — one TimelineScope id `"stagger-demo"` wraps four
  KineticBox tiles whose `cue` is `rise-in` and whose `data-stagger-index`
  attribute is `0..3`. The stagger semantic is recorded in the data
  attribute; visual stagger animation is out of scope.
- **Sequence** — one TimelineScope id `"sequence-demo"` wraps three
  KineticBox tiles whose cues form a chain: `enter`, `settle`, `pulse`.
- **Reduced motion** — same content as Stagger, wrapped in a
  `<div data-ui-transparency="reduced">` so the SSR HTML clearly shows the
  stable-state document.

Each variant has a small caption above it.

### FrameStage

Three side-by-side tiles (`.gallery-variant-grid--3col`). Each tile renders
the same Composition `Composition::new("launch-demo", 1920, 1080, 30, 180)`
at a different frame: `0`, `90`, `179`. Tile captions read
`Frame 0 / 180`, `Frame 90 / 180`, `Frame 179 / 180`.

The FrameClip and FrameLayer content inside each tile is a short title
plus a subtitle that reads the frame number, so frame-deterministic
behavior is visible in the rendered HTML.

### CaptureStage

Three tiles (`.gallery-variant-grid--3col`) for three viewport profiles.
Each tile contains a CaptureStage component whose `viewport` prop is the
lowercase profile name, plus a `.gallery-variant-label` caption above the
component showing the human-readable label:

| viewport prop value | caption label |
| --- | --- |
| `mobile` | "Mobile · 360 × 640" |
| `tablet` | "Tablet · 768 × 1024" |
| `desktop` | "Desktop · 1440 × 900" |

Each tile body contains "Frame N" text at distinct frame numbers (24, 48,
72) so the SSR HTML differentiates the profiles.

### KineticBox

Three tiles (`.gallery-variant-grid--3col`), each with a different cue:
`rise-in`, `fade-in`, `slide-up`. Each tile body contains "Cue preview"
plus the cue name label.

### PresenceGate

Two tiles (`.gallery-variant-grid--2col`):

- **Present** — `present=true`, content "Visible state".
- **Hidden** — `present=false`, with a sibling placeholder div outside the
  PresenceGate so the grid layout stays balanced. The placeholder reads
  "Hidden state".

## File Map

- Move: `crates/kinetics/` to `crates/kinetics/`.
- Modify: `Cargo.toml` workspace members.
- Modify: `crates/kinetics/Cargo.toml` package name and dependency aliases.
- Modify: every `Cargo.toml` that lists `kinetics` as a dependency, so the
  dependency now names `kinetics`. Today that includes
  `examples/component-gallery/Cargo.toml`.
- Modify: `crates/kinetics/tests/prelude.rs` — replace `kinetics::` with
  `kinetics::`. No assertion changes.
- Modify: `examples/component-gallery/src/lib.rs` — register the new
  `brand` module if needed.
- Create: `examples/component-gallery/src/brand.rs` — contains
  `pub const KINETICS_LOGO_SVG: &str = r#"..."#`, populated with the exact
  contents of `docs/assets/dioxus-kinetics-logo.svg`.
- Modify: `examples/component-gallery/src/app.rs` — replace brand block,
  remove the gallery-mark span, remove the gallery-brand h1 and p
  elements. Insert the new gallery-logo div. Replace the main header h2
  text from "Kinetics Component Gallery" to "Kinetics Component
  Gallery". Replace the eyebrow text "Dioxus SaaS library" with
  "Dioxus Kinetics library".
- Modify: `examples/component-gallery/src/docs.rs`:
  - Update preview functions: `timeline_scope_preview`,
    `frame_stage_preview`, `capture_stage_preview`, `glass_layer_preview`.
  - Add preview functions: `kinetic_box_preview`, `presence_gate_preview`.
  - Promote `KineticBox` and `PresenceGate` from implicit children of
    timeline/composition examples into their own registry entries in the
    Motion category, both with `ComponentStatus::Ready`. (They already
    exist as components; this is documentation polish, not new code.)
  - The `COMPONENT_DOCS` length grows from 25 to 27.
  - Refresh `summary` and `accessibility` strings on the affected entries
    to mention the variant axes.
- Modify: `examples/component-gallery/src/styles.rs` — add gallery-logo,
  gallery-variant-grid, gallery-variant-tile, gallery-variant-label,
  viewport-profile scaling, and visually-hidden styles. Update the small
  responsive `@media` block to collapse the variant grids on narrow
  viewports.
- Modify: `examples/component-gallery/tests/gallery.rs`:
  - Replace brand text assertions ("Kinetics" → "Kinetics").
  - Add brand SVG assertion (`<svg` present and the SVG title text
    "dioxus-kinetics logo" present).
  - Add variant-grid markers assertions for GlassLayer, TimelineScope,
    FrameStage, CaptureStage, KineticBox, PresenceGate.
  - Add an assertion that the rendered HTML does NOT contain the literal
    string "Kinetics" anywhere.
  - Update README naming test to expect `crates/kinetics` and
    `use kinetics::prelude::*`.
- Modify: `README.md`:
  - Title and prose: every "Kinetics" becomes "Kinetics" (keep the repo
    name `dioxus-kinetics` as-is).
  - Every `kinetics` becomes `kinetics`.
  - Workspace layout block: `kinetics/` becomes `kinetics/`.
  - Code examples: `use kinetics::prelude::*` becomes
    `use kinetics::prelude::*`.
  - Cargo commands: `-p kinetics` becomes `-p kinetics`.
- Modify: `docs/component-naming.md`, `docs/platform-support.md`,
  `docs/glass-materials.md` — any "Kinetics" or `kinetics` reference.
- Modify: `docs/superpowers/specs/2026-05-20-unified-ui-library-design.md`,
  `docs/superpowers/specs/2026-05-20-advanced-ui-wave-design.md`,
  `docs/superpowers/specs/2026-05-20-component-gallery-design.md`,
  `docs/superpowers/specs/2026-05-21-native-kinetics-systems-design.md`,
  `docs/superpowers/plans/2026-05-20-unified-ui-library.md`,
  `docs/superpowers/plans/2026-05-20-component-gallery.md`,
  `docs/superpowers/plans/2026-05-21-advanced-ui-wave.md`,
  `docs/superpowers/plans/2026-05-21-native-kinetics-systems.md` — search
  and replace `kinetics` → `kinetics` and "Kinetics" → "Kinetics".
  Historical context wording stays; only the names update.
- The doc filenames `docs/superpowers/specs/2026-05-20-unified-ui-library-design.md`
  and `docs/superpowers/plans/2026-05-20-unified-ui-library.md` are kept
  as-is to preserve git history continuity. Only their contents are updated.
  README's Documentation index uses the existing filenames.

## CSS Additions

The following blocks are appended inside `GALLERY_CSS` in
`examples/component-gallery/src/styles.rs`:

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

@media (max-width: 820px) {
    .gallery-variant-grid--3x3,
    .gallery-variant-grid--3col,
    .gallery-variant-grid--2col {
        grid-template-columns: 1fr;
    }
}
```

The existing brand-area selectors that styled the old text wordmark
(`.gallery-brand h1`, `.gallery-brand p`) stay in `GALLERY_CSS`; they
become dead style rules but removing them is a separate refactor and not
required for this spec.

## Tests

### Existing test updates

`crates/kinetics/tests/prelude.rs`:

- All eight existing tests continue to exist, with `kinetics::` replaced
  by `kinetics::`. The test function names do not change.

`examples/component-gallery/tests/gallery.rs`:

- `gallery_renders_ready_examples_and_coming_soon_entries`: replace
  `"Kinetics Component Gallery"` assertion with `"Kinetics Component
  Gallery"`.
- `root_readme_mentions_component_gallery`: assertions stay valid.
- `root_readme_describes_native_systems_without_bridge_language`:
  assertions stay valid.

### New tests

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

#[test]
fn gallery_renders_variant_grids_for_native_kinetics_primitives() {
    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    for tone in ["Neutral", "Warm", "Cool"] {
        for level in ["Floating", "Raised", "Sunken"] {
            assert!(
                html.contains(&format!("{level} · {tone}")),
                "missing GlassLayer tile {level} · {tone}",
            );
        }
    }

    for frame in ["Frame 0 / 180", "Frame 90 / 180", "Frame 179 / 180"] {
        assert!(html.contains(frame), "missing FrameStage tile {frame}");
    }

    for viewport in ["Mobile · 360 × 640", "Tablet · 768 × 1024", "Desktop · 1440 × 900"] {
        assert!(html.contains(viewport), "missing CaptureStage tile {viewport}");
    }

    for cue in ["rise-in", "fade-in", "slide-up"] {
        assert!(
            html.contains(&format!("data-motion-cue=\"{cue}\"")),
            "missing KineticBox cue {cue}",
        );
    }

    assert!(html.contains("Visible state"));
    assert!(html.contains("Hidden state"));
    assert!(html.contains("gallery-variant-grid--3x3"));
    assert!(html.contains("gallery-variant-grid--3col"));
}

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

### Workspace verification

- `cargo fmt --all -- --check`
- `cargo test --workspace`
- `cargo check -p component-gallery`
- Workspace-wide search:
  - `rg -n "kinetics|Kinetics" crates examples docs README.md Cargo.toml` returns no matches.

The negative-guard tests in `crates/kinetics/tests/prelude.rs` and
`examples/component-gallery/tests/gallery.rs` that check for `GSAP`,
`Remotion`, `HyperFrames`, and `Gsap` are preserved unchanged.

## Acceptance Checklist

- [ ] `crates/kinetics` directory no longer exists; `crates/kinetics`
      directory contains the same source tree.
- [ ] Workspace `Cargo.toml` lists `crates/kinetics` instead of
      `crates/kinetics`.
- [ ] `crates/kinetics/Cargo.toml` package name is `kinetics`.
- [ ] Every `use kinetics::*` is replaced with `use kinetics::*`.
- [ ] Component gallery rail renders the SVG logo and no longer shows
      the literal string "Kinetics".
- [ ] Component gallery main header reads "Kinetics Component Gallery".
- [ ] Component gallery `KineticBox` and `PresenceGate` entries exist with
      `ComponentStatus::Ready` and variant-grid previews.
- [ ] Component gallery `GlassLayer`, `TimelineScope`, `FrameStage`,
      `CaptureStage`, `KineticBox`, `PresenceGate` previews use
      `.gallery-variant-grid` containers with the documented modifiers.
- [ ] README, naming docs, platform docs, glass docs, and all docs
      under `docs/superpowers/` reference `kinetics` and "Kinetics".
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo test --workspace` passes.
- [ ] `cargo check -p component-gallery` passes.
- [ ] `rg "kinetics|Kinetics" crates examples docs README.md Cargo.toml`
      returns no matches.
- [ ] Coming-soon entries (Presence, Sequence, SharedLayout, SharedElement,
      IconButton) remain `ComponentStatus::ComingSoon` and untouched.
