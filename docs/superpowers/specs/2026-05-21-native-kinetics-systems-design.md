# Native Kinetics Systems Design

## Goal

Reframe the next `dioxus-kinetics` wave as native Rust and Dioxus infrastructure instead of optional bridges to GSAP, Remotion, or HyperFrames. The external products remain useful research references for expected capability classes, but the implementation, runtime model, public API, crate names, feature flags, and downstream story must be owned by this library.

The library should become a single downstream SaaS UI system with:

- semantic components named by function, not borrowed ecosystem terms
- Apple-like glass materials with reliable solid fallbacks
- native timeline choreography for UI motion
- native frame composition for deterministic preview and export-safe scenes
- native capture and frame seeking contracts for documentation, demos, and future render export
- web, desktop, mobile WebView, and native capability paths
- reduced-motion, reduced-transparency, high-contrast, and SSR behavior designed in from the start

## Corrected Direction

The previous bridge-oriented plan is replaced by a native-only architecture.

Old direction:

- `ui-gsap` as an optional GSAP backend boundary
- `ui-hyperframes` as an optional HyperFrames export boundary
- Remotion-style features through an external composition bridge

New direction:

- `ui-timeline`: native timeline, stagger, scroll, FLIP, path, text, and presence choreography
- `ui-composition`: native frame-based scene composition and deterministic playback
- `ui-capture`: native frame seeking, viewport profiles, export manifests, and capture-safe scene running

No default or optional feature should load GSAP, Remotion, HyperFrames, or their runtime models. The implementation may learn from those tools, but should not wrap, shell out to, or mirror their names in the public API.

## Current Context

The workspace already contains useful foundations:

- `ui-core`: semantic contracts, roles, IDs, target sizing, accessibility policy
- `ui-tokens`: color, radius, spacing, density, motion, and preference tokens
- `ui-glass`: material request and recipe resolution
- `ui-motion`: spring, transition, and presence primitives
- `ui-layout`: FLIP layout math
- `ui-dom`: CSS/style serialization for web and WebView targets
- `ui-native`: native capability planning
- `ui-dioxus`: Dioxus components
- `ui-styles`: shared CSS variables and component classes
- `kinetics`: public facade and prelude
- `examples/component-gallery`: documentation workbench

The current bridge-like crates are placeholders. They should be renamed and reworked before they become a public downstream contract.

## Naming Policy

Public component names must describe user-facing function. They must not borrow names from animation products, video products, platform frameworks, or design systems.

The default public prelude should move toward these names:

| Current MVP Name | Native Functional Name | Purpose |
|---|---|---|
| `Button` | `ActionControl` | initiates an operation |
| `TextField` | `TextEntry` | accepts plain text input |
| `Checkbox` | `ChoiceMark` | toggles inclusion in a set |
| `Switch` | `StateSwitch` | toggles one binary setting |
| `Tabs` | `ViewSwitcher` | switches between peer views |
| `Toolbar` | `ActionBar` | groups common commands |
| `Sidebar` | `NavigationRail` | persistent app navigation |
| `MetricCard` | `MetricReadout` | displays a KPI and trend |
| `EmptyState` | `BlankState` | explains an empty workflow state |
| `Dialog` | `ModalLayer` | interrupts with a contained decision |
| `Toast` | `NoticeStack` | presents transient feedback |
| `CommandMenu` | `CommandFinder` | searches and invokes commands |
| `Tooltip` | `ContextHint` | gives short contextual help |
| `Surface` | `ContentPlane` | groups content on a stable plane |
| `GlassSurface` | `GlassLayer` | displays content on a translucent material |

Because the repository is still version `0.1.0`, the implementation may make a breaking rename. If compatibility is needed for local examples during the transition, old names can exist behind an explicit `legacy-names` feature, but the default prelude should expose the new functional names.

## Public Crate Shape

Downstream SaaS apps should still use one import:

