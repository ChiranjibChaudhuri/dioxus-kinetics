# GSAP-tier Primitives — Design (SP-3)

## Goal

Land the three flagship GSAP capabilities the original user complaint
called out — **ScrollTrigger**, **SplitText**, and **MotionPath** — on
top of the SP-1 Scene Player. Each ships as a Dioxus component plus the
underlying motion-math support in `ui-timeline` / `ui-runtime`, and each
gets a cinematic showcase entry in the gallery's Scene category.

SP-1 shipped the paused-seekable Scene Player with `Autoplay` and
scrubbable transport as the only ways to drive the clock. SP-3 expands
that with a **scroll-driven** driver (the engine half of ScrollTrigger),
adds per-glyph text choreography via **SplitText**, and lets motion
follow a parametric **path** instead of just translate / scale / rotate
scalars.

## Scope

In scope:

1. New `SceneDriver` enum in `ui-runtime` with three variants:
   - `SceneDriver::Autoplay` — current SP-1 default behavior; clock
     advances via `spawn_frame_loop` on mount until `duration_ms` is
     reached.
   - `SceneDriver::Scroll { observer: ScrollObserverConfig }` — clock
     driven by scroll progress through a configured trigger region.
     Web-only; native targets construct the driver but it holds the
     clock at progress 0.
   - `SceneDriver::Manual` — autoplay disabled; clock only moves via
     explicit `seek_*` calls (the existing transport scrubber path).
2. A `driver: Option<SceneDriver>` prop on the `Scene` Dioxus component
   (defaults to `SceneDriver::Autoplay`) that selects which driver
   advances the clock.
3. Web-only `ScrollDriver` runtime: wires `IntersectionObserver` + a
   `scroll` event listener on the window, computes a `[0..=1]` progress
   from a `start` / `end` configuration, and seeks the parent clock
   via `clock.seek_progress(progress)`.
4. New `MotionCue::Path` variant in `ui-timeline` plus the supporting
   `PathPoint` enum (`Line { end }` and `Bezier { control_1, control_2,
   end }`) and a De Casteljau sampler. `to_progress` / `from_progress`
   let the cue traverse only a sub-segment. Optional
   `rotate_along_path: bool` emits a `rotate_deg` value tangent to the
   curve.
5. New `SplitText` Dioxus component in `ui-dioxus`. Props: `text:
   String`, `split_by: SplitMode` (`Character | Word`), `cue: String`.
   Emits per-glyph or per-word `KineticBox` children with sequential
   `data-stagger-index` attributes so the existing
   `TimelineScope::stagger` machinery drives them. Accessibility:
   parent `aria-label` carries the full text; each per-glyph span sets
   `aria-hidden="true"`.
6. New `MotionPath` Dioxus convenience wrapper in `ui-dioxus`. Takes
   `path: Vec<PathPoint>`, `duration_ms: f32`, `transition: Option<Transition>`,
   `rotate_along_path: Option<bool>`, and `children: Element`. Wraps
   children in a `KineticBox` whose surrounding `Sequence` carries one
   `MotionCue::Path` segment driving translate + optional rotation.
7. Three new gallery showcase entries in the `Scene` category:
   - `Scene · Scroll-pinned Story`
   - `Scene · Split Headline`
   - `Scene · Curved Trajectory`
8. Public API re-exports from `kinetics::prelude`: `SceneDriver`,
   `ScrollObserverConfig`, `SplitText`, `SplitMode`, `MotionPath`,
   `PathPoint`. The existing `MotionCue` re-export already covers the
   new `MotionCue::Path` variant.

Out of scope (deferred):

- SP-4 headless render → MP4 pipeline.
- SP-5 `kinetics` CLI.
- SP-6 catalog crate + agent skill.
- `SplitMode::Line` — line splitting requires post-layout measurement
  which SSR cannot do; defer until SP-3 has a measurement-after-layout
  story.
- SVG path-string parsing (`<path d="M 0 0 C ..." />`) — we accept
  structured `Vec<PathPoint>` only.
- GSAP's `Draggable`, `Observer`, `MorphSVG`, `Flip` — separate
  primitives, each substantial enough for its own future sub-project.
