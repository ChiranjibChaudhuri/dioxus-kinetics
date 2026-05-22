# Gallery Dynamic Examples And Visual Depth Design

## Goal

Turn the component gallery from a static catalog into a living showcase
that demonstrates the three pillars of the Kinetics library — glass
materiality, motion choreography, and accessibility policies — without
changing any public component API.

The current gallery has three concrete problems a visitor notices in
under a minute:

1. Glass surfaces are visually indistinguishable from solid ones,
   because `backdrop-filter: blur(...)` runs against a near-white page
   background and there is no content behind it to refract or saturate.
2. Motion examples never move. `Sequence` is rendered with
   `TimelineClock::Manual { elapsed_ms: 560.0 }`, `Presence` and
   `PresenceGate` render as two static "Present / Hidden" tiles
   side-by-side, `KineticBox` renders as three labelled boxes with no
   animation, and `Dialog`, `Toast`, `Tooltip` are pinned `open: true` /
   `visible: true`.
3. The page reads as flat — surfaces, cards, dialog panels, and toasts
   all sit on the same visual plane because shadows are ad-hoc per
   component instead of drawn from a coherent elevation scale, and the
   `Theme` / `Density` button groups in the control bar are inert.

This spec lands one coherent gallery refresh that addresses all three
problems together.

## Scope

This spec lands:

- A 4-tier elevation scale in `ui-tokens`, applied to surface, card,
  dialog, toast, tooltip, command-menu, and glass-floating / overlay
  classes in `ui-styles`.
- A live four-toggle preference bar in the gallery (Theme, Density,
  Motion preference, Glass policy), seeded from `localStorage` and from
  the browser's `prefers-reduced-motion` media query, persisted across
  refreshes.
- CSS plumbing in `ui-styles` so that `[data-ui-motion="reduced"]` and
  `[data-ui-glass-policy="solid"]` on an ancestor element take effect
  without re-rendering each surface.
- An ambient mesh-gradient backdrop on the gallery page and a denser
  "demo plate" backdrop under the Foundations + Surfaces sections, both
  with parallel dark-theme variants.
- Three small gallery-only demo wrappers (`ReplayFrame`, `ScrubFrame`,
  `FlipFrame`) that retrigger, scrub, or swap their children.
- Interactive previews for `Dialog`, `Toast`, `Tooltip`, replacing the
  currently frozen-open versions.
- A registry split: the 1044-line `examples/component-gallery/src/docs.rs`
  gets its preview functions moved into a new `previews/` submodule.

It excludes:

- New public components. `ReplayFrame`, `ScrubFrame`, `FlipFrame`, and
  the preference bar live inside the gallery crate, not the library.
- Public-API changes to existing components. No new props, no renamed
  props, no removed props.
- Focus-trapping for `Dialog`. Still a later helper layer per the
  existing accessibility note.
- Keyboard-driven `CommandMenu` navigation.
- A visual-regression test harness. Motion and glass perception remain
  manually verified this wave.

## Non-Goals

- Reworking the `GlassRecipe` model. `GlassPolicy::Solid` already
  exists; this spec only ensures the runtime path (ancestor CSS
  attribute) works alongside the compile-time path.
- Exposing the preference signals as a downstream library feature. A
  future spec can promote them to `kinetics` if downstream apps ask.
- Replacing the four toggles with a settings page or modal. The control
  bar stays inline at the top of `gallery-main`, sticky on scroll.

## Architecture Overview

The work splits along library plumbing and gallery composition.

The library side adds tokens and CSS selectors and adjusts existing
class rules to consume the new tokens and respect the new ancestor
attributes. No Rust component code changes.

The gallery side introduces a `GalleryPrefs` context owning four
signals, a sticky control bar that mutates those signals, and the three
demo wrappers. The registry in `docs.rs` keeps its current shape; only
the bodies of preview functions change, and those bodies move into a
new `previews/` submodule for file-size hygiene.

