<p align="center">
  <img src="docs/assets/dioxus-kinetics-logo.svg" alt="dioxus-kinetics logo" width="480">
</p>

# dioxus-kinetics

`dioxus-kinetics` is a Dioxus-first UI library workspace for downstream SaaS products. It combines semantic component names, Apple-like glass materials, renderer-neutral tokens, reusable library CSS, and a single public facade crate.

The logo in this repository is an original `dioxus-kinetics` mark. It references Dioxus-adjacent ideas like cross-platform motion, Rust UI energy, and layered glass, but it is not the Dioxus logo or a traced copy of Dioxus branding.

The intended downstream import is:

```rust
use kinetics::prelude::*;
```

## What This Repository Contains

This repository is a Rust Cargo workspace. The public API is exposed through `crates/kinetics`; the other crates keep design tokens, material recipes, motion math, layout math, renderer adapters, and optional backend boundaries focused.

Ready rendered components, grouped the way the gallery presents them. Every
component also ships a SaaS-role alias (`Button` / `ActionControl`, `Sidebar` /
`NavigationRail`, `Dialog` / `ModalLayer`, …); both name surfaces are public and
stable — see `docs/component-naming.md`. The newest addition is a dedicated
**AI-native surfaces** family: streaming answers, citations, source rails,
prompt composers, and agent surfaces for AI-native products.

- **Foundations & actions** — `Button`, `IconButton`, `CommandMenu`, `Toolbar`, `DropdownMenu`, `Heading`, `Text`
- **Inputs** — `TextField`, `Checkbox`, `Switch`, `Select`, `Combobox`, `DatePicker`, `RadioGroup`, `Slider`, `SegmentedControl`, `TagInput` / `ChipInput`
- **Forms** — `Form` / `EntryForm` with an accessible error summary, plus a renderer-neutral `FormSchema` / `FieldRules` / `validate` engine (required, length, numeric, email, cross-field `matches`, custom) and a `use_form_error` context hook
- **File upload** — `FileInput` / `FilePicker`, click-or-drop `DropZone` / `UploadZone`, `Attachment` / `FileChip`, `format_bytes`
- **Navigation** — `Breadcrumb`, `Stepper`
- **Layout & surfaces** — `Stack`, `Tabs`, `Sidebar`, `Accordion`, `Surface`, `GlassSurface`, `MetricCard`, `Badge`, `Avatar`, `LiquidGlass` (Apple "Liquid Glass" CSS recipe + the `apple_liquid` / `apple_lens` WGSL presets)
- **AI-native surfaces** — `StreamingText`, `AiStatus`, `CitationChip`, `SourceCard`, `SourceRail`, `PromptInput`, `AssistantPanel`, `AgentTimeline`, `Waveform`, `VoiceInput`, `AnswerPanel` (+ `AnswerSource`, `RelatedQuestions`) — a Perplexity-style answer surface
- **Feedback & overlays** — `Dialog`, `Sheet`, `Popover`, `Tooltip`, `Toast`, `Toaster`, `Alert`, `Progress`, `Skeleton`, `Spinner`, `EmptyState`
- **Guidance** — `Tour` (+ `TourStep`), `Spotlight`
- **Data workflows** — `DataTable`, `Pagination`, `VirtualizedDataTable` / `WindowedDataTable` (+ pure `visible_window`)
- **Charts** — `Sparkline`, `LineChart`, `BarChart`, `DonutGauge`, `AreaChart`, `FunnelChart` (+ `FunnelStage`), `GaugeChart` (semicircle), `Heatmap` (+ `HeatmapRow`), `Treemap` (+ `TreemapItem`) (shared `ChartSeries` / `ChartTone` vocabulary; SR data-table mirrors; deterministic `progress` override for capture)
- **Auth** — `SignInCard` / `AuthCard`, `OAuthButton` (+ `OAuthProvider`), `PasswordStrengthMeter` (+ pure `password_strength`), `MfaCodeInput` / `CodeInput`
- **Billing & plans** — `PricingTable` / `PlanPicker`, `PlanCard` / `PlanTile` (+ `PricingPlan`), `UsageMeter` / `QuotaBar` (+ pure `usage_fraction` / `usage_tone`), `InvoiceList` / `BillingHistory` (+ `Invoice` / `InvoiceStatus`)
- **Sortable surfaces** — `SortableList`, `KanbanBoard` (+ `apply_kanban_move`; full keyboard grab-move-drop with live-region announcements)
- **Motion** — `Presence`, `PresenceGate`, `Sequence`, `TimelineScope`, `KineticBox`, `KineticText`, `SharedLayout`, `SharedElement`, `SplitText`, `MotionPath`
- **Scene & composition** — `Scene`, `Clip` (current); `FrameStage` / `FrameClip` / `FrameLayer` (legacy deprecation shims)
- **Capture** — `CaptureStage`
- **Token studio** — `export_tokens_css(Theme)` + the `kinetics tokens --mode light|dark` CLI (re-skin every surface by injecting one stylesheet)
- **Render-to-video** — `kinetics render --scene <hello|product-intro|report|showreel> --capture-png --encode-mp4` (self-contained HTML frames → PNG via Playwright → H.264 via FFmpeg); add `--capture-pdf` for a multi-page-ready PDF of the settled frame via Playwright Chromium
- **Cinematic blocks** (`ui-blocks`, behind the default `blocks` feature) — `LowerThird`, `Caption`, `WipeTransition`, `MetricCounter`, `SocialOverlay`
- **Learning** (`ui-learn`, behind the default `learn` feature) — `CourseOutline`, `CourseProgressCard`, `ResumeLearning`, `QuestionCard` (5 question shapes + pure `grade_answer`), `QuizResults`, `QuizTimer`, `FlipCard` / `FlashcardDeck` (+ SM-2-lite `next_review` scheduler), `XpBar`, `StreakBadge`, `AchievementUnlock`, `Leaderboard`, `CertificateCard` (export-ready via kinetics-render)

