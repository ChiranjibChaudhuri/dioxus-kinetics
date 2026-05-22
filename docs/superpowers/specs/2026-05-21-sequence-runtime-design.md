# Sequence Animated Runtime Design

## Goal

Land the orchestrated `Sequence` component that drives multiple `KineticBox`
children through a coordinated timeline of typed property animations, and
finish the per-frame ticking deferred from sub-project 2.

This is sub-project 3 of a 4-part animation effort.

## Scope

This spec lands:

- Extension of `ui-timeline::MotionCue` to four typed variants (`Opacity`,
  `Translate`, `Scale`, `Rotate`) and corresponding extension of
  `ResolvedMotionState` to carry all four optional values.
- A `ResolvedMotionState::inline_style()` helper that composes valid CSS.
- Real per-frame ticking inside `ui-runtime::use_animation_value`,
  replacing the snap-to-target MVP from sub-project 2.
- A new `use_timeline_sample(timeline, clock)` hook in `ui-runtime` that
  recomputes the sample each frame.
- A new `Sequence` component in `ui-dioxus` that owns a `Timeline` and
  provides resolved state through Dioxus context.
- An updated `KineticBox` that consumes the context (when present) to
  write inline styles per frame, with the existing data-attribute-only
  behavior preserved when no Sequence ancestor exists.
- Gallery promotion of `Sequence` to `Ready` with a preview.

It excludes:

- A built-in DOM scroll observer. `Scroll` clock is supported but the
  caller provides the progress value via a Signal. A future helper hook
  may observe DOM scroll positions automatically.
- SharedLayout, SharedElement (sub-project 4).
- New motion cue variants beyond the four above.
- Refactoring or removing `TimelineScope`. It remains untouched.

## Non-Goals

- Replacing existing components or breaking their public APIs.
- Adding new Cargo features beyond what's already in `kinetics`.
- DOM-side scroll handling.
- Native (Blitz) animation support.

## Tech Stack

- Rust 2021, Cargo workspace, Dioxus 0.7.
- Existing `ui-motion`, `ui-timeline`, `ui-runtime`, `ui-dioxus` crates.
- `tokio` (non-wasm) and `wasm-bindgen` + `web-sys` (wasm) for the frame
  scheduler.
- Dioxus SSR for tests.
- PowerShell on Windows for verification commands.

## Architecture

### Concern 1: `ui-timeline` data model

`MotionCue` becomes:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MotionCue {
    Opacity {
        from: f32,
        to: f32,
        transition: Transition,
    },
    Translate {
        axis: Axis,
        from: f32,
        to: f32,
        transition: Transition,
    },
    Scale {
        from: f32,
        to: f32,
        transition: Transition,
    },
    Rotate {
        from_deg: f32,
        to_deg: f32,
        transition: Transition,
    },
}
```

The existing convenience constructor `MotionCue::opacity` stays. Add
`MotionCue::translate_x`, `MotionCue::translate_y`, `MotionCue::scale`,
`MotionCue::rotate` constructors.

`MotionCueSample` becomes:

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MotionCueSample {
    pub opacity: Option<f32>,
    pub translate_x: Option<f32>,
    pub translate_y: Option<f32>,
    pub scale: Option<f32>,
    pub rotate_deg: Option<f32>,
}
```

(Today, `MotionCueSample` carries only `opacity`. Existing fields stay;
new fields are `Option` and default to `None`.)

`ResolvedMotionState` is extended similarly:

```rust
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResolvedMotionState {
    pub target: MotionTarget,
    pub opacity: Option<f32>,
    pub translate_x: Option<f32>,
    pub translate_y: Option<f32>,
    pub scale: Option<f32>,
    pub rotate_deg: Option<f32>,
}

impl ResolvedMotionState {
    pub fn inline_style(&self) -> String {
        let mut parts = Vec::new();
        if let Some(opacity) = self.opacity {
            parts.push(format!("opacity: {opacity}"));
        }
        let mut transform = Vec::new();
        if self.translate_x.is_some() || self.translate_y.is_some() {
            let x = self.translate_x.unwrap_or(0.0);
            let y = self.translate_y.unwrap_or(0.0);
            transform.push(format!("translate({x}px, {y}px)"));
        }
        if let Some(scale) = self.scale {
            transform.push(format!("scale({scale})"));
        }
        if let Some(rotate) = self.rotate_deg {
            transform.push(format!("rotate({rotate}deg)"));
        }
        if !transform.is_empty() {
            parts.push(format!("transform: {}", transform.join(" ")));
        }
        parts.join("; ")
    }
}
```