The wiring between the two sides is intentionally narrow: the gallery
writes four `data-*` attributes onto its `gallery-shell` root, and the
library's CSS reads those attributes from any ancestor.

## Live Preferences

### Signals

A new file `examples/component-gallery/src/controls.rs` defines:

```rust
pub struct GalleryPrefs {
    pub theme:   Signal<ThemePref>,
    pub density: Signal<DensityPref>,
    pub motion:  Signal<MotionPref>,
    pub glass:   Signal<GlassPolicyUi>,
}
```

with `Light | Dark`, `Compact | Comfortable | Spacious`,
`Normal | Reduced`, and `Translucent | Solid` enums respectively.

`App` calls `use_context_provider(|| GalleryPrefs::new())` once near
the shell root. Any component below — the control bar, every preview
function, the demo wrappers — calls `use_context::<GalleryPrefs>()` to
read or write.

### Data attributes

The `gallery-shell` root spreads four `data-*` attributes computed
from the signals:

| Signal       | Attribute               | Values                                   |
| ------------ | ----------------------- | ---------------------------------------- |
| `theme`      | `data-ui-theme`         | `light` \| `dark`                        |
| `density`    | `data-ui-density`       | `compact` \| `comfortable` \| `spacious` |
| `motion`     | `data-ui-motion`        | `normal` \| `reduced`                    |
| `glass`      | `data-ui-glass-policy`  | `translucent` \| `solid`                 |

`data-ui-theme` and `data-ui-density` are already consumed by the
library; the latter two are new selectors added in this spec.

### Initial state and persistence

On first mount the `controls` module reads four `localStorage` keys
(`kx-gallery-theme`, `kx-gallery-density`, `kx-gallery-motion`,
`kx-gallery-glass`). For each missing or invalid value:

- `theme` falls back to `Light`.
- `density` falls back to `Comfortable`.
- `glass` falls back to `Translucent`.
- `motion` falls back to the result of
  `window.matchMedia('(prefers-reduced-motion: reduce)').matches`,
  i.e. `Reduced` if the user has the OS-level preference set, otherwise
  `Normal`.

A `use_effect` watching each signal writes the new value back to its
`localStorage` key on change.

All `web-sys` calls live behind `cfg(target_arch = "wasm32")`. Under
non-wasm targets, the four signals initialize to their hard-coded
defaults and the persistence effects are compiled out. The gallery
remains buildable for desktop and SSR contexts without `window`.

### The control bar UI

`app.rs` replaces the two dead `gallery-control-group` blocks with a
single `<section class="gallery-controls">` containing four
`ToggleGroup`s. `ToggleGroup` is a small gallery-local component built
on top of `ui-button` styled as radios (`role="radiogroup"`, each
button `role="radio"` with `aria-checked`). The control bar uses
`position: sticky; top: 0; z-index: 4;` so it stays reachable while
scrolling the long category list.

## Library Plumbing

### Elevation tokens

A new module `crates/ui-tokens/src/elevation.rs` defines:

```rust
pub struct ElevationScale {
    pub e0: &'static str,  // flush
    pub e1: &'static str,  // cards, metric, tooltip
    pub e2: &'static str,  // toasts, command menu, glass floating
    pub e3: &'static str,  // dialog panel, glass overlay
}
```

Two const instances `LIGHT_ELEVATION` and `DARK_ELEVATION` hold the
recipes. The light recipe uses sequential colored shadows in
`rgba(16,23,38, ...)` with progressively larger blur and y-offset. The
dark recipe uses deeper black shadows plus a 1px inner highlight
(`inset 0 1px 0 rgba(255,255,255,.06)`) so surfaces do not vanish on
dark backgrounds.

`ui-styles` emits these as CSS custom properties on `:root` and
`[data-ui-theme="dark"]`:

```css
:root { --ui-elevation-0: ...; --ui-elevation-1: ...; ... }
[data-ui-theme="dark"] { --ui-elevation-0: ...; ... }
```

Then the existing class rules in `ui-styles` replace their ad-hoc
`box-shadow: ...` literals with `box-shadow: var(--ui-elevation-N)` at
the assigned tier:

| Class                | Tier             |
| -------------------- | ---------------- |
| `.ui-surface`        | `--ui-elevation-0` |
| `.ui-metric-card`    | `--ui-elevation-1` |
| `.ui-tooltip`        | `--ui-elevation-1` |
| `.ui-toast`          | `--ui-elevation-2` |
| `.ui-command-menu`   | `--ui-elevation-2` |
| `.ui-dialog-panel`   | `--ui-elevation-3` |

Glass classes (`.ui-glass-*`) pick their elevation by glass level:
`Subtle → 0`, `Floating → 2`, `Overlay → 3`.

### Reduced-motion ancestor scope

For every existing rule in `ui-styles` that declares a `transition` or
`animation` property, a sibling rule scoped under
`[data-ui-motion="reduced"]` either drops the transition / animation
or holds at the settled state. The existing `data-ui-transparency` /
reduced-motion handling on `KineticBox`, `Sequence`, and `Presence`
stays; this is a parallel scope for the gallery's runtime toggle.
Selectors of the form

```css
[data-ui-motion="reduced"] .ui-toast { animation: none; }
[data-ui-motion="reduced"] .ui-dialog-panel { transition: none; }
```

cover the visible cases. Where a class also uses a CSS keyframe (the
ambient mesh drift in the gallery styles), the keyframe is gated by
both `@media (prefers-reduced-motion: no-preference)` and the absence
of `[data-ui-motion="reduced"]` — both must allow motion for the
keyframe to advance.

### Solid-glass ancestor scope

For every glass class in `ui-styles` and the two gallery glass
treatments (`.gallery-rail`, `.gallery-controls`), a sibling rule
scoped under `[data-ui-glass-policy="solid"]` sets
`backdrop-filter: none` and swaps the translucent background for the
solid fallback color drawn from the resolved glass recipe.

A CSS-level test asserts that the published `library_css()` contains
the `[data-ui-glass-policy="solid"]` override for every class that
declares a `backdrop-filter` rule. This prevents new glass classes
from silently skipping the override.

### Glass recipe resolution

`crates/ui-glass` already exposes `GlassPolicy`. This spec adds a
documentation note that `GlassPolicy::Solid`, when passed at resolve
time, returns the solid fallback recipe; downstream apps that prefer
compile-time policy use that path. The gallery uses the CSS-ancestor
path so flipping the toggle does not re-render every surface in the
tree.

## Visual Depth

### Ambient gallery backdrop

`examples/component-gallery/src/styles.rs` replaces the current pale
linear gradient on `body` with two layered pieces:

1. A base mesh painted via `body::before` as three large radial
   gradients (brand primary, brand accent, brand secondary, each at
   ~22% opacity) positioned on a 1600 × 1200 canvas. A
   `transform: translate3d(...)` keyframe slowly drifts the mesh on a
   ~40-second loop. The drift is gated as described in the
   reduced-motion section above.
2. A base solid below that mesh in `var(--ui-bg)` so the mesh fades
   gracefully on small screens and under solid-policy.

Dark theme parallels the same structure with a lower-saturation
deep-blue / violet / teal palette so dark mode reads as intentional.

### Dense plate under Foundations and Surfaces

A `.gallery-section--glass-stage` modifier is added to the
Foundations and Surfaces sections. The modifier paints a denser inner
mesh — overlapping color circles plus a single soft photo-like wash —
inside the section's padding. Glass tiles in those sections visibly
refract this plate when blur is on, and fall through to the solid
fallback color when the Glass policy toggle is flipped. Other sections
inherit only the ambient backdrop.

## Dynamic Example Harness

