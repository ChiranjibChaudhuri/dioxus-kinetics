# Presence And IconButton Animated Runtime Design

## Goal

Promote `Presence` and `IconButton` from coming-soon registry entries to
fully animated, production-grade components, and establish the pure-Rust
animation runtime that will also power Sequence (sub-project 3) and
SharedLayout/SharedElement (sub-project 4).

This is sub-project 2 of a 4-part animation effort. Sub-project 1
(rebrand and showcase polish) is complete and merged.

## Scope

This spec lands:

- A new `ui-runtime` crate providing a platform-abstracted frame
  scheduler and two Dioxus hooks: `use_animation_value` and
  `use_presence_state`.
- A new `ui-icons` crate with eight curated inline-SVG icon components.
- A new `IconButton` component in `ui-dioxus`, replacing the
  coming-soon stub. Pure CSS for hover/active/disabled states; no
  motion runtime needed.
- A new `Presence` component in `ui-dioxus` using the runtime, alongside
  the existing non-animated `PresenceGate`. Both stay in the public API.
- Reduced-motion handling via a runtime read of
  `prefers-reduced-motion`.
- Gallery promotion of `IconButton` and `Presence` to `Ready` with
  variant-grid previews.

It excludes:

- Sequence, SharedLayout, SharedElement (later sub-projects).
- A JS `eval` runtime path.
- FLIP layout animation.
- Native (Blitz) animation support — only web and desktop/mobile
  WebView in this sub-project.
- Icons beyond the curated set of eight.

## Non-Goals

- Achieving 60fps under high contention. The frame scheduler is a
  per-component subscriber; we do not introduce a global animation
  manager in this sub-project.
- Replacing CSS transitions for components that already use them
  (IconButton hover/press, GlassLayer fades, etc.).
- Removing or changing the `PresenceGate` API. It stays as the
  non-animated gate primitive.

## Tech Stack

- Rust 2021, Cargo workspace.
- Dioxus 0.7. Dioxus signals, hooks, SSR.
- Web target: `wasm-bindgen` + `web-sys` (Window, RequestAnimationFrame).
- Non-wasm target: `tokio::time::interval` running on Dioxus's existing
  Tokio runtime.
- Pure Rust unit tests + Dioxus SSR rendering tests.
- PowerShell on Windows for verification commands.

## Architecture

### New crate: `ui-runtime`

Lives at `crates/ui-runtime/`. Depends on `dioxus`, `ui-motion`, and
under target-cfg, on `wasm-bindgen` + `web-sys` (web) or `tokio` (non-wasm).
The crate has no other workspace dependencies.

The crate's public surface is small:

```rust
// src/lib.rs
pub use animation::{use_animation_value, ReducedMotion};
pub use presence::{use_presence_state, PresenceState};
pub use scheduler::{FrameHandle, FrameScheduler, ControlFlow};
```

Modules:

- `src/scheduler.rs` — `pub enum ControlFlow { Continue, Stop }`,
  `pub struct FrameHandle` (RAII; drop cancels the loop),
  `pub struct FrameScheduler` with a `pub fn spawn<F>(callback: F) ->
  FrameHandle where F: FnMut(f64) -> ControlFlow + 'static`. The
  callback receives `delta_ms` between frames. The scheduler
  implementation is `#[cfg(target_arch = "wasm32")]`-gated.
- `src/scheduler_web.rs` — wasm path. Uses
  `web_sys::Window::request_animation_frame` with a `Closure` retained
  inside the `FrameHandle`. On drop, the closure is detached and
  cancelled.
- `src/scheduler_native.rs` — non-wasm path. Uses
  `tokio::spawn(async { loop { tokio::time::sleep(...).await; ... } })`.
  The first tick uses `Instant::now()` as the baseline; later ticks
  compute `delta_ms` from the previous tick. On `FrameHandle` drop, the
  task is aborted via `JoinHandle::abort`.
- `src/animation.rs` — `use_animation_value(target: f32,
  transition: Transition) -> ReadOnlySignal<f32>`. The hook owns a
  `Signal<f32>` initialized to `target` and a `Signal<f32>` for velocity.
  It owns an `Option<FrameHandle>` retained across renders via
  `use_hook`. When `target` changes (compared each render), it
  starts/replaces a `FrameScheduler::spawn` callback that ticks the
  value toward target using `ui_motion::sample_tween` or
  `ui_motion::Spring::step`. When the value settles within an epsilon
  (configurable; default `0.001`), the callback returns
  `ControlFlow::Stop`. The hook also reads a `ReducedMotion` value from
  context; when set, the hook returns `target` immediately and never
  starts a scheduler.
