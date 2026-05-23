# Motion Engine Cleanup Design

## Goal

Close out the architectural shortcuts Spec 2 left behind so the WAAPI
runtime path becomes the single source of truth on wasm targets. Today
`use_animation_value_from` runs a per-frame RAF loop that writes the
animation value into a Dioxus signal, AND `use_animation_target.play_on(...)`
fires a WAAPI animation against the same element. Both update inline
style (RAF directly, WAAPI via the compositor's keyframe interpolation).
Their disagreement mid-animation is the most plausible root cause of
the three new motion regressions Spec 2's audit surfaced
(`Dialog`, `Toast`, `Tooltip`).

This spec also closes four other Spec 2 follow-ups the final review
flagged: the half-built `ReducedMotionProvider`, the two parallel
WAAPI play sites (`use_animation_target` and
`kinetics_waapi::play_cue_on_mount`), the missing per-cue stagger
delay, and three test specs whose assertions need updating for the
post-WAAPI DOM.

## Scope

This spec changes:

- `crates/ui-runtime/src/animation.rs` — when wasm + WAAPI-supported,
  `use_animation_value_from` skips the RAF loop and sets the signal to
  the target value synchronously. The actual visible interpolation
  runs on the compositor via a WAAPI animation that `use_animation_target.play_on(...)`
  starts. Under SSR (no `window`), native (no `web_sys`), or browsers
  without `Element.animate`, the legacy RAF loop continues to drive
  per-frame inline-style writes for SSR snapshot parity.
- `crates/ui-runtime/src/reduced_motion.rs` — `ReducedMotionProvider`
  gains a `use_effect` that subscribes to `prefers-reduced-motion`'s
  `MediaQueryList.onchange` and to a `MutationObserver` on the body
  watching for `data-ui-motion` attribute mutations. The provided
  `ReducedMotion(...)` value updates when either changes; consumers
  re-render through Dioxus context propagation.
- `crates/ui-runtime/src/waapi.rs` — `options_object(duration_ms)`
  becomes `options_object(duration_ms, delay_ms)` to surface WAAPI's
  `delay` field. Existing call sites pass `0.0`; the per-cue stagger
  site at `play_cue_on_mount` passes `cue.start_ms`.
- `crates/ui-dioxus/src/kinetics.rs` — the `kinetics_waapi`
  sub-module's `play_cue_on_mount` is deleted. `KineticBox`'s
  `onmounted` handler instead calls `UseAnimationTarget::play_on(...)`
  on a handle materialized by a new `use_kinetic_animation` hook (thin
  wrapper around `use_animation_target` that knows how to read the
  active cue from the parent `SequenceContext` and apply its
  `start_ms` as the WAAPI delay).
- `examples/component-gallery/e2e/tests/components/sequence.spec.ts`
  — the transform regex on `bodyEnd.transform` accepts
  `translate(0px, 0px)` in addition to `translateY(0px)`. Browsers
  normalise single-axis translates to two-arg form on settled state.
- `examples/component-gallery/e2e/tests/components/timeline-scope.spec.ts`
  — the t=0 ≤0.1 opacity assertion is removed for the autoplay
  stagger variant. After Spec 2's autoplay=true preview change, by
  the time Playwright samples the first frame the animation has
  already started; the assertion was assuming a manual clock that
  doesn't apply to autoplay. The end-state assertion at t=1200ms is
  kept.
- `examples/component-gallery/e2e/tests/components/kinetic-box.spec.ts`
  — reads computed style via `page.evaluate(el => getComputedStyle(el).opacity)`
  rather than the `readStyles(box, [...])` inline-attribute parser.
  WAAPI animates on the compositor; the inline `style="..."` attribute
  does not reflect mid-flight values.
- `docs/superpowers/specs/2026-05-23-motion-engine-modernization-design.md`
  — appends a single "Errata" section noting:
  - The gallery's preference-bar toggles are `<button role="radio" onclick>`,
    not `<input type="radio" onchange>`; `selectRadio` uses
    `.click({ force: true })`, not `dispatchEvent`.
  - `use_animation_value_from`'s RAF loop was retained as a fallback,
    not removed entirely (the spec's "the signal does not tick per
    frame" line was aspirational; Spec 3 makes it actually true on
    wasm).

It excludes:

- View Transitions API for `SharedElement` / `SharedLayout` (Spec 4).
- Scroll-driven animations (Spec 5).
- Frame-rate budgeting (Spec 6).
- Any change to `LiquidGlass` / `ui-glass-engine` / wgpu.
- Per-component reduced-motion opt-outs.
- A reactive `prefers-color-scheme` provider parallel to
  `ReducedMotionProvider`. Out of scope; theme is owned by the gallery.

## Non-Goals

- Removing the RAF fallback entirely. SSR + non-wasm consumers (Dioxus
  native, future Tauri shells) need it.
- Animating WAAPI's `currentTime` from manual scrub events. Manual
  clock continues to use the synchronous `Timeline::sample` path; this
  spec only changes the Playback clock path.
- Surfacing WAAPI `Animation.onfinish` as a callback prop on
  `Presence` / `KineticBox`. Internal use only.
- A general-purpose `use_kinetic_animation` public API. The hook lives
  inside `ui-dioxus` as an internal helper; consumers continue to
  drive motion via `Sequence` / `KineticBox` / `Presence`.

## Architecture Overview

The fix to the dual-path bug is small and load-bearing. Today
`use_animation_value_from` does:

```text
on signal change:
  if reduced: signal.set(target); return
  spawn_frame_loop {
    each frame:
      compute eased value
      signal.set(value)         ← writes inline style every frame
  }
```

`use_animation_target.play_on(element)` then ALSO calls
`Element.animate(keyframes, {fill: forwards})` against the same
element. Both writes happen in parallel.

After Spec 3 on wasm + WAAPI-supported:

```text
on signal change:
  if reduced or waapi_available: signal.set(target); return
  spawn_frame_loop { ... } // fallback only
```

The signal becomes a one-shot snapshot of the target (consistent with
"the Rust-side animation value"). The actual interpolation happens on
the compositor when `play_on(element)` is called from the consumer's
`onmounted` handler. Inline style writes still happen once at SSR
time and once when WAAPI lands the final `forwards`-filled frame; in
between, the browser owns the interpolated frames.

The consolidation of the two WAAPI play sites:

- Today: `KineticBox` (in `ui-dioxus/kinetics.rs`) reads the active
  cue from `SequenceContext`, manually calls
  `keyframes_for_transition` → `keyframes_to_js` → `WaapiAnimation::play`.
  Meanwhile `use_animation_target` (in `ui-runtime/animation.rs`)
  encapsulates exactly that pipeline.
- After: `KineticBox`'s `onmounted` calls a new
  `use_kinetic_animation(cue, kinetic_id) -> UseAnimationTarget`
  helper. Helper internally calls `use_animation_target(property, from, to, transition)`
  with the parameters extracted from the cue. The
  `kinetics_waapi::play_cue_on_mount` function is deleted along with
  `pick_animated_axis` (both moved into the new helper).

Stagger delay:

- `WaapiAnimation::play` already accepts an `options` JS object. The
  `options_object` builder gains a `delay_ms` parameter that is set
  on the JS object when non-zero. `use_animation_target` gains a
  `with_delay(ms: f32)` builder that stores the value on
  `UseAnimationTarget`; `play_on` reads it.
- The new `use_kinetic_animation` helper passes the parent
  `Cue.start_ms` as the delay when it builds the target.

Reactive `ReducedMotionProvider`:

- Inside `ReducedMotionProvider` (wasm path), a `use_effect` subscribes
  to two events:
  1. `window.matchMedia("(prefers-reduced-motion: reduce)").addEventListener("change", ...)`.
  2. `MutationObserver` watching `document.body` for the
     `data-ui-motion` attribute on body OR on any descendant. The
     observer re-runs `detect_reduced_motion_at_root()` on each
     mutation and writes to the signal.
- Both listeners are cleaned up when the provider unmounts.
- On non-wasm: the effect is a no-op (the static probe stays).

## Data Flow

After this spec, on wasm + WAAPI-supported:

```
Component renders
  └─ use_animation_target(property, initial, target, transition)
       └─ use_animation_value_from(initial, target, transition)
            └─ if WAAPI supported: signal.set(target); return
       └─ build UseAnimationTarget { property, target, transition, ... }
       └─ return (UseAnimationTarget, ReadSignal<f32>=target)

Consumer's onmounted callback fires (post-render):
  └─ UseAnimationTarget::play_on(element, current_value=from)
       └─ keyframes_for_transition(from, to, transition)
       └─ Element.animate(keyframes, { duration, delay, fill: "forwards" })
       └─ store WaapiAnimation handle for cancel-on-drop

When the consumer re-renders with a different target:
  └─ Old WaapiAnimation drops → cancel()
  └─ New keyframes computed from element's current rendered value
  └─ New WaapiAnimation plays
```

On non-wasm or WAAPI-unsupported:

```
Component renders
  └─ use_animation_value_from drives the signal via RAF as today
  └─ Consumer reads the signal each frame for inline-style writes
  └─ No WAAPI; UseAnimationTarget::play_on is a no-op
```

## Test-Side Fixes

Three Playwright specs need adjustments. The contracts the tests
assert are still correct; the assertions themselves were written
against the pre-WAAPI inline-style format.

- **`sequence.spec.ts`**: the regex
  `/translateY\(0(?:\.0+)?px\)|^$|none/` already accepts the empty
  string and `none`, but not the browser-normalised
  `translate(0px, 0px)`. Add that as an alternation.
- **`timeline-scope.spec.ts`**: the t=0 ≤0.1 opacity assertion on
  `tile0` was written assuming a manual scrub clock. The Spec 2 fix
  set autoplay=true, so the animation has already begun by the time
  the test samples. Remove the t=0 assertion; keep the t=1200ms ≥0.95
  assertion. The contract being asserted ("the stagger tiles animate
  in") is preserved by the end-state check.
- **`kinetic-box.spec.ts`**: instead of `readStyles(box, ["opacity", "transform"])`,
  use `page.evaluate((el) => { const cs = getComputedStyle(el); return { opacity: cs.opacity, transform: cs.transform }; }, await box.elementHandle())`.
  Assert at least one of `opacity ≠ "1"` or `transform ≠ "none"` is
  observed at the sampled timestamp.

## Errata For The Spec 2 Design Doc

Append the following block to
`docs/superpowers/specs/2026-05-23-motion-engine-modernization-design.md`:

```markdown
## Errata (recorded in Spec 3)

The following spec-vs-implementation drift was discovered during the
Spec 2 audit and resolved either at implementation time or carried
forward to Spec 3:

- The gallery's preference-bar toggles are `<button role="radio" onclick>`,
  NOT `<input type="radio" onchange>`. The plan's `selectRadio` snippet
  prescribed `dispatchEvent("input"/"change")`, which doesn't fire the
  onclick handler. The actual fix used `.click({ force: true })` to
  bypass Playwright's actionability check while keeping a real click
  event. Future versions of this spec should describe the actual DOM.
- The "Architecture Overview" promised that `use_animation_value_from`'s
  RAF loop would be replaced by WAAPI; Spec 2 in fact retained the RAF
  loop as a fallback path and let WAAPI run in parallel. Spec 3 makes
  the parallelism conditional on WAAPI-unsupported environments only.
- `ReducedMotionProvider`'s reactive listener was deferred to Spec 3.
  Spec 2 shipped the static probe + provider component.
- `kinetics_waapi::play_cue_on_mount` (in `crates/ui-dioxus/src/kinetics.rs`)
  ended up as a second WAAPI play site, parallel to
  `use_animation_target`. Spec 3 consolidates them.
```

## Error Handling

- The new WAAPI-availability branch in `use_animation_value_from`
  checks `ui_runtime::waapi::is_supported()` at the time the hook is
  called. If the result changes between renders (it should not — the
  feature is detected once and cached), the worst case is one
  render's worth of stale behavior. The thread-local `OnceCell` in
  `waapi::is_supported` ensures the value is stable.
- The reactive listener's `addEventListener` calls catch any JS
  exception and fall back to the static probe. The `MutationObserver`
  is created with `try` and the closure leaks (released on provider
  drop via stored handle).
- WAAPI delay values <0 are clamped to 0 in `options_object`.

## Testing

- A new wasm-bindgen test in `crates/ui-runtime/tests/reduced_motion_wasm.rs`
  exercises the reactive listener: mutate `data-ui-motion` on body
  after mounting a `ReducedMotionProvider`, assert the consumer
  observes the new value within one render cycle.
- A new unit test in `crates/ui-motion/src/lib.rs` covers
  `options_object` (well, equivalently in `ui-runtime::waapi`) with
  delay: the JS object's `delay` field is set when `delay_ms != 0` and
  omitted (or 0) otherwise.
- The 3 updated Playwright specs are re-run to confirm pass.
- A new e2e:ci audit run regenerates `audit-report.md`; the target is
  ≥ 276/280 (the only acceptable remaining failures are flaky
  network-timeouts on Toast's auto-dismiss + any other Spec 2-era
  test-only flakes that have nothing to do with the motion engine).

## Risks And Mitigations

- **Removing the RAF loop on wasm breaks Presence's `value` signal
  reads downstream.** `Presence` reads `value()` per render to advance
  `PresenceState`. If `value()` only returns `0.0` and then jumps to
  `1.0` at completion, the `Entering → Visible` transition fires
  twice (once at signal mount, once at WAAPI finish). Mitigation: the
  hook still calls `signal.set(target)` synchronously, so by the time
  the next render reads, `value() == target`. The `advance_presence`
  state-machine treats `value == target` as the trigger to advance —
  same outcome.
- **WAAPI's `delay` option counts from when `play()` is called, not
  from when the timeline started.** For stagger with N tiles each
  starting at offset Ki·100ms, the per-tile `delay` is Ki·100ms IF
  all tiles mount at the same time. Tiles that mount later see a
  shifted timeline. Mitigation: `TimelineScope` mounts all
  `KineticBox` children synchronously; the order is deterministic.
  If a future consumer mounts children asynchronously, the helper
  will need an absolute "timeline start" reference. Out of scope for
  Spec 3.
- **MutationObserver overhead.** Watching `document.body` for
  `data-ui-motion` mutations is cheap if the attribute is on the
  immediate body. Watching for the attribute on any descendant
  requires `subtree: true`, which can be expensive on large DOMs.
  Mitigation: configure the observer for body-only (no subtree); the
  gallery sets `data-ui-motion` on the shell which IS the immediate
  body child, and consumers outside the gallery should follow the
  same pattern (documented in the Errata addition above).

## Out Of Scope (Spec 4+)

- View Transitions API for SharedElement/SharedLayout.
- Scroll-driven animations.
- WebKit pointer-interception issues on tile prose (a CSS
  `pointer-events: none` on `.gallery-variant-tile span` would help;
  out of scope for this engine-level spec).
- Per-component motion opt-outs.

## Implementation Phasing

The plan (separate document) will stage:

1. **`use_animation_value_from` wasm short-circuit** (lands first;
   smallest change; reproduces the Dialog/Toast/Tooltip pointer
   interception fix).
2. **Stagger delay through `options_object` + `use_animation_target.with_delay(...)`**.
3. **Consolidate KineticBox onto `use_animation_target`**; delete the
   `kinetics_waapi` sub-module.
4. **Reactive ReducedMotionProvider listener**.
5. **Test-side fixes** for Sequence/TimelineScope/KineticBox specs.
6. **Spec 2 errata note**.
7. **Re-run audit + commit report**.

Steps 1, 2, 3 are sequential. 4, 5, 6 can land in parallel.

The audit-report after this spec ships should show every component
as `ready` across all four variants, with at most one or two flaky
network-timeout failures remaining.