`examples/component-gallery/src/demo_frame.rs` defines three small
gallery-only widgets:

### ReplayFrame

```rust
#[component]
pub fn ReplayFrame(label: &'static str, children: Element) -> Element { … }
```

Holds `play_token: Signal<u32>`. The wrapper renders a small "▶ Replay"
button (hidden when `prefs.motion == Reduced`) and the children. The
children read the token from context and key off it
(`key: "{token}"` on the inner motion element) so Dioxus tears down and
remounts on click, retriggering the enter animation. Resting state
shows the settled render.

Used by: `Presence`, `PresenceGate`, `KineticBox` (one tile per cue:
`rise-in`, `fade-in`, `slide-up`).

### ScrubFrame

```rust
#[component]
pub fn ScrubFrame(duration_ms: f32, fps: Option<u32>, label: &'static str, children: Element) -> Element { … }
```

Holds `elapsed_ms: Signal<f32>` and `playing: Signal<bool>`. UI is a
slider (`<input type="range" min="0" max="{duration_ms}">` styled to
match `ui-styles`) plus a ▶/⏸ button. When `playing == true`, an
`eval`-driven `requestAnimationFrame` loop advances `elapsed_ms` until
`duration_ms`, then stops and holds at the end. The children read
elapsed and fps from context. `Sequence` and `TimelineScope` consume
elapsed via `TimelineClock::Manual { elapsed_ms }`. `FrameStage`
derives `frame = (elapsed_ms / 1000.0 * fps).round() as u32`.

Slider input is unaffected by reduced motion (intentional drag is not
auto-animation). The play button respects reduced motion: clicking it
under `Reduced` jumps elapsed to `duration_ms` and leaves `playing`
false.

Used by: `Sequence`, `TimelineScope`, `FrameStage`. `CaptureStage`
remains the existing viewport grid; its frame numbers are static
documentation, not a timeline.

### FlipFrame

```rust
#[component]
pub fn FlipFrame(label: &'static str, layout_a: Element, layout_b: Element) -> Element { … }
```

Holds `at_b: Signal<bool>`. Renders one layout at a time inside a
`SharedLayout`; tiles within each layout use the same `SharedElement`
ids in different positions. A "Swap layout" button toggles `at_b`. The
library's FLIP runtime does the actual cross-tree movement; the
gallery only supplies the two layouts.

Used by: `SharedLayout` (two-card variant), `SharedElement`
(grid-vs-row variant).

## Interactive Overlays

### Dialog

The preview becomes a small stage with a "Show dialog" `ui-button`.
A local `open: Signal<bool>` controls the dialog. The dialog's
existing enter and exit transitions play; reduced motion collapses
both to instant open / close. Clicking the backdrop or either action
button closes the dialog. The static code snippet next to the preview
still shows `open: true` so visitors see the simplest possible markup.

### Toast

The preview holds a `Vec<ToastInstance>` signal and a small "stage"
`div` in the lower-right of the tile. Four buttons ("Success", "Info",
"Warning", "Error") each push a toast onto the stack; each toast
auto-dismisses after 3000ms via an `eval` `setTimeout`. Reduced motion
collapses enter and exit to instant. Dismissing one toast does not
disturb the others.

### Tooltip

The forced `visible: true` is dropped. The preview renders the
trigger label and binds `visible` to `use_signal(|| false)`. `onmouseenter`,
`onmouseleave`, `onfocus`, and `onblur` on the trigger flip the
signal. This matches real production usage.

## Registry Layout

`examples/component-gallery/src/docs.rs` keeps the `ComponentCategory`,
`ComponentStatus`, `ComponentDoc`, and `component_docs()` registry as
they are. The preview function bodies move into a new submodule:

```
examples/component-gallery/src/
  previews/
    mod.rs            (re-exports the public preview fns)
    motion.rs         (presence, kinetic_box, sequence, timeline_scope, presence_gate)
    surfaces.rs       (surface, glass_surface, metric_card, glass_layer)
    feedback.rs       (dialog, toast, tooltip, empty_state)
    composition.rs    (frame_stage)
    capture.rs        (capture_stage)
    shared.rs         (shared_layout, shared_element)
    inputs.rs         (text_field, checkbox, switch — unchanged bodies, just relocated)
    actions.rs        (button, icon_button, command_menu, toolbar — unchanged bodies)
    layout.rs         (stack, tabs, sidebar — unchanged bodies)
```

`docs.rs` imports each `preview` function from the new module path. The
file shrinks from 1044 lines to under 500 (registry plus snippet
constants), bringing it in line with the project's module-size
guideline.

## Testing Strategy

Unit tests, runnable under `cargo test`, cover:

- `GalleryPrefs::default()` returns the documented fallbacks under
  non-wasm cfg.
- The four `data-*` attribute names emitted by the gallery match the
  selectors used by `library_css()`. Asserted by string-match against
  the published CSS.
- `ui-tokens::elevation::LIGHT_ELEVATION` and `DARK_ELEVATION` exist
  and have non-empty recipes for all four tiers.
- `library_css()` contains:
  - `--ui-elevation-0` through `--ui-elevation-3` declarations on
    `:root` and on `[data-ui-theme="dark"]`.
  - At least one `[data-ui-motion="reduced"]` selector.
  - A `[data-ui-glass-policy="solid"]` override for every class that
    declares a `backdrop-filter` rule (CSS-substring test).
- `ui-glass::resolve_glass(theme, request)` with
  `request.policy = GlassPolicy::Solid` returns a recipe whose blur
  radius is 0 and whose background falls back to the solid color.
- The registry split: every category referenced from `categories()`
  has at least one `ComponentDoc`, and every doc's `render` function
  is reachable from the new `previews` module.

Explicitly outside automated testing this wave:

- Replay button retriggers enter animations correctly.
- Scrub slider produces visually correct intermediate frames for
  `Sequence`, `TimelineScope`, `FrameStage`.
- Dialog open / close enter and exit transitions play under
  `Normal` motion and collapse under `Reduced`.
- Toast stack inserts, advances, and dismisses correctly.
- FLIP transition between layout A and layout B looks correct.
- The ambient mesh and dense plate read as material rather than
  decoration; glass blur is visibly stronger under translucent than
  under solid.

These get a manual verification checklist in the implementation plan
that this spec drives.

## Risks And Mitigations

| Risk                                                                      | Mitigation                                                                                       |
| ------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| `requestAnimationFrame` loops on every visible `ScrubFrame` tile burn CPU. | rAF runs only while `playing == true`. Default state is paused at frame 0. Auto-play triggers on first viewport entry via `IntersectionObserver` (`eval`-bridged) and stops at end. |
| Ambient mesh keyframe conflicts with reduced-motion.                       | Drift is gated by both `@media (prefers-reduced-motion: no-preference)` and the absence of `[data-ui-motion="reduced"]`. Either signal stops the drift. |
| `localStorage` or `matchMedia` access throws under non-web targets.        | All `web-sys` calls behind `cfg(target_arch = "wasm32")` with safe fallbacks. Non-wasm builds use hard-coded defaults. |
| FLIP demo measures layout before swap renders.                             | Uses the existing `ui-runtime::measurement_web` helpers. No new measurement code introduced.    |
| Solid-policy override misses a glass class.                                | Library CSS test enumerates every `backdrop-filter` rule and asserts a matching `[data-ui-glass-policy="solid"]` sibling exists. |
| Adding a sticky control bar shifts existing layout under it.               | The bar is placed inside `gallery-main` above the header content; `position: sticky` activates only on scroll past the original position. No fixed offset is required. |

## Open Questions

None at spec time. All scope and policy questions were resolved during
brainstorming.