- WebGL shader transitions for `ScrollTrigger` (e.g. cross-fade with
  shader noise). Pure DOM transitions only.
- Audio-reactive variants of any of the three primitives.

## Architecture

### Module layout

```
crates/ui-timeline/src/
  lib.rs                       # + MotionCue::Path variant + PathPoint + sampler

crates/ui-runtime/src/
  scene_driver.rs              # NEW — SceneDriver enum + ScrollObserverConfig
  drivers/
    mod.rs                     # NEW — pub use of three drivers
    autoplay.rs                # NEW — extracted from current SceneClock::play path
    manual.rs                  # NEW — no-op driver (clock stays where seek_* leaves it)
    scroll.rs                  # NEW — web-only IntersectionObserver + window scroll
    scroll_stub.rs             # NEW — non-web no-op
  scene_clock.rs               # play() now branches on installed driver

crates/ui-dioxus/src/
  scene_player.rs              # Scene gains `driver: Option<SceneDriver>` prop
  split_text.rs                # NEW — SplitText + SplitMode
  motion_path.rs               # NEW — MotionPath convenience component
  lib.rs                       # + re-exports

crates/kinetics/src/lib.rs     # + prelude exports

examples/component-gallery/src/previews/scenes/
  scroll_story.rs              # NEW — Scene · Scroll-pinned Story
  split_headline.rs            # NEW — Scene · Split Headline
  curved_trajectory.rs         # NEW — Scene · Curved Trajectory

examples/component-gallery/e2e/tests/
  gsap-tier-primitives.spec.ts # NEW — three Playwright tests, one per primitive
```