- `src/presence.rs` — `pub enum PresenceState { Entering, Visible,
  Exiting, Unmounted }`. `use_presence_state(present: bool, enter:
  Transition, exit: Transition) -> ReadOnlySignal<PresenceState>`.
  Internally tracks an `value: Signal<f32>` driven by
  `use_animation_value` and the `present` prop. State transitions:
  - first mount with `present == true`: `Entering` until value > 0.999
    then `Visible`.
  - first mount with `present == false`: `Unmounted`.
  - `present` flips `true → false`: `Exiting` until value < 0.001
    then `Unmounted`.
  - `present` flips `false → true` from `Unmounted` or `Exiting`:
    `Entering` until value > 0.999 then `Visible`.
- `src/reduced_motion.rs` — exposes a `ReducedMotion(pub bool)`
  context value and a `pub fn use_reduced_motion() -> bool` hook. The
  hook returns the context value; if no context provider, defaults to
  `false`. (Wiring `prefers-reduced-motion` from the browser into this
  context is a one-time application-level concern: the gallery's
  `App` component provides the context with a value derived from a
  cheap initial CSS-mq probe via `web_sys::window().match_media(...)`
  on wasm; on non-wasm it provides `false`. Server-side renders use
  `false`.)

### New crate: `ui-icons`

Lives at `crates/ui-icons/`. Depends only on `dioxus`. No motion. No
other workspace deps.

Module layout:

- `src/lib.rs` — declares all icon modules and re-exports their
  components plus the path constants.
- `src/icons.rs` — for the initial eight icons, one
  `#[component] pub fn IconName(#[props(default = 16)] size: u32) ->
  Element` per icon, plus a paired
  `pub const ICON_NAME_PATH_D: &str`. Each component returns an inline
  `<svg viewBox="0 0 24 24" width="{size}" height="{size}"
  fill="currentColor" aria_hidden="true">` with the path inside.

Initial set: `Close`, `Check`, `ChevronDown`, `ChevronRight`, `Plus`,
`Minus`, `Trash`, `Search`. (8 icons.)

### Modified crate: `ui-dioxus`

Two changes:

1. New module `src/buttons.rs` for `IconButton` (plus
   `IconButtonTone` and `IconButtonSize` enums). The existing
   `Button` and `ButtonVariant` stay where they are in `src/lib.rs`.
2. Existing `src/kinetics.rs` adds a new `Presence` component next
   to the existing `PresenceGate`. The `PresenceGate` stays unchanged.

`Cargo.toml` adds `ui-runtime.workspace = true` and
`ui-icons.workspace = true`.

`src/lib.rs` exports:
- `pub use buttons::{IconButton, IconButtonTone, IconButtonSize};`
- `pub use kinetics::{KineticBox, KineticText, Presence, PresenceCue,
  PresenceGate, TimelineScope};` (Presence and PresenceCue are new.)

### Modified crate: `kinetics` (facade)

`Cargo.toml` gains two new optional features:

- `runtime` — pulls in `ui-runtime` and re-exports its hooks.
- `icons` — pulls in `ui-icons` and re-exports its components.

Both are added to the `default` feature list. Downstream apps that
want to opt out (for SSR-only deployments) can disable them.

`src/lib.rs` adds the new re-exports under appropriate `#[cfg(feature
= "...")]` gates. `public_api_names()` is updated to include
`IconButton`, `Presence`, `PresenceCue`, and the eight icon names.

### Modified crate: `ui-styles`

Adds these selectors to `COMPONENT_CSS`:

- `.ui-icon-button` (base shape, focus ring, transition for hover/active),
  `.ui-icon-button--neutral|--primary|--danger`,
  `.ui-icon-button--compact|--default|--spacious`,
  `.ui-icon-button:disabled` and `.ui-icon-button[aria-disabled="true"]`.
- `.ui-icon-button-glyph` (centers the icon, sets the icon color via
  `currentColor` inheritance).
- `.ui-presence` — base presence wrapper. Reads CSS variable
  `--ui-presence-t` (a 0..1 value the runtime writes inline) and
  derives opacity/transform.