```rust
use kinetics::prelude::*;
```

The public facade should expose stable modules:

```rust
kinetics::prelude
kinetics::components
kinetics::materials
kinetics::motion
kinetics::timeline
kinetics::composition
kinetics::capture
kinetics::platform
```

The public facade should not expose internal crate topology unless consumers deliberately opt into lower-level crates.

## Workspace Rename Plan

Implementation should update the workspace as follows:

| Current Crate | New Crate | Role |
|---|---|---|
| `ui-gsap` | `ui-timeline` | native timeline choreography |
| `ui-hyperframes` | `ui-capture` | native capture, frame seek, export manifest |
| none | `ui-composition` | native frame composition |
| `ui-motion` | keep | low-level physics, interpolation, easing |
| `ui-layout` | keep | geometry, FLIP, shared movement math |
| `ui-glass` | keep | material recipes and preference fallbacks |
| `ui-dioxus` | keep | semantic Dioxus components |
| `ui-styles` | keep | shared CSS variables and component classes |

Feature flags should also be renamed:

| Current Feature | New Feature |
|---|---|
| `gsap` | `timeline` |
| `hyperframes-export` | `capture` |
| none | `composition` |

No feature name should imply a third-party dependency.

## Architecture Overview

The advanced wave should use five layers.

### 1. Semantic Component Layer

This is the downstream SaaS API. Components render accessible Dioxus structures, receive controlled props, and expose stable classes and data attributes.

Examples:

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
- `RecordGrid`
- `FilterBuilder`
- `QueryComposer`
- `InspectorPane`
- `SplitWorkspace`
- `DockArea`
- `ActivityTimeline`
- `UploadDropzone`
- `StatusBeacon`

### 2. Material Layer

This layer resolves Apple-like glass, solid surfaces, borders, focus rings, depth, and fallback behavior. It should stay renderer-neutral and preference-aware.

Core objects:

- `MaterialRequest`
- `MaterialRecipe`
- `GlassDepth`
- `MaterialTone`
- `MaterialDensity`
- `MaterialPolicy`
- `DepthShadow`
- `FocusHalo`
- `SolidFallback`
- `ContrastGuard`

`ui-glass` can keep the existing `GlassRequest` and `GlassRecipe` types during migration, but the public facade should prefer more functional names such as `GlassLayer` and `MaterialRecipe`.

### 3. Motion Math Layer

This is pure Rust math and state. It should not know about Dioxus nodes, CSS, browser APIs, or native surfaces.

Core responsibilities:

- spring stepping
- easing functions
- interpolation
- color interpolation
- transform interpolation
- presence states
- velocity and settling rules
- reduced-motion substitution
- deterministic sampling by time or frame

### 4. Timeline Layer

This layer choreographs UI motion over time, scroll progress, gesture progress, or manual progress. It is inspired by high-end animation timelines but implemented natively.

Core objects:

- `Timeline`
- `TimelineScope`
- `TimelineClock`
- `TimelineLabel`
- `TimelineTrack`
- `MotionCue`
- `MotionSegment`
- `StaggerFlow`
- `PresenceGate`
- `SharedMove`
- `ScrollDirector`
- `GestureTrack`
- `PathTrack`
- `TextFlow`

### 5. Composition And Capture Layer

This layer renders deterministic frame-based scenes for previews, docs, demos, and future export. It is inspired by frame-accurate video composition and HTML capture tools, but implemented as native Dioxus/Rust contracts.

Core objects:

- `Composition`
- `FrameClock`
- `FrameStage`
- `FrameClip`
- `FrameLayer`
- `FrameCue`
- `FrameValue`
- `FrameSpring`
- `CaptureStage`
- `CaptureRoute`
- `CaptureMark`
- `ViewportProfile`
- `ExportManifest`
- `CaptureRunner`

## Advanced Glass Styling

The next glass system must be deeper than the current blur surface.

### Material Axes

