# Sequence Animated Runtime Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend the timeline math with typed property cues, finish the per-frame ticker deferred from sub-project 2, and ship the `Sequence` component that orchestrates multiple `KineticBox` children through a coordinated timeline.

**Architecture:** Three coupled changes: (1) `ui-timeline::MotionCue` gains `Translate`/`Scale`/`Rotate` variants and `MotionCueSample`/`ResolvedMotionState` carry all four optional property fields with an `inline_style()` composer. (2) `ui-runtime::use_animation_value` becomes a real ticker using `FrameScheduler` + `ui-motion`'s `sample_tween`/`Spring::step`. (3) A new `ui-runtime::use_timeline_sample` hook plus a new `ui-dioxus::Sequence` component provide a `Signal<SequenceContext>` to descendant `KineticBox` children, which write inline `style` from their resolved state.

**Tech Stack:** Rust 2021, Cargo workspace, Dioxus 0.7, `tokio` (non-wasm) + `wasm-bindgen`/`web-sys` (wasm), Dioxus SSR for tests, PowerShell on Windows.

---

## Scope

This plan implements sub-project 3 from `docs/superpowers/specs/2026-05-21-sequence-runtime-design.md`.

It includes:

- Four `MotionCue` variants (`Opacity`, `Translate`, `Scale`, `Rotate`) and their typed samples.
- `ResolvedMotionState::inline_style()` CSS composer.
- Real RAF/Tokio ticking in `use_animation_value`.
- `use_timeline_sample` hook.
- `Sequence` component + `Cue` + `SequenceContext`.
- `KineticBox` context consumption.
- `kinetics` facade re-exports + prelude tests.
- `.ui-sequence` CSS.
- Gallery `Sequence` preview promotion.

It excludes:

- A built-in DOM scroll observer (caller supplies progress).
- SharedLayout / SharedElement.
- Refactoring `TimelineScope` (untouched).
- Refactoring `Presence`, `IconButton`, `PresenceGate` (untouched).

## Before You Start

If running via `superpowers:subagent-driven-development`, that skill creates the worktree. Otherwise:

```powershell
git worktree add .worktrees/sequence-runtime -b sequence-runtime main
```

Run every command from inside the worktree.

## File Map

- `crates/ui-timeline/src/lib.rs` — extend `MotionCue`, `MotionCueSample`, `ResolvedMotionState`, add `Axis`, add `inline_style()`, merge multi-cue samples, expose `MotionCue::sample` publicly for tests.
- `crates/ui-timeline/tests/timeline.rs` — new variant + style tests (file already exists).
- `crates/ui-runtime/src/animation.rs` — real per-frame ticker.
- `crates/ui-runtime/src/timeline.rs` — new `use_timeline_sample` hook.
- `crates/ui-runtime/src/lib.rs` — register `timeline` module and export the hook.
- `crates/ui-runtime/tests/hooks_ssr.rs` — append `use_timeline_sample` SSR tests.
- `crates/ui-dioxus/src/kinetics.rs` — add `Sequence`, `Cue`, `SequenceContext`; update `KineticBox` to read context.
- `crates/ui-dioxus/tests/sequence_ssr.rs` — new file for Sequence SSR tests.
- `crates/ui-dioxus/tests/kinetics_ssr.rs` — add backward-compat assertion for KineticBox outside a Sequence (create if missing).
- `crates/ui-dioxus/src/lib.rs` — re-export `Sequence`, `Cue`, `SequenceContext`.
- `crates/kinetics/src/lib.rs` — extend re-exports + `public_api_names()`.
- `crates/kinetics/tests/prelude.rs` — assert new names.
- `crates/ui-styles/src/lib.rs` — add `.ui-sequence` selector.
- `crates/ui-styles/tests/css.rs` — assert `.ui-sequence` present.
- `examples/component-gallery/src/docs.rs` — add `Sequence` Ready entry with preview.
- `examples/component-gallery/tests/gallery.rs` — assert Sequence preview renders.
- `README.md` — mention `Sequence` in the ready components list.

## Task 1: Extend `MotionCue` With Typed Property Variants

**Files:**
- Modify: `crates/ui-timeline/src/lib.rs`
- Modify: `crates/ui-timeline/tests/timeline.rs`

- [ ] **Step 1: Append failing tests to `crates/ui-timeline/tests/timeline.rs`**

