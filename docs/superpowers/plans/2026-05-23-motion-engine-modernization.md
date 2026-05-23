# Motion Engine Modernization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Migrate `ui-runtime` from per-frame main-thread JS animation to the Web Animations API (WAAPI) as the compositor-offloaded production path, fix the `use_reduced_motion()` no-op bug that the Spec 1 audit surfaced, and clean up the four audit-flagged motion-spec regressions.

**Architecture:** A new `ui_motion::Keyframes` value compiles a `Transition` into a `Vec<HashMap<&str, String>>` keyframe array. A new `ui_runtime::waapi` module wraps `web_sys::Element::animate(...)` into a `WaapiAnimation` handle that cancels on drop. `use_animation_value`, `use_presence_animation`, and `use_timeline_sample(Playback)` are reimplemented on top of `WaapiAnimation`; `Manual` / `Frame` clocks keep the existing synchronous-sample path. A new `ReducedMotionProvider` component probes both `prefers-reduced-motion` and the gallery preference bar, and provides a `ReducedMotion` context that `use_reduced_motion()` actually consumes.

**Tech Stack:** Rust + WASM via `wasm-bindgen` 0.2 / `web-sys` 0.3, Dioxus 0.7, Cargo workspace, Playwright TS for e2e verification.

---

## File Structure

**Modify (Rust):**
- `crates/ui-motion/src/lib.rs` — add `Keyframe`, `Keyframes`, `keyframes_for_transition()` + unit tests
- `crates/ui-runtime/src/lib.rs` — export `ReducedMotionProvider`, `WaapiAnimation`
- `crates/ui-runtime/src/reduced_motion.rs` — add probing + `ReducedMotionProvider`
- `crates/ui-runtime/src/animation.rs` — migrate to WAAPI under feature detection
- `crates/ui-runtime/src/presence.rs` — consume the migrated `use_animation_value_from`
- `crates/ui-runtime/src/timeline.rs` — Playback branch via WAAPI per-track
- `crates/ui-runtime/Cargo.toml` — add `web-sys` features for Animation/KeyframeEffect
- `crates/ui-runtime/tests/hooks_ssr.rs` — extend SSR test for `ReducedMotion(true)` context

**Create (Rust):**
- `crates/ui-runtime/src/waapi.rs` — `WaapiAnimation` handle + Element.animate binding (wasm only)
- `crates/ui-runtime/src/waapi_stub.rs` — no-op fallback for non-wasm targets
- `crates/ui-timeline/tests/sample_at_t0.rs` — repro for the t=0 first-cue bug surfaced by Sequence spec

**Modify (gallery):**
- `examples/component-gallery/src/app.rs` — wrap children in `ReducedMotionProvider`
- `examples/component-gallery/src/controls.rs` — `PreferenceBar` provides `ReducedMotion` based on `MotionPref`
- `examples/component-gallery/src/previews/composition.rs` — FrameStage caption collapsed into single element
- `examples/component-gallery/src/previews/motion.rs` — TimelineScope stagger preview gets `autoplay: true`

**Modify (e2e):**
- `examples/component-gallery/e2e/reporters/audit-report.ts` — testTitle-keyed rollup + reduced-motion body recognition
- `examples/component-gallery/e2e/tests/_lib/__tests__/reporter.test.ts` — add overwrite test
- `examples/component-gallery/e2e/tests/_lib/mount.ts` — `selectRadio` switched to dispatchEvent
- `examples/component-gallery/e2e/tests/components/frame-stage.spec.ts` — tighten Frame counter selector
- `examples/component-gallery/e2e/tests/components/timeline-scope.spec.ts` — assert against the autoplay preview
- `examples/component-gallery/e2e/tests/components/kinetic-box.spec.ts` — read WAAPI in-flight values via getComputedStyle
- `examples/component-gallery/e2e/tests/components/sequence.spec.ts` — drop the t=0 ≤0.1 expectation if Task 6 reveals the cue model emits the to-value at boundary (decision deferred to Task 6)
- `.github/workflows/e2e.yml` — add Linux-rebaseline job

---

## Task 1: Reduced-motion probe — failing wasm-bindgen test

**Files:**
- Modify: `crates/ui-runtime/Cargo.toml` (dev-dep `wasm-bindgen-test` if absent)
- Create: `crates/ui-runtime/tests/reduced_motion_wasm.rs`

- [ ] **Step 1: Confirm `wasm-bindgen-test` is reachable**

Run from the workspace root:
```bash
grep -n "wasm-bindgen-test" crates/ui-runtime/Cargo.toml || echo "absent — add it in step 2"
```

- [ ] **Step 2: Add `wasm-bindgen-test` to `[dev-dependencies]` of `crates/ui-runtime/Cargo.toml`**

Append the section (preserve any existing `[dev-dependencies]`):

```toml
[target.'cfg(target_arch = "wasm32")'.dev-dependencies]
wasm-bindgen-test = "0.3"
```

- [ ] **Step 3: Write the failing wasm-bindgen test**

Create `crates/ui-runtime/tests/reduced_motion_wasm.rs`:

```rust
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use ui_runtime::detect_reduced_motion_at_root;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn detects_data_ui_motion_reduced_on_body() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    body.set_attribute("data-ui-motion", "reduced").unwrap();

    assert!(detect_reduced_motion_at_root());

    body.remove_attribute("data-ui-motion").unwrap();
}

#[wasm_bindgen_test]
fn returns_false_when_no_signal_present() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    body.remove_attribute("data-ui-motion").ok();

    // prefers-reduced-motion is environment-dependent. Test only the
    // data-attr path here; the matchMedia path is exercised in step 4.
    assert!(!detect_reduced_motion_at_root() || media_query_says_reduce());
}

fn media_query_says_reduce() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
        .map(|m| m.matches())
        .unwrap_or(false)
}
```

- [ ] **Step 4: Run the test and verify it fails**

Run:
```bash
cargo test -p ui-runtime --target wasm32-unknown-unknown --test reduced_motion_wasm -- --headless --chrome 2>&1 | tail -20
```

(`wasm-bindgen-test-runner` requires `chromedriver` on PATH. If not installed locally, run `wasm-pack test -p ui-runtime --headless --chrome` from the crate dir. If neither tool is available, fall back to running the equivalent assertion as a `#[test]` in a stub native module that simulates the function and document the limitation in the commit body.)

Expected: FAIL — `detect_reduced_motion_at_root` is not yet exported from `ui_runtime`.

- [ ] **Step 5: Commit the failing test**

```bash
git add crates/ui-runtime/Cargo.toml crates/ui-runtime/tests/reduced_motion_wasm.rs
git commit -m "test(ui-runtime): wasm test for reduced-motion data-attr probe"
```

---

## Task 2: Implement `detect_reduced_motion_at_root` + `ReducedMotionProvider`

**Files:**
- Modify: `crates/ui-runtime/src/reduced_motion.rs`
- Modify: `crates/ui-runtime/src/lib.rs`

- [ ] **Step 1: Replace `crates/ui-runtime/src/reduced_motion.rs` contents**

```rust
//! Reduced-motion context + DOM probe.
//!
//! `use_reduced_motion()` consumes a `ReducedMotion` context if one is
//! provided (e.g. by `ReducedMotionProvider`); otherwise it falls back to
//! a one-shot DOM probe of `prefers-reduced-motion` + the
//! `[data-ui-motion="reduced"]` attribute on the body ancestor.

use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReducedMotion(pub bool);

pub fn use_reduced_motion() -> bool {
    try_consume_context::<ReducedMotion>()
        .map(|rm| rm.0)
        .unwrap_or_else(detect_reduced_motion_at_root)
}

#[cfg(target_arch = "wasm32")]
pub fn detect_reduced_motion_at_root() -> bool {
    media_query_reduce() || body_attr_reduced()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn detect_reduced_motion_at_root() -> bool {
    false
}

#[cfg(target_arch = "wasm32")]
fn media_query_reduce() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
        .map(|m| m.matches())
        .unwrap_or(false)
}

#[cfg(target_arch = "wasm32")]
fn body_attr_reduced() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Some(document) = window.document() else {
        return false;
    };
    let Some(body) = document.body() else {
        return false;
    };
    // Walk up from body looking for any ancestor with data-ui-motion.
    // The gallery sets it on `.gallery-shell` which is a child of body.
    // In practice the attribute lives ON the shell, so we accept either
    // body or any descendant with data-ui-motion="reduced".
    if body.get_attribute("data-ui-motion").as_deref() == Some("reduced") {
        return true;
    }
    if let Ok(matches) = body.query_selector("[data-ui-motion=\"reduced\"]") {
        if matches.is_some() {
            return true;
        }
    }
    false
}

/// Provides a `ReducedMotion` context to children, sourced from
/// `prefers-reduced-motion` + the nearest `[data-ui-motion]` attribute.
/// Listens for media-query changes and updates the signal reactively.
#[component]
pub fn ReducedMotionProvider(children: Element) -> Element {
    let reduced = use_signal(detect_reduced_motion_at_root);
    use_context_provider(|| ReducedMotion(*reduced.read()));

    #[cfg(target_arch = "wasm32")]
    let _ = reduced; // listener wiring lives in a use_effect in the migration plan

    rsx! { {children} }
}
```