`GlassLayer` should support:

- `depth`: inline, raised, floating, chrome, overlay, modal
- `tone`: neutral, primary, success, warning, danger, info
- `density`: compact, comfortable, spacious
- `edge`: none, hairline, standard, emphasized
- `vibrancy`: muted, standard, vivid
- `shadow`: none, soft, lifted, modal
- `policy`: auto, solid fallback, reduced transparency, high contrast

Example:

```rust
rsx! {
    GlassLayer {
        depth: GlassDepth::Floating,
        tone: MaterialTone::Neutral,
        density: MaterialDensity::Comfortable,
        edge: MaterialEdge::Hairline,
        MetricReadout {
            label: "Net revenue",
            value: "$128.4k",
            trend: "+12.5%",
        }
    }
}
```

### CSS Output

The DOM adapter should emit:

- `background`
- `border-color`
- `box-shadow`
- `backdrop-filter`
- `-webkit-backdrop-filter`
- `color`
- CSS variables for depth, blur, saturation, opacity, and inner highlight
- solid fallback variables

The default CSS should include:

```css
.ui-glass-layer {
    background: var(--ui-material-bg);
    border: 1px solid var(--ui-material-border);
    box-shadow: var(--ui-material-shadow);
    backdrop-filter: blur(var(--ui-material-blur)) saturate(var(--ui-material-saturate));
    -webkit-backdrop-filter: blur(var(--ui-material-blur)) saturate(var(--ui-material-saturate));
}

@supports not ((backdrop-filter: blur(1px)) or (-webkit-backdrop-filter: blur(1px))) {
    .ui-glass-layer {
        background: var(--ui-material-solid-bg);
    }
}
```

### Accessibility Rules

Text and icons must be validated against the solid fallback color, not only the ideal translucent state. Reduced transparency must force solid materials. Reduced motion must not disable glass, but it must disable animated blur changes unless explicitly allowed.

## Native Timeline Engine

`ui-timeline` replaces the GSAP bridge idea.

### Design Target

The engine should support choreographed SaaS UI motion:

- page and route transitions
- panel enter and exit
- list item stagger
- command palette reveal
- overlay presence
- dashboard metric updates
- shared layout movement
- scroll-tied product tours or docs
- text/token reveal for docs and marketing-like surfaces inside apps
- press, hover, focus, drag, and swipe feedback

It should not require JavaScript timeline libraries. The Rust timeline owns the state and emits renderer-specific snapshots.

### Time Model

The timeline engine should support four clocks:

```rust
pub enum TimelineClock {
    Playback { elapsed_ms: f32 },
    Manual { progress: f32 },
    Scroll { progress: f32 },
    Frame { frame: u32, fps: u32 },
}
```

`Playback` is for runtime UI. `Manual` is for tests and controlled interactions. `Scroll` is for scroll-driven progress. `Frame` is for deterministic composition and capture.

### Timeline Graph

A `Timeline` is a graph of labeled tracks:

```rust
pub struct Timeline {
    pub id: TimelineId,
    pub duration_ms: f32,
    pub labels: Vec<TimelineLabel>,
    pub tracks: Vec<TimelineTrack>,
    pub repeat: RepeatMode,
    pub fill: FillMode,
}
```

Tracks can target semantic node IDs:

```rust
pub struct TimelineTrack {
    pub target: MotionTarget,
    pub segments: Vec<MotionSegment>,
}
```

Targets should avoid direct DOM references in core logic:

```rust
pub enum MotionTarget {
    SelfNode,
    Node(KineticId),
    Group(KineticGroupId),
    DataKey(String),
}
```

### MotionCue

`MotionCue` describes what changes:

```rust
pub struct MotionCue {
    pub opacity: Option<ValueCue<f32>>,
    pub translate_x: Option<ValueCue<Length>>,
    pub translate_y: Option<ValueCue<Length>>,
    pub scale: Option<ValueCue<f32>>,
    pub rotate: Option<ValueCue<Angle>>,
    pub blur: Option<ValueCue<f32>>,
    pub radius: Option<ValueCue<f32>>,
    pub shadow: Option<ValueCue<ShadowRecipe>>,
    pub glass: Option<ValueCue<GlassIntensity>>,
}
```

Initial helper constructors should cover common UI movements:

- `MotionCue::fade_in()`
- `MotionCue::fade_out()`
- `MotionCue::rise_in()`
- `MotionCue::sink_out()`
- `MotionCue::scale_in()`
- `MotionCue::press()`
- `MotionCue::focus_halo()`
- `MotionCue::glass_reveal()`
- `MotionCue::slide_from(edge)`
- `MotionCue::modal_enter()`
- `MotionCue::modal_exit()`

These names describe behavior and should not copy third-party naming.

### StaggerFlow

`StaggerFlow` controls delayed children:

```rust
pub enum StaggerFlow {
    ByIndex { step_ms: f32 },
    FromStart { step_ms: f32 },
    FromEnd { step_ms: f32 },
    FromCenter { step_ms: f32 },
    ByGrid { columns: u16, step_ms: f32 },
    ByDataPriority { step_ms: f32 },
}
```

The Dioxus adapter should apply it to child order through stable `KineticId` values.

### PresenceGate

`PresenceGate` controls mount, exit, and removal:

```rust
rsx! {
    PresenceGate {
        present: show_panel,
        enter: MotionCue::modal_enter(),
        exit: MotionCue::modal_exit(),
        ModalLayer { title: "Archive workspace" }
    }
}
```

Core lifecycle states:

- `Present`
- `Entering`
- `Stable`
- `Exiting`
- `Removed`

SSR should render `Present` or `Removed` only. Hydration should not create a mismatched intermediate state.

### SharedMove

`SharedMove` builds on `ui-layout` FLIP math:

```rust
rsx! {
    SharedMove {
        id: "active-customer-card",
        strategy: SharedMoveStrategy::Transform,
        MetricReadout { label: "ARR", value: "$2.4M" }
    }
}
```

It should support:

- first-last geometry capture
- invert transform
- play transform to identity
- opacity crossfade for incompatible shapes
- reduced-motion instant placement
- SSR no-op initial render

### ScrollDirector

`ScrollDirector` maps scroll progress into timeline progress:

```rust
rsx! {
    ScrollDirector {
        range: ScrollRange::between("overview", "details"),
        timeline: dashboard_reveal,
        children
    }
}
```

Core logic should only map progress. The DOM adapter can observe scroll. Native adapters can map a scroll container offset into the same progress value.

### TextFlow

`TextFlow` supports text reveal without external text animation libraries:

- split by whole text
- split by line
- split by word
- split by grapheme-like unit where available
- respect reduced motion
- preserve accessible text as a single readable label

The rendered markup should avoid making screen readers announce every token separately.

## Native Frame Composition

`ui-composition` replaces the Remotion bridge idea.

### Design Target

The library should support deterministic, frame-addressable scenes:

- component showcase videos
- animated documentation panels
- SaaS onboarding demo scenes
- release note motion snippets
- product walkthroughs
- chart and metric animation previews
- future image-sequence or video export

This is not arbitrary DOM recording in the MVP. The first target is export-safe Dioxus primitives owned by this library.

### Composition

```rust
pub struct Composition {
    pub id: CompositionId,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub frame_count: u32,
    pub background: CompositionBackground,
    pub metadata: CompositionMetadata,
}
```

Rules:

- `width`, `height`, `fps`, and `frame_count` must be positive.
- `fps` should support 24, 30, and 60 by default.
- frame indexing is zero-based.
- composition IDs must be stable strings.
- the same frame input must produce the same resolved scene output.

### FrameClock

