# Hyperframe Composition Player — Design (SP-1)

## Goal

Land the keystone sub-project of the "HyperFrames for Dioxus" track: a
single paused-seekable **`Hyperframe`** Dioxus component that owns one
clock and broadcasts `elapsed_ms` to every registered animation runtime
through a formal **`FrameAdapter`** contract. SP-1 unifies the two
parallel time models the workspace ships today (`Sequence` /
`TimelineScope` driven by `ui_timeline::TimelineClock`, and `FrameStage`
/ `FrameClip` carrying data-only attributes), proves the player with one
cinematic showcase scene in the component gallery, and reserves a new
`Hyperframe` gallery category that subsequent sub-projects expand.

> Naming note. The data struct `ui_composition::Composition`
> (id/width/height/fps/frame_count) already exists. The new Dioxus
> component is therefore named **`Hyperframe`**, not `Composition`, to
> avoid the type collision and to match the gallery category name and
> the HeyGen `hyperframes` reference. Internal modules and types use
> the `hyperframe_*` / `Hyperframe*` naming throughout.

The user's complaint is that the Motion section of the gallery has
primitive previews (`Spring enter`, `Tween exit`, 2-col scrub tiles) but
nothing GSAP-tier or HeyGen-HyperFrames-tier — i.e. no cinematic,
seekable, multi-runtime composition that *sells* the engine. The
ingredients for HyperFrames-class output already exist in
`ui-composition` (`Composition`, `FrameClip`, `FrameLayer`, `FrameCue`,
`FrameClock`), in `ui-capture` (`ExportManifest`, `ViewportProfile`,
`CaptureStageDescriptor`), in `ui-timeline` (`TimelineClock::Manual` /
`::Scroll` / `::Frame`), and in `ui-runtime` (WAAPI bridge, frame
scheduler, presence and shared-element runtimes). They are not joined.

This spec lands the join.

## Scope

In scope:

1. A `FrameAdapter` trait in `ui-runtime::frame_adapter` plus three
   built-in adapters (`SequenceAdapter`, `WaapiAdapter`,
   `CssKeyframesAdapter`).
2. A signal-backed `HyperframeClock` in `ui-runtime::hyperframe_clock`
   with `play` / `pause` / `seek_ms` / `seek_progress`, a settle state,
   and an autoplay loop that reuses the existing
   `ui_runtime::scheduler::spawn_frame_loop`.
3. A `Hyperframe` Dioxus component in `ui-dioxus::hyperframe_player`
   that hosts the clock, provides a `HyperframeContext` via Dioxus
   context, and optionally renders a scrubber + play/pause + time
   readout when `controls: true`.
4. A `Clip` Dioxus component that consumes the parent
   `HyperframeContext` and actually shows/hides + fades its children
   based on `start_ms`, `duration_ms`, and `ClipFill`, replacing the
   previous data-attribute-only `FrameClip`.
5. Adapter registration via Dioxus context + `use_effect`, so child
   components can call `use_register_frame_adapter(...)` without
   shoving `Box<dyn FrameAdapter>` through props.
6. A new gallery category `Hyperframe` (slug `hyperframe`) and one
   cinematic showcase entry, **Hyperframe · Product Intro 10s**, that
   exercises every piece end-to-end and is scrubbable in the gallery.
7. A Playwright spec under
   `examples/component-gallery/e2e/hyperframe-composition.spec.ts` that
   scrubs the showcase from 0 % to 100 % and verifies settled end state.
8. Public API re-exports from `kinetics::prelude` (`Hyperframe`, `Clip`,
   `HyperframeState`, `FrameAdapter`, `SequenceAdapter`, `WaapiAdapter`,
   `CssKeyframesAdapter`, `use_register_frame_adapter`).

Out of scope (each is a future sub-project):

- SP-2: Cinematic showcase library (more than one scene). SP-1 ships
  the *one* scene that proves the player.
- SP-3: GSAP-tier primitives. ScrollTrigger, SplitText, MotionPath are
  separate adapters and components in a later spec.
- SP-4: Headless render pipeline. PNG-sequence and MP4 export via
  FFmpeg / headless Chromium / wgpu offscreen lives downstream.
- SP-5: `kinetics` CLI (`init`, `preview`, `lint`, `render`, `doctor`).
- SP-6: Reusable cinematic blocks crate (`ui-blocks`) and agent skill
  (`/kinetics-hyperframe`).
