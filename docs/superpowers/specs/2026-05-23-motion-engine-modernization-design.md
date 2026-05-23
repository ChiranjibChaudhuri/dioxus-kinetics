# Motion Engine Modernization Design

## Goal

Migrate the Kinetics motion runtime from per-frame main-thread JS
inline-style writes to the **Web Animations API (WAAPI)** as the
compositor-offloaded production path, and fix the regressions the
Spec 1 audit surfaced in the same edit pass. The two changes are
combined because both touch `ui-runtime` and `crates/ui-dioxus/src/kinetics.rs`,
and bug-for-bug parity is easier to assert when only one motion engine
runs at a time.

The user's `audit-report.md` from the freshly-merged Spec 1 lists two
`regression` rows (`Sequence`, `FrameStage`), plus three more hidden
behind a reporter classifier bug (`TimelineScope`, `KineticBox`, and
`Presence` on WebKit). The root cause for the highest-leverage failure
— Sequence + KineticBox not honoring `prefers-reduced-motion` — is one
function: `ui_runtime::reduced_motion::use_reduced_motion()` reads a
`ReducedMotion` context that no one provides. The gallery's preference
bar sets `data-ui-motion="reduced"` on the shell, but the runtime never
consults it. Reduced motion has been a no-op in production.

This spec lands:

1. The reduced-motion wiring fix (root cause of three failures).
2. The WAAPI runtime path (the GPU-acceleration deliverable).
3. The remaining audit-surfaced fixes: FrameStage strict-mode, KineticBox
   ReplayFrame contract, reporter classifier, TimelineScope stagger
   preview, WebKit pointer interception on Presence, Win32→Linux
   baseline bootstrap.

It does **not** land:

- View Transitions API for `SharedElement` / `SharedLayout` (deferred
  to Spec 3).
- Scroll-driven animations (`@scroll-timeline` / `@view-timeline`) —
  deferred to Spec 4.
- A frame-rate budgeting harness — deferred to Spec 5.

## Scope

This spec changes:

- `crates/ui-runtime/src/animation.rs` — `use_animation_value` /
  `use_animation_value_from` are reimplemented to drive a WAAPI
  `Animation` against the mounted element instead of writing inline
  styles per RAF tick. The signal value continues to exist for SSR /
  testing parity; the visible motion runs off the main thread.
- `crates/ui-runtime/src/timeline.rs` — `use_timeline_sample` keeps its
  current synchronous-sample behavior for `TimelineClock::Manual`
  (scrub previews) and `TimelineClock::Frame` (composition stages), but
  the `TimelineClock::Playback` branch is rewritten to declare a
  per-track WAAPI animation rather than to drive a RAF loop.
- `crates/ui-runtime/src/presence.rs` — `use_presence_animation` is
  rewritten to declare an enter/exit WAAPI animation per state change
  and to listen to the `finish` event to advance the state machine,
  replacing the per-frame settling check.
- `crates/ui-runtime/src/reduced_motion.rs` — `use_reduced_motion` now
  probes `window.matchMedia("(prefers-reduced-motion: reduce)")` on
  wasm, AND consults the nearest `[data-ui-motion]` attribute on a DOM
  ancestor (so the gallery's preference bar takes effect). A new
  `ReducedMotionProvider` Dioxus component is exported so root layouts
  can override the probe.
- `crates/ui-motion/src/lib.rs` — adds `Transition::to_keyframes(from,
  to, fps)` that returns a `Vec<Keyframe>` newtype suitable for WAAPI
  consumption. Springs sample at 60fps × `settling_duration_ms`; tweens
  emit a 2-frame keyframe set with a `cubic-bezier(...)` easing string.
- `crates/ui-dioxus/src/kinetics.rs` — `KineticBox`, `Sequence`,
  `TimelineScope`, `Presence` consume the new hooks. The components'
  public prop API does not change.
- `crates/ui-styles/src/lib.rs` — drops the static
  `.ui-kinetic-box[data-motion-cue=…]` CSS keyframes (now provided
  dynamically by WAAPI) and replaces them with a `@media (prefers-reduced-motion: reduce)`
  fallback for the case where WAAPI is unavailable (very old browsers
  /  no-JS SSR snapshots).
- `examples/component-gallery/src/controls.rs` — the
  `PreferenceBar` now provides a `ReducedMotion` context for its
  children, sourced from the preference signal.
- `examples/component-gallery/src/previews/motion.rs` — the
  `TimelineScope` preview's stagger variant gets `autoplay: true` so
  the scrub frame's manual clock is no longer required (the autoplay
  RAF inside `TimelineScope` is itself a WAAPI animation after this
  spec).
