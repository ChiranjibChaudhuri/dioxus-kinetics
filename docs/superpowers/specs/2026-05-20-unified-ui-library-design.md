# Unified UI Library Design

Date: 2026-05-20
Status: Ready for user review

## Goal

Create a single Dioxus UI library for downstream SaaS products. The library will provide semantic components, Apple-like glass styling, motion, layout animation, accessibility defaults, and renderer adapters for Web, Desktop, Mobile, and Dioxus Native.

The selected direction is an adapter-first unified core:

- Build portable component contracts, tokens, glass materials, motion primitives, and renderer adapter interfaces first.
- Support Web, Desktop, Mobile, and Native as planned public targets.
- Keep GSAP and HyperFrames outside the default runtime path as optional web/export integrations.
- Expose one primary downstream dependency with stable public exports and feature flags.

## Research Summary

The local reading material recommends a layered Dioxus architecture: pure Rust motion core, Dioxus bindings, DOM/webview backend, optional native adapter, optional GSAP backend, and optional HyperFrames export. It also recommends primitives and styled examples as separate concerns, with accessibility and performance treated as core constraints.

External research reinforces the same direction:

- Dioxus components are ordinary Rust functions with typed properties, and reusable components are the intended way to scale UI across apps.
- Dioxus Web renders through WASM/DOM; Desktop and Mobile use system WebViews; Dioxus Native is experimental and uses Blitz/WebGPU.
- `backdrop-filter` is broadly available in modern browsers but still needs fallback behavior for older devices, native renderer gaps, high contrast, and reduced transparency.
- WCAG 2.2 AA contrast, focus, non-text contrast, motion, and target-size rules affect glass surfaces and animated widgets directly.
- Design-system naming guidance favors names that describe purpose and behavior, remain consistent, and scale as the system grows.

## Target Platforms

MVP target matrix:

| Target | Contract | Notes |
|---|---|---|
| Web | First-class | DOM style writer, CSS variables, `backdrop-filter`, pointer/touch handling, optional WAAPI/View Transitions |
| Desktop | First-class | WebView-backed behavior should match Web where platform WebView support allows |
| Mobile | First-class | WebView-backed behavior with touch, safe-area, density, and mobile target-size policies |
| Native | First-class planned target | Adapter included in architecture; maturity must be documented because Dioxus Native/Blitz is experimental |

Native support must not force web-only dependencies into the core. Native can render lower-fidelity glass until backdrop sampling and filters are available.

## Public Package Model

Downstream SaaS products should consume one library:

```rust
use unified_ui::prelude::*;
```

The implementation may use internal workspace crates, but downstream apps should normally depend only on `unified_ui`.

Recommended feature flags:

- `web`
- `desktop`
- `mobile`
- `native`
- `tokens`
- `glass`
- `motion`
- `layout-motion`
- `a11y`
- `a11y-tests`
- `gsap`
- `hyperframes-export`

Default features should include Web/Desktop/Mobile support, semantic components, tokens, glass materials, accessibility defaults, and basic motion. `native` should be available early but documented as less mature until renderer APIs stabilize. `gsap` and `hyperframes-export` must never be default features.

## Workspace Architecture

Internal crates:

| Crate | Responsibility |
|---|---|
| `ui-core` | Semantic component contracts, state machines, events, IDs, slots, accessibility metadata, keyboard rules, shared prop types |
| `ui-tokens` | Color, typography, spacing, radii, elevation, density, z-index, motion, and glass material tokens |
| `ui-glass` | Target-neutral material recipes for translucent and fallback surfaces |
| `ui-motion` | Pure Rust springs, tweens, easing, keyframes, timelines, presence, stagger, reduced-motion policy, deterministic clocks |
| `ui-layout` | Measurement abstractions, FLIP/shared-layout state, layout IDs, resize/scroll hooks, native layout hooks |
| `ui-dioxus` | Dioxus components and hooks that bind core contracts into `rsx!` |
| `ui-dom` | Web/Desktop/Mobile style writer, CSS variables, backdrop filters, WAAPI/View Transition integration, pointer/touch handling |
| `ui-native` | Dioxus Native/Blitz adapter for tokens, surfaces, layout, basic motion, and accessibility mapping |
| `ui-gsap` | Optional web-only advanced animation backend |
| `ui-hyperframes` | Optional export-only backend for deterministic demo/video rendering |
| `unified_ui` | Curated public crate and prelude for downstream SaaS apps |