## Design Principles

- Semantic component names based on role and behavior.
- One downstream-facing crate: `kinetics`.
- Apple-like glass styling with solid fallback behavior.
- Reusable `.ui-*` styling exposed through `ui-styles` for downstream apps.
- Web, Desktop, Mobile, and Native adapter boundaries.
- Accessibility and reduced-preference policies at the token and contract level.
- WCAG 2.2 AA as the target for default themes.
- Native timeline, composition, and capture boundaries kept inside the Rust/Dioxus system model.
- Renderer-neutral core logic wherever possible.

## Workspace Layout

```text
crates/
  ui-core/          semantic contracts, roles, IDs, target sizing, a11y policy
  ui-tokens/        color, radius, spacing, density, motion, and preference tokens
  ui-glass/         glass material requests and resolved recipes
  ui-motion/        transition, spring, and presence primitives
  ui-layout/        renderer-neutral FLIP layout math
  ui-dom/           CSS/style serialization for WebView and web targets
  ui-native/        native capability planning for glass rendering
  ui-glass-engine/  WebGPU/WebGL2 liquid-glass render engine (tiered, degrades to SVG/solid)
  ui-glass-dioxus/  Dioxus bindings for the liquid-glass engine
  ui-dioxus/        semantic Dioxus components (incl. the ai/ AI-native surfaces submodule)
  ui-styles/        shared library CSS variables and component classes
  ui-timeline/      native timeline, stagger, presence, scroll, and shared movement contracts
  ui-composition/   native frame composition and deterministic frame sampling
  ui-capture/       native capture stages, viewport profiles, marks, and export manifests
  ui-runtime/       animation runtime: frame scheduler and dioxus hooks
  ui-icons/         curated inline-svg icon components
  ui-blocks/        catalog of reusable cinematic Scene blocks
  ui-learn/         learning-management surfaces (courses, quizzes, flashcards, gamification)
  kinetics/         public facade and prelude
  kinetics-render/  frame-by-frame SSR exporter (HTML + optional PNG/MP4)
  kinetics-cli/     kinetics CLI (init/preview/render/lint/doctor)
examples/
  component-gallery/ runnable Dioxus documentation gallery
  flagship/         self-referential marketing page (full-bleed,
                    composed entirely from existing scenes and
                    components — no documentation chrome)
  showreel/         5-second cinematic showreel (live app + renderable
                    `showreel` scene → MP4 via kinetics-render)
  comet/            Perplexity-Comet-style "agentic browser" landing
                    (dark, teal, LiquidGlass browser mockup)
docs/
  component-naming.md
  glass-materials.md
  platform-support.md
  ai-cheatsheet.md
```

## Features

Default `kinetics` features:

- `web`
- `desktop`
- `mobile`
- `tokens`
- `glass`
- `motion`
- `layout-motion`
- `a11y`
- `timeline`
- `composition`
- `capture`
- `runtime`
- `icons`
- `liquid-glass` (WebGPU/WebGL2 glass via `ui-glass-dioxus`/`ui-glass-engine`)
- `blocks` (the `ui-blocks` cinematic catalog: `LowerThird`, `Caption`, `WipeTransition`, `MetricCounter`, `SocialOverlay`)