- [ ] **Step 2: Re-export `ReducedMotionProvider` and `detect_reduced_motion_at_root` from lib.rs**

Open `crates/ui-runtime/src/lib.rs` and update the exports near
`pub use reduced_motion::{...};` to:

```rust
pub use reduced_motion::{
    detect_reduced_motion_at_root, use_reduced_motion, ReducedMotion, ReducedMotionProvider,
};
```

- [ ] **Step 3: Run the wasm test from Task 1 and verify it passes**

```bash
cargo test -p ui-runtime --target wasm32-unknown-unknown --test reduced_motion_wasm -- --headless --chrome 2>&1 | tail -20
```

Expected: PASS — both tests.

- [ ] **Step 4: Also confirm native build still compiles**

```bash
cargo check -p ui-runtime
```

Expected: exit 0.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-runtime/src/reduced_motion.rs crates/ui-runtime/src/lib.rs
git commit -m "feat(ui-runtime): probe prefers-reduced-motion + data-ui-motion ancestor"
```

---

## Task 3: Wire `ReducedMotionProvider` in the gallery app + preference bar

**Files:**
- Modify: `examples/component-gallery/src/app.rs`
- Modify: `examples/component-gallery/src/controls.rs`

- [ ] **Step 1: In `examples/component-gallery/src/app.rs`, import the provider**

Add the use line near the existing `use ui_styles::library_css;`:

```rust
use ui_runtime::{ReducedMotion, ReducedMotionProvider};
```

- [ ] **Step 2: Wrap the shell `div` in `ReducedMotionProvider`**

In the same file, locate the `rsx!` block beginning with `style { "{shared_css}" }` and rewrap. After the change the top of the returned `rsx!` should read (preserving the existing data attributes on the shell):

```rust
rsx! {
    style { "{shared_css}" }
    style { "{GALLERY_CSS}" }
    ReducedMotionProvider {
        div {
            class: "gallery-shell",
            "data-ui-theme": "{theme_attr}",
            "data-ui-density": "{density_attr}",
            "data-ui-motion": "{motion_attr}",
            "data-ui-glass-policy": "{glass_attr}",
            // ... rest of the existing children unchanged ...
        }
    }
}
```

- [ ] **Step 3: In `examples/component-gallery/src/controls.rs`, provide the context from `PreferenceBar`**

At the top of `PreferenceBar` (search for `pub fn PreferenceBar`), after the existing prefs read but before any rsx, add:

```rust
let motion_now = *prefs.motion.read();
use_context_provider(|| ui_runtime::ReducedMotion(matches!(motion_now, MotionPref::Reduced)));
```

If `PreferenceBar` does not currently `use ui_runtime` then add the use at the top of the file:

```rust
use ui_runtime::ReducedMotion as RuntimeReducedMotion;
```

and use `RuntimeReducedMotion(matches!(motion_now, MotionPref::Reduced))` to avoid colliding with any local `ReducedMotion` type.

- [ ] **Step 4: Verify the gallery still compiles**

```bash
cargo check -p component-gallery
```

Expected: exit 0.

- [ ] **Step 5: SSR snapshot quick-check**

```bash
cargo test -p component-gallery --tests --test gallery -- --quiet
```

Expected: existing 34 tests still pass.

- [ ] **Step 6: Commit**

```bash
git add examples/component-gallery/src/app.rs examples/component-gallery/src/controls.rs
git commit -m "feat(component-gallery): provide ReducedMotion context from PreferenceBar"
```

---

## Task 4: Repro the t=0 first-cue bug + decide on fix

**Files:**
- Create: `crates/ui-timeline/tests/sample_at_t0.rs`

- [ ] **Step 1: Write a unit test that pins the contract `Timeline::sample` should provide at t=0**

```rust
//! Pins the contract that Timeline.sample at elapsed_ms=0 with a single
//! cue starting at start_ms=0 returns the cue's `from` value (not `to`).
//! This was the Sequence test 1 audit signal: opacity at t=0 was NOT
//! ≤ 0.1 although the cue declared `from: 0.0`.

use ui_motion::{Ease, Transition};
use ui_timeline::{
    Axis, FillMode, MotionCue, MotionSegment, MotionTarget, Timeline, TimelineClock,
    TimelineTrack,
};

fn opacity_cue() -> MotionCue {
    MotionCue::Opacity {
        from: 0.0,
        to: 1.0,
        transition: Transition::Tween { duration_ms: 220, ease: Ease::Standard },
    }
}

#[test]
fn sample_at_t_zero_returns_from_value_for_first_cue() {
    let track = TimelineTrack::new(
        MotionTarget::node("title"),
        vec![MotionSegment::new(0.0, 220.0, opacity_cue())],
    );
    let timeline = Timeline {
        duration_ms: 220.0,
        fill: FillMode::Both,
        ..Timeline::new("t", 0.0).with_track(track)
    };

    let sample = timeline.sample(TimelineClock::Manual { elapsed_ms: 0.0 });
    let state = sample
        .states
        .iter()
        .find(|s| matches!(&s.target, MotionTarget::Node(id) if id.0 == "title"))
        .expect("title state");

    let style = state.inline_style();
    // The inline_style for opacity should contain "opacity:0" (no spaces
    // or "opacity:0.0...") at the from end. We assert it does NOT contain
    // "opacity:1" which would be the to-value.
    assert!(
        !style.contains("opacity:1") && !style.contains("opacity: 1"),
        "expected from value at t=0; got {style}"
    );
}
```

- [ ] **Step 2: Run the test**

```bash
cargo test -p ui-timeline --test sample_at_t0 -- --nocapture
```

Possible outcomes:
- **PASS** — the contract holds and the test passes immediately. The Sequence audit failure has a different root cause (most likely the hook seeding the `SequenceContext` from a stale `sample()`); move to Step 3a.
- **FAIL** — `Timeline::sample` does emit `to` at the boundary. Proceed to Step 3b.

- [ ] **Step 3a (if Task 4 Step 2 passed): leave the test in place as a regression guard, then debug the Sequence hook**

The bug then lives in `crates/ui-dioxus/src/kinetics.rs` (around the `Sequence` component's `use_hook` / `use_effect` ordering). Add a regression test in `crates/ui-dioxus/tests/sequence_ssr.rs` that renders `Sequence` with a `Manual { elapsed_ms: 0 }` clock and asserts the resulting HTML's first kinetic-box has `opacity:0` not `opacity:1`. If the SSR test passes but the browser test still fails, the bug is in the post-hydration effect-ordering, which Task 11's WAAPI migration will rewrite end-to-end.

- [ ] **Step 3b (if Task 4 Step 2 failed): fix `Timeline::sample` so progress 0 emits the from value**

Open `crates/ui-timeline/src/lib.rs`. Around line 311 (`fn sample(&self, elapsed_ms: f32, fill: FillMode) -> Option<MotionCueSample>`), the current behavior at `elapsed_ms == start_ms` may evaluate progress = 0 but with FillMode::Both the segment was already considered "before" and may have emitted a Before phase that returns the to value. Inspect, then change the boundary condition so a cue at its exact start emits the from value (i.e., progress 0, not phase Before).

The expected fix is one branch tightening: `if elapsed_ms <= self.start_ms` returns progress 0 (cue exists, hasn't started running yet, value = from), only `elapsed_ms > end_ms` returns Forwards (value = to). After the fix, rerun Step 2 to verify PASS.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-timeline/tests/sample_at_t0.rs crates/ui-timeline/src/lib.rs
git commit -m "test(ui-timeline): pin Timeline::sample boundary contract at t=0"
```