When multiple cue segments on a track produce overlapping samples on the
same node, the resolver merges them by `MotionCueSample.merge(other)`:
later (higher segment index) wins for fields the latter has populated;
earlier values are preserved for fields the latter has `None`.

When a single `TimelineTrack` has multiple segments active at the same
elapsed_ms, the track's `sample()` method collects all live samples and
merges them in segment order. (Today's `sample` returns at most one
sample via `find_map`. That changes to iterate and merge.)

### Concern 2: `ui-runtime` per-frame ticking

`use_animation_value` is rewritten to actually tick:

- On first render, the value signal is initialized to `target`.
- On each subsequent render where `target` changes, the hook spawns a
  `FrameScheduler` callback that interpolates from the current value to
  the new target via `Transition::Tween` (`ui_motion::sample_tween`) or
  `Transition::Spring` (`ui_motion::Spring::step`). The callback updates
  the value signal on each frame. When the value settles within
  `0.001` of target, the callback returns `ControlFlow::Stop`.
- When reduced motion is active (`use_reduced_motion()` returns true),
  the hook skips spawning the scheduler and sets value directly to
  target.
- On SSR (`use_future` does not execute), the value signal stays at
  the initial target value and the rendered HTML reflects the settled
  state.

The hook persists the `FrameHandle` across renders via `use_hook` so
that re-renders don't restart the loop unless `target` actually
changed. When a new target replaces an in-flight animation, the old
handle is dropped (cancelling its loop) before the new one starts.

### Concern 3: `use_timeline_sample` hook

```rust
pub fn use_timeline_sample(
    timeline: Timeline,
    clock: TimelineClock,
) -> ReadSignal<TimelineSample>;
```

Behavior:

- The hook owns an internal `Signal<TimelineSample>` initialized to
  `timeline.sample(initial_clock)` where `initial_clock` is the input
  `clock` value.
- For `TimelineClock::Playback`, the hook spawns a frame scheduler
  that advances an internal `elapsed_ms` counter by the frame delta
  each tick and sets the sample signal to `timeline.sample(Playback {
  elapsed_ms })`. The loop continues while any track has an active
  segment; it stops once all tracks fall outside their fill windows.
- For `TimelineClock::Manual { elapsed_ms }`, the hook treats the
  input value as authoritative each render (no scheduler).
- For `TimelineClock::Frame { frame, fps }`, same as Manual but with
  the frame-based clock variant.
- For `TimelineClock::Scroll { progress }`, same: the caller passes
  the progress; the hook samples on each render.

The hook participates correctly in reduced motion: when reduced motion
is on, the hook skips the scheduler and immediately samples at
`elapsed_ms = timeline.duration_ms` (settled state).

### Concern 4: `Sequence` component

```rust
#[component]
pub fn Sequence(
    #[props(default)] timeline: Option<Timeline>,
    #[props(default)] cues: Option<Vec<Cue>>,
    #[props(default = "sequence".to_string())] id: String,
    #[props(default = TimelineClock::Playback { elapsed_ms: 0.0 })]
    clock: TimelineClock,
    children: Element,
) -> Element

#[derive(Clone, Debug)]
pub struct Cue {
    pub target_id: String,
    pub start_ms: f32,
    pub motion: MotionCue,
}

impl Cue {
    pub fn new(
        target_id: impl Into<String>,
        start_ms: f32,
        motion: MotionCue,
    ) -> Self;
}

#[derive(Clone, Default)]
pub struct SequenceContext {
    pub states: std::collections::HashMap<String, ResolvedMotionState>,
}
```

Logic:

1. Resolve the effective timeline:
   - If `timeline.is_some()`, use it directly.
   - Else if `cues.is_some()`, build a `Timeline` from the cues. The
     timeline's `duration_ms` is the largest cue end-time (start_ms +
     the cue's transition duration). Each cue becomes a
     `TimelineTrack` with `MotionTarget::Node(KineticId::new(target_id))`
     and a single `MotionSegment` at `(start_ms, duration_ms, motion)`.
   - Else: render children without animation (no context provided).
2. Call `use_timeline_sample(timeline, clock)` to get a
   `ReadSignal<TimelineSample>`.
3. Build a `SequenceContext` by indexing the sample's states by
   `KineticId`.
4. `use_context_provider(|| SequenceContext::default())` (initial).
   Update the context value each render using `use_effect` or by
   re-providing.
5. Render `section.ui-sequence` with `data-timeline-id` and
   `data-autoplay` attributes (mirroring `TimelineScope`) and the
   children inside.

For Dioxus 0.7, context updates: `use_context_provider` runs once with
the initial value. To update later, the context must be wrapped in a
`Signal<SequenceContext>` so children can read the live value. The
implementation stores `Signal<SequenceContext>` in context and updates
the signal on each tick.

### Concern 5: `KineticBox` context consumption

`KineticBox` becomes:

```rust
#[component]
pub fn KineticBox(
    id: String,
    #[props(default = "fade-in".to_string())] cue: String,
    children: Element,
) -> Element {
    let kinetic_id = KineticId::new(id.clone());
    let ctx = try_consume_context::<Signal<SequenceContext>>();
    let style = ctx
        .and_then(|sig| sig.read().states.get(&kinetic_id.0).cloned())
        .map(|state| state.inline_style())
        .unwrap_or_default();

    rsx! {
        div {
            class: "ui-kinetic-box",
            "data-kinetic-id": "{kinetic_id.0}",
            "data-motion-cue": "{cue}",
            style: "{style}",
            {children}
        }
    }
}
```

When there is no Sequence ancestor, `ctx` is `None`, `style` is empty,
and the rendered HTML is byte-equivalent to today's output minus an
empty `style=""` attribute. (Whether the attribute appears at all when
empty depends on Dioxus's render behavior; either is acceptable as
long as existing tests still pass.)

## Public API additions

`kinetics::prelude` gains:

- `Sequence`, `Cue`, `SequenceContext`
- `Axis`
- New `MotionCue` constructors: `translate_x`, `translate_y`, `scale`,
  `rotate`
- `use_timeline_sample`
- `ResolvedMotionState` (was internal; now public; `inline_style`)

`public_api_names()` returns its current set plus the names above.

## File Map

- Modify: `crates/ui-timeline/src/lib.rs` — extend `MotionCue`,
  `MotionCueSample`, `ResolvedMotionState`, add `Axis`, add
  `inline_style()`, merge logic.
- Modify: `crates/ui-timeline/tests/timeline.rs` — new variant tests.
- Modify: `crates/ui-runtime/src/animation.rs` — real per-frame ticker.
- Create: `crates/ui-runtime/src/timeline.rs` — `use_timeline_sample`.
- Modify: `crates/ui-runtime/src/lib.rs` — register + re-export.
- Create: `crates/ui-runtime/tests/timeline_hook.rs` — SSR-render test.
- Modify: `crates/ui-dioxus/src/kinetics.rs` — `Sequence`, `Cue`,
  `SequenceContext`, updated `KineticBox`.