```rust
pub struct FrameClock {
    pub frame: u32,
    pub fps: u32,
}

impl FrameClock {
    pub fn seconds(&self) -> f32;
    pub fn progress(&self, start: u32, duration: u32) -> f32;
}
```

All frame animation should sample from `FrameClock`, not wall-clock time.

### FrameStage

`FrameStage` is the Dioxus root for deterministic composition:

```rust
rsx! {
    FrameStage {
        composition: Composition::new("launch-demo", 1920, 1080, 30, 180),
        frame: 42,
        FrameClip {
            start: 0,
            duration: 72,
            FrameLayer {
                depth: 10,
                FrameText {
                    text: "Dioxus Kinetics",
                    cue: FrameCue::fade_slide(0, 24),
                }
            }
        }
    }
}
```

### FrameClip

`FrameClip` activates children for a frame range:

```rust
pub struct FrameClipProps {
    pub start: u32,
    pub duration: u32,
    pub fill: ClipFill,
}
```

`ClipFill` controls behavior before and after the active range:

- `None`
- `HoldStart`
- `HoldEnd`
- `HoldBoth`

### FrameLayer

`FrameLayer` controls stacking and transform scope:

```rust
pub struct FrameLayerProps {
    pub depth: i32,
    pub opacity: f32,
    pub transform: FrameTransform,
    pub material: Option<MaterialRecipe>,
}
```

Layer depth must sort deterministically.

### FrameCue

`FrameCue` expresses animation in frames:

```rust
pub struct FrameCue {
    pub start: u32,
    pub duration: u32,
    pub ease: FrameEase,
    pub values: FrameCueValues,
}
```

Helpers:

- `FrameCue::fade_in(start, duration)`
- `FrameCue::fade_out(start, duration)`
- `FrameCue::fade_slide(start, duration)`
- `FrameCue::scale_pop(start, duration)`
- `FrameCue::glass_bloom(start, duration)`
- `FrameCue::metric_count(start, duration)`
- `FrameCue::hold(start, duration)`

### Export-Safe Primitives

MVP composition should focus on primitives we can reason about:

- `FrameText`
- `FrameShape`
- `FrameImage`
- `FrameIcon`
- `FrameMetric`
- `FrameChartLine`
- `FrameGlassLayer`
- `FrameStack`
- `FrameGrid`
- `FrameMediaPlaceholder`

Arbitrary third-party Dioxus nodes can be previewed, but they are not guaranteed export-safe until a capture backend supports them.

### Audio Metadata

Audio does not need to render in the MVP, but the composition model should reserve metadata:

```rust
pub struct AudioCue {
    pub asset_id: AssetId,
    pub start_frame: u32,
    pub trim_start_frame: u32,
    pub duration_frames: u32,
}
```

This keeps the model compatible with future export without committing to audio encoding now.

## Native Capture And Frame Seeking

`ui-capture` replaces the HyperFrames bridge idea.

### Design Target

The capture system should make scenes inspectable and eventually exportable by frame:

- docs can render a specific frame
- tests can seek to a deterministic state
- gallery can preview timeline frames
- a future CLI can render frames or image sequences
- viewport profiles can verify desktop, tablet, mobile, and native layouts

No external capture orchestration service should be required by the public design.

### CaptureStage

```rust
rsx! {
    CaptureStage {
        id: "component-showcase",
        viewport: ViewportProfile::desktop(),
        frame: 72,
        SceneTrack {
            start: 0,
            end: 120,
            GlassLayer {
                depth: GlassDepth::Floating,
                MetricReadout {
                    label: "ARR",
                    value: "$2.4M",
                    trend: "+18%",
                }
            }
        }
    }
}
```

### CaptureRoute

For web and desktop preview, the library should support stable frame URLs:

```text
/__kinetics/capture/:stage_id?frame=72&viewport=desktop
```

This route contract is optional for downstream apps but required for the component gallery and future automated previews.

### CaptureMark

Named marks make docs and tests readable:

```rust
CaptureMark::new("modal-open", 24)
CaptureMark::new("metric-settled", 90)
CaptureMark::new("mobile-nav-expanded", 118)
```

The gallery can use marks instead of magic frame numbers.

### ViewportProfile

```rust
pub struct ViewportProfile {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub scale: f32,
    pub density: Density,
    pub color_scheme: ColorScheme,
    pub transparency: TransparencyPreference,
    pub motion: MotionPreference,
}
```

Built-in profiles:

- desktop: 1440 by 960
- tablet: 1024 by 768
- mobile: 390 by 844
- wide: 1920 by 1080
- square: 1080 by 1080

### ExportManifest

`ExportManifest` is the durable handoff for future capture runners:

```rust
pub struct ExportManifest {
    pub schema_version: u32,
    pub library_version: String,
    pub compositions: Vec<Composition>,
    pub stages: Vec<CaptureStageDescriptor>,
    pub assets: Vec<SceneAsset>,
    pub viewports: Vec<ViewportProfile>,
    pub marks: Vec<CaptureMark>,
}
```

The manifest should be serializable and stable enough for snapshot tests.

### CaptureRunner

MVP `CaptureRunner` does not need to produce pixels. It must:

- load an `ExportManifest`
- validate composition and stage IDs
- resolve frame ranges
- resolve viewport profiles
- seek a frame
- produce a deterministic scene snapshot

Later phases can add:

- DOM/WebView raster capture
- native renderer raster capture
- PNG sequence export
- video encoding

Those are follow-up capabilities, not bridge dependencies.

## Dioxus Adapter Requirements

`ui-dioxus` should add components that connect semantic markup to native engine state:

- `KineticBox`
- `KineticText`
- `KineticGroup`
- `TimelineScope`
- `PresenceGate`
- `SharedMove`
- `ScrollDirector`
- `FrameStage`
- `FrameClip`
- `FrameLayer`
- `CaptureStage`
- `FrameScrubber`
- `ExportPreview`

These components should:

- render SSR-safe initial markup
- expose stable `.ui-*` classes
- expose stable `data-ui-*` attributes
- accept controlled props
- avoid global hidden mutable state
- use context only for local timeline/composition scope
- keep accessibility labels stable during motion

## DOM And WebView Adapter

The DOM/WebView adapter should serialize resolved style snapshots:

```rust
pub struct ResolvedVisualState {
    pub opacity: f32,
    pub transform: Transform2D,
    pub filter: FilterState,
    pub material: MaterialStyle,
    pub pointer_events: PointerEvents,
    pub visibility: Visibility,
}
```

Output can be:

- inline style attributes for dynamic values
- CSS variables for material and motion values
- classes for stable variants
- data attributes for state and testing

The core timeline must not call browser APIs directly. Browser observation, animation frames, and scroll offsets belong in the adapter.

## Native Adapter

Native support starts with semantic parity:

- same component tree concepts
- same material recipes
- same motion snapshots
- same reduced preference rules
- same capture manifest

Native targets may use solid fallback materials until blur, vibrancy, and shadows are available in the renderer. Native limitations must be explicit in capability objects, not hidden in component code.

## Preference And Accessibility Policy

Every advanced system must honor these preferences:

- reduced motion
- reduced transparency
- high contrast
- forced colors where available
- color scheme
- density
- large text where downstream apps provide it

Policy:

- reduced motion converts springs and timeline segments to instant or short fades
- reduced transparency converts glass to solid materials
- high contrast uses stronger borders, no blur dependency, and validated text contrast
- frame composition must be deterministic under every preference combination
- screen-reader text must not be split into inaccessible animation fragments
- focus rings must remain visible over glass and solid fallback surfaces

## Data Flow

Runtime UI motion:

1. Dioxus component renders semantic structure with stable IDs.
2. `TimelineScope` registers local targets and cues.
3. A clock source advances timeline progress.
4. `ui-timeline` samples the timeline into `ResolvedVisualState`.
5. `ui-dom` or `ui-native` serializes the state.
6. The component re-renders with classes, data attributes, CSS variables, or native style snapshots.

Frame composition:

1. `Composition` defines dimensions and frame count.
2. `FrameStage` receives a frame number.
3. `FrameClock` samples all frame cues.
4. `FrameClip` and `FrameLayer` resolve active children and depth.
5. Export-safe primitives render a deterministic Dioxus tree.
6. `CaptureStage` can expose the same frame through preview routes and manifests.

Capture:

1. Gallery or downstream app registers capture stages.
2. `ExportManifest` lists stages, compositions, assets, marks, and viewports.
3. `CaptureRunner` validates and seeks frames.
4. MVP output is a deterministic scene snapshot.
5. Later output can be image sequences or video through native renderer/capture work.

## Error Handling

The new systems should fail early and predictably.

Validation errors:

- duplicate timeline IDs
- duplicate target IDs inside a timeline scope
- negative or non-finite durations
- frame ranges outside composition bounds
- zero-size compositions
- invalid viewport dimensions
- missing capture stage IDs
- unsupported material policy for a target adapter

Runtime behavior:

- invalid cues are skipped with diagnostics in debug builds
- release builds prefer safe fallback states
- unsupported glass uses solid fallback
- unsupported capture output returns a typed error
- SSR ignores runtime-only clocks and renders the configured initial state

Typed errors should live in the relevant crate, with facade re-exports where useful.

## Documentation Gallery Requirements

The component gallery should become the proof surface for this architecture.

New categories:

- Foundations: tokens, material, density, preferences
- Inputs: text, choice, switch, upload
- Navigation: rail, switcher, action bar, command finder
- Feedback: notice stack, status beacon, modal layer, context hint
- Data Workflows: record grid, filter builder, query composer, metric readout
- Layout: content plane, split workspace, dock area, inspector pane
- Motion: timeline, presence, shared move, scroll director, text flow
- Composition: frame stage, clips, layers, frame values
- Capture: capture stage, viewport profiles, marks, manifest preview

Each entry should show:

- what it is
- when to use it
- code snippet
- rendered example
- important props
- accessibility notes
- platform support
- readiness state

Coming-soon entries remain valid, but they should be dynamic records in the registry, not static placeholder markup scattered through the app.

## Testing Strategy

### Unit Tests

`ui-motion`:

- spring step handles non-finite input
- spring settles toward target
- reduced transition removes duration
- interpolation clamps progress
- color interpolation is deterministic

`ui-timeline`:

- labels resolve to correct offsets
- segments sample correct values
- repeat and yoyo modes work
- stagger order is deterministic
- reduced motion collapses cues correctly
- scroll progress maps to timeline progress
- frame clock samples match playback clock for equivalent time

`ui-composition`:

- composition validation catches zero dimensions
- frame clock progress clamps correctly
- clips activate only in range
- layer sorting is stable
- frame cues sample deterministically
- frame springs are deterministic

`ui-capture`:

- viewport profiles validate
- capture marks resolve by name
- manifests serialize deterministically
- missing stage IDs return typed errors
- frame seek returns stable snapshots

### SSR Tests

`ui-dioxus`:

- semantic component names render stable classes
- old bridge names do not appear in default rendered markup
- timeline components render SSR-safe initial states
- frame components render deterministic frame markup
- capture preview renders selected frame metadata
- split text remains accessible as a single label

### Facade Tests

`kinetics`:

- prelude exports semantic names
- prelude exports timeline/composition/capture types under native names
- old bridge features are removed from the public facade
- `library_css()` includes material, timeline, composition, and capture selectors

### Gallery Tests

`component-gallery`:

- every ready component has snippet, summary, accessibility note, and renderer
- every coming-soon component has a dynamic coming-soon state
- new categories render in stable order
- motion/composition/capture examples appear
- no GSAP, Remotion, or HyperFrames names appear in user-facing gallery copy