```rust
use ui_motion::{Ease, Transition};
use ui_timeline::{Axis, MotionCue};

fn linear_200() -> Transition {
    Transition::Tween {
        duration_ms: 200,
        ease: Ease::Linear,
    }
}

#[test]
fn motion_cue_translate_samples_linear_progress() {
    let cue = MotionCue::Translate {
        axis: Axis::X,
        from: 0.0,
        to: 100.0,
        transition: linear_200(),
    };
    let sample = cue.sample(0.5);
    assert_eq!(sample.translate_x, Some(50.0));
    assert_eq!(sample.translate_y, None);
    assert_eq!(sample.opacity, None);
}

#[test]
fn motion_cue_translate_y_axis_writes_translate_y_field() {
    let cue = MotionCue::Translate {
        axis: Axis::Y,
        from: 0.0,
        to: 40.0,
        transition: linear_200(),
    };
    let sample = cue.sample(0.25);
    assert_eq!(sample.translate_y, Some(10.0));
    assert_eq!(sample.translate_x, None);
}

#[test]
fn motion_cue_scale_interpolates_linearly() {
    let cue = MotionCue::Scale {
        from: 1.0,
        to: 1.2,
        transition: linear_200(),
    };
    assert_eq!(cue.sample(0.0).scale, Some(1.0));
    assert_eq!(cue.sample(0.5).scale, Some(1.1));
    assert_eq!(cue.sample(1.0).scale, Some(1.2));
}

#[test]
fn motion_cue_rotate_handles_negative_degrees() {
    let cue = MotionCue::Rotate {
        from_deg: -45.0,
        to_deg: 45.0,
        transition: linear_200(),
    };
    assert_eq!(cue.sample(0.5).rotate_deg, Some(0.0));
}

#[test]
fn motion_cue_opacity_still_works() {
    let cue = MotionCue::Opacity {
        from: 0.0,
        to: 1.0,
        transition: linear_200(),
    };
    let sample = cue.sample(0.5);
    assert_eq!(sample.opacity, Some(0.5));
    assert_eq!(sample.translate_x, None);
}
```

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p ui-timeline
```

Expected: compile FAIL (`Axis`, new variants, and public `sample` don't exist).

- [ ] **Step 3: Modify `crates/ui-timeline/src/lib.rs`**

Find the current `MotionCue` enum:

```rust
pub enum MotionCue {
    Opacity {
        from: f32,
        to: f32,
        transition: Transition,
    },
}
```

Replace it with:

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

Find the current `MotionCueSample`:

```rust
pub struct MotionCueSample {
    pub opacity: f32,
}
```

Replace with:

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MotionCueSample {
    pub opacity: Option<f32>,
    pub translate_x: Option<f32>,
    pub translate_y: Option<f32>,
    pub scale: Option<f32>,
    pub rotate_deg: Option<f32>,
}

impl MotionCueSample {
    pub fn merge(self, other: Self) -> Self {
        Self {
            opacity: other.opacity.or(self.opacity),
            translate_x: other.translate_x.or(self.translate_x),
            translate_y: other.translate_y.or(self.translate_y),
            scale: other.scale.or(self.scale),
            rotate_deg: other.rotate_deg.or(self.rotate_deg),
        }
    }
}
```

Find `impl MotionCue` and:

- Change `fn sample(...) -> MotionCueSample` to `pub fn sample(self, progress: f32) -> MotionCueSample`.
- Replace the body to handle all four variants:

```rust
pub fn sample(self, progress: f32) -> MotionCueSample {
    let p = ui_motion::finite_or_zero_clamped(progress); // see Step 4 helper
    match self {
        Self::Opacity {
            from,
            to,
            transition,
        } => {
            let eased = apply_transition_progress(p, transition);
            MotionCueSample {
                opacity: Some(interpolate(from, to, eased, Clamp::Yes)),
                ..Default::default()
            }
        }
        Self::Translate {
            axis,
            from,
            to,
            transition,
        } => {
            let eased = apply_transition_progress(p, transition);
            let value = interpolate(from, to, eased, Clamp::Yes);
            let mut sample = MotionCueSample::default();
            match axis {
                Axis::X => sample.translate_x = Some(value),
                Axis::Y => sample.translate_y = Some(value),
            }
            sample
        }
        Self::Scale {
            from,
            to,
            transition,
        } => {
            let eased = apply_transition_progress(p, transition);
            MotionCueSample {
                scale: Some(interpolate(from, to, eased, Clamp::Yes)),
                ..Default::default()
            }
        }
        Self::Rotate {
            from_deg,
            to_deg,
            transition,
        } => {
            let eased = apply_transition_progress(p, transition);
            MotionCueSample {
                rotate_deg: Some(interpolate(from_deg, to_deg, eased, Clamp::Yes)),
                ..Default::default()
            }
        }
    }
}
```

Update `MotionCue::reduced_motion` to handle the new variants. The reduced variant collapses the transition's duration to 0:

```rust
fn reduced_motion(self) -> Self {
    match self {
        Self::Opacity {
            from,
            to,
            transition,
        } => Self::Opacity {
            from,
            to,
            transition: transition.reduced(),
        },
        Self::Translate {
            axis,
            from,
            to,
            transition,
        } => Self::Translate {
            axis,
            from,
            to,
            transition: transition.reduced(),
        },
        Self::Scale {
            from,
            to,
            transition,
        } => Self::Scale {
            from,
            to,
            transition: transition.reduced(),
        },
        Self::Rotate {
            from_deg,
            to_deg,
            transition,
        } => Self::Rotate {
            from_deg,
            to_deg,
            transition: transition.reduced(),
        },
    }
}
```

- [ ] **Step 4: Add the `apply_transition_progress` helper**

Add this private helper near the top of `crates/ui-timeline/src/lib.rs`:

```rust
fn apply_transition_progress(progress: f32, transition: Transition) -> f32 {
    match transition {
        Transition::Tween { ease, .. } => apply_ease(progress.clamp(0.0, 1.0), ease),
        Transition::Spring(_) => progress.clamp(0.0, 1.0),
    }
}
```