- `examples/component-gallery/src/previews/composition.rs` — the
  `FrameStage` preview's frame caption is moved into a single element
  so the Playwright strict-mode locator collision goes away (test bug
  surfaced something a sighted user wouldn't notice but the audit
  flagged honestly).
- `examples/component-gallery/e2e/reporters/audit-report.ts` — the
  variant classifier is fixed: instead of scraping the test title for
  `@variant`, it consults `test.titlePath()` and recognises the bespoke
  specs' `"reduced motion"` body text. The reporter's row keying also
  switches from `(name, layer, variant)` to `(name, layer, variant,
  testTitle)` so two tests in the same `(name, layer, variant)` cell
  no longer overwrite each other; the worse-of-N rollup happens in
  `renderTable`.
- `examples/component-gallery/e2e/tests/components/frame-stage.spec.ts`,
  `timeline-scope.spec.ts`, `kinetic-box.spec.ts`,
  `sequence.spec.ts`, `presence.spec.ts` — selector tightenings to
  match the post-fix DOM. The motion contracts they assert remain the
  same; the assertions were correct, the gallery is now matching them.
- `examples/component-gallery/e2e/tests/_lib/mount.ts` — the
  `selectRadio` helper switches from a `getByRole("radio").click()`
  call to a `page.evaluate` that dispatches `input` / `change` on the
  underlying `<input type="radio">`. This sidesteps the WebKit pointer
  interception issue and makes the preference-bar interaction
  deterministic across both browsers.

It excludes:

- Any change to `crates/ui-glass-engine` or `crates/ui-glass-dioxus`.
  Liquid Glass already has its own GPU path (wgpu); this spec is about
  the *motion* engine, not the *glass* engine.
- Any change to `crates/ui-composition` / `FrameStage`'s internal
  frame-counting model. Only the gallery's *preview* of FrameStage is
  touched (single-element caption).
- Any change to `Spring`'s math or the `ui-motion` numerical API
  beyond adding `to_keyframes`. Existing call sites in `ui-runtime`
  continue to compile.

## Non-Goals

- WAAPI is not exposed as a public API. `Element.animate(...)` calls
  happen inside `ui-runtime` only. Components consume the same hooks
  (`use_animation_value`, `use_timeline_sample`, etc.) as before. The
  migration is internal.
- No new motion primitives. The cue families (`Opacity`, `Translate`,
  `Scale`, `Rotate`) and the `Transition` enum stay as they are. Only
  the runtime that consumes them changes.
- No SSR change. SSR continues to emit the settled-state inline style
  by sampling the timeline at `Manual { elapsed_ms: 0 }` (or
  `duration_ms` under reduced motion). WAAPI runs only after hydration.
- No reduced-motion *opt-out* per component. If
  `prefers-reduced-motion: reduce` is set globally, every animation in
  the page collapses. Per-component opt-outs would require new props
  on every motion component; out of scope.
- No animation-event observability (`Animation.onfinish` is consumed
  internally; not surfaced as a callback prop). A future spec can add
  `on_enter_done` / `on_exit_done` props to `Presence` if user feedback
  demands it.

## Architecture Overview

The runtime stack splits into three layers:

```
┌─────────────────────────────────────────────────────────────────┐
│  Components (KineticBox, Sequence, Presence, TimelineScope ...) │
│  — declare motion intent via props + cues                        │
└─────────────────────────┬───────────────────────────────────────┘
                          │ ui_motion::Transition, ui_timeline::Cue
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│  Hooks (ui-runtime)                                              │
│  use_animation_value, use_presence_animation, use_timeline_sample│
│  — Translate intent to a WAAPI Animation handle                  │
└─────────────────────────┬───────────────────────────────────────┘
                          │ web_sys::Element.animate(keyframes, opts)
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│  Browser compositor                                              │
│  — runs the animation off the main thread                        │
└─────────────────────────────────────────────────────────────────┘
```

The middle layer is the thinnest. It owns:

- A `WaapiHandle` newtype around `web_sys::Animation` that cancels on
  drop (so a re-render with a new target doesn't leak a paused
  animation behind the current one).
- A small `keyframes_for_transition(from, to, transition)` function
  that lives in `ui_motion` and returns a `Keyframes` value (a
  `Vec<HashMap<&'static str, String>>` newtype) plus a
  `KeyframeAnimationOptions`-like struct for duration + easing.
- A `MountedHandle` lookup that takes a Dioxus `onmounted` event,
  extracts the `web_sys::Element`, and stores it under a stable key so
  the hook can find it on next signal change without going through
  `document.getElementById`.

The Manual / Frame clock paths in `use_timeline_sample` keep emitting a
synchronous `inline_style()` string into the `SequenceContext` (no
WAAPI involved). Scrubbing a 6-second timeline at 60fps through WAAPI
would require setting `Animation.currentTime` every input event, which
both leaks the slider's debounce semantics into the runtime and is no
faster than the existing direct-style write. Scrub stays
synchronous; *playback* goes through WAAPI.

The Playback clock path is the interesting one. Today
`use_timeline_sample` spawns a RAF loop, evaluates `Timeline.sample(...)`
each tick, and writes a signal that propagates a style string down to
each `KineticBox`. After this spec:

1. On the first render with `Playback`, the hook computes one
   `WaapiAnimation` per timeline track (each track has one target
   element identified by `MotionTarget::Node`).
2. For each track, `Transition::to_keyframes(from, to, fps)` produces
   the keyframe array. Track start offset becomes the WAAPI animation's
   `delay`. Track duration becomes the WAAPI `duration`. Easing is
   either a `cubic-bezier(...)` string (tweens) or the keyframes
   themselves carry the spring-sampled values with `easing: "linear"`.
3. The animation is created via `element.animate(keyframes,
   { duration, delay, easing, fill: "forwards", iterations: 1 })`. The
   returned `Animation` handle is stored on the runtime so it can be
   cancelled on cleanup.
4. The animation runs entirely on the compositor. The Rust side does
   not tick. The signal still exists but only updates on completion
   (via the `finish` event) so timeline-driven callbacks fire at the
   right moment.

The `use_animation_value` hook (driving single-value transitions for
`Presence` enter/exit and `Switch` thumb position etc.) follows the same
pattern but for a single keyframe set.

## Reduced-Motion Wiring

A `ReducedMotionProvider` Dioxus component is added to `ui-runtime`:

```rust
#[component]
pub fn ReducedMotionProvider(children: Element) -> Element {
    let reduced = use_signal(|| detect_reduced_motion_at_root());

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        // Listen to prefers-reduced-motion changes + the
        // [data-ui-motion="reduced"] attribute walk-up.
        // Update the signal when either changes.
    });

    use_context_provider(|| ReducedMotion(*reduced.read()));

    rsx! { {children} }
}
```

`detect_reduced_motion_at_root()` returns true if either:

- `window.matchMedia("(prefers-reduced-motion: reduce)").matches` is
  true, **or**
- The body element (or its closest ancestor with a `data-ui-motion`
  attribute) has `data-ui-motion="reduced"`.

Components opt in by wrapping the application root in
`ReducedMotionProvider`. The gallery does this once in `app.rs`. Apps
that already pass a `ReducedMotion(true)` via context (testing,
storybook etc.) override the provider; the provider only sets the
context when a parent hasn't already.

`use_reduced_motion()` keeps its current signature. The implementation
falls back to a one-shot `detect_reduced_motion_at_root()` if no
context is present, so unwrapped apps still degrade correctly under
prefers-reduced-motion (just without reactivity).