- `.ui-presence[data-presence-cue="fade"]` — only opacity from
  `--ui-presence-t`.
- `.ui-presence[data-presence-cue="rise"]` — opacity and
  `translateY((1 - var(--ui-presence-t)) * 8px)`.
- `.ui-presence[data-presence-cue="slide"]` — opacity and
  `translateX((1 - var(--ui-presence-t)) * 16px)`.
- `.ui-presence[data-presence-cue="scale"]` — opacity and
  `scale(calc(0.92 + var(--ui-presence-t) * 0.08))`.

The CSS uses `calc` and `var` to interpret the inline `--ui-presence-t`
value the runtime writes. CSS never tries to animate `--ui-presence-t`
itself; the runtime ticks the Rust signal and Dioxus writes the new
value each frame. The `--ui-presence-t` declaration on `.ui-presence`
defaults to `1` so SSR snapshots render the settled state.

A `@media (prefers-reduced-motion: reduce)` block forces
`--ui-presence-t: 1` on `.ui-presence`. This is a belt-and-suspenders
fallback for cases where the runtime did not detect reduced motion;
the runtime path still skips the animation, but CSS guarantees the
rendered visual is the settled state.

### Component contracts

#### `IconButton`

```rust
#[component]
pub fn IconButton(
    label: String,
    #[props(default)] tone: IconButtonTone,
    #[props(default)] size: IconButtonSize,
    #[props(default = false)] disabled: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element
```

`IconButtonTone`: `Neutral` (default), `Primary`, `Danger`.

`IconButtonSize`: `Compact` (28px control), `Default` (32px),
`Spacious` (40px).

Renders a `<button type="button">` with classes
`ui-icon-button ui-icon-button--{tone} ui-icon-button--{size}`,
`aria-label="{label}"`, and the `disabled` attribute when set. The
icon (children) is wrapped in `<span class="ui-icon-button-glyph">`.

Disabled state styles via `:disabled` and `[aria-disabled="true"]`.
Focus visible ring via `:focus-visible`. The label string is exposed
on `aria-label`, so the children SVG can stay decorative
(`aria-hidden="true"` which is already true of the curated icons).

#### `Presence`

```rust
#[component]
pub fn Presence(
    present: bool,
    #[props(default = Transition::tween(220))] enter: Transition,
    #[props(default = Transition::tween(180))] exit: Transition,
    #[props(default = PresenceCue::Fade)] cue: PresenceCue,
    children: Element,
) -> Element
```

`PresenceCue`: `Fade` (default), `Rise`, `Slide`, `Scale`. The enum
exposes `pub const fn as_str(self) -> &'static str` returning
`"fade" | "rise" | "slide" | "scale"`.

`PresenceState` exposes `pub const fn as_str(self) -> &'static str`
returning `"entering" | "visible" | "exiting" | "unmounted"`.

Behavior:

1. Calls `use_presence_state(present, enter, exit)`.
2. If the state signal reads `Unmounted`, returns `rsx!{}`.
3. Otherwise renders:

```rust
rsx! {
    div {
        class: "ui-presence",
        "data-presence-cue": cue.as_str(),
        "data-presence-state": state.as_str(),
        style: "--ui-presence-t: {value};",
        {children}
    }
}
```

Where `value` is the underlying animation value (0..1) and
`state.as_str()` is `"entering" | "visible" | "exiting"`.

SSR fallback: `use_animation_value` is implemented using
`dioxus::hooks::use_future`. The future is responsible for spawning a
`FrameScheduler` and ticking the signal. `dioxus-ssr` does not execute
futures during render, so the scheduler never starts, and the hook's
initial state (set to `target` synchronously) is what the SSR HTML
renders. Concretely: for `present=true`, the SSR HTML contains
`data-presence-state="visible"` and `--ui-presence-t: 1`. For
`present=false`, `use_presence_state` synchronously returns `Unmounted`
and the Presence component returns `rsx!{}`, so the element is
omitted entirely.

For the non-wasm interactive runtime (dioxus-desktop/mobile), the
scheduler's native path is gated by a runtime probe: `FrameScheduler`
attempts `tokio::runtime::Handle::try_current()` and, if that fails,
returns a no-op `FrameHandle`. This keeps `use_animation_value` safe in
contexts where Tokio isn't running.

#### `PresenceGate` (unchanged)

Stays as the existing non-animated, present/absent gate. The gallery
keeps a separate registry entry for it.

## File Map