This replaces any inline ease handling that existed for the old `Opacity` cue. The `Spring` case treats progress linearly inside the cue; the actual spring shape is applied in the runtime ticker.

If `ui_motion::finite_or_zero_clamped` doesn't exist, inline a local helper:

```rust
fn finite_or_zero_clamped(progress: f32) -> f32 {
    if progress.is_finite() {
        progress.clamp(0.0, 1.0)
    } else {
        0.0
    }
}
```

and use it in `sample(self, progress)` instead of `ui_motion::finite_or_zero_clamped`.

- [ ] **Step 5: Update existing call sites in `lib.rs`**

`TimelineTrack::sample` currently does:

```rust
let cue = self.segments.iter().find_map(|segment| segment.sample(elapsed_ms, fill))?;
Some(ResolvedMotionState {
    target: self.target.clone(),
    opacity: cue.opacity,
})
```

The `ResolvedMotionState` is renamed/extended in Task 2 — for Task 1 just keep this call site building a `ResolvedMotionState` with the new `MotionCueSample` shape, mapping each `Option<f32>` field through. After Task 2 runs you'll touch this again.

For Task 1's minimum viable change, replace the above with:

```rust
let mut merged = MotionCueSample::default();
let mut any = false;
for segment in &self.segments {
    if let Some(sample) = segment.sample(elapsed_ms, fill) {
        merged = merged.merge(sample);
        any = true;
    }
}
if !any {
    return None;
}
Some(ResolvedMotionState {
    target: self.target.clone(),
    opacity: merged.opacity,
    translate_x: merged.translate_x,
    translate_y: merged.translate_y,
    scale: merged.scale,
    rotate_deg: merged.rotate_deg,
})
```

This requires `ResolvedMotionState` to already have all five fields. Add the fields and a `#[derive(Default)]` next:

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
```

For `MotionTarget::default()` to exist (required by `#[derive(Default)]` on the struct), add:

```rust
impl Default for MotionTarget {
    fn default() -> Self {
        Self::SelfNode
    }
}
```

`MotionSegment::sample` keeps returning `Option<MotionCueSample>`. Its body needs to match the new `MotionCueSample` shape — the existing body calls `self.cue.sample(progress)`, which now returns the new shape, so no other changes.

If any existing test or downstream consumer reads `state.opacity` as `f32` (not `Option<f32>`), it will compile-fail. There are some inside `crates/ui-dioxus/src/kinetics.rs` and gallery code. The implementer must update those reads to `state.opacity.unwrap_or(1.0)` or similar in Step 5 — but only the minimum needed to make the workspace build. Larger refactors land in later tasks.

- [ ] **Step 6: Run tests**

```powershell
cargo test -p ui-timeline
cargo build --workspace
```

Expected: timeline tests PASS, workspace builds. If any other crate has a hard `state.opacity: f32` access, fix it locally with `.unwrap_or(1.0)`.

- [ ] **Step 7: Commit**

```powershell
git add crates/ui-timeline crates/ui-dioxus
git commit -m "feat: extend motion cue with translate scale rotate"
```

## Task 2: `ResolvedMotionState::inline_style()`

**Files:**
- Modify: `crates/ui-timeline/src/lib.rs`
- Modify: `crates/ui-timeline/tests/timeline.rs`

- [ ] **Step 1: Append failing tests**

```rust
use ui_timeline::{MotionTarget, ResolvedMotionState};

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
    assert!(css.contains("opacity: 0.6"), "got {css}");
    assert!(
        css.contains("transform: translate(12px, 0px) scale(0.95)"),
        "got {css}",
    );
}

#[test]
fn resolved_motion_state_inline_style_only_opacity() {
    let state = ResolvedMotionState {
        opacity: Some(0.4),
        ..Default::default()
    };
    assert_eq!(state.inline_style(), "opacity: 0.4");
}

#[test]
fn resolved_motion_state_inline_style_only_rotate() {
    let state = ResolvedMotionState {
        rotate_deg: Some(5.0),
        ..Default::default()
    };
    assert_eq!(state.inline_style(), "transform: rotate(5deg)");
}

#[test]
fn resolved_motion_state_inline_style_translate_y_only() {
    let state = ResolvedMotionState {
        translate_y: Some(8.0),
        ..Default::default()
    };
    assert_eq!(state.inline_style(), "transform: translate(0px, 8px)");
}

#[test]
fn resolved_motion_state_inline_style_empty_state_returns_empty_string() {
    let state = ResolvedMotionState::default();
    assert_eq!(state.inline_style(), "");
}
```

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p ui-timeline
```

Expected: FAIL (no `inline_style` method).

- [ ] **Step 3: Implement `inline_style()`**

Append to `impl ResolvedMotionState` in `crates/ui-timeline/src/lib.rs`:

```rust
pub fn inline_style(&self) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(opacity) = self.opacity {
        parts.push(format!("opacity: {opacity}"));
    }
    let mut transform: Vec<String> = Vec::new();
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
```

- [ ] **Step 4: Run tests**

```powershell
cargo test -p ui-timeline
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add crates/ui-timeline
git commit -m "feat: compose resolved motion state inline style"
```

## Task 3: Real Per-Frame Ticker In `use_animation_value`

**Files:**
- Modify: `crates/ui-runtime/src/animation.rs`

- [ ] **Step 1: Inspect the current hook**

Read the current `crates/ui-runtime/src/animation.rs`. The MVP version sets the value to target synchronously and ignores `transition`. You'll replace the body but keep the function signature stable.

- [ ] **Step 2: Replace the implementation**

Replace the entire body of `crates/ui-runtime/src/animation.rs` with:

```rust
//! Animation value hook with per-frame ticking.

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use ui_motion::{sample_tween, Clamp, Ease, Spring, Transition};