## Spring → Keyframe Sampling

WAAPI does not know what a spring is. The lossy-but-correct migration:

```rust
fn spring_keyframes(from: f32, to: f32, spring: Spring) -> Keyframes {
    let settle = spring.settling_duration_ms(0.005).clamp(50.0, 4_000.0);
    let frames = (settle * 0.06).ceil() as usize; // ~60fps
    let mut value = from;
    let mut velocity = 0.0;
    let mut out = Vec::with_capacity(frames + 1);
    out.push(keyframe(from));
    let dt = 1.0 / 60.0;
    for _ in 0..frames {
        let step = spring.step(value, to, velocity, dt);
        value = step.value;
        velocity = step.velocity;
        out.push(keyframe(value));
    }
    out
}
```

The keyframe array is then attached to a WAAPI animation with
`easing: "linear"` and `duration: settle_ms`. WAAPI interpolates linearly
between the spring-sampled points; visual fidelity is high because the
sampling is dense.

Cached: once computed for a given `(Spring, from, to)` triple, the
keyframes are reused (the same `Spring::snappy()` enter from 0 → 1
recurs across every Presence enter on the page). The cache is bounded
to 64 entries with LRU eviction.

Tween transitions are NOT mapped to WAAPI's `cubic-bezier(...)` easing
string. Although Material Design's "standard easing"
(`cubic-bezier(0.4, 0.0, 0.2, 1)`) is visually similar, it is
mathematically distinct from `ui_motion::Ease::Standard`, which is the
smoothstep function `p² · (3 − 2p)`. Substituting one for the other
would silently change motion across the app. Instead, tweens are also
sampled into a keyframe array — at a fixed 30fps × `duration_ms` (so a
220ms tween yields 7 keyframes plus the endpoints) — and attached with
`easing: "linear"`. This keeps the smoothstep curve bit-exact with
the Rust-side `apply_ease` math, at the cost of a few extra keyframes
per tween. `Ease::Linear` is a 2-frame keyframe set with
`easing: "linear"`.

## Reduced-Motion Behavior at the Runtime

When `use_reduced_motion()` returns true:

- `use_animation_value`: skip the WAAPI call; set the signal value to
  the target immediately.
- `use_presence_animation`: skip the enter/exit animation; transition
  the state machine synchronously.
- `use_timeline_sample(Playback)`: behave as if the clock were
  `Manual { elapsed_ms: timeline.duration_ms }` — emit the settled
  state once, never animate.
- `use_timeline_sample(Manual)`: unchanged — the user is scrubbing
  manually, the runtime respects their explicit clock.

The change cascades into the gallery: with the preference-bar fix +
the new provider, toggling Reduced now collapses motion across the
entire page deterministically. The Spec 1 audit's reduced-motion
variant tests will pass.

## Data Flow

```
Component renders
  └─ use_animation_value(target, transition)
       └─ if reduced_motion
            └─ signal.set(target); return
       └─ keyframes = ui_motion::keyframes_for_transition(...)
       └─ on next mounted callback:
            element.animate(keyframes, options)
            store handle in hook state
       └─ on signal change (target moves):
            handle.cancel()
            recompute keyframes
            element.animate(...) again
       └─ on Animation.finish event:
            signal.set(target)  // canonical Rust-side state
```

The signal still mirrors the Rust-side animation value (start, target)
so SSR + tests that read the inline style still see a final state.
The signal does **not** tick per frame; instead the browser
compositor owns the in-flight interpolation.

## Browser Support

WAAPI is supported in:

- Chromium 84+ (2020) — all features used here.
- Firefox 75+ (2020).
- Safari 13.1+ (2020), with `KeyframeEffect.composite` quirks that we
  do not rely on.

For older browsers (WebView shells with browser engines pinned below
those versions), the runtime falls back to the existing RAF + inline
style behavior. The fallback path is selected at module init time via
`typeof Element.prototype.animate === 'function'`. Today the gallery's
CI matrix (Chromium 1223 + WebKit 26.4) both support WAAPI; the
fallback is for defensive runtime detection only.

