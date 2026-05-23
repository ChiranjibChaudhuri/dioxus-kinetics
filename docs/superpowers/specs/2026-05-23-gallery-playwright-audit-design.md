# Gallery Playwright Audit Design

## Goal

Land a Playwright-based end-to-end harness for the component gallery
that catches the regressions visitors are currently hitting — buttons
that do not respond, motion previews that never animate, glass surfaces
that render flat — and that emits a structured defect catalog. The
catalog becomes the input to the follow-on spec
(*Advanced motion + GPU acceleration*).

The harness covers three layers, applied in this order to every gallery
entry:

1. **Interaction smoke** — every clickable control responds, every
   preview renders, no `console.error` fires during hydration or the
   first 2 s of idle.
2. **Motion contract** — animations actually progress. `ScrubFrame`
   advances inline `transform`/`opacity` style values as its range
   input is driven from `0` → `duration_ms`. `ReplayFrame` advances
   the same values under a fixed `page.clock` schedule. Reduced motion
   collapses to the settled state.
3. **Visual regression** — pixel snapshots at named states for each
   preview tile, baselined per browser.

The harness does not attempt to *fix* anything. Its only job is to make
brokenness measurable and to keep future fixes from regressing.

## Scope

This spec lands:

- A new `examples/component-gallery/e2e/` Node project containing
  `package.json`, `playwright.config.ts`, `tsconfig.json`, a `tests/`
  tree, a `fixtures/` tree, and `__snapshots__/`.
- Two Playwright `projects` configurations:
  - `static` — runs against a `dx build --release --package
    component-gallery` artifact, served by Playwright's `webServer`
    block via a small static file server. Used by CI and by visual
    regression.
  - `dev-loop` — runs against `dx serve --package component-gallery`
    on `http://localhost:9173`, with the hot-reload WebSocket noise
    explicitly silenced. Used locally when validating the dev
    experience itself. Not run in CI.
- A `globalSetup` step that, for the `static` project, invokes
  `dx build --release --package component-gallery` and verifies a
  fresh `dist/` exists before the test run starts.
- A small `tests/_lib/` helper module exposing:
  - `mountGallery(page, { variant })` — navigates to the gallery,
    applies one of `default | dark | reduced-motion | solid-glass`
    via Playwright media-emulation + the gallery's preference bar,
    waits for `gallery-shell` to be visible.
  - `scrubTo(page, frameLocator, ms)` — drives the `ScrubFrame`
    range input by setting `value` and dispatching `input`, returns
    the parsed elapsed value from the `.gallery-demo-frame-elapsed`
    span.
  - `installClock(page)` — calls `page.clock.install({ time: 0 })`
    and exposes a `tickMs(n)` helper that wraps
    `page.clock.fastForward(n)`.
  - `readStyles(locator, props)` — reads inline `style="..."`
    attribute (not computed style) and parses `transform`,
    `opacity`, `--ui-presence-t`, etc. into a flat object.
  - `expectNoConsoleErrors(page)` — installs a console listener that
    fails the test on any non-allowlisted error (the dev-loop
    WebSocket reconnect message is allowlisted in the `dev-loop`
    project only).
- A discovery-driven **smoke** suite at `tests/smoke.spec.ts` that
  reads the gallery's component registry through the rendered DOM
  (every `article.gallery-entry`) and, for each entry, asserts:
  - the entry's `h4` matches a known component name from a checked-in
    `tests/_lib/component-manifest.ts`,
  - the entry has either a `.gallery-preview--ready` with at least
    one rendered child or a `.gallery-preview--soon` placeholder,
  - the entry's `pre.gallery-code` is non-empty,
  - no `console.error` fires while the entry scrolls into view.