use crate::reduced_motion::use_reduced_motion;
use crate::scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};

#[derive(Clone)]
struct AnimationContext {
    handle: Rc<RefCell<Option<FrameHandle>>>,
    velocity: Rc<RefCell<f32>>,
    last_target: Rc<RefCell<f32>>,
}

pub fn use_animation_value(target: f32, transition: Transition) -> ReadSignal<f32> {
    let reduced = use_reduced_motion();
    let mut value = use_signal(|| target);

    let ctx = use_hook(|| AnimationContext {
        handle: Rc::new(RefCell::new(None)),
        velocity: Rc::new(RefCell::new(0.0)),
        last_target: Rc::new(RefCell::new(target)),
    });

    use_effect(move || {
        let current_target = target;
        let mut last = ctx.last_target.borrow_mut();
        if *last == current_target && ctx.handle.borrow().is_some() {
            return;
        }
        *last = current_target;
        drop(last);

        if reduced {
            *ctx.handle.borrow_mut() = None;
            value.set(current_target);
            return;
        }

        let start_value = value();
        let velocity_cell = ctx.velocity.clone();
        let mut signal = value;

        let handle = spawn_frame_loop(move |dt_ms| {
            let current = signal();
            let next = match transition {
                Transition::Tween { duration_ms, ease } => {
                    let progress_step = if duration_ms == 0 {
                        1.0
                    } else {
                        dt_ms as f32 / duration_ms as f32
                    };
                    let next = step_tween(current, current_target, progress_step, ease);
                    if (next - current_target).abs() < 0.001 {
                        signal.set(current_target);
                        return ControlFlow::Stop;
                    }
                    signal.set(next);
                    next
                }
                Transition::Spring(spring) => {
                    let v = *velocity_cell.borrow();
                    let step = spring.step(current, current_target, v, dt_ms as f32 / 1000.0);
                    *velocity_cell.borrow_mut() = step.velocity;
                    signal.set(step.value);
                    if (step.value - current_target).abs() < 0.001
                        && step.velocity.abs() < 0.01
                    {
                        signal.set(current_target);
                        *velocity_cell.borrow_mut() = 0.0;
                        return ControlFlow::Stop;
                    }
                    step.value
                }
            };
            let _ = next;
            ControlFlow::Continue
        });

        *ctx.handle.borrow_mut() = Some(handle);
        // Also nudge the start point in case the caller expects a fresh
        // animation from start_value (not strictly required because the
        // signal starts at the previous value).
        let _ = start_value;
    });

    ReadSignal::from(value)
}

fn step_tween(current: f32, target: f32, progress_step: f32, ease: Ease) -> f32 {
    let raw_progress = progress_step.clamp(0.0, 1.0);
    let eased = ui_motion::apply_ease(raw_progress, ease);
    current + (target - current) * eased
}
```

The hook uses Dioxus's `use_hook` to retain a per-component `AnimationContext` across renders. The `FrameHandle` retained inside it cancels its loop on drop, so replacing it with a new one terminates the old loop. SSR-safe: `use_effect` doesn't run during SSR, so the value stays at the initial `target`.

If `ReadSignal` isn't available in your Dioxus 0.7 version, substitute the alias that the existing animation MVP returned (it was `ReadSignal<f32>` in sub-project 2's implementation).

- [ ] **Step 3: Existing SSR tests still pass**

```powershell
cargo test -p ui-runtime
```

Expected: PASS (the SSR tests in `tests/hooks_ssr.rs` still pass because in SSR the hook returns target synchronously).

- [ ] **Step 4: Commit**

```powershell
git add crates/ui-runtime/src/animation.rs
git commit -m "feat: per-frame ticker in animation value hook"
```

## Task 4: `use_timeline_sample` Hook

**Files:**
- Create: `crates/ui-runtime/src/timeline.rs`
- Modify: `crates/ui-runtime/src/lib.rs`
- Modify: `crates/ui-runtime/tests/hooks_ssr.rs`
- Modify: `crates/ui-runtime/Cargo.toml`

- [ ] **Step 1: Add `ui-timeline` dep**

In `crates/ui-runtime/Cargo.toml`, add to `[dependencies]`:

```toml
ui-timeline = { path = "../ui-timeline" }
```

- [ ] **Step 2: Append SSR tests**

Append to `crates/ui-runtime/tests/hooks_ssr.rs`:

```rust
use ui_timeline::{
    MotionCue, MotionSegment, MotionTarget, Timeline, TimelineClock, TimelineTrack,
};
use ui_runtime::use_timeline_sample;