- External-runtime bridges (Lottie, GSAP JS, Anime.js, Three.js,
  Motion One) beyond the three built-in adapters above.
- Audio tracks. HyperFrames composes audio via `<audio>` clips; this
  spec is video-visual only. Audio is a follow-up under SP-2 or SP-4.
- Native (non-web) WAAPI / CSS keyframe equivalents. `WaapiAdapter`
  and `CssKeyframesAdapter` are `#[cfg(target_arch = "wasm32")]`; on
  desktop and native targets they no-op (`SequenceAdapter` is
  cross-platform and carries the showcase scene off the web).

## Architecture

### Module layout

```
crates/ui-runtime/src/
  hyperframe_clock.rs    # signal-backed transport state
  frame_adapter.rs       # FrameAdapter trait + adapter registry context
  adapters/
    mod.rs
    sequence.rs          # wraps ui_timeline::Timeline::sample
    waapi.rs             # web-only paused WAAPI bridge
    css_keyframes.rs     # web-only animation-delay tricks
  lib.rs                 # re-exports

crates/ui-dioxus/src/
  hyperframe_player.rs   # Hyperframe + Clip + scrubber UI
  lib.rs                 # re-exports

crates/ui-styles/src/
  hyperframe_player.css  # scrubber, transport bar, settled tag

examples/component-gallery/src/previews/
  hyperframe.rs                # gallery preview module
  hyperframe_scenes/
    mod.rs
    product_intro.rs           # the 10s cinematic showcase
    flip_card_deck.rs
    metric_counter.rs
    cta_pulse.rs
```

### `FrameAdapter` trait

```rust
pub trait FrameAdapter: 'static {
    fn id(&self) -> &str;
    fn duration_ms(&self) -> f32;
    /// Called every time the parent Composition clock advances or is
    /// scrubbed. MUST be deterministic in elapsed_ms; MUST be infallible
    /// (adapters internally clamp / no-op on bad input).
    fn seek(&self, elapsed_ms: f32, reduced: bool);
}
```

Adapters are wrapped in a workspace-local `FrameAdapterHandle` newtype
that boxes the trait object. The handle is `Rc<RefCell<…>>` on web,
`Arc<Mutex<…>>` on native, matching the pattern already used by
`SharedElementRegistry` in `ui-runtime::shared`. Adapter identity is by
`id()` string, so registration is idempotent and re-renders do not
duplicate.

### `HyperframeClock`

`hyperframe_clock.rs` exposes:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HyperframeState {
    Paused,
    Playing,
    Settled,   // reached duration_ms; respects FillMode::Forwards
}

#[derive(Clone, Copy)]
pub struct HyperframeClock {
    pub duration_ms: Signal<f32>,
    pub elapsed_ms:  Signal<f32>,
    pub state:       Signal<HyperframeState>,
    pub fps:         Signal<u32>,
    pub reduced:     Signal<bool>,
}