Core rule: semantic state belongs in Dioxus. Per-frame visual updates belong in renderer controllers so animation does not force full subtree rerenders.

## Component Naming

Use semantic names. The name must describe the user-facing role or behavior, not an implementation detail, inspiration source, or visual style. Public components must not use names like `Radix*`, `Shadcn*`, `Motion*`, `Fluent*`, or `Material*`.

Component groups:

| Group | Components |
|---|---|
| Actions | `Button`, `IconButton`, `SplitButton`, `ButtonGroup` |
| Text entry | `TextField`, `TextArea`, `SearchField`, `PasswordField`, `NumberField` |
| Choice | `Checkbox`, `RadioGroup`, `Switch`, `Select`, `Combobox`, `SegmentedControl` |
| Disclosure | `Accordion`, `Collapsible`, `Details` |
| Navigation | `Tabs`, `Breadcrumbs`, `NavRail`, `NavBar`, `Pagination`, `CommandMenu` |
| Feedback | `Alert`, `Toast`, `Progress`, `Spinner`, `Skeleton`, `Meter` |
| Overlays | `Dialog`, `Drawer`, `Popover`, `Tooltip`, `Menu`, `ContextMenu` |
| Surfaces | `Surface`, `GlassSurface`, `Card`, `Panel`, `Sheet`, `Toolbar`, `Sidebar` |
| Data display | `Table`, `List`, `Tree`, `Badge`, `Avatar`, `Timeline` |
| Layout | `Stack`, `Cluster`, `Grid`, `Inset`, `Divider`, `ScrollArea` |
| Motion | `Presence`, `Transition`, `Sequence`, `SharedLayout`, `SharedElement` |
| Export/demo | `Composition`, `Frame`, `Scene`, `RenderTrack` |

Renaming examples:

- `motion::div` should become a semantic wrapper such as `AnimatedBox`, or behavior props on `Box`/`Surface`.
- `AnimatePresence` should become `Presence`.
- `LayoutGroup` should become `SharedLayout`.
- Shadcn-style components should be described as editable component templates.
- GSAP-specific public components should become backend selections, for example `TimelineBackend::Gsap`.

## Visual Direction

The default visual language should be Apple-like in discipline without copying platform-owned names or exact native controls.

Design qualities:

- restrained translucency
- crisp typography
- quiet chrome
- precise spacing
- subtle layered depth
- polished focus and hover states
- fast, short motion
- strong light and dark themes
- mobile-safe hit targets and safe-area behavior

Glass should be used for app chrome, overlays, command palettes, floating toolbars, sidebars, drawers, and selected dashboard panels. Dense SaaS content such as data tables, long forms, settings, and high-information cards should default to solid or near-solid surfaces for readability.

The library should feel at home in an Apple ecosystem while remaining a cross-platform SaaS design system.

## Glass Material System

Glass is a tokenized material system, not scattered CSS.

Material axes:

```rust
pub enum GlassLevel {
    Subtle,
    Floating,
    Overlay,
    Chrome,
}

pub enum GlassTone {
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

pub enum GlassDensity {
    Compact,
    Comfortable,
    Spacious,
}

pub enum GlassPolicy {
    Auto,
    SolidFallback,
    HighContrast,
    ReducedTransparency,
}
```

Material tokens:

- background tint color and alpha
- backdrop blur radius
- saturation and brightness adjustment
- border tint
- inner highlight
- shadow/elevation
- content contrast color
- focus ring color
- fallback solid background
- hover, pressed, disabled, selected, and danger overlays

Renderer behavior:

- Web/Desktop/Mobile use `backdrop-filter` where supported.
- Web/Desktop/Mobile render a tokenized solid fallback when backdrop filtering is unavailable.
- Native maps material tokens to available renderer primitives.
- If native cannot sample the backdrop, it uses simulated glass: tint, border, elevation, and optional content-layer blur where available.
- Reduced transparency, high contrast, forced colors, and low-power modes can collapse glass into solid or near-solid surfaces.