#[component]
fn TimelineSampleProbe(timeline: Timeline, clock: TimelineClock) -> Element {
    let sample = use_timeline_sample(timeline, clock);
    let opacity = sample().states.first().and_then(|s| s.opacity).unwrap_or(-1.0);
    rsx! {
        div { "data-opacity": "{opacity}" }
    }
}

#[test]
fn use_timeline_sample_in_ssr_returns_initial_sample() {
    let cue = MotionCue::Opacity {
        from: 0.0,
        to: 1.0,
        transition: Transition::Tween {
            duration_ms: 220,
            ease: Ease::Linear,
        },
    };
    let timeline = Timeline::new("t", 220.0).with_track(TimelineTrack::new(
        MotionTarget::node("hero"),
        vec![MotionSegment::new(0.0, 220.0, cue)],
    ));
    let html = dioxus_ssr::render_element(rsx! {
        TimelineSampleProbe {
            timeline: timeline,
            clock: TimelineClock::Manual { elapsed_ms: 110.0 }
        }
    });
    assert!(html.contains("data-opacity=\"0.5\""), "got {html}");
}
```

- [ ] **Step 3: Run and verify failure**

```powershell
cargo test -p ui-runtime
```

Expected: FAIL (`use_timeline_sample` missing).

- [ ] **Step 4: Implement the hook**

Create `crates/ui-runtime/src/timeline.rs`:

```rust
//! Timeline sampling hook.

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use ui_timeline::{Timeline, TimelineClock, TimelineSample};

use crate::reduced_motion::use_reduced_motion;
use crate::scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};

#[derive(Clone)]
struct TimelineRuntime {
    handle: Rc<RefCell<Option<FrameHandle>>>,
    elapsed_ms: Rc<RefCell<f32>>,
}

pub fn use_timeline_sample(timeline: Timeline, clock: TimelineClock) -> ReadSignal<TimelineSample> {
    let reduced = use_reduced_motion();
    let initial = if reduced {
        // Sample at the final frame to render the settled state.
        timeline.sample(TimelineClock::Manual {
            elapsed_ms: timeline.duration_ms,
        })
    } else {
        timeline.sample(clock)
    };
    let mut sample = use_signal(|| initial);

    let runtime = use_hook(|| TimelineRuntime {
        handle: Rc::new(RefCell::new(None)),
        elapsed_ms: Rc::new(RefCell::new(0.0)),
    });

    use_effect(move || {
        if reduced {
            *runtime.handle.borrow_mut() = None;
            sample.set(timeline.sample(TimelineClock::Manual {
                elapsed_ms: timeline.duration_ms,
            }));
            return;
        }

        match clock {
            TimelineClock::Playback { elapsed_ms: start } => {
                *runtime.elapsed_ms.borrow_mut() = start;
                let timeline_clone = timeline.clone();
                let elapsed_cell = runtime.elapsed_ms.clone();
                let mut sample_signal = sample;
                let total = timeline.duration_ms;
                let handle = spawn_frame_loop(move |dt_ms| {
                    let mut elapsed = elapsed_cell.borrow_mut();
                    *elapsed += dt_ms as f32;
                    let now = *elapsed;
                    drop(elapsed);
                    sample_signal.set(timeline_clone.sample(TimelineClock::Playback {
                        elapsed_ms: now,
                    }));
                    if now >= total {
                        return ControlFlow::Stop;
                    }
                    ControlFlow::Continue
                });
                *runtime.handle.borrow_mut() = Some(handle);
            }
            other => {
                // Manual / Frame / Scroll: caller-driven, no scheduler.
                *runtime.handle.borrow_mut() = None;
                sample.set(timeline.sample(other));
            }
        }
    });

    ReadSignal::from(sample)
}
```

- [ ] **Step 5: Register and export**

Update `crates/ui-runtime/src/lib.rs`. Add:

```rust
pub mod timeline;
pub use timeline::use_timeline_sample;
```

near the other `pub mod` / `pub use` lines.

- [ ] **Step 6: Run tests**

```powershell
cargo test -p ui-runtime
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add crates/ui-runtime
git commit -m "feat: add timeline sample hook"
```

## Task 5: `Sequence` Component + `Cue` + `SequenceContext`

**Files:**
- Modify: `crates/ui-dioxus/src/kinetics.rs`
- Modify: `crates/ui-dioxus/src/lib.rs`
- Create: `crates/ui-dioxus/tests/sequence_ssr.rs`

- [ ] **Step 1: Write Sequence SSR tests**

Create `crates/ui-dioxus/tests/sequence_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::{Cue, KineticBox, Sequence};
use ui_motion::{Ease, Transition};
use ui_timeline::{MotionCue, Timeline, TimelineClock, TimelineTrack, MotionSegment, MotionTarget};

fn linear_220() -> Transition {
    Transition::Tween {
        duration_ms: 220,
        ease: Ease::Linear,
    }
}