- **Bespoke** suites under `tests/components/` — one file per ready
  component, named after the doc entry (`button.spec.ts`,
  `dialog.spec.ts`, `toast.spec.ts`, `tooltip.spec.ts`,
  `command-menu.spec.ts`, `tabs.spec.ts`, `switch.spec.ts`,
  `checkbox.spec.ts`, `text-field.spec.ts`, `sidebar.spec.ts`,
  `toolbar.spec.ts`, `metric-card.spec.ts`, `empty-state.spec.ts`,
  `presence.spec.ts`, `presence-gate.spec.ts`, `kinetic-box.spec.ts`,
  `sequence.spec.ts`, `timeline-scope.spec.ts`, `shared-element.spec.ts`,
  `shared-layout.spec.ts`, `frame-stage.spec.ts`,
  `capture-stage.spec.ts`, `glass-surface.spec.ts`,
  `glass-layer.spec.ts`, `liquid-surface.spec.ts`,
  `icon-button.spec.ts`, `stack.spec.ts`, `surface.spec.ts`).
- A **visual-regression** suite at `tests/visual.spec.ts` that, for
  every ready entry, screenshots its `.gallery-preview` element across
  the four named variants and stores the baselines in
  `__snapshots__/<entry>/<variant>-<browser>.png`.
- A custom Playwright reporter at
  `e2e/reporters/audit-report.ts` that, regardless of pass/fail, emits
  `e2e/audit-report.md` summarising every component across the three
  layers and four variants in a single table. CI uploads this file as
  a build artifact.
- A `.github/workflows/e2e.yml` running the `static` project on
  Chromium + WebKit on PRs that touch `examples/component-gallery/**`
  or `crates/ui-*/**`. The workflow caches `~/.cache/ms-playwright`
  and `examples/component-gallery/e2e/node_modules`.
- A `README.md` at `examples/component-gallery/e2e/README.md`
  documenting `npm install`, `npm run e2e`, `npm run e2e:dev-loop`,
  `npm run e2e:update`, and how to interpret `audit-report.md`.
- One `.gitignore` line scoping ignores to
  `examples/component-gallery/e2e/{node_modules,test-results,playwright-report}/`.

It excludes:

- Any fix to a broken animation, broken interaction, or broken glass
  surface. The harness catalogs; it does not repair. Repairs belong to
  Spec 2.
- Any refactor of `ui-runtime`, `ui-motion`, `ui-glass-engine`, or the
  gallery's preview modules. They are read by the harness, not edited.
- Firefox coverage. Added later if WebKit reveals divergence patterns
  worth triangulating.
- Mobile viewport coverage beyond what `CaptureStage` already exposes
  as a static preview. Cross-device emulation is a later spec.
- Accessibility audits (`axe-core`, keyboard traversal trees). The
  smoke layer only asserts that interactive controls *exist* and
  *respond*, not that they meet WCAG.
- Performance budgets (frame-rate, long-task counts). A later spec
  layered on top of this harness can hook into the same `static`
  project.

## Non-Goals

- A general-purpose Rust → Playwright bridge. The `playwright` crate is
  out of scope; this harness is a Node project that lives next to the
  Cargo workspace but does not feed back into Cargo.
- Replacing the existing `dioxus-ssr`-based tests at
  `examples/component-gallery/tests/`. Those validate the registry,
  the rendered HTML, and the SSR contract; they remain authoritative
  for things that do not depend on a running browser.
- Visual regression on `LiquidSurface` rendered through WebGPU. WebGPU
  output is non-deterministic across drivers; `LiquidSurface` is
  visually asserted via its CSS fallback path (forced by selecting
  the `Solid` glass policy in the preference bar, which puts
  `[data-ui-glass-policy="solid"]` on the gallery shell and routes
  the surface through the SVG-filter / solid fallback) and
  behaviourally asserted via canvas attribute checks (`width`,
  `height`, presence of a `<canvas>` child).
- A snapshot-update bot. `npm run e2e:update` stays manual.

## Architecture Overview

