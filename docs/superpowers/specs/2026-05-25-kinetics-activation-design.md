# Kinetics Activation — Design

## Goal

Activate the kinetics motion runtime end-to-end in production. Chrome
DevTools investigation confirmed that the workspace ships a complete
architecture (Scene clock → adapter registry → adapter.seek → inline
styles) but the leaf components (`KineticText`, `KineticBox` outside
Sequence, `TimelineScope`, `SplitText`, `WipeTransition`, and the three
ui-blocks scaffolds) never consume the clock. They render static
markup with motion-cue attributes that nothing reads. Every Scene
showcase in the gallery is visually inert.

This spec is **Tier C+**: full leaf activation (F1–F8) plus three
selected out-of-scope items:

- **F9**: the wgpu texture-format mismatch in `ui-glass-engine` that
  spams 1,000+ "Invalid CommandBuffer" warnings per scroll-pass on
  Windows Chromium.
- **F10**: the scroll-story default-progress investigation (DevTools
  showed it correctly at `elapsed_ms=0` on cold mount, but the prior
  Playwright trace caught it at `10000`. The cause is suspected to be
  the initial `compute_progress` call running before the trigger
  element's geometry is settled).
- **F11**: richer `WipeTransition` variants — `Linear` (current),
  `Conic`, `MaskPosition`, `Iris` — all pure CSS, no WebGL, no
  external libraries.

Out of scope (explicit, per user direction): no external library
integrations. No GSAP, no Lottie, no Anime.js, no Motion One — the
runtime stays pure-Rust + CSS + WAAPI. Audio-reactive motion is also
out of scope (no audio content in the gallery to react to; a separate
future spec can introduce it).

## Scope

In scope:

1. **F1 — Cue keyframe stylesheet.** Ship CSS keyframes for the
   existing cue keywords used across the codebase:
   - `fade-in` (opacity 0 → 1)
   - `rise-in` (translateY 12px → 0 + opacity 0 → 1)
   - `slide-up` (translateY 24px → 0)
   - `text-flow` (per-character entrance, alias of rise-in with
     slightly more travel)
   - `pop-in` (scale 0.94 → 1 + opacity 0 → 1, used by CTA buttons)
   Bundled into `ui-styles` as `kinetic_cues.css`, exposed via
   `library_css()` so any consumer who already imports the CSS gets
   the cues for free.

2. **F2 — `KineticText` and `KineticBox` self-drive.** Both
   components consume `SceneContext` (preferred) or
   `SequenceContext` (fallback for legacy use). On each render, they
   compute an inline `style` of the form:

   ```
   animation-name: ui-cue-{cue};
   animation-duration: 600ms;
   animation-fill-mode: forwards;
   animation-play-state: paused;
   animation-delay: -{elapsed_ms}ms;
   ```

   This is the **CssKeyframesAdapter** pattern from SP-1, inlined at
   the leaf so no adapter registration is needed. The browser handles
   the per-frame interpolation; the Rust side writes one style
   attribute per parent-clock change. Falls back to no animation
   (returning the existing static markup) when no Scene/Sequence
   ancestor is found, preserving SSR baselines.

3. **F3 — `Sequence` reads `SceneContext` when present.** Sequence's
   `clock` prop currently dominates. New behaviour: if a
   `SceneContext` is available, `elapsed_ms` flows from there;
   otherwise the prop wins. Explicit `clock: TimelineClock::Manual {
   elapsed_ms: ... }` still overrides (escape hatch for tests).

4. **F4 — `TimelineScope` becomes a real stagger driver.** Today it
   is a static marker. New behaviour:
   - Consume `SceneContext.clock` (or fall back to its own autoplay
     clock when `autoplay = true` and no SceneContext is present).
   - Walk children at render and find all elements with
     `data-stagger-index="N"`. For each child, compute
     `local_elapsed = parent_elapsed - (N * step_ms)`.
     `step_ms` defaults to 80ms (per the existing
     `StaggerFlow::ByIndex { step_ms: 80.0 }` pattern); overridable
     via a new `stagger_step_ms: Option<f32>` prop.
   - Provide each child its local elapsed via a `StaggerContext`
     that `KineticText` / `KineticBox` consume preferentially over
     the parent SceneContext.

5. **F5 — `SplitText` glyphs/words gain a default cue.** Each
   per-character or per-word span gains `data-motion-cue="rise-in"`
   (overridable via a new `cue: Option<String>` prop on `SplitText`).
   Combined with F4, glyphs animate per-stagger when wrapped in a
   `TimelineScope`. Parent `aria-label` and `data-stagger-index`
   contract preserved.

6. **F6 — `WipeTransition` consumes `SceneContext`.** Inline style
   becomes:
   - `animation-name: ui-block-wipe-transition-{variant}`
   - `animation-duration: {duration_ms}ms`
   - `animation-delay: -{elapsed_ms}ms`
   - `animation-play-state: paused`
   - `animation-fill-mode: forwards`

   The variant comes from F11's new `WipeVariant` prop.

7. **F7 — `LowerThird`, `SocialOverlay`, `MetricCounter`
   choreographed.** Each block wraps its internal markup in a
   default `Sequence` + cued children:
   - `LowerThird`: bar slides in (200ms), name fades up (600ms +
     200ms delay), role fades up (600ms + 400ms delay).
   - `SocialOverlay`: card slides in from top-right (400ms), then
     handle + message stagger up (200ms apart).
   - `MetricCounter`: label fades in (200ms), value rises in (400ms
     delay + 600ms), delta fades in (1000ms delay + 600ms).

   These are baked-in choreographies — consumers don't need to wrap
   the blocks in additional Sequence/TimelineScope. Each block
   composes one internal Sequence + cued children that ride the
   parent SceneContext.

8. **F8 — Motion regression tests.** Two new test surfaces:
   - **SSR test**: at construction with `elapsed_ms = 0`, the
     leaf's inline style sets `animation-delay: 0ms`. At
     `elapsed_ms = duration_ms / 2`, the inline style sets
     `animation-delay: -<duration_ms/2>ms`. Assert the style string
     changes.
   - **E2E test**: `examples/component-gallery/e2e/tests/
     animation-motion.spec.ts` samples computed `transform` /
     `opacity` at two timestamps for every Scene category entry
     that has animated content; asserts the values differ. Catches
     "renders but doesn't animate" bugs at the canonical browser
     level.

9. **F9 — `ui-glass-engine` wgpu format negotiation.** The current
   engine assumes `TextureFormat::BGRA8Unorm` for its render target;
   on Windows Chromium the surface advertises
   `TextureFormat::RGBA8UnormSrgb` and the pipeline-vs-encoder format
   mismatch generates 1 invalid command buffer per glass frame
   (~1,000 warnings in a typical session). Fix: call
   `surface.get_capabilities(adapter)` and pick a compatible format
   from the returned list, preferring `BGRA8UnormSrgb` first and
   `RGBA8UnormSrgb` as a fallback. Update both the encoder and
   pipeline construction paths.

10. **F10 — Scroll-story default-progress on mount.** The
    `ScrollDriver::install` function seeds an initial
    `compute_progress` call before the trigger DOM is laid out. If
    `trigger.get_bounding_client_rect()` returns zeroed values, the
    formula degenerates to `(start - 0) / total ≈ 1.0` and the scene
    settles instantly. Fix: gate the initial seed on a
    `requestAnimationFrame` callback so the trigger is laid out
    before the first compute_progress runs.

11. **F11 — Richer `WipeTransition` variants.** New
    `WipeVariant` enum: `Linear` (current behaviour), `Conic`,
    `MaskPosition`, `Iris`. Each variant ships its own keyframe
    `@keyframes` definition. Backwards-compat: existing
    `WipeTransition { duration_ms, angle_deg }` callers stay
    correct, with `Linear` as the default variant.

   - `Linear`: linear-gradient mask sweeps across the children at
     `angle_deg` (current).
   - `Conic`: conic-gradient mask rotates around the centre.
   - `MaskPosition`: linear-gradient mask whose `mask-position`
     sweeps from 0% to 200% over `duration_ms`.
   - `Iris`: radial-gradient mask shrinks toward / expands from the
     centre.

12. **Showcase additions.** Three new Scene category entries for
    F11 variants and one updated baseline:
    - `Scene · Wipe Conic Demo`
    - `Scene · Wipe Iris Demo`
    - `Scene · Wipe Mask-Position Demo`
    The existing `Scene · Wipe Transition Demo` becomes the explicit
    `Linear` variant.

Out of scope:

- Any external library integration (GSAP, Lottie, Anime.js, Motion
  One, Three.js).
- Audio-reactive motion (no Web Audio API consumption).
- WebGL/wgpu shader transitions (Linear/Conic/MaskPosition/Iris
  are CSS-only).
- Per-Caption stagger pace (SP-3's `reading_pace_ms_per_word` stays
  advisory — the TimelineScope stagger step is workspace-wide via
  the new `stagger_step_ms` prop on TimelineScope).
- New `MotionCue` variants beyond what already exists.
- Per-glyph SplitText `SplitMode::Line` — still needs
  post-layout measurement.

## Architecture

### F1 — Cue keyframes

```css
/* crates/ui-styles/src/kinetic_cues.css */

@keyframes ui-cue-fade-in {
  from { opacity: 0; }
  to   { opacity: 1; }
}

@keyframes ui-cue-rise-in {
  from { opacity: 0; transform: translateY(12px); }
  to   { opacity: 1; transform: translateY(0); }
}

@keyframes ui-cue-slide-up {
  from { transform: translateY(24px); }
  to   { transform: translateY(0); }
}

@keyframes ui-cue-text-flow {
  from { opacity: 0; transform: translateY(8px); }
  to   { opacity: 1; transform: translateY(0); }
}

@keyframes ui-cue-pop-in {
  from { opacity: 0; transform: scale(0.94); }
  to   { opacity: 1; transform: scale(1); }
}
```

Wired into `ui-styles::library_css()` so any consumer that imports
the shared CSS gets the keyframes available globally.

### F2 — `KineticText` / `KineticBox` inline style

New helper in `ui-dioxus::cue_style`:

```rust
pub(crate) fn cue_inline_style(
    cue: &str,
    elapsed_ms: f32,
    duration_ms: f32,
) -> String {
    format!(
        "animation-name: ui-cue-{cue}; \
         animation-duration: {duration_ms}ms; \
         animation-fill-mode: forwards; \
         animation-play-state: paused; \
         animation-delay: -{elapsed_ms}ms;"
    )
}

pub(crate) fn cue_animation_duration_ms(cue: &str) -> f32 {
    match cue {
        "fade-in" => 600.0,
        "rise-in" => 720.0,
        "slide-up" => 600.0,
        "text-flow" => 600.0,
        "pop-in" => 480.0,
        _ => 600.0,
    }
}
```

`KineticText` / `KineticBox` consume context preference order:

1. `StaggerContext` (the per-glyph offset from a TimelineScope) — if
   present, that's the effective `elapsed_ms`.
2. `SequenceContext` (legacy SP-1 path) — falls through if no
   StaggerContext.
3. `SceneContext` (canonical Scene clock) — falls through if no
   SequenceContext.
4. No context — render static markup (existing behaviour, preserves
   SSR snapshots for unwrapped use).

### F3 — Sequence/SceneContext bridge

In `Sequence`, after the existing `clock: Option<TimelineClock>`
prop is resolved, check for `SceneContext`:

```rust
let clock = match clock {
    Some(c) => c,
    None => try_consume_context::<SceneContext>()
        .map(|ctx| TimelineClock::Manual {
            elapsed_ms: *ctx.clock.elapsed_ms.read(),
        })
        .unwrap_or(TimelineClock::Playback { elapsed_ms: 0.0 }),
};
```

A consumer that explicitly passes `clock: Some(...)` keeps the old
behaviour (escape hatch for tests).

### F4 — TimelineScope stagger driver

`TimelineScope` gains an internal autoplay clock (when
`autoplay: true` and no SceneContext is available) plus a
`StaggerContext` provider:

```rust
#[derive(Clone, Copy)]
pub struct StaggerContext {
    pub elapsed_ms: Signal<f32>,
    pub step_ms: f32,
}

#[component]
pub fn TimelineScope(
    id: String,
    autoplay: Option<bool>,
    stagger_step_ms: Option<f32>,
    children: Element,
) -> Element {
    let step_ms = stagger_step_ms.unwrap_or(80.0);
    let elapsed = use_hook(|| {
        if let Some(scene) = try_consume_context::<SceneContext>() {
            scene.clock.elapsed_ms
        } else if autoplay.unwrap_or(false) {
            // Spawn own rAF loop, similar to SceneClock::play() but
            // unbounded (loops forever or until unmount).
            spawn_internal_autoplay_clock()
        } else {
            Signal::new(0.0)
        }
    });
    use_context_provider(|| StaggerContext { elapsed_ms: elapsed, step_ms });
    rsx! {
        section { class: "ui-timeline-scope", "data-autoplay": "{autoplay:?}",
            {children}
        }
    }
}
```

Each `KineticText` / `KineticBox` consumes the StaggerContext if
present. The child uses its DOM `data-stagger-index` (which Dioxus
emits onto the rendered element) to compute its local elapsed_ms
via `local = max(0, parent - index * step_ms)`.

Reading `data-stagger-index` from the rendered DOM during render
is not idiomatic Dioxus; instead, the stagger index is passed
through context-by-position. The actual implementation:

- `TimelineScope` wraps each child in a `StaggerChild` that
  captures the child's positional index (0, 1, 2, ...) at compile
  time via `for (i, child) in ...`.
- `KineticText` / `KineticBox` consume a `StaggerOffsetContext`
  (an `Option<usize>`) provided by `StaggerChild`.

### F5 — SplitText default cue

`SplitText` gains a `cue: Option<String>` prop. Per-character spans
are emitted with `class="ui-split-text__glyph"` plus inline style
generated by F2's helper. When wrapped in a `TimelineScope`, each
glyph automatically inherits its stagger offset.

```rust
#[component]
pub fn SplitText(
    text: String,
    split_by: Option<SplitMode>,
    cue: Option<String>,
) -> Element {
    let cue = cue.unwrap_or_else(|| "rise-in".to_string());
    // ... existing aria-label parent ...
    // Each glyph/word now wraps in StaggerChild and renders via
    // KineticBox-equivalent inline-style generation.
}
```

### F6 — WipeTransition

```rust
#[component]
pub fn WipeTransition(
    duration_ms: f32,
    angle_deg: Option<f32>,
    variant: Option<WipeVariant>,
    children: Element,
) -> Element {
    let variant = variant.unwrap_or(WipeVariant::Linear);
    let angle = angle_deg.unwrap_or(90.0);
    let elapsed = try_consume_context::<SceneContext>()
        .map(|ctx| *ctx.clock.elapsed_ms.read())
        .unwrap_or(0.0);

    let style = format!(
        "animation-name: ui-block-wipe-{kind}; \
         animation-duration: {duration_ms}ms; \
         animation-delay: -{elapsed}ms; \
         animation-play-state: paused; \
         animation-fill-mode: forwards; \
         --wipe-angle: {angle}deg;",
        kind = variant.css_keyword(),
    );

    rsx! { div { class: "ui-block-wipe-transition", style: "{style}", {children} } }
}
```

`WipeVariant`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum WipeVariant {
    #[default]
    Linear,
    Conic,
    MaskPosition,
    Iris,
}

impl WipeVariant {
    fn css_keyword(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Conic => "conic",
            Self::MaskPosition => "mask-position",
            Self::Iris => "iris",
        }
    }
}
```

### F11 — Wipe variant keyframes

In `ui-styles/src/gsap_primitives.css` (extend the existing file):

```css
@keyframes ui-block-wipe-linear {
  from { mask-image: linear-gradient(var(--wipe-angle), black 0%, transparent 0%); }
  to   { mask-image: linear-gradient(var(--wipe-angle), black 100%, transparent 100%); }
}