#[test]
fn sequence_provides_state_map_via_context() {
    let timeline = Timeline::new("hero", 220.0).with_track(TimelineTrack::new(
        MotionTarget::node("title"),
        vec![MotionSegment::new(
            0.0,
            220.0,
            MotionCue::Opacity {
                from: 0.0,
                to: 1.0,
                transition: linear_220(),
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
    assert!(
        html.contains("opacity: 0"),
        "expected KineticBox to write opacity: 0 in inline style; got {html}",
    );
}

#[test]
fn sequence_with_cues_vec_equivalent_to_timeline_prop() {
    let cues = vec![Cue::new(
        "title",
        0.0,
        MotionCue::Opacity {
            from: 0.0,
            to: 1.0,
            transition: linear_220(),
        },
    )];
    let html = dioxus_ssr::render_element(rsx! {
        Sequence {
            cues: Some(cues),
            clock: TimelineClock::Manual { elapsed_ms: 0.0 },
            KineticBox { id: "title", "Hello" }
        }
    });
    assert!(html.contains("opacity: 0"), "got {html}");
}

#[test]
fn sequence_renders_data_timeline_id_attribute() {
    let html = dioxus_ssr::render_element(rsx! {
        Sequence {
            id: "hero".to_string(),
            cues: Some(vec![Cue::new("title", 0.0, MotionCue::Opacity {
                from: 0.0, to: 1.0, transition: linear_220(),
            })]),
            clock: TimelineClock::Manual { elapsed_ms: 0.0 },
            KineticBox { id: "title", "Hello" }
        }
    });
    assert!(html.contains("data-timeline-id=\"hero\""));
}
```

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p ui-dioxus --test sequence_ssr
```

Expected: FAIL (`Sequence`, `Cue` don't exist).

- [ ] **Step 3: Add `Sequence`, `Cue`, `SequenceContext` to `kinetics.rs`**

Append to `crates/ui-dioxus/src/kinetics.rs`:

```rust
use std::collections::HashMap;
use ui_timeline::{
    MotionCue, MotionSegment, MotionTarget, ResolvedMotionState, Timeline, TimelineClock,
    TimelineTrack,
};
use ui_runtime::use_timeline_sample;

#[derive(Clone, Debug)]
pub struct Cue {
    pub target_id: String,
    pub start_ms: f32,
    pub motion: MotionCue,
}

impl Cue {
    pub fn new(target_id: impl Into<String>, start_ms: f32, motion: MotionCue) -> Self {
        Self {
            target_id: target_id.into(),
            start_ms,
            motion,
        }
    }
}

#[derive(Clone, Default)]
pub struct SequenceContext {
    pub states: HashMap<String, ResolvedMotionState>,
}

fn cues_to_timeline(id: &str, cues: Vec<Cue>) -> Timeline {
    let mut max_end = 0.0_f32;
    let mut timeline = Timeline::new(id, 0.0);
    for cue in cues {
        let duration_ms = cue_duration_ms(&cue.motion);
        let end = cue.start_ms + duration_ms;
        if end > max_end {
            max_end = end;
        }
        let track = TimelineTrack::new(
            MotionTarget::node(cue.target_id.clone()),
            vec![MotionSegment::new(cue.start_ms, duration_ms, cue.motion)],
        );
        timeline = timeline.with_track(track);
    }
    Timeline {
        duration_ms: max_end,
        ..timeline
    }
}

fn cue_duration_ms(motion: &MotionCue) -> f32 {
    let transition = match motion {
        MotionCue::Opacity { transition, .. } => transition,
        MotionCue::Translate { transition, .. } => transition,
        MotionCue::Scale { transition, .. } => transition,
        MotionCue::Rotate { transition, .. } => transition,
    };
    match transition {
        ui_motion::Transition::Tween { duration_ms, .. } => *duration_ms as f32,
        ui_motion::Transition::Spring(_) => 600.0,
    }
}

#[component]
pub fn Sequence(
    #[props(default)] timeline: Option<Timeline>,
    #[props(default)] cues: Option<Vec<Cue>>,
    #[props(default = "sequence".to_string())] id: String,
    #[props(default = TimelineClock::Playback { elapsed_ms: 0.0 })] clock: TimelineClock,
    children: Element,
) -> Element {
    let resolved_timeline = timeline
        .clone()
        .or_else(|| cues.clone().map(|cues| cues_to_timeline(&id, cues)));

    let Some(timeline_value) = resolved_timeline else {
        return rsx! {
            section {
                class: "ui-sequence",
                "data-timeline-id": "{id}",
                {children}
            }
        };
    };

    let sample = use_timeline_sample(timeline_value, clock);
    let mut ctx_signal = use_signal(|| SequenceContext::default());
    use_context_provider(|| ctx_signal);

    use_effect(move || {
        let snapshot = sample();
        let mut states = HashMap::new();
        for state in snapshot.states {
            if let MotionTarget::Node(kinetic_id) = &state.target {
                states.insert(kinetic_id.0.clone(), state.clone());
            }
        }
        ctx_signal.set(SequenceContext { states });
    });

    rsx! {
        section {
            class: "ui-sequence",
            "data-timeline-id": "{id}",
            {children}
        }
    }
}
```

- [ ] **Step 4: Update `KineticBox`**

Replace the existing `KineticBox` body in `crates/ui-dioxus/src/kinetics.rs` with:

```rust
#[component]
pub fn KineticBox(
    id: String,
    #[props(default = "fade-in".to_string())] cue: String,
    children: Element,
) -> Element {
    let kinetic_id = KineticId::new(id.clone());
    let style = try_consume_context::<Signal<SequenceContext>>()
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

- [ ] **Step 5: Re-export from `lib.rs`**

In `crates/ui-dioxus/src/lib.rs`, add `Sequence`, `Cue`, `SequenceContext` to the `pub use kinetics::{...};` export list.

- [ ] **Step 6: Run tests**

```powershell
cargo test -p ui-dioxus
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add crates/ui-dioxus
git commit -m "feat: add sequence component with context driven kinetic boxes"
```

## Task 6: `kinetics` Facade Re-Exports

**Files:**
- Modify: `crates/kinetics/src/lib.rs`
- Modify: `crates/kinetics/tests/prelude.rs`

- [ ] **Step 1: Append assertions**

Append to `crates/kinetics/tests/prelude.rs`:

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

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p kinetics public_api_includes_sequence_runtime_names -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Update kinetics re-exports**

In `crates/kinetics/src/lib.rs`, ensure the `pub use ui_dioxus::{...}` block includes `Sequence`, `Cue`, `SequenceContext`. Add to the `pub use ui_timeline::*;` (or appropriate path) so `Axis` and `ResolvedMotionState` are reachable. Behind the existing `#[cfg(feature = "runtime")]` block, add `use_timeline_sample`:

```rust
#[cfg(feature = "runtime")]
pub use ui_runtime::{
    use_animation_value, use_presence_state, use_reduced_motion, use_timeline_sample,
    PresenceState, ReducedMotion,
};
```

Extend `public_api_names()` to push the new strings: `"Sequence"`, `"Cue"`, `"SequenceContext"`, `"Axis"`, `"use_timeline_sample"`, `"ResolvedMotionState"`. Under `#[cfg(feature = "runtime")]` for the hook, plain for the rest.

- [ ] **Step 4: Run tests**

```powershell
cargo test -p kinetics
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add crates/kinetics
git commit -m "feat: re-export sequence runtime through facade"
```

## Task 7: `.ui-sequence` CSS

**Files:**
- Modify: `crates/ui-styles/src/lib.rs`
- Modify: `crates/ui-styles/tests/css.rs`

- [ ] **Step 1: Append test**

Append to `crates/ui-styles/tests/css.rs`:

```rust
#[test]
fn component_css_covers_sequence_wrapper() {
    assert!(COMPONENT_CSS.contains(".ui-sequence"));
}
```

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p ui-styles component_css_covers_sequence_wrapper -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Add CSS**

Append inside `COMPONENT_CSS` in `crates/ui-styles/src/lib.rs`, before the closing `"#;`:

```css
.ui-sequence {
    display: block;
}
```

- [ ] **Step 4: Run tests**

```powershell
cargo test -p ui-styles
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add crates/ui-styles
git commit -m "style: add sequence wrapper selector"
```

## Task 8: Gallery `Sequence` Preview

**Files:**
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/tests/gallery.rs`

- [ ] **Step 1: Append gallery test**

Append to `examples/component-gallery/tests/gallery.rs`:

```rust
#[test]
fn gallery_sequence_preview_renders_three_cues_with_inline_styles() {
    let docs = component_gallery::component_docs();
    let s = docs
        .iter()
        .find(|d| d.name == "Sequence")
        .expect("Sequence doc exists");
    assert_eq!(s.status, component_gallery::ComponentStatus::Ready);
    assert!(s.render.is_some());

    let html = dioxus_ssr::render_element(rsx! {
        component_gallery::App {}
    });

    let inline_style_count = html.matches("style=\"opacity").count()
        + html.matches("style=\"transform").count();
    assert!(
        inline_style_count >= 3,
        "expected at least 3 inline-style KineticBox descendants in the Sequence preview; got {inline_style_count}",
    );
}
```

- [ ] **Step 2: Run and verify failure**

```powershell
cargo test -p component-gallery gallery_sequence_preview_renders_three_cues_with_inline_styles -- --exact
```

Expected: FAIL.

- [ ] **Step 3: Add Sequence registry entry**

In `examples/component-gallery/src/docs.rs`:

(a) Increment the `COMPONENT_DOCS` array length by 1.

(b) Insert this entry adjacent to the existing `TimelineScope` entry, after it:

```rust
ComponentDoc {
    name: "Sequence",
    category: ComponentCategory::Motion,
    status: ComponentStatus::Ready,
    summary: "Orchestrates multiple kinetic boxes through a coordinated timeline of property cues.",
    snippet: SEQUENCE_SNIPPET,
    accessibility: "The sample is deterministic per clock; reduced-motion policies render the settled state.",
    render: Some(sequence_preview),
},
```

(c) Add the snippet near other `*_SNIPPET` constants:

```rust
const SEQUENCE_SNIPPET: &str = r#"Sequence {
    cues: Some(vec![
        Cue::new("title", 0.0, MotionCue::Opacity { from: 0.0, to: 1.0, transition: tween(220) }),
        Cue::new("body", 120.0, MotionCue::Translate { axis: Axis::Y, from: 12.0, to: 0.0, transition: tween(200) }),
        Cue::new("cta", 320.0, MotionCue::Scale { from: 0.94, to: 1.0, transition: tween(240) }),
    ]),
    KineticBox { id: "title", h3 { "Welcome" } }
    KineticBox { id: "body", p { "Subtle entry" } }
    KineticBox { id: "cta", Button { "Get started" } }
}"#;
```

(d) Add the preview function:

```rust
fn sequence_preview() -> Element {
    let tween_short = Transition::Tween {
        duration_ms: 220,
        ease: Ease::Standard,
    };
    let tween_med = Transition::Tween {
        duration_ms: 200,
        ease: Ease::Standard,
    };
    let tween_long = Transition::Tween {
        duration_ms: 240,
        ease: Ease::Standard,
    };
    let cues = vec![
        Cue::new(
            "title",
            0.0,
            MotionCue::Opacity {
                from: 0.0,
                to: 1.0,
                transition: tween_short,
            },
        ),
        Cue::new(
            "body",
            120.0,
            MotionCue::Translate {
                axis: Axis::Y,
                from: 12.0,
                to: 0.0,
                transition: tween_med,
            },
        ),
        Cue::new(
            "cta",
            320.0,
            MotionCue::Scale {
                from: 0.94,
                to: 1.0,
                transition: tween_long,
            },
        ),
    ];

    rsx! {
        Sequence {
            cues: Some(cues),
            clock: TimelineClock::Manual { elapsed_ms: 560.0 },
            KineticBox { id: "title",
                h4 { "Welcome" }
            }
            KineticBox { id: "body",
                p { "Subtle entry choreography" }
            }
            KineticBox { id: "cta",
                Button { "Get started" }
            }
        }
    }
}
```

If `MotionCue`, `Transition`, `Ease`, `Axis`, `TimelineClock`, `Cue`, `Sequence`, `KineticBox`, `Button` aren't in scope via `use kinetics::prelude::*;`, add the missing imports.

The preview uses `TimelineClock::Manual { elapsed_ms: 560.0 }` (past the longest cue end) so the SSR HTML shows the settled (final) state — opacity 1.0, translate y 0, scale 1.0. The test requires inline styles to be present, and the settled state still produces `style="opacity: 1` and `style="transform: translate(0px, 0px) scale(1)"` strings.

- [ ] **Step 4: Run tests**

```powershell
cargo test -p component-gallery
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add examples/component-gallery
git commit -m "feat: add sequence preview to gallery"
```

## Task 9: README Component List

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Update the ready components list**

In `README.md`, find the "Ready rendered components" bullet list (or the equivalent). Add `- \`Sequence\`` adjacent to other Motion components.

- [ ] **Step 2: Run README test**

```powershell
cargo test -p component-gallery root_readme_mentions_component_gallery -- --exact
cargo test -p component-gallery root_readme_uses_kinetics_crate_name -- --exact
```

Expected: PASS.

- [ ] **Step 3: Commit**

```powershell
git add README.md
git commit -m "docs: note sequence in readme components list"
```

## Task 10: Full Verification

- [ ] **Step 1: Format check**

```powershell
cargo fmt --all -- --check
```

If it fails, run `cargo fmt --all` and commit with `style: apply rustfmt`.

- [ ] **Step 2: Full workspace tests**

```powershell
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 3: Wasm target sanity**

```powershell
cargo check -p kinetics --target wasm32-unknown-unknown
cargo check -p ui-runtime --target wasm32-unknown-unknown
cargo check -p ui-dioxus --target wasm32-unknown-unknown
```

Expected: all PASS.

- [ ] **Step 4: Gallery compile**

```powershell
cargo check -p component-gallery
```

Expected: PASS.

- [ ] **Step 5: Scope sanity**

```powershell
rg -n "name: \"SharedLayout\"|name: \"SharedElement\"" examples/component-gallery/src/docs.rs
```

These entries should still be `ComponentStatus::ComingSoon` and `render: None`. Verify by reading those entries.

- [ ] **Step 6: Acceptance checklist**

Manually confirm each item from the spec's Acceptance Checklist. If all green, hand off to `superpowers:finishing-a-development-branch`.

## Acceptance Checklist

- [ ] `MotionCue` has variants `Opacity`, `Translate`, `Scale`, `Rotate`.
- [ ] `Axis` enum exists with `X` and `Y` variants.
- [ ] `MotionCueSample` and `ResolvedMotionState` carry all four optional fields plus `opacity`.
- [ ] `ResolvedMotionState::inline_style()` returns valid CSS.
- [ ] `use_animation_value` per-frame ticks (verified manually via gallery; SSR tests still pass).
- [ ] `use_timeline_sample` recomputes the sample for Playback and uses the input clock for Manual/Frame/Scroll.
- [ ] `Sequence` accepts either `timeline` or `cues` prop.
- [ ] `Sequence` provides `Signal<SequenceContext>` via Dioxus context.
- [ ] `KineticBox` reads context and writes inline style when present.
- [ ] `KineticBox` outside a `Sequence` renders as before.
- [ ] `Sequence` is `Ready` in the gallery with a 3-cue preview.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo test --workspace` passes.
- [ ] `cargo check -p kinetics --target wasm32-unknown-unknown` passes.
- [ ] `TimelineScope`, `PresenceGate`, `Presence`, `IconButton` unchanged.
- [ ] `SharedLayout`, `SharedElement` remain `ComingSoon`.