Contrast rule:

Text and icons on glass must be validated against the worst-case fallback background, not only against the ideal blurred state.

## Theme System

Themes must be safe for downstream SaaS branding without forking.

Theme dimensions:

- brand palette
- semantic colors
- light/dark mode
- density
- radius scale
- typography stack
- glass strength
- elevation scale
- contrast mode
- reduced motion
- reduced transparency
- platform defaults

Tokens should use semantic intent rather than hard-coded product color names. Apps should be able to define product-specific themes while reusing the same component contracts.

## Motion System

Motion is part of the unified library, but advanced web tooling remains optional.

Core motion features:

- springs
- tweens
- easing curves
- keyframes
- timelines
- presence lifecycle
- stagger
- deterministic clocks
- reduced-motion transforms

Public semantic APIs:

- `Presence`
- `Transition`
- `Sequence`
- `SharedLayout`
- `SharedElement`

Default SaaS motion:

- fade
- scale
- slide
- sheet/drawer movement
- shared underline transitions
- toast stacking
- table/list reorder transitions
- command palette entrance/exit

Avoid surprise parallax, scroll theatrics, and hero-style animation in ordinary SaaS workflows.

## Layout Animation

Layout animation uses progressive capability:

1. Transform and opacity transitions for simple changes.
2. FLIP measurement where mounted layout boxes are available.
3. Shared-layout registries through stable IDs.
4. WAAPI/View Transitions where web support is available.
5. GSAP Flip only through the optional `gsap` feature.

`SharedLayout` owns layout coordination. `SharedElement` identifies paired elements. Components that participate in layout animation must use stable keys/IDs.

SSR/fullstack behavior:

- Server output renders deterministic initial state.
- Clocks, observers, measurements, and layout animation start after mount/hydration.
- Measurement-dependent effects must degrade to non-animated or transform-only behavior during SSR.

## Backend Strategy

### DOM/WebView Backend

Default for Web, Desktop, and Mobile.

Responsibilities:

- CSS variable emission
- class/style writing
- transform, opacity, filter, and backdrop-filter application
- requestAnimationFrame scheduling
- pointer/touch handling
- safe-area handling
- capability detection
- fallback material rendering

### Native Backend

First-class planned target.

Responsibilities:

- render semantic tokens through native/Blitz-supported style primitives
- map glass to best available material representation
- support focus, hit targets, layout primitives, and basic motion
- expose unsupported capabilities cleanly rather than silently breaking

Native support starts with semantic parity and reduced visual fidelity where needed. Full visual parity is not required for MVP if the native renderer lacks required primitives.

### GSAP Backend

Optional web-only backend for:

- complex timelines
- scroll choreography
- observer gestures
- path animation
- plugin-heavy demos
- advanced FLIP scenarios

GSAP is an implementation backend, not a naming source and not a default dependency.

### HyperFrames Backend

Optional export-only backend for deterministic demo/video rendering.

Use cases:

- marketing videos
- demo recordings
- deterministic component animation captures
- benchmark export scenes

HyperFrames must not be used as the normal interactive UI runtime.

## Accessibility Requirements

Target WCAG 2.2 AA for default themes.

Required policies:

- visible focus states over glass and solid surfaces
- keyboard behavior aligned with WAI-ARIA Authoring Practices where applicable
- reduced-motion support
- reduced-transparency support
- high-contrast and forced-colors support
- mobile target-size policy
- no semantic reparenting that breaks screen readers during presence/layout transitions
- focus restoration for overlays
- escape/close behavior for modal surfaces
- drag alternatives where dragging is not essential

Complex widgets requiring explicit accessibility contracts:

- `Dialog`
- `Drawer`
- `Popover`
- `Tooltip`
- `Menu`
- `ContextMenu`
- `Tabs`
- `Accordion`
- `Combobox`
- `Select`
- `Tree`
- `Table`
- `CommandMenu`

## Testing Strategy

Rust unit tests:

- token resolution
- theme merging
- glass material fallback selection
- component state machines
- keyboard interaction state
- presence lifecycle
- layout math
- spring/tween outputs
- reduced-motion transformations

