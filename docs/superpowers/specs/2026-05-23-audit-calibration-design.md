# Audit Calibration Design

## Goal

Bring the component-gallery audit from 266/268 to 268/268 (100% on
Chromium static) by tightening the two motion@default specs that the
Spec 3 WAAPI migration left out of sync with the new compositor-driven
runtime. The production runtime itself is correct; the assertions are
reading the wrong observable.

This is a deliberately small spec — the original two-spec initiative
plus its two cleanup follow-ups (Spec 2, Spec 3) is complete and
shipped. Spec 4 exists only to close the residual test-side
calibration before the next major capability (View Transitions API,
deferred to Spec 5).

## Scope

This spec changes:

- `examples/component-gallery/e2e/tests/components/kinetic-box.spec.ts`
  — the "each of the three cues drives an inline style under a clock"
  test currently reads `getComputedStyle` at `clock.tickMs(400)`. The
  cue durations are ~220ms, so by t=400ms the animation has already
  reached its `to` value and `getComputedStyle` returns the settled
  state. The fix samples at t=110ms (mid-animation) and also
  asserts `Animation.playState` to prove the animation actually ran.
- `examples/component-gallery/e2e/tests/components/timeline-scope.spec.ts`
  — the "stagger variant tiles animate in across the scrub window"
  test asserts opacity > 0.95 at t=1200ms. Under the post-Spec-3
  autoplay path the stagger tiles are driven by WAAPI animations on
  the compositor, not by the `ScrubFrame`'s elapsed-ms signal. The
  scrub slider does not advance the compositor clock. The fix removes
  the scrubTo + opacity-at-1200ms assertion and instead reads
  `Animation.currentTime` or the rendered `opacity` after a
  `page.waitForFunction(...)` that polls until both tiles have settled.

It excludes:

- Any change to production code. The runtime is correct.
- Any new component coverage. The audit's matrix is stable.
- View Transitions API for `SharedElement`/`SharedLayout` — deferred to
  Spec 5.
- Scroll-driven animations — deferred to Spec 6.
- Cross-browser baseline regeneration on Linux — already documented
  in Spec 1's README as a one-time CI bootstrap.

## Non-Goals

- No helper-library API changes. The `installClock` / `readStyles`
  helpers stay as they are; the fix is local to the two affected
  spec files.
- No reporter changes. The classifier and aggregation are stable.
- No re-baselining of visual snapshots.

## Architecture Overview

Both failures share the same root cause: the Spec 1 motion specs
were written against the pre-WAAPI inline-style world, where the
Rust-side RAF loop wrote `style="opacity: 0.3"` per frame. Spec 3
moved the in-flight interpolation onto the compositor, so the inline
`style` attribute now reads as the start state (until the animation
finishes and `fill: forwards` commits the end state). Mid-flight
values are only observable via `getComputedStyle` OR via the WAAPI
`Animation` object's `currentTime` / `playState`.

The two specs need two different sample strategies:

- **KineticBox**: the animation runs on mount immediately (no scrub
  driver, no manual clock). The current test installs a `page.clock`,
  clicks the ReplayFrame button, advances 400ms, and reads. Because
  the cues are ~220ms tweens, the animation completes by ~250ms and
  the sampled state at 400ms is fully settled. The fix samples
  earlier (110ms — half the cue duration) AND asserts the WAAPI
  animation's `playState === "finished"` at the original 400ms point
  to prove it actually ran. Together these assertions verify
  "in-flight value differs from start" + "animation reached completion".
- **TimelineScope**: the stagger tiles autoplay on mount under WAAPI.
  Playwright's `page.clock.install()` does NOT advance the
  compositor's animation clock — the WAAPI runtime uses the
  high-resolution `document.timeline`, which is separate from the
  patched `performance.now()` / `Date.now()` clocks. The fix replaces
  the scrub-based assertion with a `page.waitForFunction(...)` that
  polls until the tiles' `getComputedStyle(...).opacity` equals "1"
  (animation settled). Timeout is generous (3s) to accommodate the
  stagger delays.