impl HyperframeClock {
    pub fn new(duration_ms: f32, fps: u32, reduced: bool) -> Self;
    pub fn play(&self);
    pub fn pause(&self);
    pub fn seek_ms(&self, ms: f32);
    pub fn seek_progress(&self, fraction: f32);   // 0.0..=1.0
    pub fn settle(&self);                         // jump to duration_ms
    pub fn frame_clock(&self) -> FrameClock;       // derived from elapsed + fps
}
```

`HyperframeClock` is `Copy` because every field is a Dioxus `Signal<T>`
(which is `Copy`); this matches the existing `Sequence` / `TimelineScope`
state pattern. `play()` calls into
`ui_runtime::scheduler::spawn_frame_loop` and advances `elapsed_ms` by
`dt_ms` per tick until it reaches `duration_ms`, then transitions to
`Settled`. `pause()` cancels the handle. `seek_*` clamps to
`[0, duration_ms]` and leaves `state` unchanged (scrubbing while paused
stays paused; scrubbing while playing keeps playing from the new
position). Constructing the clock with `reduced = true` short-circuits
autoplay and forces `Settled` after one seek to `duration_ms`.

### `HyperframeContext`

Provided via `use_context_provider`:

```rust
#[derive(Clone, Copy)]
pub struct HyperframeContext {
    pub id: Signal<Rc<str>>,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub duration_ms: f32,
    pub clock: HyperframeClock,                  // exposes signals + transport
    pub adapters: FrameAdapterRegistry,          // for child registration
}
```

Consumers read `clock.elapsed_ms`, `clock.state`, and `clock.reduced`
directly (each is a `Signal<T>`). The `FrameAdapterRegistry` is a small
`Rc<RefCell<Vec<(String, FrameAdapterHandle)>>>` on web /
`Arc<Mutex<…>>` on native, indexed by `id()`. A child registers by
calling:

```rust
pub fn use_register_frame_adapter<A: FrameAdapter + 'static>(adapter: A);
```

which inserts on mount, removes on drop, and replaces any existing
entry with the same `id()` on hot-reload.

### `Hyperframe` component

```rust
#[component]
pub fn Hyperframe(
    id: String,
    width: u32,
    height: u32,
    duration_ms: f32,
    fps: Option<u32>,                  // default 60
    autoplay: Option<bool>,            // default true
    controls: Option<bool>,            // default false
    children: Element,
) -> Element;
```

Renders the same hyperframes-compatible data attributes used today
(`data-composition-id`, `data-width`, `data-height`, `data-fps`) plus
new attrs the render pipeline (SP-4) and external tooling will need:
`data-duration-ms`, `data-elapsed-ms` (live), `data-state`
(`playing|paused|settled`), `data-reduced` (`true` when reduced motion
forces the settled state). The root element also sets
`style="aspect-ratio: <width> / <height>"` so downstream layout decides
scaling without a dedicated aspect prop.

When `controls: true`, a `<div class="ui-hyperframe-transport">` is
rendered after children with a play/pause `<button>`, a
`<input type="range" min=0 max=duration_ms>` scrubber, and a
`<span class="ui-hyperframe-time">` time readout.

`Hyperframe` is responsible for:

- Constructing the `HyperframeClock` on first render.
- Providing the `HyperframeContext` via context.
- Running an effect with explicit dependencies on
  `clock.elapsed_ms` *and* `clock.state` so the final settle-driven
  seek fires even when `elapsed_ms` is already at `duration_ms` at
  the moment `Settled` is set.
- The effect iterates the adapter registry and calls
  `adapter.seek(elapsed_ms, reduced)`. This is the *only* place
  adapters are driven.
- Mounting and unmounting the scheduler frame loop based on `state`.

### `Clip` component

```rust
#[component]
pub fn Clip(
    start_ms: f32,
    duration_ms: f32,
    fill: Option<ClipFill>,        // default ClipFill::None
    children: Element,
) -> Element;
```

Subscribes to `HyperframeContext.clock.elapsed_ms` and computes
`active = clip.active_at_ms(elapsed_ms)` from a new
`FrameClip::active_at_ms(ms)` helper added to `ui-composition`. Active
clips render at opacity 1; outside the range, behavior depends on
`fill`:

- `ClipFill::None`: render with `visibility: hidden; opacity: 0;
  pointer-events: none`.
- `ClipFill::HoldStart`: pre-roll holds at opacity 1.
- `ClipFill::HoldEnd`: post-roll holds at opacity 1.
- `ClipFill::HoldBoth`: always opacity 1.

A clip's inline style is applied directly (no rAF mutation per frame
beyond the parent's broadcast). Data attrs (`data-start-ms`,
`data-duration-ms`, `data-fill`, `data-active`) stay on the DOM so the
SP-4 render pipeline can audit clip schedules without re-deriving them.

### Built-in adapters

**`SequenceAdapter`** (cross-platform): owns a
`ui_timeline::Timeline` and a list of registered target node refs. On
`seek(ms, reduced)` it samples the timeline (or its `reduced_motion()`
counterpart when `reduced` is true) and writes the resolved
`ResolvedMotionState.inline_style()` onto each target via the
existing `ui-dom::CssStyleWriter`. This is exactly what
`TimelineScope` does internally; `SequenceAdapter` is the same code
path exposed under the new trait.

**`WaapiAdapter`** (`#[cfg(target_arch = "wasm32")]`): wraps a
`web_sys::Animation` constructed paused
(`Animation::play(); Animation::pause()` before the first frame). On
`seek(ms, reduced)`, calls
`animation.set_current_time(Some(elapsed_ms as f64))` after ensuring
`animation.play_state() == "paused"`. If `reduced`, jumps to
`duration_ms` once and ignores subsequent seeks until reset. Cleanup
uses the existing pattern in `ui-runtime::scheduler_web` —
`Closure::wrap` + `add_event_listener_with_callback` with a manually
tracked closure handle dropped on adapter unregister. No new `gloo`
dependency is introduced; the workspace already binds via raw
`web-sys` / `wasm-bindgen` / `js-sys`.