The harness is a self-contained Node project rooted at
`examples/component-gallery/e2e/`. It has no compile-time coupling to
the Cargo workspace; the only runtime coupling is the `globalSetup`
shelling out to `dx build` (or the user-side `dx serve` for the
`dev-loop` project). All inputs the harness needs from Rust code — the
list of ready components, expected motion cue names, expected scrub
durations — live in `tests/_lib/component-manifest.ts`, a hand-authored
TypeScript constant kept in sync with `examples/component-gallery/src/docs.rs`.
A small Cargo test at `examples/component-gallery/tests/manifest.rs`
asserts that every `ComponentStatus::Ready` entry in `docs.rs` appears
in the TS manifest by name, so drift is caught at `cargo test` time, on
the Rust side, before the Node project even runs.

The three audit layers map to three suites:

- **Smoke** (`tests/smoke.spec.ts`) iterates the manifest, walks each
  `article.gallery-entry` in DOM order, exercises any obvious
  interaction (`button[type=button]`, `[role=tab]`, `[role=switch]`,
  `input[type=range]`, etc.), and checks the console stream. The
  smoke suite runs against all four variants.
- **Motion contract** (`tests/components/*.spec.ts`, motion-relevant
  files) drives the deterministic `ScrubFrame` for sequence- and
  timeline-style components and the clock-controlled `ReplayFrame`
  for presence- and kinetic-box-style components. Each test reads
  inline-style values (`transform`, `opacity`, CSS custom properties
  set on `KineticBox` like `--ui-presence-t`) at named timestamps and
  asserts they fall inside an expected tolerance band derived from
  the `ui-motion` math (the same `Spring::settling_duration_ms` and
  `apply_ease` functions that drive production). Reduced-motion
  variants assert that the same reads return the settled value at
  t=0.
- **Visual regression** (`tests/visual.spec.ts`) screenshots each
  preview tile in each variant after the motion suite has settled the
  page. WebGPU surfaces are masked with
  `expect.toHaveScreenshot({ mask: [...] })` so driver-level pixel
  noise does not flake the suite.

The custom reporter consumes the standard Playwright test events
(`onTestEnd`) and groups results by `(component, layer, variant)` into
a single Markdown table written at the end of the run. Failures and
flakes are summarised next to each row; the report is generated even
when tests fail, so the catalog is the deliverable regardless of CI
status.

`dx build` produces a static `dist/` rooted at
`examples/component-gallery/dist/`. The `static` project's
`webServer.command` runs a tiny static server
(`npx http-server dist -p 4173 --silent`) and waits for the port; the
harness then visits `http://localhost:4173`. The `dev-loop` project
points at `http://localhost:9173` and assumes the user (or an `npm run
e2e:dev-loop` script) has `dx serve` already running.

## Component Coverage Matrix

Every entry in `component_docs()` with `ComponentStatus::Ready` gets at
least the smoke layer. Motion and visual layers are scoped by
relevance:

| Component        | Smoke | Motion | Visual | Notes                                                  |
|------------------|-------|--------|--------|--------------------------------------------------------|
| Button           | yes   | no     | yes    | Hover and active states snapshotted; no motion.        |
| IconButton       | yes   | no     | yes    | 3×3 tone × size matrix snapshotted.                    |
| Surface          | yes   | no     | yes    | Snapshot only.                                         |
| GlassSurface     | yes   | no     | yes    | Snapshot the SVG-filter fallback path.                 |
| GlassLayer       | yes   | no     | yes    | 3×3 tone × level grid.                                 |
| LiquidSurface    | yes   | no     | yes    | Visual via forced SVG-filter fallback; canvas attrs.   |
| Stack            | yes   | no     | yes    | Snapshot at three densities.                           |
| TextField        | yes   | no     | yes    | Snapshot default + focused + disabled.                 |
| Checkbox         | yes   | no     | yes    | Snapshot unchecked + checked + indeterminate.          |
| Switch           | yes   | yes    | yes    | Motion: thumb transform on toggle.                     |
| Tabs             | yes   | yes    | yes    | Motion: indicator slide between tabs.                  |
| Sidebar          | yes   | yes    | yes    | Motion: collapse/expand.                               |
| Toolbar          | yes   | no     | yes    | Snapshot only.                                         |
| MetricCard       | yes   | no     | yes    | Snapshot only.                                         |
| EmptyState       | yes   | no     | yes    | Snapshot only.                                         |
| Dialog           | yes   | yes    | yes    | Motion: open enter; backdrop-filter present.           |
| Toast            | yes   | yes    | yes    | Motion: enter + dismiss for each tone.                 |
| Tooltip          | yes   | yes    | yes    | Motion: hover-in delay + exit.                         |
| CommandMenu      | yes   | yes    | yes    | Motion: open + filter.                                 |
| Presence         | yes   | yes    | yes    | All 4 variant tiles: tween/spring × enter/exit.        |
| PresenceGate     | yes   | yes    | yes    | Gate-on/gate-off; assert children mount/unmount.       |
| KineticBox       | yes   | yes    | yes    | 3 cues: rise-in, fade-in, slide-up.                    |
| Sequence         | yes   | yes    | yes    | Scrub 0 → 560 ms; assert 3 children animate.           |
| TimelineScope    | yes   | yes    | yes    | Scrub stagger + sequence variants.                     |
| FrameStage       | yes   | yes    | yes    | Frame caption advances as scrub moves.                 |
| SharedElement    | yes   | yes    | yes    | Flip-frame swap measures position handoff.             |
| SharedLayout     | yes   | yes    | yes    | Flip-frame swap snapshots both layouts.                |
| CaptureStage     | yes   | no     | yes    | Three viewport profiles snapshotted.                   |

Coming-soon entries are smoke-only and assert the "Coming soon"
placeholder is visible.

## Variants

A variant is a `(media-emulation, preference-bar-state)` pair applied
before each test runs:

- **default** — light theme, comfortable density, normal motion,
  translucent glass. `prefers-reduced-motion: no-preference`,
  `prefers-color-scheme: light`.
- **dark** — dark theme via preference bar; `prefers-color-scheme:
  dark` emulated.
- **reduced-motion** — `page.emulateMedia({ reducedMotion: 'reduce' })`
  plus the gallery's motion preference set to `reduced`. Motion suites
  assert *no progression* (initial = settled at t=0).
- **solid-glass** — glass policy set to `Solid` via the preference
  bar (matching `data-ui-glass-policy="solid"` on the shell). Visual
  suite re-baselines glass surfaces against the solid fallback path.

Other preference combinations (e.g. spacious + dark + reduced) are
left out by design. The four named variants cover the dimensions that
behave non-locally (motion timing, theme palette, glass policy);
density is a layout-only knob with no motion or rendering coupling and
is exercised by the smoke layer once at its default.

## Motion Assertion Strategy

`ScrubFrame` exposes a deterministic clock through its
`input[type=range]`. Motion tests for sequence/timeline-style
components drive the slider directly:

```
await scrubTo(page, frame, 0);
const initial = await readStyles(child, ['transform', 'opacity']);

await scrubTo(page, frame, 120);
const mid = await readStyles(child, ['transform', 'opacity']);

await scrubTo(page, frame, 560);
const settled = await readStyles(child, ['transform', 'opacity']);

expect(initial.opacity).toBeCloseTo(0, 2);
expect(settled.opacity).toBeCloseTo(1, 2);
expect(mid.opacity).toBeGreaterThan(initial.opacity);
expect(mid.opacity).toBeLessThan(settled.opacity);
```

`ReplayFrame` runs an internal `requestAnimationFrame` loop, so tests
install `page.clock.install({ time: 0 })` before clicking the
**Replay** button, then advance with `page.clock.fastForward(ms)` and
sample after each step. The clock pause is essential — without it,
inline-style reads race against the RAF callback and the assertions
flake on slow CI runners.

For `Presence`, the test sequence is:

1. Mount with `present: true` and `prefers-reduced-motion: reduce` —
   assert `data-presence-state="present"` and `--ui-presence-t: 1`.
2. Mount with `present: true` under default motion — install clock,
   sample at `0`, `120`, `280` ms, assert monotonic `--ui-presence-t`
   progression to `1`.