## Verification Commands

Implementation plans should end with:

```powershell
cargo fmt --all -- --check
cargo check -p component-gallery
cargo test -p ui-motion
cargo test -p ui-layout
cargo test -p ui-glass
cargo test -p ui-timeline
cargo test -p ui-composition
cargo test -p ui-capture
cargo test -p ui-dioxus
cargo test -p kinetics
cargo test -p component-gallery
cargo test --workspace
```

If crates are introduced incrementally, the plan should run the subset that exists at each checkpoint plus `cargo test --workspace` before completion.

## Implementation Phases

### Phase 1: Rename Bridge Boundaries

- rename `ui-gsap` to `ui-timeline`
- rename `ui-hyperframes` to `ui-capture`
- add `ui-composition`
- update workspace members
- update feature flags
- update README and platform docs to remove bridge language
- keep tests proving old third-party names do not appear in default public docs

### Phase 2: Semantic Public Renames

- introduce functional component names in `ui-dioxus`
- export functional names from `kinetics::prelude`
- update gallery snippets and examples
- decide whether old names are removed or gated behind `legacy-names`
- update component naming docs

### Phase 3: Glass Material Upgrade

- expand material axes
- add DOM style serialization for material recipes
- add reduced transparency and high contrast test coverage
- update gallery material examples

### Phase 4: Native Timeline MVP

- implement timeline model
- implement cue sampling
- implement stagger flow
- implement presence gate
- connect Dioxus components
- update gallery with motion examples

### Phase 5: Shared Movement And Scroll

- connect `ui-layout` FLIP math to `SharedMove`
- add scroll progress model
- add text flow model
- add reduced-motion behavior

### Phase 6: Native Composition MVP

- add composition, frame clock, clips, layers, frame cues
- add export-safe frame primitives
- add Dioxus frame preview
- add deterministic SSR tests

### Phase 7: Native Capture MVP

- add capture stage, marks, route descriptors, viewport profiles
- add export manifest
- add capture runner validation
- add gallery capture preview

### Phase 8: Documentation And Acceptance

- rebuild docs and README around native systems
- expand gallery category pages
- verify web, desktop, mobile WebView, and native support tables
- run workspace verification

## Non-Goals

This design does not implement:

- GSAP integration
- Remotion integration
- HyperFrames integration
- arbitrary third-party DOM video recording in the MVP
- full MP4 export in the MVP
- audio rendering in the MVP
- global animation authoring GUI
- cloud rendering service
- dependency on a JavaScript animation runtime

Future image or video export must build from native capture/composition contracts, not from bridge wrappers.

## Acceptance Criteria

The design is satisfied when:

- public docs describe native timeline, composition, and capture systems
- bridge-oriented crate names are removed from the default workspace story
- the public prelude exposes functional names
- timeline examples run without external animation libraries
- composition examples can seek exact frames
- capture manifests serialize deterministic stage, frame, asset, and viewport data
- glass materials have reduced-transparency and high-contrast fallbacks
- gallery shows category-wise docs with snippets and rendered examples
- tests prove no default public API or gallery copy depends on GSAP, Remotion, or HyperFrames names

## Research Notes

The capability targets are informed by current web and platform primitives:

- CSS `backdrop-filter` is the relevant web primitive for glass blur, with solid fallback needed where unsupported.
- The Web Animations and View Transition APIs show what browser-native animation can offer, but the core engine here remains Rust-owned.
- Apple Human Interface Guidelines reinforce the need for material depth, motion restraint, accessibility preferences, and legible fallbacks.
- Timeline, frame-composition, and capture tools demonstrate useful product capabilities, but this library recreates those classes of behavior under native Dioxus Kinetics names.

The implementation should cite official docs where it relies on a platform behavior, but the public API should remain original to this library.