SSR tests:

- deterministic markup snapshots
- hydration-safe initial state
- no server-side clock advancement
- no measurement-dependent SSR output

Playwright tests:

- Web preview routes
- Desktop WebView smoke routes where automation supports it
- Mobile viewport and touch interactions
- keyboard navigation
- focus restoration
- pointer/touch gestures
- glass fallback behavior
- visual regression
- reduced motion and reduced transparency

Native smoke tests:

- token rendering
- semantic component rendering
- glass fallback rendering
- basic motion adapter behavior
- focus and keyboard behavior

Performance benchmarks:

- gzipped WASM size by feature set
- frame time for many simultaneous animations
- mount/unmount cleanup
- layout animation measurement cost
- memory growth in repeated presence cycles
- Web/Desktop/Mobile parity checks

## Release Plan

1. Foundation

   Tokens, semantic naming, theme engine, glass materials, component contracts, public crate/prelude, and feature flag model.

2. Component MVP

   Actions, text entry, choice controls, navigation, overlays, feedback, surfaces, and layout primitives.

3. Motion MVP

   `Presence`, `Transition`, `Sequence`, `SharedLayout`, basic springs/tweens, and DOM/webview style writer.

4. Native adapter

   Semantic component parity, token rendering, glass fallback, focus behavior, and basic motion.

5. Advanced web backends

   WAAPI/View Transitions first, optional GSAP after the portable path is stable.

6. Export tooling

   HyperFrames deterministic scene export for demos and marketing assets.

7. SaaS hardening

   Documentation, examples, migration guide, versioning policy, performance budgets, visual regression suite, and accessibility audit scripts.

## Versioning And Migration Policy

Downstream SaaS users need predictable upgrades.

Rules:

- Public semantic components and props follow semantic versioning.
- Component renames require aliases before removal.
- Deprecated APIs include migration notes.
- Breaking visual token changes are treated as major-version changes unless opt-in.
- Optional backend changes must not break core component usage.
- Default feature changes require migration notes because they affect bundle size and platform behavior.
- Native maturity must be documented per release.

## Open Risks

Native renderer maturity:

Dioxus Native/Blitz is experimental, so native support must begin with semantic parity and graceful visual fallback.

Glass accessibility:

Translucent surfaces can fail contrast in real layouts. The library must test fallback backgrounds and offer solid/high-contrast policies.

Bundle size:

Motion, layout measurement, and optional web backends can increase cost. Feature flags and benchmark budgets are required.

Per-frame rerender risk:

Animation must use renderer controllers for per-frame visual updates. Full Dioxus subtree rerenders per frame are not acceptable.

Backend fragmentation:

DOM, webview, and native may differ. Adapters must expose capabilities explicitly and share the same semantic contracts.

## Acceptance Criteria

The design is implementation-ready when the written implementation plan can produce:

- one `unified_ui` public crate with a stable prelude
- semantic component APIs with no borrowed naming
- Apple-like glass material tokens and fallbacks
- Web/Desktop/Mobile renderer support through DOM/webview
- Native adapter support with documented maturity
- basic motion and layout transitions without per-frame subtree rerenders
- accessibility policies for focus, keyboard, reduced motion, reduced transparency, and contrast
- optional GSAP and HyperFrames integrations kept out of defaults
- downstream SaaS theming and migration policy
- test strategy covering unit, SSR, Playwright, native smoke, accessibility, and performance

## Sources

- Local reading: `Reading_material/deep-research-report_1.md`
- Local reading: `Reading_material/deep-research-report_2.md`
- Dioxus components and properties: https://dioxuslabs.com/learn/0.7/essentials/ui/components/
- Dioxus project structure and renderers: https://dioxuslabs.com/learn/0.7/beyond/project_structure/
- Dioxus mobile support: https://dioxuslabs.com/learn/0.6/guides/mobile/
- MDN `backdrop-filter`: https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/backdrop-filter
- WCAG 2.2: https://www.w3.org/TR/WCAG22/
- VA.gov Design System naming conventions: https://design.va.gov/about/naming-conventions/