## Data Flow

For KineticBox (per cue tile):

```
Test starts → installClock → clickReplay → wait for next render
  → tickMs(110)
  → getComputedStyle(el).opacity → in-flight value (not 0, not 1)
  → assert in-flight
  → tickMs(290)  // total 400ms; cue ~220ms, so finished
  → page.evaluate to grab el.getAnimations()[0].playState
  → assert playState === "finished"
```

For TimelineScope stagger:

```
Test starts → mountGallery → page.waitForFunction(() => {
  const tile0 = document.querySelector('[data-stagger-index="0"] [data-kinetic-id="stagger-0"]');
  const tile3 = document.querySelector('[data-stagger-index="3"] [data-kinetic-id="stagger-3"]');
  if (!tile0 || !tile3) return false;
  const o0 = parseFloat(getComputedStyle(tile0).opacity);
  const o3 = parseFloat(getComputedStyle(tile3).opacity);
  return o0 > 0.95 && o3 > 0.95;
}, { timeout: 3000 })
```

If the wait times out, the test fails with a clear diagnostic and the
audit reports the real regression (autoplay not progressing). If it
resolves, the contract — "the stagger animation reaches settled
state" — is verified.

## Error Handling

- The `waitForFunction` is bounded by a 3-second timeout. If the
  animation legitimately fails to progress (production bug), the test
  fails. If the timeout is hit due to slow CI hardware, the test
  fails too — but the 3-second window is generous (the longest
  stagger tile starts at 900ms × stagger and runs ~400ms after that,
  ~1.5s total).
- The KineticBox `getAnimations()` call returns the array of active
  Web Animations on the element. If no animation is present (WAAPI
  not supported, or KineticBox rendered outside a Sequence), the
  array is empty and the assertion fails with a clear message.

## Testing

- The two affected specs are re-run locally:
  ```bash
  cd examples/component-gallery/e2e
  npm run e2e -- tests/components/kinetic-box.spec.ts tests/components/timeline-scope.spec.ts --project=static
  ```
  Expected: 4/4 pass (2 tests × the two affected specs each have one
  motion@default test plus a reduced-motion variant).

- A full audit re-run regenerates `audit-report.md`. Target: 268/268
  pass.

## Risks And Mitigations

- **`getAnimations()` may not be defined on the element in some
  browsers.** Chromium 84+ supports it; same minimum as WAAPI itself.
  Safe.
- **`waitForFunction` is a soft contract.** If the gallery preview
  changes the data attributes the wait selector looks for, the test
  will time out silently and look like a regression. Mitigation: the
  test's error message includes the locator string and the last
  observed opacity values, so a real change in the gallery is easy to
  diagnose.
- **`getAnimations()[0]` is order-dependent.** A KineticBox with both
  opacity and transform cues would have two animations; the first one
  may not be the one we asserted progressed. Mitigation: the test
  asserts `getAnimations().length > 0 && all finished`, not specifically
  the first.

## Implementation Phasing

The plan (separate document) will stage:

1. **KineticBox motion test fix** — replace the t=400ms read with a
   t=110ms in-flight check + `getAnimations()` playState assertion.
2. **TimelineScope motion test fix** — replace the scrub-based
   assertion with `waitForFunction` on rendered opacity.
3. **Audit re-run + report regen** — confirm 268/268.

Three tasks total. All TypeScript-only. No Rust changes.

## Out Of Scope (Spec 5+)

- View Transitions API for SharedElement/SharedLayout.
- Scroll-driven animations.
- Frame-rate budgeting.
- Cross-browser Linux baselines (deferred to first CI run after merge).
- Per-component reduced-motion opt-outs.

The next spec opens new motion capabilities. This one only closes the
audit.
