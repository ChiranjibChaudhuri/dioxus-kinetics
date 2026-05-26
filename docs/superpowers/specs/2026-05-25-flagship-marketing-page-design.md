# Flagship Marketing Page — Design

## Goal

Ship the first example in this workspace that *looks like a real product
page*, not documentation. A single self-referential marketing page for
`dioxus-kinetics` that lives in its own `examples/flagship/` package,
built only from existing crates and scenes, with a binding visual
pass/fail check on the autoplay hero. The page exists to make the
library look like it deserves the showcase, and to expose any
primitive-level gaps the gallery hides behind 148 px preview tiles.

This is "marketing scene #1" in a two-scene sequence. Scene #2 (a macOS
Sonoma Control Center / iOS Settings app surface that exercises the
SaaS-chrome side of `ui-dioxus`) is intentionally out of scope here and
lands as its own spec after this one ships.

## Why the gap exists today

The workspace has Apple-grade ingredients. `ProductIntroScene` is
already a 1920×1080 / 10 s / 5-clip composition (`rise-in` title → body
→ flip-card deck → metric counter → CTA pulse). `ScrollPinnedStoryScene`
already scrubs a 10-second timeline against window scroll on a
`position: sticky` shell. `MetricCounterScene`, `CtaPulseScene`,
`FlipCardDeckScene`, `WipeTransitionScene` variants, `SplitText`,
`MotionPath`, the WebGPU `ui-glass-engine`, and the full token /
elevation / motion-cue system are all live.

These ingredients live inside `examples/component-gallery`, which is
explicitly documentation-shaped: sticky 280 px rail of category links,
sticky preference bar, two-column entry cards (copy + snippet + a
~148 px preview tile holding the scene). The scenes never get to fill
the viewport. The page chrome screams "design system docs," not
"product." No single artifact in the repo currently answers the
question "what does kinetics actually look like when you ship with it?"

The flagship answers that question. Nothing else.

## Scope

In scope:

1. A new `examples/flagship/` Cargo package with its own
   `Cargo.toml`, `Dioxus.toml`, `src/main.rs`, `src/app.rs`, and
   `src/styles.rs` (flagship-only CSS). Built and served independently
   of `component-gallery`.
2. A single-route Dioxus app that renders the five-section spine
   below, top-to-bottom, with no nav rail, no preference bar, no
   category sidebar, and no code blocks. Marketing chrome only.
3. Reuse — not re-implementation — of the existing scene components
   from `examples/component-gallery/src/previews/scenes/` plus
   semantic components from `kinetics::prelude`. The flagship's
   contribution is composition, copy, and identity CSS, not new
   primitives.
4. A flagship-only CSS file (`flagship.css`, declared inline via
   `style { … }` near the app root after `library_css()`) that adds:
   one display-tier type ramp on top of the existing token ramp, a
   single signature accent inflection on `--ui-primary`, and the
   section-level layout primitives the page needs (full-bleed hero
   shell, sticky-scroll story shell, three-up feature grid, metric
   strip, CTA band, minimal footer).
5. A `screenshot.png` artifact, captured during development via
   chrome-devtools-mcp at viewport 1440×900, of the first paint of
   the autoplay hero. This screenshot is the artifact the
   "Hero-3-seconds" pass/fail check is evaluated against.
6. A Playwright spec under `examples/flagship/e2e/tests/flagship.spec.ts`
   that boots the app, waits for the hero scene to settle at its
   hold-end frame, captures a viewport screenshot, and asserts the
   page has zero gallery-shell markers (`.gallery-rail`,
   `.gallery-controls`, `.gallery-entry`, `.gallery-code`).
7. README pointer at the bottom of the existing repository README
   describing how to run the flagship alongside the gallery.

Out of scope (each is a future sub-project):

- Scene #2 — the macOS / iOS app-surface flagship. Separate spec.
- New primitives. If a section reveals that a primitive is too
  generic (e.g., button press lacks spring feel), file the
  improvement as a follow-up ticket; do not block flagship ship on
  it. The flagship is allowed to be "as good as the current
  primitives let it be."
- A multi-page marketing site. Single page, single scroll.
- Internationalisation. Copy is English-only for this iteration.
- Real GitHub / docs URLs in the CTA. Use `#` placeholders; wiring
  the real URLs is a write-time edit, not a design question.
- New raster assets. The flagship renders **zero** PNG/JPG imagery.
  Every visual is produced by the library (text, glass, motion,
  gradient meshes).