### `SceneDriver`

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum SceneDriver {
    /// SP-1 default: the clock advances via spawn_frame_loop on mount
    /// until duration_ms is reached.
    Autoplay,
    /// Clock progress = scroll progress through `observer`'s region.
    /// Web-only; native targets construct but hold at 0.
    Scroll(ScrollObserverConfig),
    /// Clock only moves via explicit seek_* calls. The transport
    /// scrubber still works; autoplay does not.
    Manual,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScrollObserverConfig {
    /// CSS selector for the trigger region. The scene's progress is
    /// 0 when the trigger's top edge enters the viewport at the `start`
    /// offset, and 1 when it exits at the `end` offset.
    pub trigger_selector: String,
    /// Vertical viewport offset (px from top) at which progress = 0.
    /// Default: viewport_height.
    pub start_offset_px: Option<f32>,
    /// Vertical viewport offset (px from top) at which progress = 1.
    /// Default: 0.
    pub end_offset_px: Option<f32>,
}
```

### Scroll driver lifecycle (web)

```
Scene mounts
  -> use_effect installs IntersectionObserver on `trigger_selector` element
  -> Also installs window scroll listener (passive, throttled via rAF)
  -> On scroll OR intersection change:
       compute progress = ((vp_top - trigger_top) / (vp_height + trigger_height)).clamp(0, 1)
       clock.seek_progress(progress)
  -> Cleanup on Scene unmount: disconnect observer, remove listener
```

The progress formula above is the simple default. With
`start_offset_px` / `end_offset_px` set, the formula scales between
those offsets instead of the full viewport.

### `MotionCue::Path` and Bézier sampling

```rust
pub enum MotionCue {
    // existing variants unchanged
    Opacity { from: f32, to: f32, transition: Transition },
    Translate { axis: Axis, from: f32, to: f32, transition: Transition },
    Scale { from: f32, to: f32, transition: Transition },
    Rotate { from_deg: f32, to_deg: f32, transition: Transition },
    // NEW
    Path {
        points: Vec<PathPoint>,
        from_progress: f32,      // default 0.0
        to_progress: f32,        // default 1.0
        rotate_along_path: bool, // default false
        transition: Transition,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum PathPoint {
    /// Straight line to `end`. The previous point is the line's start.
    /// The first point in a path MUST be `Line` (its `end` is the
    /// starting position).
    Line { end: (f32, f32) },
    /// Cubic Bézier from the previous point through two control points
    /// to `end`.
    Bezier {
        control_1: (f32, f32),
        control_2: (f32, f32),
        end: (f32, f32),
    },
}
```

**Sampling**: `MotionCue::Path::sample(progress)` returns a
`MotionCueSample { translate_x, translate_y, rotate_deg }`. The
implementation:

1. Eases `progress` via `apply_transition_progress(progress, transition)`.
2. Maps eased progress through `[from_progress, to_progress]` to a
   global path parameter `t ∈ [0, 1]`.
3. Walks segments, computing each segment's arc length (cached on
   first sample via a `OnceCell`-style mechanism) so the path is
   sampled uniformly by arc length, not by parameter — otherwise
   Bézier curves "speed up" through high-curvature regions in a way
   that looks wrong.
4. Once the target segment + segment-local `t` are found:
   - Position: De Casteljau evaluation for `Bezier`, linear interp
     for `Line`.
   - Tangent (if `rotate_along_path`): the derivative at `t`,
     converted to degrees via `atan2(dy, dx)`.

**Reduced motion** behavior: collapses to the endpoint position at
`to_progress`, no traversal, no rotation update past initial.

### `SplitText`

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SplitMode {
    Character,
    Word,
}

#[component]
pub fn SplitText(
    text: String,
    split_by: Option<SplitMode>,        // default Character
    cue: Option<String>,                // default "rise-in"
    stagger_step_ms: Option<f32>,       // default 35.0 for Character, 80.0 for Word
) -> Element;
```

Renders:

```html
<span class="ui-split-text" aria-label="Hello world">
  <span class="ui-split-text-glyph" data-stagger-index="0" aria-hidden="true">H</span>
  <span class="ui-split-text-glyph" data-stagger-index="1" aria-hidden="true">e</span>
  ...
</span>
```

When wrapped in a `TimelineScope` (or `Scene` containing one), the
existing stagger machinery already targets `[data-stagger-index]`
attributes and walks them in sequence. Each glyph also carries a
`KineticBox` so the existing motion-cue selection on `cue:` (e.g.
`"rise-in"`, `"fade-in"`, `"slide-up"`) flows through.

For `SplitMode::Word`, whitespace is preserved as non-staggered text
nodes between word spans so layout doesn't collapse.

**Accessibility contract**: the outer `aria-label` carries the
unsplit text. The per-glyph spans are `aria-hidden="true"`. Screen
readers read the parent label once; they do not enumerate glyphs.

### `MotionPath` component

```rust
#[component]
pub fn MotionPath(
    id: String,                          // KineticBox id
    path: Vec<PathPoint>,
    duration_ms: f32,
    rotate_along_path: Option<bool>,     // default false
    ease: Option<Ease>,                  // default Ease::Standard
    children: Element,
) -> Element;
```

Generates:

1. A `Timeline` with one `TimelineTrack` targeting the child node,
   containing one `MotionSegment` with a `MotionCue::Path { ... }`.
2. Wraps `children` in a `KineticBox { id }` and (for SSR) sets
   `data-motion-path` attribute carrying the path as JSON so render
   tooling can introspect.

The component does not own a clock — it expects a parent
`TimelineScope` or `Scene` + `Sequence` setup to drive it. If no
parent timeline is present, the path is rendered at its `from_progress`
position (similar to the `Clip` orphan handling).

### Adapter reuse

No new `FrameAdapter` impls. `ScrollDriver` is a **clock source**, not
a `FrameAdapter`. SplitText is purely composition; `SequenceAdapter`
already handles its targets. `MotionPath` adds a new `MotionCue`
variant; `SequenceAdapter`'s existing sample path picks it up via the
extended `MotionCueSample`.

### Public API additions

`kinetics::prelude`:
- `SceneDriver`, `ScrollObserverConfig`
- `SplitText`, `SplitMode`
- `MotionPath`, `PathPoint`

`ui-timeline` re-exports:
- `MotionCue` already re-exports `Path` variant.
- New: `PathPoint`.

`ui-runtime` re-exports:
- `SceneDriver`, `ScrollObserverConfig`.

`ui-dioxus` re-exports:
- `SplitText`, `SplitMode`, `MotionPath`.

### Gallery integration

Three new `ComponentDoc` entries under `ComponentCategory::Scene`:

| Entry | Showcase |
|---|---|
| `Scene · Scroll-pinned Story` | A 4-beat narrative pinned to a 200vh tall trigger region. Headline / body / decorator card / CTA appear in sequence as the user scrolls. Driver: `SceneDriver::Scroll`. |
| `Scene · Split Headline` | A 2-second hero title that animates per character with a 35ms stagger. Uses `SplitText` inside `Scene` + `TimelineScope`. |
| `Scene · Curved Trajectory` | A 4-second loop where a single `MotionPath`-wrapped KineticBox traces an S-curve from upper-left to lower-right and back. Optional rotation tangent. |

Each scene file lives at
`examples/component-gallery/src/previews/scenes/<slug>.rs` and is
wired through `previews/scene.rs` (same pattern as SP-1's product
intro).

### Reduced motion

- ScrollTrigger: scroll events are ignored; the scene settles
  immediately at `duration_ms` (matches SP-1 reduced-motion policy).
  Scrubber stays disabled.
- SplitText: glyphs render at final state; the `TimelineScope`
  parent's reduced-motion logic collapses the stagger to instant.
- MotionPath: collapses to `to_progress` endpoint translation; no
  traversal, no rotation animation.

### Cross-platform support matrix

| Feature | Web (wasm32) | Native desktop / mobile |
|---|---|---|
| `SceneDriver::Autoplay` | works (SP-1) | works (SP-1) |
| `SceneDriver::Manual` | works | works |
| `SceneDriver::Scroll` | works (IntersectionObserver + scroll) | stub holds clock at 0 |
| `SplitText` | works | works (no scroll dependency) |
| `MotionPath` | works | works (no DOM dependency for the math) |

## Testing

### Unit (`ui-timeline`)

- `motion_cue::path::tests::line_at_endpoints` — `from_progress=0, to_progress=1, points=[Line(0,0), Line(100,0)]`; sample at 0 → (0,0); at 0.5 → (50, 0); at 1.0 → (100, 0).
- `motion_cue::path::tests::cubic_bezier_at_midpoint` — Bézier with control points; sample at 0.5 → expected midpoint via De Casteljau (asserted to 1e-3 tolerance).
- `motion_cue::path::tests::arc_length_uniformity` — a sharply curved Bézier; sample at 0.5 should be approximately at the half-arc-length point (within 5% of total length), not the half-parameter point.
- `motion_cue::path::tests::rotate_along_path_tangent` — straight line at 45° → rotate_deg ≈ 45.0.
- `motion_cue::path::tests::reduced_motion_collapses_to_endpoint` — `reduced` flag samples always return `to_progress` position.
- `motion_cue::path::tests::sub_segment_traversal` — `from_progress=0.25, to_progress=0.75` clamps the visible portion.

### Unit (`ui-runtime`)

- `scene_driver::tests::autoplay_is_default` — constructing `SceneDriver::default()` returns `Autoplay`.
- `scene_driver::tests::manual_does_not_advance` — a Manual-driven clock does not advance under a tick simulation.
- `scene_driver::tests::scroll_stub_holds_at_zero` — on native targets, constructing a Scroll-driven clock leaves `elapsed_ms` at 0 even after the would-be observer would have fired.

### Integration (`ui-dioxus` SSR)

- `split_text::tests::renders_per_glyph_spans_with_data_stagger_index` — `text="Hi"` produces two spans with `data-stagger-index="0"` and `data-stagger-index="1"` plus the parent `aria-label="Hi"`.
- `split_text::tests::word_mode_preserves_whitespace` — `text="Hello world", split_by=Word` produces two word spans plus a literal space text node between them.
- `split_text::tests::aria_label_carries_full_text` — full assertion.
- `motion_path::tests::wraps_children_in_kinetic_box` — output contains `data-motion-path` attribute and a child KineticBox.
- `scene_player::tests::scene_with_manual_driver_skips_autoplay` — `Scene { driver: Some(SceneDriver::Manual), autoplay: Some(true), ... }` does NOT call play() on mount; the clock stays Paused.

### E2E Playwright

`examples/component-gallery/e2e/tests/gsap-tier-primitives.spec.ts`:

1. **Scroll-pinned Story**: navigate to the showcase; scroll the page;
   assert the scene's `data-elapsed-ms` increases monotonically and
   reaches `data-duration-ms` at the end-of-trigger position.
   Reduced-motion: assert the scrubber is disabled and `data-state =
   "settled"` regardless of scroll position.

2. **Split Headline**: navigate; assert N `<span class="ui-split-text-glyph">`
   children exist where N = len(text); assert parent
   `aria-label = "<full text>"`.

3. **Curved Trajectory**: navigate; sample the inline transform on
   the KineticBox at t=0, t=duration/2, t=duration; assert the
   translate values move from start point through midpoint to end
   point. Reduced-motion: assert the transform is the endpoint.

Both Chromium and WebKit projects.

### Visual snapshot

One new visual baseline per scene (Chromium only, matches SP-1
asymmetric convention).

## Files (final)

New:

- `crates/ui-timeline/src/path.rs` — `PathPoint`, sampler, tests-as-doc.
- `crates/ui-runtime/src/scene_driver.rs`
- `crates/ui-runtime/src/drivers/mod.rs`
- `crates/ui-runtime/src/drivers/autoplay.rs`
- `crates/ui-runtime/src/drivers/manual.rs`
- `crates/ui-runtime/src/drivers/scroll.rs`
- `crates/ui-runtime/src/drivers/scroll_stub.rs`
- `crates/ui-runtime/tests/scene_driver.rs`
- `crates/ui-dioxus/src/split_text.rs`
- `crates/ui-dioxus/src/motion_path.rs`
- `crates/ui-dioxus/tests/split_text_ssr.rs`
- `crates/ui-dioxus/tests/motion_path_ssr.rs`
- `examples/component-gallery/src/previews/scenes/scroll_story.rs`
- `examples/component-gallery/src/previews/scenes/split_headline.rs`
- `examples/component-gallery/src/previews/scenes/curved_trajectory.rs`
- `examples/component-gallery/e2e/tests/gsap-tier-primitives.spec.ts`

Edited:

- `crates/ui-timeline/src/lib.rs` — extend `MotionCue` enum; extend
  `MotionCueSample` if rotate_deg path-sampling requires;
  reduced-motion arms; re-export `PathPoint`.
- `crates/ui-runtime/src/lib.rs` — `pub mod scene_driver;` +
  re-exports.
- `crates/ui-runtime/src/scene_clock.rs` — `play()` now consults the
  installed `SceneDriver` (Autoplay = current path; Manual = no-op;
  Scroll = install observer rather than rAF loop).
- `crates/ui-dioxus/src/lib.rs` — `pub mod split_text; pub mod motion_path;`
  + re-exports.
- `crates/ui-dioxus/src/scene_player.rs` — accept `driver: Option<SceneDriver>`
  prop, default Autoplay.
- `crates/kinetics/src/lib.rs` — prelude + `public_api_names()`
  additions.
- `crates/ui-styles/src/lib.rs` — add a `gsap_primitives.css` file
  for SplitText baseline styling (display: inline-block on the glyph
  spans so transforms work).
- `examples/component-gallery/src/previews/scenes/mod.rs` — `pub mod`
  the three new scenes.
- `examples/component-gallery/src/previews/scene.rs` — three new
  preview functions.
- `examples/component-gallery/src/docs.rs` — three new `ComponentDoc`
  entries + snippet consts.
- `examples/component-gallery/e2e/tests/_lib/component-manifest.ts`
  — three new manifest entries.

## Risks and mitigations

| Risk | Mitigation |
|------|------------|
| `IntersectionObserver` not cleaned up on Scene unmount → memory leak across hot-reload. | The web-only `ScrollDriver` returns a handle whose `Drop` impl `disconnect()`s the observer and removes the scroll listener. `Scene`'s `use_effect` returns the handle so Dioxus drops it on rerender / unmount. |
| Bézier arc-length sampling is expensive every frame. | Compute arc length once per Path at first sample, cache in a `Rc<RefCell<Option<Vec<f32>>>>` slot keyed inside the cue. SP-3 ships with simple linear scan; SP-3.5 could move to a binary-search lookup if profiling warrants. |
| SSR can't measure layout → `SplitMode::Line` impossible without a measurement-after-render pass. | Defer `Line` mode entirely; document it in the spec under "Out of scope." |
| SplitText accessibility: per-glyph spans break screen readers. | Parent `aria-label` carries the full text; per-glyph spans set `aria-hidden="true"`. Verified by SSR test. |
| Cross-platform: native targets have no real scroll. | `ScrollDriver` on native is a stub that constructs the config and holds the clock at progress 0; no panic, no DOM access. Documented in the cross-platform support matrix. |
| `MotionPath` with `rotate_along_path: true` adds a `rotate_deg` value that may conflict with an independent `MotionCue::Rotate` on the same target. | Document that combining is unsupported; the path's rotate wins (it's emitted later in the sample order). Update `MotionCueSample::merge` to ensure deterministic precedence. |
| Bézier numerical instability at extreme control points. | Sampler clamps `t` to `[0, 1]` and falls back to endpoint coordinates on NaN. Unit tests cover degenerate cases. |
| Showcase scenes inflate gallery first-paint cost. | The new scenes are inside the existing `ReplayFrame` / `ScrubFrame` lazy-render scaffolding (or equivalent — match SP-1's product intro pattern). Off-screen scenes don't autoplay. |

## Decisions and rationale

1. **`SceneDriver` enum, not separate Scene components per driver.** A
   `ScrollScene { ... }` component would couple scroll wiring into the
   Scene type. The enum keeps `Scene` the single component the user
   reasons about; the driver is a behavioral mode, not a different
   surface.
2. **`PathPoint` is structured, not SVG-string.** Parsing
   `<path d="M ..." />` is a substantial subproject and the gallery
   showcases don't need it. Defer to a future SP that lands a real SVG
   path parser.
3. **No new `FrameAdapter` impls.** SplitText and MotionPath are
   composition / new cue variants that the existing `SequenceAdapter`
   path picks up automatically. `ScrollDriver` is a clock *source*
   (calls `clock.seek_progress`), not an adapter.
4. **Arc-length-uniform sampling, not parameter-uniform.** Bézier
   curves accelerate / decelerate through high-curvature regions if
   sampled by raw parameter `t`. Arc-length sampling gives the
   visually expected constant speed.
5. **`SplitMode::Character` first, `Word` second, `Line` deferred.**
   Character / Word are SSR-renderable. Line requires post-layout
   measurement which would force the component into a two-pass render
   pattern; not worth the complexity for SP-3.
6. **One Playwright spec file with three tests, not three spec
   files.** All three exercise the same Scene category and share
   selectors. One file matches the SP-1 `scene-player.spec.ts`
   precedent.
7. **No WebGL.** Shader-based transitions and effects are deferred to
   a future SP. SP-3 stays in DOM-only territory.

## Acceptance criteria

- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets
  -- -D warnings`, `cargo test --workspace`, and `cargo check -p
  ui-runtime --target wasm32-unknown-unknown` all pass.
- `cargo test -p ui-timeline` includes the new path sampler tests,
  all passing.
- `cargo test -p ui-runtime --test scene_driver` passes.
- `cargo test -p ui-dioxus --test split_text_ssr` and
  `--test motion_path_ssr` pass.
- `cargo test -p ui-dioxus --test scene_player_ssr` retains all
  SP-1 + SP-3 tests passing (the new driver prop must not regress
  the existing 10 SSR tests).
- The three new gallery entries render in the Scene category.
- `npx playwright test --project=static tests/gsap-tier-primitives.spec.ts`
  passes on Chromium.
- `npx playwright test --project=static-webkit tests/gsap-tier-primitives.spec.ts`
  passes on WebKit.
- Existing `scene-player.spec.ts` continues to pass (driver prop
  default is backwards-compatible).

## Follow-ups

- **SP-3.5 / extension**: `SplitMode::Line` once a post-layout
  measurement primitive exists. Possibly the same primitive that
  underlies `Flip` layout transitions.
- **SP-3.x**: SVG path-string parser (`d="..."` → `Vec<PathPoint>`)
  if user demand surfaces.
- **SP-4**: Render pipeline can now snapshot scroll-driven scenes
  frame-by-frame using `SceneDriver::Manual` and stepping
  `clock.seek_ms` in a loop.
- **GSAP next-tier**: `Draggable`, `Observer`, `MorphSVG`, `Flip`.
  Each warrants its own sub-project; none are blocked by SP-3.