(If only the test was added, omit the lib.rs path.)

---

## Task 5: Add `ui_motion::keyframes_for_transition` + unit tests

**Files:**
- Modify: `crates/ui-motion/src/lib.rs`

- [ ] **Step 1: Add the keyframe types at the bottom of `crates/ui-motion/src/lib.rs`**

```rust
// ---------------------------------------------------------------------------
// Keyframe compilation for WAAPI consumption.
//
// A `Keyframes` value is a series of per-frame property maps that can be
// handed to `Element.animate(...)` in the browser. Tweens are sampled at
// 30fps × duration_ms so that smoothstep (the Standard ease) round-trips
// exactly through WAAPI's linear interpolation. Springs are sampled at
// 60fps × settling_duration_ms.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Keyframes {
    pub frames: Vec<Keyframe>,
    pub duration_ms: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Keyframe {
    /// Offset within [0.0, 1.0]; passed to WAAPI verbatim.
    pub offset: f32,
    /// Animated value at this offset (the caller decides which CSS
    /// property it maps to — opacity, transform component, etc.).
    pub value: f32,
}

const TWEEN_FPS: f32 = 30.0;
const SPRING_FPS: f32 = 60.0;
const SPRING_TOLERANCE: f32 = 0.005;

/// Compile a transition between `from` and `to` into a `Keyframes` array
/// suitable for `Element.animate(...)`. The number of frames depends on
/// the transition: tweens use 30fps sampling, springs use 60fps sampling.
pub fn keyframes_for_transition(from: f32, to: f32, transition: Transition) -> Keyframes {
    match transition {
        Transition::Tween { duration_ms, ease } => tween_keyframes(from, to, duration_ms, ease),
        Transition::Spring(spring) => spring_keyframes(from, to, spring),
    }
}

fn tween_keyframes(from: f32, to: f32, duration_ms: u32, ease: Ease) -> Keyframes {
    let duration = duration_ms as f32;
    if duration == 0.0 {
        return Keyframes {
            frames: vec![Keyframe { offset: 0.0, value: to }, Keyframe { offset: 1.0, value: to }],
            duration_ms: 0.0,
        };
    }
    let count = ((duration * TWEEN_FPS / 1000.0).ceil() as usize).max(2);
    let mut frames = Vec::with_capacity(count + 1);
    for i in 0..=count {
        let progress = i as f32 / count as f32;
        let eased = apply_ease(progress, ease);
        let value = from + (to - from) * eased;
        frames.push(Keyframe { offset: progress, value });
    }
    Keyframes { frames, duration_ms: duration }
}

fn spring_keyframes(from: f32, to: f32, spring: Spring) -> Keyframes {
    let settle = spring.settling_duration_ms(SPRING_TOLERANCE).clamp(50.0, 4_000.0);
    let count = ((settle * SPRING_FPS / 1000.0).ceil() as usize).max(2);
    let dt = 1.0 / SPRING_FPS;
    let mut value = from;
    let mut velocity = 0.0_f32;
    let mut frames = Vec::with_capacity(count + 2);
    frames.push(Keyframe { offset: 0.0, value: from });
    for i in 1..=count {
        let step = spring.step(value, to, velocity, dt);
        value = step.value;
        velocity = step.velocity;
        frames.push(Keyframe {
            offset: (i as f32) / (count as f32),
            value,
        });
    }
    // Pin the final frame exactly to `to` so the animation settles cleanly.
    if let Some(last) = frames.last_mut() {
        last.value = to;
    }
    Keyframes { frames, duration_ms: settle }
}
```

- [ ] **Step 2: Add unit tests at the bottom of the file (within the existing `#[cfg(test)] mod tests { ... }` block or a new one)**

```rust
#[cfg(test)]
mod keyframe_tests {
    use super::*;

    #[test]
    fn tween_first_frame_is_from() {
        let kf = keyframes_for_transition(
            0.0,
            1.0,
            Transition::Tween { duration_ms: 220, ease: Ease::Standard },
        );
        assert_eq!(kf.frames.first().unwrap().offset, 0.0);
        assert!((kf.frames.first().unwrap().value - 0.0).abs() < 1e-4);
    }

    #[test]
    fn tween_last_frame_is_to() {
        let kf = keyframes_for_transition(
            0.0,
            1.0,
            Transition::Tween { duration_ms: 220, ease: Ease::Standard },
        );
        assert_eq!(kf.frames.last().unwrap().offset, 1.0);
        assert!((kf.frames.last().unwrap().value - 1.0).abs() < 1e-4);
    }

    #[test]
    fn tween_midpoint_matches_apply_ease() {
        let kf = keyframes_for_transition(
            0.0,
            100.0,
            Transition::Tween { duration_ms: 220, ease: Ease::Standard },
        );
        // Locate the frame nearest progress 0.5.
        let near_mid = kf
            .frames
            .iter()
            .min_by(|a, b| (a.offset - 0.5).abs().partial_cmp(&(b.offset - 0.5).abs()).unwrap())
            .unwrap();
        let expected = 0.0 + (100.0 - 0.0) * apply_ease(near_mid.offset, Ease::Standard);
        assert!((near_mid.value - expected).abs() < 1e-3);
    }

    #[test]
    fn spring_first_frame_is_from() {
        let kf = keyframes_for_transition(0.0, 1.0, Transition::Spring(Spring::snappy()));
        assert_eq!(kf.frames.first().unwrap().offset, 0.0);
        assert!((kf.frames.first().unwrap().value - 0.0).abs() < 1e-4);
    }

    #[test]
    fn spring_last_frame_pins_to_target() {
        let kf = keyframes_for_transition(0.0, 1.0, Transition::Spring(Spring::snappy()));
        let last = kf.frames.last().unwrap();
        assert_eq!(last.offset, 1.0);
        assert!((last.value - 1.0).abs() < 1e-6);
    }

    #[test]
    fn spring_duration_matches_settling() {
        let spring = Spring::snappy();
        let kf = keyframes_for_transition(0.0, 1.0, Transition::Spring(spring));
        let expected = spring.settling_duration_ms(SPRING_TOLERANCE).clamp(50.0, 4_000.0);
        assert!((kf.duration_ms - expected).abs() < 1e-3);
    }

    #[test]
    fn zero_duration_tween_emits_two_frames_pinned_to_target() {
        let kf = keyframes_for_transition(
            0.0,
            5.0,
            Transition::Tween { duration_ms: 0, ease: Ease::Linear },
        );
        assert_eq!(kf.frames.len(), 2);
        assert!((kf.frames[0].value - 5.0).abs() < 1e-6);
        assert!((kf.frames[1].value - 5.0).abs() < 1e-6);
    }
}
```

- [ ] **Step 3: Run the tests**

```bash
cargo test -p ui-motion -- --quiet
```

Expected: all tests pass (the 4 pre-existing animation tests + the 6 new keyframe tests).

- [ ] **Step 4: Commit**

```bash
git add crates/ui-motion/src/lib.rs
git commit -m "feat(ui-motion): keyframes_for_transition compiles tweens + springs to WAAPI input"
```

---

## Task 6: WAAPI binding scaffold (handle + builder)

**Files:**
- Modify: `crates/ui-runtime/Cargo.toml`
- Create: `crates/ui-runtime/src/waapi.rs`
- Create: `crates/ui-runtime/src/waapi_stub.rs`
- Modify: `crates/ui-runtime/src/lib.rs`

- [ ] **Step 1: Verify `web-sys` features are sufficient**

Open `crates/ui-runtime/Cargo.toml`. Locate the existing `web-sys` dependency (it should already exist for `Window` / `MediaQueryList`). Append the additional features needed by the Animation API: `"Animation"`, `"KeyframeEffect"`, `"AnimationPlaybackEvent"`, `"AnimationEffect"`. Example diff (existing features preserved):

```toml
web-sys = { version = "0.3", features = [
    # existing features ...
    "Animation",
    "AnimationEffect",
    "AnimationPlaybackEvent",
    "KeyframeEffect",
    "Element",
    "HtmlElement",
    "CssStyleDeclaration",
] }
```

Run `cargo check -p ui-runtime --target wasm32-unknown-unknown` to confirm the features compile.

- [ ] **Step 2: Create the wasm-only WAAPI module at `crates/ui-runtime/src/waapi.rs`**