@keyframes ui-block-wipe-conic {
  from { mask-image: conic-gradient(from 0deg, black 0deg, transparent 0deg); }
  to   { mask-image: conic-gradient(from 0deg, black 360deg, transparent 360deg); }
}

@keyframes ui-block-wipe-mask-position {
  from { mask-image: linear-gradient(var(--wipe-angle), transparent 0%, black 50%); mask-position: 0% 0%; }
  to   { mask-image: linear-gradient(var(--wipe-angle), transparent 0%, black 50%); mask-position: 200% 0%; }
}

@keyframes ui-block-wipe-iris {
  from { mask-image: radial-gradient(circle at center, black 0%, transparent 0%); }
  to   { mask-image: radial-gradient(circle at center, black 100%, transparent 100%); }
}
```

### F7 — Block choreography

Each block wraps its content in a Sequence with named cues. Example
`LowerThird`:

```rust
#[component]
pub fn LowerThird(
    name: String,
    role: String,
    accent: Option<LowerThirdAccent>,
) -> Element {
    let accent = accent.unwrap_or_default();
    let accent_class = match accent {
        LowerThirdAccent::Primary => "ui-block-lower-third--primary",
        LowerThirdAccent::Secondary => "ui-block-lower-third--secondary",
    };
    let aria = format!("{name}, {role}");
    rsx! {
        div { class: "ui-block-lower-third {accent_class}",
            "aria-label": "{aria}",
            "data-block": "lower-third",
            TimelineScope { id: "lower-third-stagger", autoplay: false,
                StaggerChild { index: 0,
                    KineticBox { id: "lower-third-bar", cue: "slide-up",
                        div { class: "ui-block-lower-third__bar" }
                    }
                }
                StaggerChild { index: 1,
                    KineticText { id: "lower-third-name", cue: "rise-in", text: name }
                }
                StaggerChild { index: 2,
                    KineticText { id: "lower-third-role", cue: "fade-in", text: role }
                }
            }
        }
    }
}
```

`SocialOverlay` and `MetricCounter` follow the same pattern with
their own choreographies.

### F8 — Motion regression tests

#### SSR (per-leaf)

In `crates/ui-dioxus/tests/cue_inline_style_ssr.rs`:

```rust
#[test]
fn kinetic_text_in_scene_context_sets_animation_delay_negative_elapsed() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(false),
            Scene {
                id: "test", width: 100, height: 100, duration_ms: 5_000.0,
                autoplay: Some(false),
                KineticText { id: "x", text: "hi".to_string(), cue: "fade-in" }
            }
        }
    });
    assert!(html.contains("animation-name: ui-cue-fade-in"), "{html}");
    assert!(html.contains("animation-delay: -0ms"), "{html}");
}
```

Plus a paired test where the Scene context provides a non-zero
`elapsed_ms` (using a custom test provider) and the assertion is
`animation-delay: -<elapsed>ms`.

#### E2E (whole-scene)

`examples/component-gallery/e2e/tests/animation-motion.spec.ts`:

For each Scene category entry whose autoplay is true, sample the
animated leaf at the first frame and again 200ms later. Assert
that the inline `style` attribute's `animation-delay` value has
changed — proves the clock is feeding the leaves.

```ts
const sample = async (locator: Locator) => {
  return await locator.evaluate((el: HTMLElement) => el.style.animationDelay);
};