3. Switch `present` to `false` — install clock, advance, assert the
   exit cue brings `--ui-presence-t` back toward `0` and the element
   is eventually `data-presence-state="removed"`.

For springs (e.g. `Spring::snappy()`), the tolerance band at sample
time `t` is computed from
`Spring::settling_duration_ms(0.005)` in `ui-motion`. Rather than
reimplement that math in TypeScript, the harness reads the
authoritative settling time at test setup time from a small
`tests/_lib/spring-reference.json` file generated by a Cargo test
fixture (`examples/component-gallery/tests/spring_reference.rs`) that
serialises the settling durations for the spring presets the gallery
uses. The Cargo test fails if the JSON is stale, so the TypeScript
side never drifts from the Rust spring model.

## Visual Regression Strategy

Visual snapshots are stored at
`examples/component-gallery/e2e/__snapshots__/<entry>/<variant>-<browser>.png`,
where `<browser>` is `chromium` or `webkit`. Baselines are committed
to git. `npm run e2e:update` runs the suite with
`--update-snapshots`; the resulting diff is reviewed in the PR by
humans like any other file change.

Defaults:

- `expect.toHaveScreenshot({ maxDiffPixelRatio: 0.05, animations: 'disabled' })`.
- `animations: 'disabled'` is the Playwright setting that pauses CSS
  animations and transitions at their final state for screenshot
  comparisons. The motion suite still asserts mid-states explicitly;
  the visual suite only screenshots known-stable states.
- `LiquidSurface` previews are masked over their `<canvas>` element to
  avoid WebGPU driver noise; the surrounding chrome (caption, border,
  fallback `<svg>`) is asserted.

CI runs on `ubuntu-24.04` with Playwright's bundled Chromium and
WebKit binaries. Local snapshots from other OSes are not authoritative;
the README notes this and points contributors at `npm run e2e:ci` (a
script that runs the suite inside the same container image CI uses)
for parity.

## Reporter And Audit Output

The custom reporter at `e2e/reporters/audit-report.ts` implements the
Playwright `Reporter` interface and produces `audit-report.md` at the
end of every run, regardless of suite outcome. The report has:

- A header with the run timestamp, the git SHA at `dx build` time,
  and the browser/OS pair.
- A summary table with one row per component:

  | Component | Smoke | Motion | Visual | Status     | Notes                       |
  |-----------|-------|--------|--------|------------|-----------------------------|
  | Button    | pass  | n/a    | pass   | ready      |                             |
  | Sequence  | pass  | fail   | pass   | regression | t=280 opacity stuck at 0.0  |
  | Toast     | fail  | n/a    | n/a    | broken     | trigger click no-ops        |

- A "Defects" section listing each failure with file/line, the
  variant it fired under, and the inline-style snapshot at the time
  of failure.
- A "Coming soon" section listing every entry whose status is
  `ComponentStatus::ComingSoon`, so the report doubles as a forward
  plan.

CI uploads `audit-report.md` and the Playwright HTML report
(`playwright-report/`) as workflow artifacts. The CI summary embeds
the top of the audit report inline so PR reviewers see the table
without having to download an artifact.

## Data Flow

```
Rust (component-gallery)
  cargo test → SSR-based gallery.rs / controls.rs / demo_frame.rs tests
  cargo test → manifest.rs (TS manifest parity)
  cargo test → spring_reference.rs (settling-duration JSON parity)
  dx build  → examples/component-gallery/dist/

Node (examples/component-gallery/e2e)
  globalSetup → spawn dx build (static project only)
              → assert dist/ present
  webServer   → http-server dist on :4173
  tests       → mountGallery → smoke / motion / visual suites
  reporter    → audit-report.md
  CI          → upload audit-report.md + playwright-report/
```

## Error Handling

The harness is strict by default:

- Any `console.error` not in the per-project allowlist fails the test.
- Any unhandled promise rejection inside the page fails the test.
- Any test that depends on `dx build` output fails immediately with a
  diagnostic if `dist/` is absent (no auto-rebuild inside a single
  test run; the rebuild is `globalSetup`'s job).
- Snapshot tests fail with a side-by-side diff written to
  `test-results/<entry>-<variant>-<browser>/`.
- The custom reporter never throws; failures inside the reporter are
  logged with `console.warn` and the rest of the report is still
  emitted. A best-effort `audit-report.md` is better than none.

The `dev-loop` project allowlists the dev-server WebSocket reconnect
message visible in
`.playwright-mcp/console-2026-05-22T19-51-46-008Z.log` so the
existing dev experience can be exercised without a wall of red.

## Testing The Harness Itself

The harness is the test infrastructure for the gallery; it does not
itself ship to users. It still needs its own confidence checks:

- A `tests/_lib/__tests__/` directory with a handful of Node-side
  unit tests run by `npm run test:unit` (Vitest) covering
  `readStyles`, `scrubTo`, the reporter's table-rendering function,
  and the manifest loader. These run in CI on the same workflow.
- A `--smoke` Playwright invocation that exercises only the
  smoke suite against the static project, run on every PR
  regardless of the changed-paths filter; this catches harness-level
  regressions even when the rest of the workflow is skipped.

## Phasing

The implementation plan (separate document) will stage the work as:

1. **Bootstrap** — `package.json`, `playwright.config.ts`, the two
   project blocks, `globalSetup`, the smoke suite over the manifest,
   the manifest-parity Cargo test. Lands the catalog skeleton with
   every component showing `smoke=pass` and motion/visual as `n/a`.
2. **Motion layer** — `tests/components/*.spec.ts` for the 13
   motion-relevant components; the spring-reference JSON Cargo
   fixture; the reduced-motion variant. Lands a measured catalog of
   which animations actually progress in a real browser.
3. **Visual layer** — `tests/visual.spec.ts`, baseline generation,
   per-browser snapshots committed. Lands deterministic regressions
   on the static UI.
4. **CI + reporter** — `.github/workflows/e2e.yml`, the audit-report
   reporter, the README. Closes the loop so every PR that touches
   gallery or `ui-*` produces a fresh catalog.

Steps 1–3 are sequential because each layer depends on the
preceding layer's helpers. Step 4 can land in parallel with step 3 as
soon as the reporter has any data to summarise.

## Risks And Mitigations

- **Node entering a Rust-only repo.** Mitigation: `e2e/` is the only
  directory with `package.json`; the `.gitignore` line and the
  README block reduce the surface. The Cargo workspace is unchanged.
- **Snapshot churn.** Mitigation: `animations: 'disabled'` for
  visual; mask `<canvas>`; baselines per browser; tight scope of the
  four named variants. Reviewers see snapshot diffs in PRs.
- **CI runtime.** Mitigation: caches; `--shard` support left
  available for the workflow if the suite exceeds 10 min; visual
  layer can be skipped via a `[skip-visual]` PR tag.
- **Drift between the TS manifest and `docs.rs`.** Mitigation: a
  Cargo test asserts parity; the Rust side fails first.
- **WebKit on Ubuntu CI is not Safari.** Mitigation: the README
  notes this; Playwright's bundled WebKit is a useful canary even if
  it is not pixel-identical to ship Safari.

## Out Of Scope (carries to Spec 2)

- All motion engine work (RAF coalescing, FLIP shared-element fixes,
  GPU-accelerated compositor offload, WAAPI/View Transitions
  adoption, scroll-driven animations).
- Any change to `ui-motion`, `ui-runtime`, or `kinetics`. The harness
  reads them; Spec 2 rewrites the ones the audit flags.
- Performance benchmarking. Spec 2 owns frame-rate targets and the
  associated harness extension.
- Cross-device emulation. Layered on top of this harness later.

The deliverable that connects Spec 1 to Spec 2 is
`audit-report.md` from a clean run on `main` after Spec 1 ships. That
report becomes the input to Spec 2's *Goal* section.