`liquid-glass` and `blocks` are on by default but can be dropped with
`--no-default-features`.

Optional (non-default) features:

- `native`
- `a11y-tests`

Example:

```powershell
cargo test -p kinetics --no-default-features --features native
cargo test -p kinetics --no-default-features --features "native timeline composition capture"
```

## Component Gallery

The runnable documentation app lives in `examples/component-gallery`.

It shows the component library category by category:

- a short writeup for each component
- a Rust usage snippet
- a rendered example for ready components
- accessibility notes for every entry
- shared library CSS plus gallery-only layout CSS
- static theme and density preview controls
- disabled coming-soon entries for planned components

Check the gallery:

```powershell
cargo check -p component-gallery
```

Run the gallery with the Dioxus CLI when available:

```powershell
dx serve --package component-gallery
```

The CLI defaults to port `8080`. If another process already owns 8080 (a common culprit on Windows is the Apache instance bundled with EnterpriseDB Postgres at `httpd.exe`), `dx serve` will print "Serving your app" but you will see the other process at `http://localhost:8080`. Diagnose with `netstat -ano | findstr :8080` and pass a free port:

```powershell
dx serve --package component-gallery --port 9173
```

The gallery is registry-driven. To add a future component to the docs, update the registry in `examples/component-gallery/src/docs.rs` with its category, status, summary, snippet, accessibility note, and renderer.

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

## Render & CLI

The workspace ships a Rust frame-by-frame SSR exporter and a CLI
front-end.

`kinetics-render` walks any `Scene` via `SceneClock { driver: Manual }`,
serializes each frame via `dioxus-ssr`, and writes per-frame HTML +
a minimal composition manifest JSON of shape
`{ schema_version, composition: { id, width, height, fps, frame_count } }`.
Optional stages capture PNGs via a Playwright sidecar and encode MP4
via FFmpeg; both stages gracefully skip when their tools are not on
PATH.

The `kinetics` CLI wraps the renderer plus the dev-loop:

```powershell
cargo run --bin kinetics -- --help
cargo run --bin kinetics -- doctor
cargo run --bin kinetics -- render --scene product-intro --out ./out --frames 60 --fps 30
```

See `crates/kinetics-cli/src/main.rs` for the full subcommand
surface.

## AI Agent Integration

A Claude Code skill ships at `.claude/skills/kinetics-scene/SKILL.md`.
Agents loaded into this workspace can use it to author Scene
compositions with the correct API surface: `Scene` / `Clip` /
`SceneDriver`, `SplitText` / `MotionPath`, the `ui-blocks` catalog,
reduced-motion + accessibility patterns, and the workspace's TDD
conventions.

To activate it in an external editor session that uses Claude Code,
clone the workspace and let the agent auto-discover `.claude/skills/`.

## Quick Start

Clone the repository, then run:

```powershell
cargo test --workspace
```

Format check:

```powershell
cargo fmt --all -- --check
```

Focused gallery checks:

```powershell
cargo check -p component-gallery
cargo test -p component-gallery
```

## Public Usage Example

```rust
use kinetics::prelude::*;

let css = library_css();
assert!(css.contains(".ui-command-menu"));

let theme = Theme::default();
let recipe = resolve_glass(
    &theme,
    GlassRequest::new(
        GlassLevel::Floating,
        GlassTone::Neutral,
        GlassDensity::Comfortable,
    ),
);

assert_eq!(ButtonVariant::Primary.class_name(), "ui-button ui-button--primary");
assert_eq!(recipe.backdrop_blur_px, 18.0);
```

Example Dioxus usage:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
fn Example() -> Element {
    rsx! {
        Stack {
            gap: "md".to_string(),
            TextField {
                id: "workspace-name",
                label: "Workspace name",
                value: "Acme Ops",
                help_text: "Visible to teammates",
            }
            Switch {
                id: "auto-renew",
                label: "Auto renew",
                checked: true,
                description: "Keep billing active",
            }
            MetricCard {
                label: "Net revenue",
                value: "$128.4k",
                delta: "+12.5%",
                tone: MetricTone::Success,
            }
            Toast {
                tone: ToastTone::Success,
                title: "Report exported",
                description: "The PDF is ready.",
            }
        }
    }
}
```

Shared CSS can be rendered once near the application root:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
fn App() -> Element {
    let css = library_css();

    rsx! {
        style { "{css}" }
        div {
            "data-ui-theme": "light",
            "data-ui-density": "comfortable",
            Stack {
                Button { "Save changes" }
            }
        }
    }
}
```