- Create: `crates/ui-runtime/Cargo.toml`
- Create: `crates/ui-runtime/src/lib.rs`
- Create: `crates/ui-runtime/src/scheduler.rs`
- Create: `crates/ui-runtime/src/scheduler_web.rs`
- Create: `crates/ui-runtime/src/scheduler_native.rs`
- Create: `crates/ui-runtime/src/animation.rs`
- Create: `crates/ui-runtime/src/presence.rs`
- Create: `crates/ui-runtime/src/reduced_motion.rs`
- Create: `crates/ui-runtime/tests/scheduler.rs`
- Create: `crates/ui-runtime/tests/animation.rs`
- Create: `crates/ui-runtime/tests/presence.rs`
- Create: `crates/ui-icons/Cargo.toml`
- Create: `crates/ui-icons/src/lib.rs`
- Create: `crates/ui-icons/src/icons.rs`
- Create: `crates/ui-icons/tests/icons.rs`
- Create: `crates/ui-dioxus/src/buttons.rs`
- Modify: `crates/ui-dioxus/src/lib.rs` (register buttons module, export IconButton + Presence + PresenceCue)
- Modify: `crates/ui-dioxus/src/kinetics.rs` (add Presence component beside PresenceGate)
- Modify: `crates/ui-dioxus/Cargo.toml` (deps on ui-runtime + ui-icons)
- Modify: `crates/kinetics/Cargo.toml` (new `runtime` + `icons` features, optional deps)
- Modify: `crates/kinetics/src/lib.rs` (re-exports gated on features; update `public_api_names()`)
- Modify: `crates/kinetics/tests/prelude.rs` (assert new names appear)
- Modify: `crates/ui-styles/src/lib.rs` (icon-button + presence CSS)
- Modify: `crates/ui-styles/tests/css.rs` (selector assertions)
- Modify: `examples/component-gallery/src/docs.rs` (promote IconButton + Presence to Ready; preview functions; registry growth)
- Modify: `examples/component-gallery/src/styles.rs` (if any preview-specific CSS is needed)
- Modify: `examples/component-gallery/tests/gallery.rs` (variant-grid + lifecycle assertions)
- Modify: `Cargo.toml` workspace members (add ui-runtime, ui-icons)
- Modify: `README.md` (note the new crates in the workspace layout, document the new optional features)

## Public API additions

`kinetics::prelude` gains:

- `IconButton`, `IconButtonTone`, `IconButtonSize`
- `Presence`, `PresenceCue`
- `Close`, `Check`, `ChevronDown`, `ChevronRight`, `Plus`, `Minus`,
  `Trash`, `Search`
- `use_animation_value`, `use_presence_state`, `use_reduced_motion`
- `PresenceState`, `ReducedMotion`

`public_api_names()` returns a superset of its current set plus the
items above.

## Tests

### `ui-runtime`

- `scheduler::tests::frame_handle_drop_cancels_callback` — non-wasm:
  spawn a scheduler that increments a counter on each tick; drop the
  handle after one tick; sleep a few intervals; assert the counter
  did not grow.
- `scheduler::tests::stop_returned_from_callback_halts` — non-wasm:
  spawn a scheduler whose callback returns `Stop` on the second tick;
  assert the counter only reached `2`.
- `animation::tests::reduced_motion_returns_target_immediately` —
  with `ReducedMotion(true)` provided in context, calling
  `use_animation_value(1.0, Transition::tween(220))` returns a
  signal whose value is already `1.0` on first render, with no
  scheduler started.
- `animation::tests::tween_converges_to_target` — non-wasm: provide a
  fake clock; tick `use_animation_value` 20 times at 16ms each;
  assert the value converged within epsilon of target by the
  expected number of ticks.
- `presence::tests::initial_present_true_transitions_entering_to_visible` —
  with `present=true` initially, the state is `Entering`; after the
  animation converges, the state is `Visible`.
- `presence::tests::flipping_to_false_transitions_to_unmounted_after_exit` —
  with state at `Visible`, set `present=false`; state becomes
  `Exiting`; once value reaches `<= epsilon`, state becomes `Unmounted`.

Web-target tests are not introduced in this sub-project. The web
scheduler is exercised manually via the gallery; unit tests focus on
the non-wasm scheduler and the platform-independent lifecycle logic.

### `ui-icons`