test("KineticText in Product Intro updates animation-delay over time", async ({ page }) => {
  await page.goto("/");
  await page.locator("#scene").scrollIntoViewIfNeeded();
  const card = page.locator(
    "article.gallery-entry:has(h4:has-text('Scene · Product Intro 10s'))",
  );
  const title = card.locator('[data-kinetic-id="intro-title"]').first();
  const first = await sample(title);
  await page.waitForTimeout(250);
  const second = await sample(title);
  expect(first).not.toBe(second);
});
```

Repeat for every scene that should animate.

### F9 — Glass engine surface format

Currently in `ui-glass-engine`, the render pipeline + encoder
hard-code `TextureFormat::BGRA8Unorm`. New behaviour:

```rust
let caps = surface.get_capabilities(adapter);
let preferred = [
    TextureFormat::BGRA8UnormSrgb,
    TextureFormat::RGBA8UnormSrgb,
    TextureFormat::BGRA8Unorm,
    TextureFormat::RGBA8Unorm,
];
let format = preferred
    .iter()
    .copied()
    .find(|f| caps.formats.contains(f))
    .unwrap_or_else(|| caps.formats.first().copied().unwrap_or(TextureFormat::BGRA8Unorm));
```

Pass the resolved `format` into both the surface config and the
pipeline color target. Verify by re-running the gallery in Chrome
DevTools — the "Invalid CommandBuffer" warnings should disappear.

### F10 — Scroll driver mount progress

In `crates/ui-runtime/src/drivers/scroll.rs::install_scroll_driver`,
defer the initial `compute_progress` call:

```rust
// Old: seed immediately
let initial = compute_progress(&window, &trigger, start_offset, end_offset);
(on_progress.borrow_mut())(initial);