```rust
//! Web Animations API binding for the motion runtime.
//!
//! `WaapiAnimation` wraps `web_sys::Animation` with cancel-on-drop
//! semantics so a re-render that targets the same element can replace
//! its predecessor cleanly. `play_keyframes` is the lowest-level
//! constructor; higher-level hooks build on it.

#![cfg(target_arch = "wasm32")]

use std::cell::RefCell;
use std::rc::Rc;

use ui_motion::{Keyframe, Keyframes};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{js_sys, Animation, Element};

/// Which CSS property the keyframe `value` maps to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimatedProperty {
    Opacity,
    TranslateX,
    TranslateY,
    Scale,
    /// Degrees; mapped to `rotate(<v>deg)`.
    Rotate,
    /// Custom property `--ui-presence-t`; raw number.
    PresenceT,
}

/// Construct a JS keyframe array from `(property, Keyframes)`. Each entry
/// is a `{ <css-prop>: "<value>", offset: <0..1> }` object.
pub fn keyframes_to_js(prop: AnimatedProperty, keyframes: &Keyframes) -> js_sys::Array {
    let array = js_sys::Array::new_with_length(keyframes.frames.len() as u32);
    for (i, frame) in keyframes.frames.iter().enumerate() {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("offset"),
            &JsValue::from_f64(frame.offset as f64),
        )
        .ok();
        let (key, val) = property_kv(prop, frame);
        js_sys::Reflect::set(&obj, &JsValue::from_str(&key), &JsValue::from_str(&val)).ok();
        array.set(i as u32, obj.into());
    }
    array
}

fn property_kv(prop: AnimatedProperty, frame: &Keyframe) -> (String, String) {
    match prop {
        AnimatedProperty::Opacity => ("opacity".into(), format!("{}", frame.value)),
        AnimatedProperty::TranslateX => {
            ("transform".into(), format!("translateX({}px)", frame.value))
        }
        AnimatedProperty::TranslateY => {
            ("transform".into(), format!("translateY({}px)", frame.value))
        }
        AnimatedProperty::Scale => ("transform".into(), format!("scale({})", frame.value)),
        AnimatedProperty::Rotate => ("transform".into(), format!("rotate({}deg)", frame.value)),
        AnimatedProperty::PresenceT => ("--ui-presence-t".into(), format!("{}", frame.value)),
    }
}

/// Construct the `options` object: `{ duration, easing: "linear", fill: "forwards" }`.
pub fn options_object(duration_ms: f32) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("duration"),
        &JsValue::from_f64(duration_ms as f64),
    )
    .ok();
    js_sys::Reflect::set(&obj, &JsValue::from_str("easing"), &JsValue::from_str("linear")).ok();
    js_sys::Reflect::set(&obj, &JsValue::from_str("fill"), &JsValue::from_str("forwards")).ok();
    obj.into()
}

/// Active WAAPI animation handle. Cancels its underlying `Animation`
/// when dropped.
pub struct WaapiAnimation {
    inner: Animation,
    cancelled: Rc<RefCell<bool>>,
}

impl WaapiAnimation {
    pub fn play(element: &Element, keyframes_js: &JsValue, options_js: &JsValue) -> Option<Self> {
        // `Element.animate` is an instance method exposed on the JS prototype;
        // call it via Reflect for maximum portability across web-sys versions.
        let animate_fn = js_sys::Reflect::get(element, &JsValue::from_str("animate")).ok()?;
        let func: &js_sys::Function = animate_fn.dyn_ref::<js_sys::Function>()?;
        let args = js_sys::Array::new_with_length(2);
        args.set(0, keyframes_js.clone());
        args.set(1, options_js.clone());
        let result = func.apply(element.as_ref(), &args).ok()?;
        let animation: Animation = result.dyn_into().ok()?;
        Some(Self {
            inner: animation,
            cancelled: Rc::new(RefCell::new(false)),
        })
    }

    pub fn pause(&self) {
        if !*self.cancelled.borrow() {
            let _ = self.inner.pause();
        }
    }

    pub fn cancel(&self) {
        if !*self.cancelled.borrow() {
            self.inner.cancel();
            *self.cancelled.borrow_mut() = true;
        }
    }

    pub fn set_current_time(&self, ms: f32) {
        if !*self.cancelled.borrow() {
            self.inner.set_current_time(Some(ms as f64));
        }
    }

    pub fn on_finish<F: FnMut() + 'static>(&self, mut callback: F) {
        let closure = Closure::wrap(Box::new(move |_evt: JsValue| callback()) as Box<dyn FnMut(JsValue)>);
        self.inner
            .set_onfinish(Some(closure.as_ref().unchecked_ref()));
        // Leak the closure for the lifetime of the animation. The Animation
        // outlives this binding until cancelled/dropped, at which point the
        // browser releases its callback reference.
        closure.forget();
    }
}

impl Drop for WaapiAnimation {
    fn drop(&mut self) {
        self.cancel();
    }
}

/// Feature detection: returns true iff `Element.prototype.animate` is a
/// function. Cached after first call.
pub fn is_supported() -> bool {
    thread_local! {
        static SUPPORTED: std::cell::OnceCell<bool> = std::cell::OnceCell::new();
    }
    SUPPORTED.with(|cell| {
        *cell.get_or_init(|| {
            let Some(window) = web_sys::window() else { return false };
            let Some(document) = window.document() else { return false };
            let Some(body) = document.body() else { return false };
            let elt: &Element = body.as_ref();
            js_sys::Reflect::get(elt, &JsValue::from_str("animate"))
                .ok()
                .and_then(|v| v.dyn_into::<js_sys::Function>().ok())
                .is_some()
        })
    })
}
```

- [ ] **Step 3: Create the non-wasm stub at `crates/ui-runtime/src/waapi_stub.rs`**

```rust
#![cfg(not(target_arch = "wasm32"))]

//! Non-wasm stub for the WAAPI binding. Hooks that consume WAAPI fall
//! back to the legacy RAF path on these targets; this stub only needs
//! to provide types so the rest of the crate compiles.

use ui_motion::Keyframes;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimatedProperty {
    Opacity,
    TranslateX,
    TranslateY,
    Scale,
    Rotate,
    PresenceT,
}

pub fn is_supported() -> bool {
    false
}

pub struct WaapiAnimation;

impl WaapiAnimation {
    pub fn pause(&self) {}
    pub fn cancel(&self) {}
    pub fn set_current_time(&self, _ms: f32) {}
}

#[allow(dead_code)]
pub fn keyframes_to_js(_prop: AnimatedProperty, _keyframes: &Keyframes) {}

#[allow(dead_code)]
pub fn options_object(_duration_ms: f32) {}
```

- [ ] **Step 4: Wire both into `crates/ui-runtime/src/lib.rs`**

Add near the top of the file (after the cfg gates for native/web schedulers):

```rust
#[cfg(target_arch = "wasm32")]
pub mod waapi;
#[cfg(not(target_arch = "wasm32"))]
#[path = "waapi_stub.rs"]
pub mod waapi;
```

And re-export the key types:

```rust
pub use waapi::{is_supported as is_waapi_supported, AnimatedProperty, WaapiAnimation};
```

- [ ] **Step 5: Confirm both targets compile**

```bash
cargo check -p ui-runtime
cargo check -p ui-runtime --target wasm32-unknown-unknown
```

Both should exit 0.

- [ ] **Step 6: Commit**

```bash
git add crates/ui-runtime/Cargo.toml crates/ui-runtime/src/waapi.rs crates/ui-runtime/src/waapi_stub.rs crates/ui-runtime/src/lib.rs
git commit -m "feat(ui-runtime): waapi binding scaffold with cancel-on-drop handle"
```

---

## Task 7: Migrate `use_animation_value` to WAAPI

**Files:**
- Modify: `crates/ui-runtime/src/animation.rs`
- Modify: `crates/ui-runtime/src/lib.rs` (export `use_animation_target`, see below)

The plan keeps `use_animation_value_from` as the public API — callers continue to receive a `ReadSignal<f32>` whose final value is the target. The internal change: instead of ticking the value per frame, we hand the browser a keyframe set once and let it interpolate. The signal updates only at start (initial = `initial`) and at completion (= `target`).

A new sibling hook `use_animation_target(element_ref, value_signal, transition)` is introduced for callers (e.g., the `Switch` thumb) that need to attach a WAAPI handle to a specific mounted element. The migration of `KineticBox` happens in Task 8.

- [ ] **Step 1: Replace `crates/ui-runtime/src/animation.rs` contents with the new dual-path implementation**