## Audit-Finding Fixes (rolled into this spec)

| Finding | Fix |
|---|---|
| `use_reduced_motion()` returns false even when the gallery sets Reduced | `ReducedMotionProvider` wired in `app.rs`; provider probes media query + data-attr ancestor |
| Sequence test 1: t=0 opacity not at `from` value | Manual clock keeps the synchronous-sample path, so this is NOT a side effect of WAAPI. Plan task debugs `Timeline::sample(Manual { elapsed_ms: 0 })` for the FillMode::Both first-cue case; expected fix is in `ui_timeline::Timeline::sample` (a cue with `start_ms == 0` and the current FillMode::Both still emits the `to` value at t=0). Repro is a unit test in `crates/ui-timeline`, fix is in the same crate |
| Sequence test 2 (reduced motion) | Resolved by the reduced-motion wiring fix |
| FrameStage strict-mode locator collision | `previews/composition.rs` collapses the label + frame counter into a single `<p>` element |
| KineticBox ReplayFrame doesn't drive inline styles under Playwright clock | After WAAPI migration, the animation no longer depends on Playwright's clock at all — `Animation.currentTime` is owned by the browser, and Playwright's `page.clock` fastForward correctly advances WAAPI's time origin |
| Reporter classifier overwrites motion rows | Switch to `(name, layer, variant, testTitle)` keying; classifier recognises `"reduced motion"` body text |
| WebKit pointer interception on Presence Replay button | `mount.ts` `selectRadio` dispatches input/change directly instead of click; gallery's variant tiles get `pointer-events: none` on the prose nodes so the Replay button is reachable |
| TimelineScope stagger preview tiles don't animate at t=0 | Set `autoplay: true` in the preview; the post-spec autoplay path goes through WAAPI so it's deterministic |
| Win32 baselines on Linux CI | A new `e2e:rebaseline-linux` npm script + CI workflow step regenerates baselines on the ubuntu runner and uploads as an artifact for review; first post-merge PR commits the Linux baselines |

## Error Handling

- WAAPI feature detection is checked once at runtime init. If absent,
  the entire `ui-runtime` module flips into the legacy RAF path. The
  same hook signatures work; behavior degrades but does not crash.
- `Element.animate()` throws on invalid keyframes. The Rust side
  catches the JS exception, logs to `console.warn`, and falls back to
  the legacy path for that one hook call (per-call, not module-wide).
- `Animation.finish` is debounced: if the user's target signal moves
  again before the previous animation finishes, the new animation is
  played and the old `finish` listener is detached. Stale `finish`
  events cannot incorrectly mark the new animation as settled.
- `ReducedMotionProvider` listens to `MediaQueryList.onchange`. When
  the user toggles reduced-motion mid-session, in-flight animations
  are cancelled and their settled state is committed. No flicker (the
  cancelled animation's `fill: forwards` keeps the last computed
  frame visible).

## Testing

- `ui-motion::keyframes_for_transition` is pure and unit-tested:
  - Tween → 2 keyframes with the correct cubic-bezier easing string.
  - Spring → N keyframes with the first frame == from, last frame
    within `0.005` of to.
  - Reduced sampler edge cases (Spring with zero stiffness, NaN
    inputs).
- `ui-runtime::reduced_motion::detect_reduced_motion_at_root` gains a
  wasm-bindgen test that mocks `matchMedia` and the data-attr ancestor.
- The Playwright suite's motion specs (`sequence.spec.ts`,
  `kinetic-box.spec.ts`, `presence.spec.ts`, `timeline-scope.spec.ts`,
  `frame-stage.spec.ts`) are re-run end-to-end. Pass criterion: every
  motion test, both default and reduced-motion variants, passes on
  Chromium static. WebKit pointer-interception tests pass after the
  `selectRadio` helper fix.
- A new `e2e/tests/_lib/__tests__/reporter.test.ts` case covers the
  classifier's new `testTitle` keying so the `Sequence::motion::default`
  cell no longer gets overwritten by the reduced-motion test.