## Section spine

Five sections, top to bottom. Each section maps to an existing scene
component or composes one from `kinetics::prelude`. The flagship's job
is to host them at full bleed with the right surrounding copy and
spacing.

### 1. Hero film (autoplay)

`ProductIntroScene` rendered into a full-viewport shell
(`height: 100vh; width: 100vw`), autoplay-once, transport controls
hidden. The existing 10 s timeline plays the title rise-in → body
fade-in → flip-card deck → metric counter → CTA pulse beat sequence
that already exists in `examples/component-gallery/src/previews/scenes/product_intro.rs`.

The hero is the only section the **Hero-3-seconds** pass/fail check
evaluates. It must read as apple.com-grade in the first frame after
autoplay starts and the title clip lands. No other section is
evaluated against an external reference.

Copy for this section already lives inside `ProductIntroScene`:
"Kinetics moves like light." / "Composable motion for downstream
SaaS." We accept this copy as-is for v1. Re-voicing is a write-time
edit, not a design question.

### 2. Scroll-driven product story

`ScrollPinnedStoryScene` lifted out of its 148 px preview tile and
pinned at full bleed. The scene already uses
`SceneDriver::Scroll(ScrollObserverConfig::new("#scroll-story-trigger"))`,
`height: 200vh` outer trigger, `position: sticky; top: 0; height: 100vh`
inner sticky. The flagship hosts it directly — no extra wrapper, no
shrink-to-card behaviour. Scroll scrubs the 10-second timeline so the
viewer feels they are *driving* the reveal.