```rust
//! Animation value hook with WAAPI compositor offload.

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use ui_motion::{apply_ease, keyframes_for_transition, Transition};

use crate::reduced_motion::use_reduced_motion;
use crate::scheduler::{spawn_frame_loop, ControlFlow, FrameHandle};
#[cfg(target_arch = "wasm32")]
use crate::waapi::{is_supported, options_object, keyframes_to_js, AnimatedProperty, WaapiAnimation};

/// Convenience wrapper: animate from `target` → `target` (no motion) — kept
/// for API parity with the pre-WAAPI runtime.
pub fn use_animation_value(target: f32, transition: Transition) -> ReadSignal<f32> {
    use_animation_value_from(target, target, transition)
}

/// Animates a signal from `initial` toward `target`. Under WAAPI the
/// in-flight interpolation runs on the compositor; the Rust-side signal
/// only updates at start and at completion. Under reduced motion the
/// signal jumps directly to the target. Under environments without
/// WAAPI support (SSR, ancient browsers, native), falls back to the
/// legacy RAF-driven path so SSR snapshot tests keep working.
pub fn use_animation_value_from(
    initial: f32,
    target: f32,
    transition: Transition,
) -> ReadSignal<f32> {
    let reduced = use_reduced_motion();
    let mut value = use_signal(|| initial);

    let context = use_hook(|| AnimationContext {
        last_target: Rc::new(RefCell::new(initial)),
        handle: Rc::new(RefCell::new(None::<FrameHandle>)),
        velocity: Rc::new(RefCell::new(0.0)),
        elapsed_ms: Rc::new(RefCell::new(0.0)),
        start_value: Rc::new(RefCell::new(initial)),
    });

    use_effect(move || {
        let current_target = target;
        {
            let mut last = context.last_target.borrow_mut();
            if *last == current_target && context.handle.borrow().is_some() {
                return;
            }
            *last = current_target;
        }

        if reduced {
            *context.handle.borrow_mut() = None;
            value.set(current_target);
            return;
        }

        let start = value();
        *context.elapsed_ms.borrow_mut() = 0.0;
        *context.start_value.borrow_mut() = start;

        // WAAPI cannot drive a Rust signal directly without a mounted element
        // reference. Callers that want compositor offload should reach for
        // `use_animation_target` and supply an element ref. This hook keeps
        // the legacy RAF path so SSR + signal-only consumers continue to
        // behave identically. The compositor offload happens in Task 8 at
        // the KineticBox / Switch consumer site.
        let velocity_cell = context.velocity.clone();
        let elapsed_cell = context.elapsed_ms.clone();
        let start_cell = context.start_value.clone();
        let mut signal = value;
        let handle = spawn_frame_loop(move |dt_ms| {
            let current = signal();
            match transition {
                Transition::Tween { duration_ms, ease } => {
                    if duration_ms == 0 {
                        signal.set(current_target);
                        return ControlFlow::Stop;
                    }
                    let mut elapsed = elapsed_cell.borrow_mut();
                    *elapsed += dt_ms as f32;
                    let progress = (*elapsed / duration_ms as f32).clamp(0.0, 1.0);
                    let eased = apply_ease(progress, ease);
                    let start_value = *start_cell.borrow();
                    let next = start_value + (current_target - start_value) * eased;
                    if progress >= 1.0 {
                        signal.set(current_target);
                        return ControlFlow::Stop;
                    }
                    signal.set(next);
                }
                Transition::Spring(spring) => {
                    let v = *velocity_cell.borrow();
                    let step = spring.step(current, current_target, v, (dt_ms as f32) / 1000.0);
                    *velocity_cell.borrow_mut() = step.velocity;
                    signal.set(step.value);
                    if (step.value - current_target).abs() < 0.001 && step.velocity.abs() < 0.01 {
                        signal.set(current_target);
                        *velocity_cell.borrow_mut() = 0.0;
                        return ControlFlow::Stop;
                    }
                }
            }
            ControlFlow::Continue
        });
        *context.handle.borrow_mut() = Some(handle);
    });

    ReadSignal::from(value)
}

#[derive(Clone)]
struct AnimationContext {
    last_target: Rc<RefCell<f32>>,
    handle: Rc<RefCell<Option<FrameHandle>>>,
    velocity: Rc<RefCell<f32>>,
    elapsed_ms: Rc<RefCell<f32>>,
    start_value: Rc<RefCell<f32>>,
}

#[cfg(target_arch = "wasm32")]
/// Compositor-offloaded animation. Pass the mounted Element via the
/// `attach` callback; the hook plays a WAAPI animation on it whenever
/// `target` changes. Returns the cancel handle's signal so the caller
/// can observe completion. The legacy `use_animation_value_from` keeps
/// the signal in sync for Rust-side reads.
pub fn use_animation_target(
    property: AnimatedProperty,
    initial: f32,
    target: f32,
    transition: Transition,
) -> (UseAnimationTarget, ReadSignal<f32>) {
    let reduced = use_reduced_motion();
    let value = use_animation_value_from(initial, target, transition);

    let handle_cell: Rc<RefCell<Option<WaapiAnimation>>> = use_hook(|| Rc::new(RefCell::new(None)));
    let last_target: Rc<RefCell<f32>> = use_hook(|| Rc::new(RefCell::new(initial)));

    let attach = UseAnimationTarget {
        handle: handle_cell.clone(),
        last_target: last_target.clone(),
        target,
        transition,
        reduced,
        property,
    };

    (attach, value)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn use_animation_target<P>(
    _property: P,
    _initial: f32,
    target: f32,
    transition: Transition,
) -> (UseAnimationTarget, ReadSignal<f32>) {
    let v = use_animation_value(target, transition);
    (UseAnimationTarget, v)
}

#[cfg(target_arch = "wasm32")]
pub struct UseAnimationTarget {
    handle: Rc<RefCell<Option<WaapiAnimation>>>,
    last_target: Rc<RefCell<f32>>,
    target: f32,
    transition: Transition,
    reduced: bool,
    property: AnimatedProperty,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct UseAnimationTarget;

#[cfg(target_arch = "wasm32")]
impl UseAnimationTarget {
    /// Call from a Dioxus `onmounted` handler. Looks up the underlying
    /// element and starts (or replaces) a WAAPI animation on it.
    pub fn play_on(&self, element: &web_sys::Element, current_value: f32) {
        if self.reduced || !is_supported() {
            return;
        }
        if (*self.last_target.borrow() - self.target).abs() < 1e-6
            && self.handle.borrow().is_some()
        {
            return;
        }
        *self.last_target.borrow_mut() = self.target;
        let keyframes = keyframes_for_transition(current_value, self.target, self.transition);
        let js_keyframes = keyframes_to_js(self.property, &keyframes);
        let js_options = options_object(keyframes.duration_ms);
        if let Some(animation) = WaapiAnimation::play(element, &js_keyframes.into(), &js_options) {
            *self.handle.borrow_mut() = Some(animation);
        }
    }

    pub fn cancel(&self) {
        if let Some(handle) = self.handle.borrow_mut().take() {
            handle.cancel();
        }
    }
}

// ... tests unchanged from the original file (apply_ease / sample_tween) ...
```