- Modify: `crates/ui-dioxus/tests/kinetics_ssr.rs` — Sequence SSR test
  (create file if it doesn't exist).
- Modify: `crates/ui-dioxus/Cargo.toml` — confirm `ui-timeline` and
  `ui-runtime` are present (they already are).
- Modify: `crates/kinetics/src/lib.rs` — re-exports; update
  `public_api_names()`.
- Modify: `crates/kinetics/tests/prelude.rs` — assert new names.
- Modify: `crates/ui-styles/src/lib.rs` — `.ui-sequence` minimal styling.
- Modify: `crates/ui-styles/tests/css.rs` — selector assertion.
- Modify: `examples/component-gallery/src/docs.rs` — promote
  `Sequence` to Ready with preview. `COMPONENT_DOCS` length grows by 1.
- Modify: `examples/component-gallery/tests/gallery.rs` — Sequence
  preview test.
- Modify: `README.md` — note Sequence in the components list (no
  workspace layout change since both `ui-timeline` and `ui-runtime`
  already exist).

## Tests

### `ui-timeline`

```rust
#[test]
fn motion_cue_translate_samples_linear_progress() {
    let cue = MotionCue::Translate {
        axis: Axis::X,
        from: 0.0,
        to: 100.0,
        transition: Transition::Tween { duration_ms: 200, ease: Ease::Linear },
    };
    let sample = cue.sample(0.5);
    assert_eq!(sample.translate_x, Some(50.0));
    assert_eq!(sample.translate_y, None);
    assert_eq!(sample.opacity, None);
}

#[test]
fn motion_cue_scale_clamps_at_endpoints() {
    let cue = MotionCue::Scale {
        from: 1.0,
        to: 1.2,
        transition: Transition::Tween { duration_ms: 200, ease: Ease::Linear },
    };
    assert_eq!(cue.sample(0.0).scale, Some(1.0));
    assert_eq!(cue.sample(1.0).scale, Some(1.2));
    assert_eq!(cue.sample(1.5).scale, Some(1.2));
}

#[test]
fn motion_cue_rotate_handles_negative_degrees() {
    let cue = MotionCue::Rotate {
        from_deg: -45.0,
        to_deg: 45.0,
        transition: Transition::Tween { duration_ms: 200, ease: Ease::Linear },
    };
    assert_eq!(cue.sample(0.5).rotate_deg, Some(0.0));
}

#[test]
fn resolved_motion_state_inline_style_composes_transform() {
    let state = ResolvedMotionState {
        target: MotionTarget::self_node(),
        opacity: Some(0.6),
        translate_x: Some(12.0),
        translate_y: None,
        scale: Some(0.95),
        rotate_deg: None,
    };
    let css = state.inline_style();
    assert!(css.contains("opacity: 0.6"));
    assert!(css.contains("transform: translate(12px, 0px) scale(0.95)"));
}

#[test]
fn resolved_motion_state_inline_style_omits_unset_fields() {
    let state = ResolvedMotionState {
        target: MotionTarget::self_node(),
        opacity: Some(0.4),
        ..Default::default()
    };
    let css = state.inline_style();
    assert_eq!(css, "opacity: 0.4");
}

#[test]
fn timeline_sample_at_zero_returns_from_values() {
    let cue = MotionCue::Opacity {
        from: 0.0,
        to: 1.0,
        transition: Transition::Tween { duration_ms: 220, ease: Ease::Linear },
    };
    let timeline = Timeline::new("t", 220.0)
        .with_track(TimelineTrack::new(
            MotionTarget::node("hero"),
            vec![MotionSegment::new(0.0, 220.0, cue)],
        ));
    let sample = timeline.sample(TimelineClock::Manual { elapsed_ms: 0.0 });
    assert_eq!(sample.states.len(), 1);
    assert_eq!(sample.states[0].opacity, Some(0.0));
}
```

### `ui-runtime`

The `animation_value_in_ssr_returns_target_synchronously` and
`animation_value_with_reduced_motion_returns_target` tests stay green.

```rust
#[test]
fn use_timeline_sample_in_ssr_returns_initial_sample() {
    let timeline = Timeline::new("hero", 220.0)
        .with_track(TimelineTrack::new(
            MotionTarget::node("hero"),
            vec![MotionSegment::new(
                0.0,
                220.0,
                MotionCue::Opacity {
                    from: 0.0,
                    to: 1.0,
                    transition: Transition::Tween {
                        duration_ms: 220,
                        ease: Ease::Linear,
                    },
                },
            )],
        ));
    let html = dioxus_ssr::render_element(rsx! {
        TimelineSampleProbe {
            timeline: timeline,
            clock: TimelineClock::Manual { elapsed_ms: 110.0 }
        }
    });
    assert!(html.contains("opacity=\"0.5\""), "got {html}");
}
```

(The `TimelineSampleProbe` is a test helper component that takes a
timeline + clock, calls `use_timeline_sample`, and renders the first
state's opacity in a data attribute.)

### `ui-dioxus`

```rust
#[test]
fn sequence_provides_state_map_via_context() {
    let timeline = Timeline::new("hero", 220.0)
        .with_track(TimelineTrack::new(
            MotionTarget::node("title"),
            vec![MotionSegment::new(
                0.0,
                220.0,
                MotionCue::Opacity {
                    from: 0.0,
                    to: 1.0,
                    transition: Transition::Tween {
                        duration_ms: 220,
                        ease: Ease::Linear,
                    },
                },
            )],
        ));
    let html = dioxus_ssr::render_element(rsx! {
        Sequence {
            timeline: Some(timeline),
            clock: TimelineClock::Manual { elapsed_ms: 0.0 },
            KineticBox { id: "title", "Hello" }
        }
    });
    assert!(html.contains("style=\"opacity: 0\"") || html.contains("opacity: 0;"));
}

#[test]
fn sequence_with_cues_vec_equivalent_to_timeline_prop() {
    let cues = vec![Cue::new(
        "title",
        0.0,
        MotionCue::Opacity {
            from: 0.0,
            to: 1.0,
            transition: Transition::Tween {
                duration_ms: 220,
                ease: Ease::Linear,
            },
        },
    )];
    let html = dioxus_ssr::render_element(rsx! {
        Sequence {
            cues: Some(cues),
            clock: TimelineClock::Manual { elapsed_ms: 0.0 },
            KineticBox { id: "title", "Hello" }
        }
    });
    assert!(html.contains("opacity: 0"));
}

#[test]
fn kinetic_box_outside_sequence_still_renders_data_attrs() {
    let html = dioxus_ssr::render_element(rsx! {
        KineticBox { id: "solo", cue: "fade-in", "Hello" }
    });
    assert!(html.contains("data-kinetic-id=\"solo\""));
    assert!(html.contains("data-motion-cue=\"fade-in\""));
}
```

### `kinetics`

```rust
#[test]
fn public_api_includes_sequence_runtime_names() {
    let names = kinetics::public_api_names();
    for expected in [
        "Sequence",
        "Cue",
        "SequenceContext",
        "Axis",
        "use_timeline_sample",
        "ResolvedMotionState",
    ] {
        assert!(names.contains(&expected), "missing {expected}");
    }
}
```

### `ui-styles`

```rust
#[test]
fn component_css_covers_sequence_wrapper() {
    assert!(COMPONENT_CSS.contains(".ui-sequence"));
}
```

### Gallery

The gallery promotes `Sequence` to Ready with a preview that demonstrates
a 3-cue choreography. A test asserts the preview HTML contains inline
`style="..."` strings on at least 3 different KineticBox descendants.

## Acceptance Checklist

- [ ] `ui-timeline::MotionCue` has variants `Opacity`, `Translate`,
      `Scale`, `Rotate`.
- [ ] `MotionCueSample` and `ResolvedMotionState` carry all four
      optional fields.
- [ ] `ResolvedMotionState::inline_style()` produces valid CSS that
      composes opacity + transform without trailing semicolons.
- [ ] `use_animation_value` per-frame ticks on web (RAF) and desktop
      (tokio interval).
- [ ] Reduced-motion path skips the scheduler.
- [ ] `use_timeline_sample` recomputes the sample each frame for
      Playback clock; uses the input clock value as authoritative for
      Manual/Frame/Scroll.
- [ ] `Sequence` accepts either `timeline` or `cues` prop.
- [ ] `Sequence` provides `Signal<SequenceContext>` via Dioxus context.
- [ ] `KineticBox` reads the context when available and writes inline
      style.
- [ ] `KineticBox` outside a `Sequence` renders the existing data
      attributes without animation.
- [ ] `Sequence` is `Ready` in the gallery with a preview.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo test --workspace` passes.
- [ ] `cargo check -p kinetics --target wasm32-unknown-unknown` passes.
- [ ] `TimelineScope` is unchanged.
- [ ] `PresenceGate`, `Presence`, `IconButton` all unchanged.
- [ ] Coming-soon entries (`SharedLayout`, `SharedElement`) remain
      `ComingSoon`.