**`CssKeyframesAdapter`** (`#[cfg(target_arch = "wasm32")]`): owns a
target `web_sys::HtmlElement` and a keyframe name. On `seek`, mutates
inline style:
`animation-play-state: paused; animation-delay: -<elapsed_ms>ms;
animation-duration: <duration_ms>ms; animation-fill-mode: forwards`.
This is the lightest-weight bridge — useful for designers who already
ship `@keyframes` in CSS.

### Data flow

```
[scrubber drag]
    ↓ on_input
[clock.seek_ms(ms)]
    ↓ Signal<f32>::set
[HyperframeContext.clock.elapsed_ms updates]
    ↓ use_effect in Hyperframe (deps: elapsed_ms + state)
[for each adapter: adapter.seek(ms, reduced)]
    ↓                              ↓
[SequenceAdapter writes      [WaapiAdapter / CssKeyframesAdapter
  inline transform on          retime native animation]
  Sequence targets]
    ↓
[non-adapter Sequence / KineticBox children re-sample via
 TimelineClock::Manual { elapsed_ms } because they subscribe
 to HyperframeContext.clock.elapsed_ms through use_timeline_sample]
```

Autoplay path: `play()` spawns a frame loop whose handler advances
`elapsed_ms` by `dt_ms`. When `elapsed_ms >= duration_ms` the loop
calls `clock.settle()` which stops the loop and emits one final
`seek(duration_ms, reduced)`. Pausing cancels the handle but leaves
`elapsed_ms` where it is.

### Reduced motion

`reduced` is sourced from the existing `ReducedMotion` context the
preference bar already provides. When `reduced` is `true`:

- `Hyperframe` skips `autoplay`; it constructs the clock and
  immediately calls `clock.settle()`.
- The scrubber renders disabled (`aria-disabled="true"`,
  `pointer-events: none`, dimmed visual).
- A `<span class="ui-hyperframe-reduced-tag">Reduced motion · settled
  state</span>` annotation is appended to the transport bar so the
  policy is visible to product reviewers and assistive tech.
- Adapters receive `seek(duration_ms, true)` exactly once. They MUST
  render the final, settled visual.

This matches the policy `Sequence` and `TimelineScope` already
implement via `Timeline::reduced_motion()`.

### Public API surface

`kinetics::prelude` re-exports:

- `Hyperframe`, `Clip`, `HyperframeState`
- `FrameAdapter`, `SequenceAdapter`, `WaapiAdapter`,
  `CssKeyframesAdapter`
- `use_register_frame_adapter`
- `HyperframeContext` (read-only handle for advanced consumers)

The existing `FrameStage`, `FrameClip`, `FrameLayer` Dioxus components
are kept as deprecation shims under
`#[deprecated(note = "use kinetics::Hyperframe / kinetics::Clip")]`,
so the existing `Composition` gallery category and downstream code do
not break. The Composition section keeps its existing entries and
gains a "Legacy data-attr stage" note linking to the Hyperframe
category.

### Gallery integration

`examples/component-gallery/src/docs.rs`:

- Add `ComponentCategory::Hyperframe` after `ComponentCategory::Capture`.
- Label: `Hyperframe`. Slug: `hyperframe`. Description:
  `Seekable cinematic compositions driven by paused frame adapters.
  HyperFrames-class output, native Dioxus.`
- One `ComponentDoc` entry,
  `name: "Hyperframe · Product Intro 10s"`,
  status `Ready`, render `Some(product_intro_preview)`, snippet
  inlined from `previews/hyperframe_scenes/product_intro.rs`.
- The `populated_categories()` filter in `app.rs` will automatically
  pick up the new category once the entry lands.

`examples/component-gallery/src/previews/hyperframe_scenes/product_intro.rs`:

A 10-second composition at 1920×1080 / 60 fps:

| Time (ms)  | Beat                                                                            |
|------------|----------------------------------------------------------------------------------|
| 0 – 1200   | Hero title `KineticText` fades + slide-up (`SequenceAdapter`)                    |
| 800 – 2400 | Body line fades + slide-up (overlap)                                             |
| 1800 – 3400| `Clip` swap: secondary headline holds end → primary headline fade out            |
| 3000 – 5200| FLIP card row enters via `SharedLayout` + `SharedElement`                        |
| 4800 – 7000| Animated metric counter (`Sequence` on numeric children)                         |
| 6800 – 9000| CTA button pop + ambient pulse (`WaapiAdapter` paused keyframes on web)          |
| 9000 – 10000| Settle frame — every element in final state                                     |

Wrapped in a `Hyperframe { controls: true }` so reviewers can scrub.

### Showcase snippet (illustrative)

```rust
rsx! {
    Hyperframe {
        id: "product-intro",
        width: 1920,
        height: 1080,
        duration_ms: 10_000.0,
        fps: Some(60),
        autoplay: true,
        controls: true,
        Clip { start_ms: 0.0, duration_ms: 2400.0, fill: ClipFill::HoldEnd,
            KineticText { id: "intro-title",
                text: "Kinetics moves like light.",
                cue: "rise-in",
            }
        }
        Clip { start_ms: 800.0, duration_ms: 2400.0, fill: ClipFill::HoldEnd,
            KineticText { id: "intro-body",
                text: "Composable motion for downstream SaaS.",
                cue: "fade-in",
            }
        }
        Clip { start_ms: 3000.0, duration_ms: 4000.0,
            SharedLayout {
                FlipCardDeckScene {}
            }
        }
        Clip { start_ms: 4800.0, duration_ms: 2200.0,
            MetricCounterScene {}
        }
        Clip { start_ms: 6800.0, duration_ms: 3200.0, fill: ClipFill::HoldEnd,
            CtaPulseScene {}
        }
    }
}
```

`FlipCardDeckScene`, `MetricCounterScene`, and `CtaPulseScene` live in
sibling files under `hyperframe_scenes/` to keep `product_intro.rs`
under the 500-line guideline.

## Testing

### Unit (`ui-runtime`)

- `hyperframe_clock::tests::play_advances_elapsed` — under a fake
  rAF source, `elapsed_ms` reaches `duration_ms` and transitions to
  `Settled`.
- `hyperframe_clock::tests::seek_clamps` — `seek_ms(-50.0)` clamps
  to 0; `seek_ms(duration_ms + 100.0)` clamps to `duration_ms` and
  transitions to `Settled`.
- `hyperframe_clock::tests::reduced_freezes_at_settled` —
  constructing with `reduced=true` produces `Settled` state and
  `elapsed_ms = duration_ms` after one tick.
- `frame_adapter::tests::registry_is_idempotent_by_id` — two
  registrations of the same `id()` overwrite the first, drop removes
  exactly one entry.
- `frame_adapter::tests::seek_fans_out_to_all_registered` — a fake
  adapter records calls; after `clock.seek_ms(123.0)` every
  registered adapter sees exactly one `seek(123.0, false)`.

### Integration (`ui-dioxus`)

Using `dioxus-ssr` (already in the workspace dev-dependencies):

- `hyperframe_player::tests::ssr_renders_settled_when_reduced` —
  reduced-motion `ReducedMotion` context produces the same DOM as
  `elapsed_ms = duration_ms`.
- `hyperframe_player::tests::clip_visibility_across_fill_modes` —
  matrix of `(ClipFill, elapsed_ms before/inside/after)` against
  expected `data-active` and inline-style output.
- `hyperframe_player::tests::transport_attrs_track_state` — after
  `play()` then `pause()`, `data-state` reflects `paused` and the
  scrubber `value` matches `data-elapsed-ms`.

### E2E (`examples/component-gallery/e2e`)

- `hyperframe-composition.spec.ts`:
  1. Visit `/component-gallery/#hyperframe`.
  2. Find the `Hyperframe · Product Intro 10s` card.
  3. Assert the scrubber exists, is enabled, and has `min=0`,
     `max=10000`.
  4. Drive the scrubber to `value=10000`; assert
     `[data-state="settled"]` and every nested `[data-active]`
     reflects the final frame.
  5. Toggle the gallery's Motion preference to `Reduced`; assert
     the scrubber is `aria-disabled="true"` and a
     `.ui-hyperframe-reduced-tag` is visible.
  6. Snapshot the settled-state DOM and the reduced-state DOM; both
     must be equal except for the reduced tag.
  7. Run under both the `static` (Chromium) and `static-webkit`
     Playwright projects already configured in
     `playwright.config.ts`.