- A new SSR snapshot test (`crates/ui-dioxus/tests/sequence_ssr.rs`,
  extended) confirms that under simulated reduced-motion (a
  `ReducedMotion(true)` context), the rendered HTML carries the
  settled-state inline style — proves the SSR contract didn't regress.

## Risks And Mitigations

- **Spring fidelity loss.** Pre-sampled keyframes lose continuous spring
  damping if the target changes mid-animation. Mitigation: on target
  change, `Animation.cancel()` is called and a fresh keyframe set is
  computed from the *current* animated value (read via
  `getComputedStyle` or `Animation.currentTime`), not the original
  `from`. The cost is one read per change; the benefit is correct
  velocity continuity.
- **WAAPI cancellation race.** Cancelling a WAAPI animation can race
  with a pending `finish` event. Mitigation: listeners check a
  generation counter on the runtime; only events whose generation
  matches the latest animation are honored.
- **Test flake from compositor timing.** WAAPI runs on a separate thread,
  so the moment the Rust side dispatches `play()` is not the moment
  the first frame paints. Mitigation: Playwright tests use
  `await page.waitForFunction(() => animation.playState === 'running')`
  before sampling computed styles. The `clock.ts` helper grows a
  `waitForAnimation` utility.
- **Reduced-motion fallback for SSR.** Server-rendered HTML never knows
  the client's `prefers-reduced-motion`. Mitigation: SSR emits the
  pre-animation state (current behavior); the client either runs the
  animation or, if reduced-motion is detected after hydration, snaps
  to settled via `Animation.finish()` synchronously. This matches the
  existing contract.
- **Performance regression on low-end devices.** WAAPI is generally
  faster than RAF + JS interpolation, but the per-animation handle
  setup cost is non-zero. Mitigation: profile on a representative
  low-end profile (CPU 4× throttled) before merge; if the per-call
  cost exceeds 1ms, batch-create animations with `KeyframeEffect`
  reuse.

## Implementation Phasing

The plan (separate document) will stage:

1. **Reduced-motion wiring** — small, lands first as a pure bugfix.
   Re-runs the Spec 1 audit and surfaces the now-passing reduced-motion
   tests. No WAAPI yet.
2. **`ui-motion::keyframes_for_transition`** — pure addition, no
   runtime change yet. Unit-tested.
3. **WAAPI hook bindings** — `use_animation_value` switches to WAAPI
   under the hood. `Presence` and `Switch` are the first consumers;
   their motion tests run.
4. **Timeline playback via WAAPI** — `use_timeline_sample(Playback)`
   migrates. `Sequence` Playback test passes.
5. **Manual clock preservation** — explicit non-migration of
   `Manual` / `Frame` clocks. Scrub previews continue to work.
6. **Gallery wiring** — `ReducedMotionProvider` in `app.rs`; preference
   bar feeds the provider; preview FrameStage and TimelineScope are
   adjusted; CSS keyframes are removed.
7. **Reporter classifier** — fix the row keying + variant detection.
8. **Test selector tightenings** — update the 5 motion specs to
   match the post-fix DOM.
9. **WebKit `selectRadio`** — switch from click to dispatchEvent.
10. **Linux baseline bootstrap** — new CI workflow step + first commit
    of `*-linux.png` baselines.

Steps 1–6 are sequential. 7–10 can land in parallel.

The audit-report after this spec ships should show every component as
`ready` across all four variants, with the WAAPI path documented as
the active production runtime.

## Out Of Scope (carries to Spec 3 / Spec 4)

- View Transitions API for shared layout transitions.
- Scroll-driven animations (`@scroll-timeline` / `@view-timeline`).
- Frame-rate budgeting and performance benchmarks.
- Per-component reduced-motion opt-outs.
- Animation worklet integration.
- New cue families (color-interpolate, path-along, etc.).

The connection between this spec and a future Spec 3 (View Transitions)
is the `MountedHandle` lookup added here — Spec 3 will reuse it to
snapshot bounding boxes before the layout swap and animate the deltas
through the same WAAPI handle code path.