- `icons::tests::close_icon_renders_svg_with_viewbox` — SSR-render
  `Close { size: 24 }`; assert `viewBox="0 0 24 24"`, the path `d`
  attribute equals `CLOSE_PATH_D`, and `aria-hidden="true"`.
- `icons::tests::size_prop_controls_width_and_height` — for each
  icon, render at `size: 12`; assert `width="12"` and `height="12"`
  appear in the SSR output.
- `icons::tests::all_icons_export_path_constants` — assert that each
  of the eight `*_PATH_D` constants is non-empty.

### `ui-dioxus`

- `tests/icon_button_ssr.rs::icon_button_renders_with_label_and_glyph` —
  SSR-render an `IconButton` with `label="Close dialog"` containing a
  `Close` icon; assert the rendered HTML contains
  `aria-label="Close dialog"`, `class="ui-icon-button ...`, and the
  inner `<svg`.
- `tests/icon_button_ssr.rs::icon_button_disabled_emits_attribute` —
  with `disabled: true`, assert the rendered HTML contains
  `disabled` and no `onclick`-bound handler (SSR doesn't execute
  handlers; the test just confirms the attribute is set).
- `tests/presence_ssr.rs::presence_true_renders_content_with_data_attrs` —
  with `present=true`, assert the SSR HTML contains
  `data-presence-cue="fade"`, `data-presence-state="visible"`, and
  `--ui-presence-t: 1`.
- `tests/presence_ssr.rs::presence_false_renders_nothing` — with
  `present=false`, assert the SSR HTML for the Presence subtree is
  empty (no `data-presence-state` attribute).

### `kinetics`

- `tests/prelude.rs::public_api_includes_runtime_and_icons` — assert
  `IconButton`, `Presence`, `PresenceCue`, `Close`, `Check`,
  `ChevronDown`, `ChevronRight`, `Plus`, `Minus`, `Trash`, `Search`,
  `use_animation_value`, `use_presence_state`, `use_reduced_motion`,
  `PresenceState`, `ReducedMotion` all appear in
  `public_api_names()`.

### `ui-styles`

- `tests/css.rs::component_css_covers_icon_button_and_presence` —
  assert selectors `.ui-icon-button`, `.ui-icon-button--primary`,
  `.ui-icon-button--danger`, `.ui-icon-button--compact`,
  `.ui-icon-button--spacious`, `.ui-icon-button-glyph`,
  `.ui-presence`, `.ui-presence[data-presence-cue="fade"]`,
  `.ui-presence[data-presence-cue="rise"]`,
  `.ui-presence[data-presence-cue="slide"]`,
  `.ui-presence[data-presence-cue="scale"]` all appear.

### Gallery

- Promote `IconButton` to `Ready` and add a 3×3 preview (tones ×
  sizes). New gallery test asserts the matrix HTML contains tile
  labels for every `(tone, size)` pair plus an `<svg` for each tile.
- Add a new `Presence` registry entry (separate from `PresenceGate`).
  Preview shows two side-by-side tiles: `Present` (present=true,
  Rise cue) and `Hidden` (present=false; renders nothing). New
  gallery test asserts the SSR HTML contains
  `data-presence-cue="rise"` and `data-presence-state="visible"` in
  the Present tile.
- The `PresenceGate` registry entry stays as it is.

## Acceptance Checklist

- [ ] `crates/ui-runtime` and `crates/ui-icons` exist as workspace
      members.
- [ ] `ui-runtime` builds for `wasm32-unknown-unknown` and native
      targets (the workspace's default targets).
- [ ] `Presence` and `IconButton` are `ComponentStatus::Ready` in the
      gallery registry with variant-grid previews.
- [ ] `PresenceGate` remains in the public API and remains
      non-animated.
- [ ] `public_api_names()` includes all new symbols listed under
      "Public API additions".
- [ ] `prefers-reduced-motion` mapping: when `ReducedMotion(true)`
      is provided in context, `use_animation_value` returns the
      target immediately and `use_presence_state` skips intermediate
      states.
- [ ] CSS `@media (prefers-reduced-motion: reduce)` forces
      `--ui-presence-t: 1` as a fallback.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo test --workspace` passes.
- [ ] `cargo check -p component-gallery` passes.
- [ ] `cargo check -p kinetics --target wasm32-unknown-unknown` passes
      (sanity check that the wasm path compiles).
- [ ] Coming-soon entries (Sequence, SharedLayout, SharedElement)
      remain untouched.