### Visual regression

The existing Playwright snapshot infra already captures key gallery
sections per `2026-05-23-gallery-playwright-audit-design.md`. We add
one snapshot: `hyperframe-composition--settled.png`. We do **not** add
snapshots at intermediate scrub positions in SP-1; those land with
SP-4 when render-to-PNG is deterministic.

## Files

New:

- `crates/ui-runtime/src/hyperframe_clock.rs`
- `crates/ui-runtime/src/frame_adapter.rs`
- `crates/ui-runtime/src/adapters/mod.rs`
- `crates/ui-runtime/src/adapters/sequence.rs`
- `crates/ui-runtime/src/adapters/waapi.rs`
- `crates/ui-runtime/src/adapters/css_keyframes.rs`
- `crates/ui-dioxus/src/hyperframe_player.rs`
- `crates/ui-styles/src/hyperframe_player.css`
- `examples/component-gallery/src/previews/hyperframe.rs`
- `examples/component-gallery/src/previews/hyperframe_scenes/mod.rs`
- `examples/component-gallery/src/previews/hyperframe_scenes/product_intro.rs`
- `examples/component-gallery/src/previews/hyperframe_scenes/flip_card_deck.rs`
- `examples/component-gallery/src/previews/hyperframe_scenes/metric_counter.rs`
- `examples/component-gallery/src/previews/hyperframe_scenes/cta_pulse.rs`
- `examples/component-gallery/e2e/hyperframe-composition.spec.ts`

Edited:

- `crates/ui-runtime/src/lib.rs` — re-export new modules.
- `crates/ui-dioxus/src/lib.rs` — re-export `Hyperframe`, `Clip`,
  `HyperframeState`.
- `crates/ui-dioxus/src/composition.rs` — mark `FrameStage`,
  `FrameClip`, `FrameLayer` `#[deprecated]`.
- `crates/ui-composition/src/lib.rs` — add `FrameClip::active_at_ms`
  helper and minor doc additions; keep frame-based `active_at` for
  the legacy shim.
- `crates/ui-styles/src/lib.rs` — include the new
  `hyperframe_player.css` in `library_css()`.
- `crates/kinetics/src/lib.rs` — extend `prelude` and
  `public_api_names()`.
- `examples/component-gallery/src/previews/mod.rs` — add
  `pub mod hyperframe;`.
- `examples/component-gallery/src/docs.rs` — `ComponentCategory::Hyperframe`,
  slug, description, doc entry.

## Risks and mitigations

| Risk | Mitigation |
|------|------------|
| `Vec<Box<dyn FrameAdapter>>` props would break Dioxus `PartialEq` requirements and re-render diff. | Adapters register via context + `use_effect`, not props. The registry is keyed by `id()`. |
| Scrubbing a WAAPI animation while it is "running" causes jank or sub-frame jumps in WebKit. | `WaapiAdapter::seek` pauses before setting `currentTime`. The clock owns the play/pause concept; adapters live in scrubbed mode always. |
| Existing `FrameStage` / `FrameClip` consumers in the gallery break. | Keep them as `#[deprecated]` shims; Composition gallery category continues to render them. Migration is opt-in. |
| Reduced-motion users see a scrubber that does nothing, which feels broken. | Scrubber is `aria-disabled` and a "Reduced motion · settled state" tag is visible. Decision is documented in the spec and surfaced in the UI, not hidden. |
| Adapter registry over-renders when child adapter components mount / unmount frequently (e.g. inside a `for` loop). | Registry is `Rc<RefCell<…>>` with stable id keys; effect runs only on mount and on `adapter.id()` change. The `Hyperframe` effect that fans out `seek` reads the registry once per frame, not per registration event. |
| `spawn_frame_loop` on native (non-web) targets does not preserve perfect timing under load. | SP-1 ships the *scaffolding*; perfect-determinism native playback is SP-4 territory (render pipeline). Native autoplay is best-effort; native scrub is exact. |
| `WaapiAdapter::seek` on stale (unmounted) DOM elements panics in `wasm-bindgen`. | Adapter stores a captured `web_sys::Element` it owns; on `seek` it checks `Node::is_connected()` and no-ops when detached. No `gloo` dependency required. |
| The new Hyperframe gallery entry slows the gallery's first paint. | `autoplay` defaults to `true` only inside the dedicated showcase; the gallery preview wraps it in the existing `ReplayFrame` / `ScrubFrame` analog so it's offscreen until scrolled into view via `IntersectionObserver` (already used by `ScrubFrame`). |