## Glass Materials

Glass is represented by a renderer-neutral recipe:

- `GlassLevel`
- `GlassTone`
- `GlassDensity`
- `GlassPolicy`

Web, Desktop, and Mobile WebView paths use `backdrop-filter` when supported. Native targets use the same recipe and map it through `NativeCapabilities`. Reduced transparency and solid fallback policies force a non-blurred surface.

With the default `liquid-glass` feature, `ui-glass-engine` resolves the best
available render tier — WebGPU → WebGL2 → SVG `backdrop-filter` → solid CSS.
Reduced-transparency and high-contrast preferences snap to the solid tier, and
reduced-motion deliberately avoids the live GPU loop.

See `docs/glass-materials.md` for the material model.

## Platform Support

| Target | Status | Backend |
|---|---|---|
| Web | MVP | DOM style adapter |
| Desktop | MVP | WebView DOM style adapter |
| Mobile | MVP | WebView DOM style adapter |
| Native | MVP contract | Native capability adapter |

Timeline, composition, and capture are native Rust/Dioxus systems usable through web, desktop, mobile WebView, and platform-native adapters. They do not depend on third-party animation, video, or capture runtimes.

See `docs/platform-support.md` for more detail.

## Current Status

This is an MVP library foundation. The current implementation includes:

- semantic token scales
- accessibility contracts
- glass material recipes and fallbacks
- motion primitives
- FLIP layout math
- DOM/WebView style adapter
- native capability adapter
- Dioxus semantic component MVP
- advanced SaaS controls and surfaces
- AI-native surfaces (streaming text, status, citations, source cards/rails, prompt input, assistant panel, agent timeline)
- expanded primitive set: typography, feedback, navigation, inputs, overlays, and a data table
- WebGPU/WebGL2 liquid-glass engine with SVG and solid degradation
- shared focus-trap plus Escape/backdrop dismissal across modal and anchored overlays
- Playwright end-to-end coverage (smoke, motion, visual)
- reusable shared CSS crate
- runtime-reactive preference providers (`ReducedMotionProvider`,
  `ThemeProvider`/`use_theme_mode`/`use_density`) that source from
  `prefers-color-scheme` and `data-ui-theme`/`data-ui-density`
- forced-colors (Windows High Contrast) and `prefers-contrast` CSS fallbacks
- native timeline boundary
- native frame composition boundary
- native capture manifest boundary
- unified facade crate
- component gallery example app
- flagship marketing page example (full-bleed hero, scroll-driven
  story, glass triplet, metric strip, CTA — composed from existing
  primitives)

Future phases should add richer keyboard engines, deeper native fidelity work, and deeper backend integrations.

## Documentation

- `docs/component-naming.md`
- `docs/glass-materials.md`
- `docs/platform-support.md`
- `docs/superpowers/specs/2026-05-20-unified-ui-library-design.md`
- `docs/superpowers/specs/2026-05-20-component-gallery-design.md`
- `docs/superpowers/specs/2026-05-20-advanced-ui-wave-design.md`
- `docs/superpowers/plans/2026-05-20-unified-ui-library.md`
- `docs/superpowers/plans/2026-05-20-component-gallery.md`
- `docs/superpowers/plans/2026-05-21-advanced-ui-wave.md`
- `docs/superpowers/specs/2026-05-24-scene-player-design.md` — SP-1 Scene player (Scene, Clip, SceneDriver)
- `docs/superpowers/plans/2026-05-24-scene-player.md`
- `docs/superpowers/specs/2026-05-25-gsap-tier-primitives-design.md` — SP-3 motion primitives (SplitText, MotionPath)
- `docs/superpowers/plans/2026-05-25-gsap-tier-primitives.md`
- `docs/superpowers/specs/2026-05-25-render-cli-catalog-design.md` — SP-4+5+6 render + CLI + ui-blocks catalog
- `docs/superpowers/plans/2026-05-25-render-cli-catalog.md`
- `docs/superpowers/specs/2026-05-25-flagship-marketing-page-design.md` — flagship marketing page (cinematic hero, full-bleed composition)
- `docs/superpowers/plans/2026-05-25-flagship-marketing-page.md`
