---
name: kinetics-scene
description: Use when authoring or modifying Dioxus Kinetics Scene compositions. Covers Scene/Clip/SceneDriver, SplitText/MotionPath, ui-blocks catalog, reduced-motion patterns, and workspace TDD conventions. Trigger on requests to build cinematic scenes, scroll-driven storytelling, animated text, motion paths, or any composition using kinetics::prelude.
---

# kinetics-scene — authoring Scene compositions

This skill teaches how to author and modify kinetics Scene
compositions in the `dioxus-kinetics` workspace.

## Quick start

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_composition::ClipFill;

#[component]
fn HelloScene() -> Element {
    rsx! {
        Scene {
            id: "hello",
            width: 1280,
            height: 720,
            duration_ms: 5_000.0,
            autoplay: Some(true),
            controls: Some(true),
            Clip { start_ms: 0.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "hello-title",
                    text: "Hello, kinetics.".to_string(),
                    cue: "rise-in",
                }
            }
            Clip { start_ms: 1_500.0, duration_ms: 3_000.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "hello-body",
                    text: "Composable cinematic motion.".to_string(),
                    cue: "fade-in",
                }
            }
        }
    }
}
```

## Drivers

`Scene` accepts an optional `driver: Option<SceneDriver>` that selects
how the clock advances:

- `SceneDriver::Autoplay` (default if `driver = None` and
  `autoplay = true`) — clock advances via the platform frame loop.
- `SceneDriver::Manual` — clock only moves via explicit `seek_*`.
  Pick this for scenes driven externally (the `kinetics-render`
  pipeline uses `Manual`).
- `SceneDriver::Scroll(ScrollObserverConfig::new("#trigger"))` —
  scroll position drives the clock. Web-only; native targets hold at
  progress 0.

```rust
Scene {
    id: "scroll-pinned",
    duration_ms: 10_000.0,
    driver: Some(SceneDriver::Scroll(
        ScrollObserverConfig::new("#story-trigger"),
    )),
    /* ... */
}
```

## Clips

`Clip { start_ms, duration_ms, fill }` gates a child element's
visibility based on the parent clock:

- `ClipFill::None` (default) — visible inside `[start, start+duration)`.
- `ClipFill::HoldStart` — visible before `start` too.
- `ClipFill::HoldEnd` — visible after `start+duration` too (use this
  for clips you want to "stay visible" at the settled state).
- `ClipFill::HoldBoth` — always visible.

## Per-glyph text (SplitText)

```rust
TimelineScope { id: "title-timeline", autoplay: true,
    SplitText {
        text: "Hello, world.".to_string(),
        split_by: Some(SplitMode::Character), // or Word
    }
}
```

- Parent carries `aria-label = "<full text>"`.
- Per-glyph spans set `aria-hidden = "true"`.
- A surrounding `TimelineScope` walks the `data-stagger-index`
  attribute to animate glyphs in sequence.

## Curved motion (MotionPath)

```rust
use ui_motion::{Ease, Transition};
use ui_timeline::{MotionSegment, MotionTarget, Timeline, TimelineTrack};

let pts = vec![
    PathPoint::Line { end: (0.0, 0.0) },
    PathPoint::Bezier {
        control_1: (200.0, -200.0),
        control_2: (400.0, 200.0),
        end: (600.0, 0.0),
    },
];
let cue = MotionCue::Path {
    points: pts.clone(),
    from_progress: 0.0,
    to_progress: 1.0,
    rotate_along_path: false,
    transition: Transition::Tween { duration_ms: 4_000, ease: Ease::Standard },
};
let timeline = Timeline::new("trajectory", 4_000.0).with_track(
    TimelineTrack::new(
        MotionTarget::node("icon"),
        vec![MotionSegment::new(0.0, 4_000.0, cue)],
    ),
);

rsx! {
    Sequence {
        timeline: Some(timeline),
        clock: TimelineClock::Manual { elapsed_ms: 0.0 },
        MotionPath { id: "icon".to_string(), path: pts, duration_ms: 4_000.0,
            KineticBox { id: "icon", "•" }
        }
    }
}
```

Sampling is arc-length-uniform — equal `t` covers equal visual
distance. `rotate_along_path: true` emits a tangent angle so the
KineticBox rotates to match the curve direction.

## Catalog blocks (ui-blocks)

Five reusable cinematic blocks. Compose them into Scenes:

- `LowerThird { name, role, accent }` — chyron with name + role.
- `Caption { text, reading_pace_ms_per_word }` — subtitle bar with
  per-word stagger.
- `WipeTransition { duration_ms, angle_deg, children }` — CSS mask
  sweep across children.
- `MetricCounter { label, value, delta_text }` — three-line metric
  display.
- `SocialOverlay { platform, handle, message }` — notification card
  with platform accent.

## Reduced motion

Every component respects the `ReducedMotion` context:

- `Scene` settles immediately at `duration_ms` and disables the
  scrubber when reduced.
- Adapters render the final, settled state when their `reduced`
  flag is set.
- `MotionPath` collapses to the endpoint position.
- `SplitText` renders glyphs at final state, no stagger.

Wrap a subtree in `ReducedMotionProvider { reduced: Some(true), ... }`
to force reduced motion (e.g. for testing). Without the prop, the
provider reads `prefers-reduced-motion` from the browser and the
`data-ui-motion="reduced"` attribute from the document body.

## Accessibility

- `SplitText`: parent `aria-label` always carries the unsplit text;
  glyph spans set `aria-hidden = "true"` so screen readers do not
  enumerate.
- Scene-level decoration (icons, animated `MotionPath` glyphs) should
  not have aria labels — they're visual flourish.
- Always test reduced-motion paths — they're the canonical
  "this is what the scene looks like at rest" state.

## Workspace conventions

- TDD: write the failing test first, run it red, implement, run green,
  commit. Each step is one commit-able action.
- Signal writes use the `let mut s = …; s.set(…);` idiom, not
  `signal.clone().set(…)`. `Signal<T>` is `Copy` in Dioxus 0.7;
  the `.clone()` form was a workaround that's gone from the workspace.
- Conventional Commits with a `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>`
  trailer when an AI agent participated.
- Tests live in `tests/<name>.rs` (integration-style) — the workspace
  precedent for `dioxus-ssr` tests.
- Never push, amend, or `--no-verify` without explicit user request.