## Decisions and rationale

1. **Adapter contract is infallible.** Animation runtimes fail in
   wildly different ways; codifying error variants would either be a
   leaky abstraction or a stringly-typed grab bag. Instead adapters
   clamp / no-op internally and the contract guarantees the
   Composition never has to handle adapter errors. This matches the
   GSAP / WAAPI / Anime.js convention.
2. **Time is `f32` milliseconds, not `u32` frames.** HyperFrames uses
   seconds in HTML attributes; this workspace already speaks
   milliseconds across `ui-timeline`, `ui-motion`, and `ui-runtime`.
   We expose `FrameClock { frame, fps }` derived from `elapsed_ms` so
   frame-indexed consumers (the future SP-4 render pipeline) still
   work.
3. **`Hyperframe` is *not* a renderer.** It is a player. The render
   pipeline (PNG sequence, MP4) lives in SP-4. Keeping these separate
   means SP-1 ships as a pure Dioxus library change with no FFmpeg or
   headless-Chromium dependency, and SP-4 can iterate on backends
   without re-touching the player.
4. **Legacy `FrameStage` stays.** Deprecation, not deletion. The
   existing audit reports cite `FrameStage`; ripping it out would
   churn snapshots and break downstream apps that imported it.
5. **Only three built-in adapters in SP-1.** Lottie / GSAP-JS /
   Anime.js bridges are obvious next adapters but each is its own
   small spec; SP-1 lands the *contract* and proves it with three
   adapters that already have native homes in this workspace.
6. **One showcase scene, not five.** This is brainstorming Tier-3
   work decomposed into spec-sized slices; SP-2 expands the showcase
   library to five+ scenes. One scene is enough to prove the player
   and gallery wiring; more is risk without more learning.

## Acceptance criteria

- `cargo check`, `cargo test`, `cargo clippy -- -D warnings`, and
  `cargo fmt --check` all pass.
- The Playwright spec in `e2e/hyperframe-composition.spec.ts` passes
  in CI on Chromium and WebKit.
- The gallery renders a new `Hyperframe` section between `Capture`
  and the end of the page (or wherever `populated_categories()`
  surfaces it given category order in `docs.rs`).
- Scrubbing the `Hyperframe · Product Intro 10s` entry from 0 % to
  100 % visibly drives every nested scene (title, body, FLIP deck,
  metric counter, CTA pulse) to its end state.
- Toggling Motion to `Reduced` in the preference bar produces a
  disabled scrubber, a visible "Reduced motion · settled state" tag,
  and a DOM equal to the settled state.
- The existing Composition / Motion gallery entries continue to
  render unchanged (deprecation, not removal).

## Follow-ups

- **SP-2:** Cinematic showcase library — port four more scenes
  (shader wipe, social overlay, lower-third, audio-reactive title)
  to the same `Hyperframe` API. Decide whether to ship as gallery
  entries only or also a standalone `examples/hyperframes-showcase`
  Dioxus app.
- **SP-3:** GSAP-tier primitives — `ScrollTrigger` (a `FrameAdapter`
  driven by `IntersectionObserver` + scroll progress feeding
  `TimelineClock::Scroll`), `SplitText` (per-glyph cues extending
  `KineticText`), `MotionPath` (sample SVG path → translate /
  rotate cue stream).
- **SP-4:** Headless render pipeline — drive `Hyperframe` frame by
  frame from a CDP / headless-Chromium harness, snapshot the DOM
  via `Page.captureScreenshot`, emit a PNG sequence into the
  `ExportManifest` directory and pipe through FFmpeg. Native target
  via wgpu offscreen render is a stretch goal.
- **SP-5:** `kinetics` CLI — `kinetics init my-comp`,
  `kinetics preview`, `kinetics lint`, `kinetics render`, and
  `kinetics doctor` subcommands.
- **SP-6:** `ui-blocks` catalog crate of reusable cinematic blocks
  (lower thirds, captions, charts, transitions) plus a
  `/kinetics-hyperframe` agent skill.