// New: defer until next rAF so layout is settled.
let on_progress_seed = on_progress.clone();
let window_seed = window.clone();
let trigger_seed = trigger.clone();
let cb = Closure::once_into_js(move || {
    let progress = compute_progress(&window_seed, &trigger_seed, start_offset, end_offset);
    (on_progress_seed.borrow_mut())(progress);
});
let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
```

### Gallery integration

Existing scenes are preserved as-is — they automatically activate
because their leaves now consume SceneContext. Three new entries for
F11 wipe variants:

- `Scene · Wipe Conic Demo` — `WipeTransition { variant: Conic }`
- `Scene · Wipe Iris Demo` — `WipeTransition { variant: Iris }`
- `Scene · Wipe Mask-Position Demo` — `WipeTransition { variant: MaskPosition }`

The existing `Scene · Wipe Transition Demo` stays as the explicit
Linear baseline.

### Reduced motion

When `clock.reduced = true`, the leaf's inline style sets
`animation-delay: -<duration_ms>ms; animation-play-state: paused`
which freezes the animation at its endpoint. Same final-frame
behaviour as today; tested via the existing reduced-motion
Playwright path.

## Public API additions

- `WipeVariant` enum (in `ui-blocks::wipe_transition`).
- `StaggerOffsetContext` (internal — not in prelude).
- `TimelineScope` gains `stagger_step_ms: Option<f32>` prop
  (defaults to 80ms).
- `SplitText` gains `cue: Option<String>` prop (defaults to
  `"rise-in"`).
- `WipeTransition` gains `variant: Option<WipeVariant>` prop
  (defaults to `Linear`).

## Files (final)

New:

- `crates/ui-styles/src/kinetic_cues.css`
- `crates/ui-dioxus/src/cue_style.rs` (the helper functions
  `cue_inline_style` and `cue_animation_duration_ms`)
- `crates/ui-dioxus/src/stagger.rs` (`StaggerChild`,
  `StaggerOffsetContext`)
- `crates/ui-dioxus/tests/cue_inline_style_ssr.rs`
- `examples/component-gallery/e2e/tests/animation-motion.spec.ts`
- `examples/component-gallery/src/previews/scenes/wipe_conic_demo.rs`
- `examples/component-gallery/src/previews/scenes/wipe_iris_demo.rs`
- `examples/component-gallery/src/previews/scenes/wipe_mask_position_demo.rs`

Edited:

- `crates/ui-styles/src/lib.rs` — include `kinetic_cues.css` in
  `library_css()`.
- `crates/ui-styles/src/gsap_primitives.css` — append the four
  wipe variant keyframes.
- `crates/ui-dioxus/src/lib.rs` — `pub mod cue_style; pub mod stagger;`.
- `crates/ui-dioxus/src/kinetics.rs` — `KineticText`, `KineticBox`,
  `Sequence`, `TimelineScope` to consume SceneContext + emit
  cue-driven inline styles.
- `crates/ui-dioxus/src/split_text.rs` — emit cue-driven children
  + accept `cue` prop.
- `crates/ui-blocks/src/wipe_transition.rs` — `WipeVariant` enum +
  SceneContext-driven inline style.
- `crates/ui-blocks/src/lower_third.rs` — wrap in TimelineScope
  with cued children.
- `crates/ui-blocks/src/social_overlay.rs` — same.
- `crates/ui-blocks/src/metric_counter.rs` — same.
- `crates/ui-runtime/src/drivers/scroll.rs` — defer initial
  compute_progress to rAF.
- `crates/ui-glass-engine/...` — surface format negotiation. Exact
  files determined during implementation; the engine's pipeline
  + surface config sites are the targets.
- `examples/component-gallery/src/docs.rs` — three new
  `ComponentDoc` entries for wipe variants. Bump array length
  55 → 58.
- `examples/component-gallery/src/previews/scenes/mod.rs` — pub
  mod the three new wipe variant scenes.
- `examples/component-gallery/src/previews/scene.rs` — three new
  preview functions.
- `examples/component-gallery/e2e/tests/_lib/component-manifest.ts`
  — three new manifest entries.
- `crates/kinetics/src/lib.rs` — re-export `WipeVariant`.

## Testing

### Unit (Rust)

- `crates/ui-dioxus/tests/cue_inline_style_ssr.rs` — per-leaf SSR
  assertions that the inline `animation-name` / `animation-delay`
  reflect the parent Scene's `elapsed_ms`.
- `crates/ui-dioxus/tests/timeline_scope_stagger_ssr.rs` — assert
  that each StaggerChild's child receives a different
  StaggerOffsetContext value.

### E2E (Playwright)

- `examples/component-gallery/e2e/tests/animation-motion.spec.ts`
  — for every Scene category entry whose autoplay is true, sample
  the animated leaf at the first frame and again 250ms later.
  Assert the inline `style.animationDelay` value differs.
- Existing specs (`scene-player`, `gsap-tier-primitives`,
  `catalog-blocks`, `exposure-polish`) must remain green —
  regression check.
- New manifest entries trigger visual.spec.ts baseline generation
  on first run; commit the resulting Chromium baselines.

### Manual (Chrome DevTools)

After all commits, re-run the DevTools investigation that
surfaced this spec. Confirm:
- All Scene entries have animated leaves at runtime (inline
  `style.animationDelay` is non-zero on settled-state scenes).
- Glass-engine console no longer spams "Invalid CommandBuffer"
  warnings.
- Scroll-story `data-elapsed-ms` starts at `0` on cold mount and
  advances correctly when the user scrolls.

## Risks and mitigations

| Risk | Mitigation |
|---|---|
| Setting `style.animationDelay` per-frame causes DOM thrash. | Only re-set when the delay would change by ≥1ms (round to int ms). Dioxus diffs the prop at the framework level so identical strings don't re-mount the attribute. |
| Reduced-motion edge case: `animation-delay: -duration_ms` should freeze at endpoint, not loop. | The keyframes use `animation-fill-mode: forwards` so the endpoint state persists. Verified by an explicit SSR test with `ReducedMotionProvider { reduced: Some(true) }`. |
| `TimelineScope` autoplay clock when no SceneContext present spawns a rAF loop that may leak. | Reuse `SceneClock`'s `HandleSlot` cleanup pattern; the handle is dropped on TimelineScope unmount. |
| `data-stagger-index` walking via StaggerChild instead of DOM scan changes the public contract. | The `data-stagger-index` attribute is still emitted on each child (preserving SP-3's Playwright assertions); StaggerChild is the new internal positional walker. Both signals stay in sync. |
| Glass engine format change may break native Vulkan/Metal targets. | The fallback chain ends with `BGRA8Unorm` (current behaviour), so any target whose surface only advertises that format is unchanged. Tested via wgpu's existing capability negotiation. |
| Conic / Iris mask-image CSS may not be supported on every WebKit version. | Provide CSS feature-detection via `@supports` — fall back to Linear when the variant's primary mask-image function isn't recognized. |
| Inline `animation-name: ui-cue-{cue}` requires the keyframe to exist globally. | The cue keyframes are bundled into `library_css()` which the gallery already injects via `<style>`. Downstream apps that don't include `library_css()` get no cue animation but no error either — graceful degradation. |
| `SplitText` per-glyph spans growing inline styles inflates DOM size. | Only emit the inline style on glyphs when wrapped in a Scene/Sequence/TimelineScope. Bare `SplitText` outside any ancestor stays static (existing behaviour). |
| Wipe variant `Iris` / `Conic` not currently supported by Firefox `mask-image`. | Document the cross-browser support matrix in the variant docs. Chromium + WebKit are the gallery's e2e targets. |

## Decisions and rationale

1. **CssKeyframesAdapter pattern inlined into leaves, not via
   registry.** Per-leaf adapter registration would require every
   leaf to identify itself to the registry by id, which is heavy
   for thousands of glyphs. Inline `animation-delay` setting is
   the same trick the registry-based `CssKeyframesAdapter` would
   do — just collapsed into the leaf so no registration is
   needed.
2. **Sequence prefers SceneContext when no explicit clock prop.**
   Reverse — preferring the prop — would mean Scene-wrapped
   Sequences would need every caller to pass a manual clock.
   That's the SP-3 limitation we accepted; this fix retires it.
3. **TimelineScope ships its own autoplay clock as fallback.**
   A TimelineScope outside a Scene must still animate (it's the
   canonical wrapper for non-Scene staggers, e.g. inside a
   non-cinematic component). Falling back to its own clock
   preserves that.
4. **StaggerChild over data-stagger-index walking.** Reading DOM
   attributes back during render is anti-pattern in Dioxus.
   StaggerChild captures the positional index at compile time;
   the `data-stagger-index` attribute on the rendered DOM is
   parallel signal kept for backward-compat with SP-3's tests.
5. **WipeVariant defaults to Linear.** Existing
   `WipeTransition { duration_ms, angle_deg }` callers stay
   correct. Tier C+ is additive on the wipe surface.
6. **Glass engine wgpu format fix is in scope** (not deferred to
   a separate spec) because the DevTools console noise is now
   the user's most-visible visual symptom of "broken animations"
   even though it's an orthogonal bug.
7. **Scroll driver rAF defer.** A simpler fix would be to delay
   the entire driver install via rAF; the chosen fix defers
   only the initial seed so listener attachment is still
   immediate. This avoids a 16ms window where scroll events go
   uncaught.
8. **No external library integrations.** Explicit user direction.

## Acceptance criteria

- `cargo fmt --all -- --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo test --workspace`,
  `cargo check -p ui-runtime --target wasm32-unknown-unknown`,
  `cargo check -p ui-dioxus --target wasm32-unknown-unknown`, and
  `cargo check -p ui-blocks --target wasm32-unknown-unknown` all
  pass.
- `cargo test -p ui-dioxus --test cue_inline_style_ssr` passes.
- `examples/component-gallery/e2e/tests/animation-motion.spec.ts`
  passes on Chromium and WebKit.
- Existing Playwright specs (`scene-player`,
  `gsap-tier-primitives`, `catalog-blocks`, `exposure-polish`)
  remain green — no regressions.
- The four new Wipe variant Scene category entries are visible in
  the gallery.
- Manual Chrome DevTools verification: every autoplay Scene's
  primary animated leaf has a non-zero / changing
  `style.animationDelay` over time.
- Glass engine console no longer emits "Invalid CommandBuffer"
  warnings.
- Scroll-story Scene starts at `data-elapsed-ms="0"` on cold
  mount.

## Follow-ups

- Audio-reactive `MotionCue` variants — Web Audio API
  `AnalyserNode` drives a custom `MotionCue::AudioAmplitude` for
  level-meter-style animations. Separate future spec.
- `SplitMode::Line` — requires post-layout measurement; depends
  on a `use_element_rect`-based two-pass render. Separate future
  spec.
- WebGL/wgpu shader transitions — extend `WipeTransition` to
  accept a fragment-shader source. Separate future spec.
- `MotionPath` `rotate_along_path` propagation from the
  `MotionCue::Path` tangent to the `MotionPath` Dioxus prop.
  Small follow-up.
- Per-Caption stagger pace (the SP-3 limitation) — pipe the
  `reading_pace_ms_per_word` prop through SplitText to the
  surrounding TimelineScope's `stagger_step_ms`.