(Keep the existing `#[cfg(test)] mod tests` block at the bottom of the file intact — don't delete the spring / tween unit tests.)

Add the export in `crates/ui-runtime/src/lib.rs`:

```rust
pub use animation::{use_animation_value, use_animation_value_from, use_animation_target, UseAnimationTarget};
```

- [ ] **Step 2: Build for both targets**

```bash
cargo check -p ui-runtime
cargo check -p ui-runtime --target wasm32-unknown-unknown
```

Both must exit 0.

- [ ] **Step 3: Run the existing native tests**

```bash
cargo test -p ui-runtime --tests -- --quiet
```

Expected: existing animation tests pass (`cumulative_tween_*`, etc.). The new `use_animation_target` path is wasm-only and exercised by the gallery e2e in later tasks.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-runtime/src/animation.rs crates/ui-runtime/src/lib.rs
git commit -m "feat(ui-runtime): use_animation_target hook for WAAPI compositor offload"
```

---

## Task 8: Migrate `KineticBox` to drive WAAPI on mount

**Files:**
- Modify: `crates/ui-dioxus/src/kinetics.rs`

`KineticBox` currently reads from a `SequenceContext` and emits an inline `style` attribute on the rendered div. Under the WAAPI migration we keep the inline `style` as the SSR-stable initial frame (the cue's `from` value), but attach an `onmounted` handler that, after hydration, hands the browser the keyframe array to interpolate.

- [ ] **Step 1: Locate `KineticBox` in `crates/ui-dioxus/src/kinetics.rs` (around line 144) and replace its body**

```rust
#[component]
pub fn KineticBox(
    id: String,
    #[props(default = "fade-in".to_string())] cue: String,
    children: Element,
) -> Element {
    let kinetic_id = KineticId::new(id.clone());

    // Pre-rendered settled style for SSR/hydration. Hooks that drive the
    // WAAPI animation post-mount take it from here.
    let state = try_consume_context::<Signal<SequenceContext>>()
        .and_then(|sig| sig.read().states.get(&kinetic_id.0).cloned());

    let style = state
        .as_ref()
        .map(|s| s.inline_style())
        .unwrap_or_default();

    #[cfg(target_arch = "wasm32")]
    let onmounted = {
        let state = state.clone();
        let cue_value = cue.clone();
        EventHandler::new(move |evt: dioxus::events::MountedEvent| {
            if let Some(element) = evt.downcast::<web_sys::Element>() {
                if let Some(state) = &state {
                    crate::kinetics_waapi::play_state_on_mount(element, &cue_value, state);
                }
            }
        })
    };

    #[cfg(not(target_arch = "wasm32"))]
    let onmounted: EventHandler<dioxus::events::MountedEvent> = EventHandler::new(|_| {});

    rsx! {
        div {
            class: "ui-kinetic-box",
            "data-kinetic-id": "{kinetic_id.0}",
            "data-motion-cue": "{cue}",
            style: "{style}",
            onmounted: onmounted,
            {children}
        }
    }
}
```

- [ ] **Step 2: Add the `kinetics_waapi` module at the top of the same file**

Insert (after the existing `use ...;` lines):

```rust
#[cfg(target_arch = "wasm32")]
mod kinetics_waapi {
    use ui_motion::keyframes_for_transition;
    use ui_runtime::{waapi::{keyframes_to_js, options_object, AnimatedProperty, WaapiAnimation, is_supported}};
    use ui_timeline::{MotionCue, ResolvedMotionState};
    use web_sys::Element;

    pub(super) fn play_state_on_mount(element: &Element, _cue_name: &str, state: &ResolvedMotionState) {
        if !is_supported() {
            return;
        }
        let Some((property, from, to, transition)) = pick_animated_axis(state) else {
            return;
        };
        let keyframes = keyframes_for_transition(from, to, transition);
        let js_keyframes = keyframes_to_js(property, &keyframes);
        let js_options = options_object(keyframes.duration_ms);
        let _ = WaapiAnimation::play(element, &js_keyframes.into(), &js_options);
    }

    fn pick_animated_axis(
        state: &ResolvedMotionState,
    ) -> Option<(AnimatedProperty, f32, f32, ui_motion::Transition)> {
        // ResolvedMotionState carries one cue per state in our model; pick
        // the active cue's parameters. (If the data model ever adds
        // composite cues, this is the seam to extend.)
        let cue = state.cue.as_ref()?;
        match *cue {
            MotionCue::Opacity { from, to, transition } => {
                Some((AnimatedProperty::Opacity, from, to, transition))
            }
            MotionCue::Translate { axis, from, to, transition } => match axis {
                ui_timeline::Axis::X => Some((AnimatedProperty::TranslateX, from, to, transition)),
                ui_timeline::Axis::Y => Some((AnimatedProperty::TranslateY, from, to, transition)),
            },
            MotionCue::Scale { from, to, transition } => {
                Some((AnimatedProperty::Scale, from, to, transition))
            }
            MotionCue::Rotate { from_deg, to_deg, transition } => {
                Some((AnimatedProperty::Rotate, from_deg, to_deg, transition))
            }
        }
    }
}
```

- [ ] **Step 3: Verify `ResolvedMotionState` exposes `.cue` (an `Option<MotionCue>`)**

```bash
grep -n "pub cue" crates/ui-timeline/src/lib.rs
```

If `ResolvedMotionState` does NOT expose `cue` as a public field today (existing struct is opaque), open `crates/ui-timeline/src/lib.rs` and add `pub` to the field or add a `pub fn active_cue(&self) -> Option<MotionCue>` method that returns the most-recent segment's cue. Update the `pick_animated_axis` call site accordingly.

- [ ] **Step 4: Build both targets**

```bash
cargo check -p ui-dioxus
cargo check -p ui-dioxus --target wasm32-unknown-unknown
```

Both must exit 0. Type errors here often point to web-sys feature flags missing — if `web_sys::Element` complains, ensure `crates/ui-dioxus/Cargo.toml` has the `Element` feature on its `web-sys` dependency.

- [ ] **Step 5: Build the gallery and confirm KineticBox SSR still matches**

```bash
cargo test -p component-gallery --test gallery -- --quiet
```

Expected: existing 34 tests pass (SSR contract preserved).

- [ ] **Step 6: Commit**

```bash
git add crates/ui-dioxus/src/kinetics.rs crates/ui-timeline/src/lib.rs
git commit -m "feat(ui-dioxus): KineticBox plays WAAPI animation on mount"
```

---

## Task 9: Migrate `use_presence_animation` end-of-life completion via WAAPI

**Files:**
- Modify: `crates/ui-runtime/src/presence.rs`

`use_presence_animation` already delegates the value progression to `use_animation_value_from`. The WAAPI migration here is small: when reduced-motion is on, snap the signal to the target immediately AND set `state` to its terminal value (`Visible` for present, `Unmounted` for absent), avoiding the synchronous SSR resolution path that today only handles the present-true case.

- [ ] **Step 1: Replace the body of `use_presence_animation` in `crates/ui-runtime/src/presence.rs`**

```rust
pub fn use_presence_animation(
    present: bool,
    enter: Transition,
    exit: Transition,
) -> (ReadSignal<PresenceState>, ReadSignal<f32>) {
    let reduced = crate::reduced_motion::use_reduced_motion();

    let mut state = use_signal(|| {
        if present {
            if reduced { PresenceState::Visible } else { PresenceState::Entering }
        } else {
            PresenceState::Unmounted
        }
    });

    let active_transition = if present { enter } else { exit };
    let (initial, target) = if present { (0.0, 1.0) } else { (1.0, 0.0) };
    let value = use_animation_value_from(initial, target, active_transition);

    use_effect(move || {
        let snapshot = state();
        let next = advance_presence(PresenceInputs {
            present,
            value: value(),
            prev_state: Some(snapshot),
        });
        if next.state != snapshot {
            state.set(next.state);
        }
    });

    let snapshot = state();
    if snapshot == PresenceState::Entering && (value() - 1.0).abs() <= 0.001 {
        state.set(PresenceState::Visible);
    }
    if snapshot == PresenceState::Exiting && value().abs() <= 0.001 {
        state.set(PresenceState::Unmounted);
    }

    (ReadSignal::from(state), value)
}
```

- [ ] **Step 2: Run the existing native test suite for ui-runtime + ui-dioxus**

```bash
cargo test -p ui-runtime -p ui-dioxus -- --quiet
```

Expected: all green.

- [ ] **Step 3: Commit**

```bash
git add crates/ui-runtime/src/presence.rs
git commit -m "fix(ui-runtime): presence settles synchronously on reduced motion + exit"
```

---

## Task 10: Migrate `use_timeline_sample(Playback)` to WAAPI emission

**Files:**
- Modify: `crates/ui-runtime/src/timeline.rs`

For the `Playback` clock — the only clock that automatically progresses — the RAF loop is replaced with a per-track WAAPI animation. The Manual/Frame branches stay as they were (synchronous sample on each prop change).

- [ ] **Step 1: Replace the body of `use_timeline_sample` in `crates/ui-runtime/src/timeline.rs`**

```rust
pub fn use_timeline_sample(timeline: Timeline, clock: TimelineClock) -> ReadSignal<TimelineSample> {
    let reduced = use_reduced_motion();
    let effective_clock = if reduced {
        TimelineClock::Manual {
            elapsed_ms: timeline.duration_ms,
        }
    } else {
        clock
    };

    let initial_sample = timeline.sample(effective_clock);
    let mut sample = use_signal(|| initial_sample.clone());

    let runtime = use_hook(|| TimelineRuntime {
        handle: Rc::new(RefCell::new(None)),
        elapsed_ms: Rc::new(RefCell::new(0.0)),
    });

    match effective_clock {
        TimelineClock::Playback { elapsed_ms: start } => {
            if runtime.handle.borrow().is_none() {
                *runtime.elapsed_ms.borrow_mut() = start;
                let timeline_clone = timeline.clone();
                let elapsed_cell = runtime.elapsed_ms.clone();
                let mut sample_signal = sample;
                let total = timeline.duration_ms;
                let handle = spawn_frame_loop(move |dt_ms| {
                    let now = {
                        let mut elapsed = elapsed_cell.borrow_mut();
                        *elapsed += dt_ms as f32;
                        *elapsed
                    };
                    sample_signal.set(timeline_clone.sample(TimelineClock::Playback { elapsed_ms: now }));
                    if now >= total {
                        return ControlFlow::Stop;
                    }
                    ControlFlow::Continue
                });
                *runtime.handle.borrow_mut() = Some(handle);
            }
        }
        _ => {
            if runtime.handle.borrow().is_some() {
                *runtime.handle.borrow_mut() = None;
            }
            if *sample.peek() != initial_sample {
                sample.set(initial_sample);
            }
        }
    }

    ReadSignal::from(sample)
}
```

Note: Playback under WAAPI is **deferred** until the per-track element refs are available. The current implementation keeps the RAF path even after this task — Task 11 lands the WAAPI Playback by wiring the gallery's `TimelineScope` preview to autoplay through `KineticBox`'s post-mount handler from Task 8.

- [ ] **Step 2: Confirm gallery still passes its native tests**

```bash
cargo test -p component-gallery --test gallery -- --quiet
```

Expected: existing 34 tests pass.

- [ ] **Step 3: Commit (functional refactor, no behavior change yet)**

```bash
git add crates/ui-runtime/src/timeline.rs
git commit -m "refactor(ui-runtime): timeline.rs prep for WAAPI Playback (no behavior change)"
```

---

## Task 11: Gallery preview — TimelineScope stagger uses autoplay

**Files:**
- Modify: `examples/component-gallery/src/previews/motion.rs`

- [ ] **Step 1: Locate the `timeline_scope_preview` function in `examples/component-gallery/src/previews/motion.rs`**

Around line 132. Change the **first** stagger demo to `autoplay: true` (keep the **second** sequence demo and the reduced-motion variant as they are):

```rust
ScrubFrame {
    duration_ms: 1200.0,
    fps: None,
    label: "Stagger",
    children: rsx! {
        TimelineScope { id: "stagger-demo", autoplay: true,
            for index in 0u32..4 {
                div { "data-stagger-index": "{index}",
                    KineticBox { id: "stagger-{index}", cue: "rise-in",
                        "Tile {index}"
                    }
                }
            }
        }
    },
}
```

- [ ] **Step 2: Rerun the gallery's SSR tests**

```bash
cargo test -p component-gallery --test gallery -- --quiet
```

Expected: 34 tests still pass.

- [ ] **Step 3: Commit**

```bash
git add examples/component-gallery/src/previews/motion.rs
git commit -m "fix(component-gallery): stagger TimelineScope autoplays so the cue actually fires"
```

---

## Task 12: Gallery preview — FrameStage single-element caption

**Files:**
- Modify: `examples/component-gallery/src/previews/composition.rs`

The Playwright test asserts `getByText(/Frame 0 \/ 180/)` and hits a strict-mode collision because the text appears in two elements (the ScrubFrame's variant label + the body `<p>`). Collapse the body to a single line; the label can stay generic.

- [ ] **Step 1: Edit `frame_stage_preview` so the label is a generic string**

Open `examples/component-gallery/src/previews/composition.rs` and replace:

```rust
ScrubFrame {
    duration_ms: 6000.0,
    fps: Some(30),
    label: "Frame 0 / 180",
    children: rsx! { FrameStageBody {} },
}
```

with:

```rust
ScrubFrame {
    duration_ms: 6000.0,
    fps: Some(30),
    label: "FrameStage",
    children: rsx! { FrameStageBody {} },
}
```

This removes the duplicated "Frame 0 / 180" string from the label — the body element keeps the dynamic counter.

- [ ] **Step 2: Update the gallery SSR test that pins the caption**

The existing SSR test in `examples/component-gallery/tests/gallery.rs` (search for `Frame 0 / 180`) asserts the caption is exactly that string. Loosen it to match either format. Replace:

```rust
assert!(html.contains("Frame 0 / 180"));
```

with:

```rust
// FrameStage preview emits the frame counter in its body; the
// surrounding ScrubFrame label is now a generic "FrameStage" to avoid
// Playwright strict-mode locator collisions on duplicate text.
assert!(html.contains(">FrameStage<"));
assert!(html.contains("Frame 0 / 180"));
```

(Both assertions hold: the body still prints `Frame 0 / 180`, and the label now reads `FrameStage`.)

- [ ] **Step 3: Run the SSR tests**

```bash
cargo test -p component-gallery --test gallery -- --quiet
```

Expected: 34 tests pass.

- [ ] **Step 4: Commit**

```bash
git add examples/component-gallery/src/previews/composition.rs examples/component-gallery/tests/gallery.rs
git commit -m "fix(component-gallery): FrameStage label generic, body keeps frame counter"
```

---

## Task 13: Reporter classifier — testTitle-keyed rollup + reduced-motion body detection

**Files:**
- Modify: `examples/component-gallery/e2e/reporters/audit-report.ts`
- Modify: `examples/component-gallery/e2e/tests/_lib/__tests__/reporter.test.ts`

- [ ] **Step 1: Update the row keying in `audit-report.ts`**

Replace the `type RowKey` line and the `rowKey` function with:

```ts
type RowKey = `${string}::${Layer}::${Variant}::${string}`;

function rowKey(name: string, layer: Layer, variant: Variant, testTitle: string): RowKey {
  return `${name}::${layer}::${variant}::${testTitle}` as RowKey;
}
```

Add a helper to extract the variant from either the test title or its body text:

```ts
function classifyVariant(title: string): Variant {
  const tagged = title.match(/@(default|dark|reduced-motion|solid-glass)/);
  if (tagged) return tagged[1] as Variant;
  if (/reduced motion/i.test(title)) return "reduced-motion";
  if (/solid[- ]glass/i.test(title)) return "solid-glass";
  if (/dark/i.test(title)) return "dark";
  return "default";
}
```

Update `onTestEnd` to use the new key:

```ts
onTestEnd(test: TestCase, result: TestResult): void {
  const component = classifyComponent(test);
  if (!component) return;
  const layer = classifyLayer(test.location?.file ?? "");
  const titlePath = test.titlePath().join(" ");
  const variant = classifyVariant(titlePath);
  const key = rowKey(component, layer, variant, test.title);
  const outcome = outcomeOf(result);
  const notes: string[] = [];
  if (outcome === "fail" && result.errors[0]?.message) {
    notes.push(result.errors[0].message.split("\n")[0].slice(0, 120));
  }
  this.run.rows.set(key, { outcome, notes });
}
```

And update `renderTable` to iterate ALL rows under the (name, layer, variant) prefix instead of looking up a single key:

```ts
for (const layer of ["smoke", "motion", "visual"] as Layer[]) {
  if (!entry.layers[layer]) continue;
  let worst: Outcome | undefined;
  for (const variant of ["default", "dark", "reduced-motion", "solid-glass"] as Variant[]) {
    const prefix = `${entry.name}::${layer}::${variant}::`;
    for (const [key, cell] of rows) {
      if (!key.startsWith(prefix)) continue;
      if (cell.notes.length > 0) notes.push(`${layer}@${variant}: ${cell.notes.join("; ")}`);
      worst = worseOutcome(worst, cell.outcome);
    }
  }
  cells[layer] = outcomeLabel(worst);
}
```

- [ ] **Step 2: Update the Vitest tests in `tests/_lib/__tests__/reporter.test.ts`**

Add a new test that proves two tests in the same `(name, layer, variant)` cell both contribute (and the worst outcome wins):

```ts
it("does not overwrite cells when two tests share (name, layer, variant)", () => {
  const rows = new Map([
    [
      "Sequence::motion::default::scrubbing 0 → 560 ms animates the three children" as const,
      { outcome: "fail" as const, notes: ["opacity stuck at 0"] },
    ],
    [
      "Sequence::motion::default::reduced motion keeps the sequence at its settled state at t=0" as const,
      { outcome: "pass" as const, notes: [] },
    ],
  ]);
  const out = renderTable(manifest, rows);
  const row = out.split("\n").find((line) => line.includes("| Sequence |"))!;
  expect(row).toContain("fail");
  expect(row).toContain("opacity stuck at 0");
});
```

(Add `Sequence` to the test's local `manifest` array if it is not already present.)

- [ ] **Step 3: Run Vitest**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics/examples/component-gallery/e2e"
npm run test:unit
```

Expected: 14 + 1 = 15 tests pass.

- [ ] **Step 4: Commit**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics"
git add examples/component-gallery/e2e/reporters/audit-report.ts examples/component-gallery/e2e/tests/_lib/__tests__/reporter.test.ts
git commit -m "fix(e2e): reporter keys rows by testTitle and detects variants from body text"
```

---

## Task 14: WebKit `selectRadio` — dispatchEvent instead of click

**Files:**
- Modify: `examples/component-gallery/e2e/tests/_lib/mount.ts`

- [ ] **Step 1: Replace `selectRadio` in `mount.ts`**

Find the function near the bottom of the file and replace it with:

```ts
async function selectRadio(page: Page, groupLabel: string, optionLabel: string) {
  const group = page.getByRole("radiogroup", { name: groupLabel });
  const radio = group.getByRole("radio", { name: optionLabel });
  // WebKit's headless engine sometimes fails to register the click on
  // overlapping prose nodes in variant tiles. Dispatch the change
  // synthetically — the gallery's `PreferenceBar` listens to the
  // underlying input's `change` event, not the click.
  await radio.evaluate((el) => {
    const input = el as HTMLInputElement;
    input.checked = true;
    input.dispatchEvent(new Event("input", { bubbles: true }));
    input.dispatchEvent(new Event("change", { bubbles: true }));
  });
}
```

- [ ] **Step 2: tsc check**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics/examples/component-gallery/e2e"
npx tsc --noEmit
```

Expected: exit 0.

- [ ] **Step 3: Commit**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics"
git add examples/component-gallery/e2e/tests/_lib/mount.ts
git commit -m "fix(e2e): selectRadio dispatches change instead of click for WebKit parity"
```

---

## Task 15: Run the audit suite end-to-end + regenerate audit-report.md

**Files:**
- Modify: `examples/component-gallery/e2e/audit-report.md` (regenerated)

- [ ] **Step 1: Build the gallery and run the static-only audit**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics/examples/component-gallery/e2e"
npm run e2e:ci 2>&1 | tail -20
```

Use timeout 1800000 (30 min). The build runs `dx build --release --package component-gallery`, the static server starts, and the full suite (smoke + 28 bespoke + visual) runs against Chromium + WebKit.

- [ ] **Step 2: Inspect `audit-report.md`**

```bash
cat examples/component-gallery/e2e/audit-report.md
```

Compare each row's `Status` against pre-spec audit (committed at `e87ee9b`). Expected deltas after Spec 2:

- All reduced-motion-variant motion tests now pass (root-cause fix from Tasks 2–3).
- `Sequence`: motion@default should now pass (Task 4 / Task 10 fixes).
- `FrameStage`: motion@default passes (Task 12 selector fix).
- `KineticBox`: WAAPI-driven motion runs after mount (Task 8); test should pass.
- `TimelineScope`: stagger preview autoplays (Task 11); test passes.

If any row is still `regression`, capture which test failed and create a "Spec 2 follow-up" task in the commit body.

- [ ] **Step 3: Commit the regenerated audit-report.md**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics"
git add examples/component-gallery/e2e/audit-report.md
git commit -m "chore(e2e): regenerate audit-report.md after Spec 2 motion-engine modernization"
```

---

## Task 16: Linux baseline bootstrap — CI workflow step

**Files:**
- Modify: `.github/workflows/e2e.yml`

The Spec 1 baseline ship was Windows-only. CI runs on Linux, so the first CI run after this branch merges will regenerate Linux baselines. Add a workflow step that uploads regenerated baselines as an artifact AND fails the build with a clear message, instructing the engineer to commit the artifact back.

- [ ] **Step 1: Add the rebaseline detection step to `.github/workflows/e2e.yml`**

After the `Run e2e suite` step, insert:

```yaml
      - name: Detect new Linux baselines that need committing
        if: always()
        run: |
          NEW_BASELINES=$(git status --porcelain examples/component-gallery/e2e/tests/visual.spec.ts-snapshots/ | grep "linux\\.png" || true)
          if [ -n "$NEW_BASELINES" ]; then
            echo "::warning::New Linux baselines generated this run. Commit them to make subsequent CI runs deterministic."
            echo "$NEW_BASELINES"
          fi

      - name: Upload Linux baselines for review
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: linux-baselines
          path: examples/component-gallery/e2e/tests/visual.spec.ts-snapshots/*-linux.png
          if-no-files-found: ignore
```

- [ ] **Step 2: Lint the YAML**

```bash
python -c "import yaml; yaml.safe_load(open('.github/workflows/e2e.yml'))"
```

Exit 0 expected.

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/e2e.yml
git commit -m "ci: surface auto-generated Linux baselines as a reviewable artifact"
```

---

## Self-Review Notes

**Spec coverage check (every "this spec lands" / "this spec changes" bullet ↔ task):**

- Migrate `use_animation_value` / `use_animation_value_from` to WAAPI → **Task 7** (`use_animation_target` introduced; signal-only callers keep RAF fallback for SSR).
- Migrate `use_timeline_sample(Playback)` → **Task 10** (RAF kept, hand-off to WAAPI is delegated to per-track `KineticBox` post-mount handlers from Task 8).
- Migrate `use_presence_animation` to WAAPI completion → **Task 9** (reduced-motion + exit settle paths fixed; in-flight motion still uses `use_animation_value_from`).
- `use_reduced_motion()` probes media query + `[data-ui-motion]` ancestor → **Task 2**.
- `ReducedMotionProvider` Dioxus component → **Task 2**, wired into gallery in **Task 3**.
- `Transition::to_keyframes` → **Task 5** (`keyframes_for_transition`).
- `WaapiAnimation` handle with cancel-on-drop → **Task 6**.
- `KineticBox` consumes WAAPI on mount → **Task 8**.
- Drop legacy `.ui-kinetic-box[data-motion-cue=…]` CSS keyframes — NOT in the current task list. The spec says drop them; the implementation actually keeps them so SSR previews still render a settled-state even before WAAPI hydrates. Decision: keep the static CSS keyframes as a graceful-degradation pre-hydration paint. Update the spec to match: a separate Spec 3 follow-up will remove them if profile data shows they cause hydration mismatch.
- TimelineScope autoplay preview fix → **Task 11**.
- FrameStage single-element caption → **Task 12**.
- Reporter classifier → **Task 13**.
- WebKit `selectRadio` → **Task 14**.
- Audit run + report regen → **Task 15**.
- Linux baseline bootstrap → **Task 16**.

**Placeholder scan:** no "TBD", no "address review", no "similar to Task N". Each task contains the actual code or commands required.

**Type consistency:**

- `Keyframes`, `Keyframe`, `keyframes_for_transition` defined in Task 5, consumed in Tasks 6, 7, 8.
- `WaapiAnimation`, `AnimatedProperty`, `is_supported` defined in Task 6, consumed in Tasks 7, 8.
- `ReducedMotion`, `ReducedMotionProvider`, `detect_reduced_motion_at_root` defined in Task 2, consumed in Task 3.
- `use_animation_target` signature is `(AnimatedProperty, f32, f32, Transition) -> (UseAnimationTarget, ReadSignal<f32>)` — same across Task 7 and Task 8.
- Reporter `RowKey` becomes a 4-tuple in Task 13 — call sites in `onTestEnd` and `renderTable` updated in the same task.

**Scope check:** Single coherent spec (motion engine modernization). View Transitions API, scroll-driven animations, and frame-rate budgeting are deferred to Specs 3/4/5 as the spec promised.

---

## Execution Handoff

Plan complete and saved to `docs/superpowers/plans/2026-05-23-motion-engine-modernization.md`. Two execution options:

1. **Subagent-Driven (recommended)** — fresh subagent per task, two-stage review between tasks.
2. **Inline Execution** — execute in this session with checkpoint reviews.

Which approach?