Surrounding copy is rendered by the scene itself ("Scroll-driven
storytelling." / "Same Scene API. Scroll instead of autoplay." /
"Built on IntersectionObserver + window scroll." / "Pin a story to the
page."). No additional flagship copy in this section.

### 3. Glass feature triplet

Three full-bleed `GlassSurface` cards laid out as a single-row 3-up
grid on desktop, vertical stack on mobile, sitting over a colorful
ambient gradient mesh so the glass material has something to refract.
The mesh reuses the existing `body::before` ambient gradient pattern
from `GALLERY_CSS`, scoped to this section.

Per-card content:

- **Glass** — `GlassLevel::Floating`, `GlassTone::Info`. Title "Liquid
  glass. Honestly rendered." Body "WebGPU when it's available. SVG
  filter fallback. Solid fallback when accessibility says so."
- **Scenes** — `GlassLevel::Floating`, `GlassTone::Primary`. Title
  "One clock. Every runtime." Body "`Scene` owns the time. `Clip`,
  `SplitText`, `MotionPath`, presence, and shared-element layout all
  read from it."
- **Render + CLI** — `GlassLevel::Floating`, `GlassTone::Success`.
  Title "Frame-perfect render." Body "`kinetics render` walks any
  scene with `SceneDriver::Manual`, writes per-frame HTML, ships a
  manifest, and optionally encodes PNG / MP4."

Each card has one hover-press microinteraction: the existing
`.ui-icon-button:hover { transform: translateY(-1px); }` lift, applied
to the glass card itself, plus the existing `--ui-motion-fast`
transition. No new motion primitive. If the press feels generic in
practice, that is a primitive ticket, not a flagship ticket.

This section is the one that actually demands the WebGPU glass engine
to be on. It is the visual proof for the "Apple-like glass materials"
claim in the README.

### 4. Live metric strip

`MetricCounterScene` revealed on scroll-entry via the existing
presence machinery. Four metrics, honest numbers, no inflated claims:

- **N components ready** — pull from `kinetics::prelude` symbol count
  enumerated in the README's "Ready rendered components" list at
  write time.
- **60 fps target** — copy from the existing scene-player contract.
- **4 platform adapters** — Web, Desktop, Mobile, Native, matching
  the README platform-support table.
- **WebGPU glass** — yes/no chip, honest based on engine status.

If `MetricCounterScene` doesn't natively support 4 metrics in its
current shape, the flagship composes four `MetricCounter` instances in
a row inside one `Scene` and reveals them with a 200 ms stagger. The
choice is an implementation detail for the writing-plans phase, not a
design fork.

### 5. CTA + footer

The flagship composes its own CTA band from `Button` primitives —
not by rendering `CtaPulseScene`, which only ships one button ("Start
building") and is sized for a gallery preview tile. The band holds
two buttons side by side: "View on GitHub" (`ButtonVariant::Primary`),
"Open the gallery" (`ButtonVariant::Ghost`). Press feedback comes
from the same `--ui-motion-fast` transition and `:hover { transform:
translateY(-1px) }` rule that `CtaPulseScene` relies on, so the
behavior matches without coupling to a preview-shaped wrapper. Below
the buttons, a single line of caption text — copy decided at write
time, kinetics-appropriate ("Built in Rust. MIT licensed." is the
likely default; "Free to try. No credit card." from `CtaPulseScene`
is SaaS-tone and is *not* reused here).

Footer below is one row: brand mark on the left (reuse
`crate::brand::KINETICS_LOGO_SVG` from the gallery), license note
("MIT") and version pulled from `Cargo.toml` on the right. No
multi-column footer, no sitemap, no social icons.

## Visual identity

The flagship inherits the entire token system from `ui-tokens` and
`ui-styles`. It does **not** redefine `--ui-bg`, `--ui-fg`,
`--ui-primary`, `--ui-radius-*`, `--ui-space-*`, etc. The only
identity moves on top of the library defaults are:

1. **Display-tier type ramp.** Add three flagship-only variables in
   `flagship.css`:

   - `--flagship-display-1: clamp(56px, 8vw, 96px);` (hero title)
   - `--flagship-display-2: clamp(40px, 5vw, 64px);` (section
     headlines)
   - `--flagship-eyebrow: 13px;` (uppercase section labels)

   Font weight `800` on display-1 and display-2. Line height
   `1.05` on display-1, `1.12` on display-2. Inter is already the
   default `--ui-font-sans`; we do not introduce a new font.

2. **Single signature accent inflection.** The library's
   `--ui-primary` is `#0066cc` (light) / inherits to `#0066cc` (dark
   leaves it unchanged today). The flagship redefines `--ui-primary`
   *within its own scope only* to lean a half-step warmer:
   `#0a7aff` — still inside Apple's blue family, slightly more vivid
   than the SaaS-default `#0066cc`. No other token is overridden.
   No new colors are introduced.

3. **Ambient backdrop.** Reuse the `body::before` radial-mesh
   pattern from `GALLERY_CSS`, scoped to the flagship body. Same
   drift animation, same reduced-motion suppression.

4. **Zero raster imagery.** Every visual on the page is produced by
   the library: text, glass, motion, gradients, the kinetics brand
   mark (SVG). No photographs, no screenshots of UI, no illustration
   PNGs.

## Motion principles

The flagship does not introduce new motion primitives. It only uses
what exists:

- Autoplay `Scene` clock for the hero film.
- Scroll-driven `Scene` clock for the product story.
- Presence cues (`fade`, `rise`, `slide`) for entry animations of
  cards and metric strip.
- `--ui-motion-fast` / `--ui-motion-normal` token-driven transitions
  for hover/press on glass cards and CTA buttons.

If any of these read as "generic web app" in practice, that is a
ticket against the underlying primitive (e.g., "add spring physics
to `--ui-motion-press`"). The flagship is the forcing function that
generates such tickets, not the place we fix them.

## Reduced motion

The flagship inherits all reduced-motion plumbing from `ui-runtime`
and `ui-styles`. Specifically:

- `[data-ui-motion="reduced"]` and `@media (prefers-reduced-motion:
  reduce)` already kill the ambient mesh drift, the kinetic-box
  CSS animations, the presence transitions, and the progress
  shimmer. The flagship adds nothing here.
- The hero `Scene` autoplay must respect `ReducedMotion(true)`:
  when reduced, the scene skips its autoplay loop and renders the
  hold-end frame as a static composition. This already works via
  `ui-runtime::ReducedMotion` context; the flagship just provides
  it in `App` exactly the way `component-gallery::app.rs` does.
- The scroll-driven story scene continues to scrub on scroll even
  under reduced motion; scrolling is a user-driven input, not an
  autoplaying animation, and the existing motion plumbing leaves it
  alone. We do not add a special case.

## Accessibility

The flagship inherits a11y contracts from the underlying components.
Specific assertions to verify:

- Heading hierarchy is sequential — one `<h1>` for the hero, then
  `<h2>` per section. Section headlines must be real headings, not
  styled `<div>`s.
- Every interactive control is reachable via keyboard. The two CTA
  buttons receive visible focus rings from the existing
  `.ui-button:focus-visible` rule.
- Decorative ambient mesh has `aria-hidden="true"` and lives in
  `body::before`, so it is already non-focusable and non-readable.
- The hero `Scene` transport controls are hidden visually but the
  Scene component continues to expose its semantic state to AT — no
  custom hiding beyond `controls: Some(false)`.

## Package layout

```
examples/flagship/
  Cargo.toml                  # binary crate, depends on kinetics
                              # + dioxus + the workspace scenes used
  Dioxus.toml                 # standalone dx config (port, asset dir)
  README.md                   # one-paragraph orientation
  src/
    main.rs                   # dioxus::launch(App)
    app.rs                    # five sections, top to bottom
    styles.rs                 # FLAGSHIP_CSS constant (display ramp,
                              # accent inflection, section layout)
    sections/
      hero.rs                 # wraps ProductIntroScene
      story.rs                # wraps ScrollPinnedStoryScene
      features.rs             # glass triplet
      metrics.rs              # metric strip
      cta.rs                  # CtaPulseScene + footer
  e2e/
    tests/
      flagship.spec.ts        # Playwright: capture hero, assert no
                              # gallery markers
```

`examples/flagship/Cargo.toml` declares a `[lib]`-less binary crate
with a path dep on `kinetics` and the same scene helpers the gallery
uses (path dep into `examples/component-gallery` is acceptable for v1,
or the reused scenes get promoted into a small shared crate during
implementation — that choice belongs to writing-plans, not here).

`README.md` updates: add a short subsection under "Component Gallery"
titled "Flagship Marketing Page" with one `dx serve --package flagship
--port <free>` invocation. Do not move the gallery section.

## Pass / fail check

The binding pass/fail check, settled when we ship v1:

**Hero-3-seconds.** After `dx serve --package flagship` is running and
the first paint has settled (hero autoplay has reached the hold-end of
the title clip — approximately 2.4 s in), a screenshot of the viewport
at 1440×900 must read as an apple.com-grade product hero. The
screenshot is captured via chrome-devtools-mcp during development and
saved as `examples/flagship/docs/hero-screenshot.png`. The check is
explicitly subjective; the artifact is what we point at when we
disagree.

Two informal checks accompany it, never block ship:

- **Cold-eyes test.** A viewer who hasn't seen the docs cannot tell
  this is documentation. Run by showing the page to two
  non-contributors and asking "what is this site selling?"
- **Reference parity.** A side-by-side with one apple.com product
  hero shows comparable typography weight, glass-card material
  depth, and motion easing curves at the corresponding moment. Used
  to generate tickets against primitives that fall short, not to
  block flagship ship.

## Toward scene #2

Once scene #1 ships, scene #2 (macOS Sonoma Control Center / iOS
Settings app surface) becomes its own spec at
`docs/superpowers/specs/<date>-flagship-app-surface-design.md`. The
expected reuse: the same `examples/flagship/` package gains a second
route, the existing `Sidebar`, `CommandMenu`, `Dialog`, `Switch`,
`TextField`, `SharedLayout`, `SharedElement` components carry the
weight, and the flagship `FLAGSHIP_CSS` grows a second section
covering the app-surface chrome. This is mentioned here so the file
layout above does not assume single-route forever.

## Open questions / risks

- **Reused scenes live inside `examples/component-gallery`.** Either
  the flagship takes a path dep on the gallery crate, or the
  scenes get promoted into a shared crate (e.g.,
  `examples/flagship-scenes/` or `crates/ui-blocks-flagship/`).
  Resolved in writing-plans.
- **Glass engine availability.** The WebGPU path is the visual
  promise of section 3. If the browser falls back to SVG filter or
  solid policy during local development, the section visibly
  collapses. Mitigation: the spec already documents this fallback
  chain; tickets land against the engine if the fallback reads
  worse than expected, not against the flagship.
- **MetricCounter fan-out to 4 stats.** The scene as currently
  shipped may animate a single counter, not four. If it does not
  fan out cleanly, the flagship composes four `MetricCounter`s in
  one `Scene` with a stagger.
- **`CtaPulseScene` inside the hero film.** The autoplay hero
  (section 1) reuses `ProductIntroScene` verbatim, which embeds
  `CtaPulseScene` as its final clip — meaning the "Free to try. No
  credit card." caption plays for ~3 s near the end of the hero
  beat. If the cold-eyes test flags it as off-tone, the caption is
  edited at the scene source; the spec does not block on this.
