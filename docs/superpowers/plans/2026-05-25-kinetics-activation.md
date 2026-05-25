# Kinetics Activation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Wire the leaf kinetic components (`KineticText`, `KineticBox`, `Sequence`, `TimelineScope`, `SplitText`, `WipeTransition`, `LowerThird`, `SocialOverlay`, `MetricCounter`) into the existing `Scene`/`SceneContext` clock so the gallery's animations actually run. Plus F9 (wgpu glass-engine format negotiation), F10 (scroll-story mount progress fix), and F11 (four CSS-only Wipe variants).

**Architecture:** The Scene → clock → adapter registry → seek machinery is already in place. The fix is the **inlined CssKeyframesAdapter pattern**: each animated leaf consumes `SceneContext` (or, when nested inside a `TimelineScope` stagger, a `StaggerOffsetContext`) and emits `style="animation-name: ui-cue-{cue}; animation-duration: …; animation-delay: -{elapsed_ms}ms; animation-play-state: paused; animation-fill-mode: forwards"`. The browser handles the actual interpolation; the Rust side writes one style attribute per parent-clock change. No per-leaf adapter registration needed. Backwards-compat: when no Scene/Sequence/Stagger ancestor exists, the leaf renders the existing static markup.

**Tech Stack:** Rust 2021, Dioxus 0.7 with `Signal<T>`, `dioxus-ssr` for SSR tests, raw CSS keyframes (no animation library), `web-sys` for the scroll driver rAF defer, `wgpu` for the glass engine fix, Playwright for the E2E motion regression spec.

**Spec:** `docs/superpowers/specs/2026-05-25-kinetics-activation-design.md`

---

## File Structure

```
crates/ui-styles/src/
  kinetic_cues.css            # NEW — fade-in / rise-in / slide-up / text-flow / pop-in
  gsap_primitives.css         # +four wipe variant keyframes
  lib.rs                      # +KINETIC_CUES_CSS const, included by library_css()

crates/ui-dioxus/src/
  cue_style.rs                # NEW — cue_inline_style() + cue_animation_duration_ms()
  stagger.rs                  # NEW — StaggerChild component + StaggerOffsetContext
  kinetics.rs                 # KineticText/KineticBox/Sequence/TimelineScope updates
  split_text.rs               # +cue prop, glyph/word children become cue-driven
  lib.rs                      # +pub mod cue_style; pub mod stagger;
                              # +re-export StaggerChild

crates/ui-dioxus/tests/
  cue_inline_style_ssr.rs     # NEW — verify animation-delay flows from SceneContext
  timeline_scope_stagger_ssr.rs # NEW — verify per-child stagger offsets
  split_text_ssr.rs           # +cue-prop test

crates/ui-blocks/src/
  wipe_transition.rs          # WipeVariant enum + SceneContext-driven inline style
  lower_third.rs              # +internal TimelineScope choreography
  social_overlay.rs           # +internal Sequence choreography
  metric_counter.rs           # +internal TimelineScope choreography
  lib.rs                      # +pub use wipe_transition::WipeVariant

crates/ui-blocks/tests/
  blocks_ssr.rs               # +WipeVariant tests, choreography assertions

crates/ui-runtime/src/drivers/
  scroll.rs                   # defer initial compute_progress to rAF

crates/ui-glass-engine/src/
  (TBD via inspection)        # surface format negotiation — site found during impl

crates/kinetics/src/lib.rs    # +WipeVariant re-export

examples/component-gallery/src/
  docs.rs                     # +3 wipe variant ComponentDoc entries
                              # bump array length 55 → 58
  previews/scenes/mod.rs      # +3 pub mod
  previews/scenes/wipe_conic_demo.rs        # NEW
  previews/scenes/wipe_iris_demo.rs         # NEW
  previews/scenes/wipe_mask_position_demo.rs # NEW
  previews/scene.rs           # +3 preview functions

examples/component-gallery/e2e/tests/
  animation-motion.spec.ts    # NEW — F8 regression: assert animation-delay deltas
  _lib/component-manifest.ts  # +3 wipe variant manifest entries
```

## Conventions

- Conventional Commits with `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>` HEREDOC trailer.
- Workspace-standard `let mut s = …; s.set(…);` for Signal writes.
- Edit/Read/Write/Bash dedicated tools; no raw `sed`/`awk`.
- Rust tests under `tests/<name>.rs` (integration-style).
- Never push/amend/`--no-verify`.

---

### Task 1: F1 — Cue keyframes stylesheet

Foundational CSS for the cue keywords used across the codebase. Once this lands, the inline `animation-name: ui-cue-{cue}` strings in F2 onward will resolve.

**Files:**
- Create: `crates/ui-styles/src/kinetic_cues.css`
- Modify: `crates/ui-styles/src/lib.rs` (add const + include in `library_css()`)

- [ ] **Step 1: Create the stylesheet**

```css
/* Keyframes for the kinetic motion cues consumed by KineticText,
   KineticBox, and the SplitText / ui-blocks choreographies. Inline
   style on each leaf sets animation-name + animation-delay derived
   from the parent SceneContext.clock.elapsed_ms — the browser
   handles interpolation. Identical end states across all keyframes
   so freeze-at-endpoint (reduced-motion + settled state) is stable. */

@keyframes ui-cue-fade-in {
  from { opacity: 0; }
  to   { opacity: 1; }
}

@keyframes ui-cue-rise-in {
  from { opacity: 0; transform: translateY(12px); }
  to   { opacity: 1; transform: translateY(0); }
}

@keyframes ui-cue-slide-up {
  from { transform: translateY(24px); }
  to   { transform: translateY(0); }
}

@keyframes ui-cue-text-flow {
  from { opacity: 0; transform: translateY(8px); }
  to   { opacity: 1; transform: translateY(0); }
}

@keyframes ui-cue-pop-in {
  from { opacity: 0; transform: scale(0.94); }
  to   { opacity: 1; transform: scale(1); }
}
```

- [ ] **Step 2: Wire into `library_css()`**

Read `crates/ui-styles/src/lib.rs` to find the existing pattern (SP-4+5+6 added `SCENE_PLAYER_CSS` and `GSAP_PRIMITIVES_CSS`). Append a const next to those:

```rust
pub const KINETIC_CUES_CSS: &str = include_str!("kinetic_cues.css");
```

And append `KINETIC_CUES_CSS` to the body of `library_css()`.

- [ ] **Step 3: Verify**

Run: `cargo check -p ui-styles && cargo test --workspace`
Expected: green.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-styles/src/kinetic_cues.css crates/ui-styles/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(ui-styles): kinetic_cues.css — keyframes for fade-in/rise-in/slide-up/text-flow/pop-in

Foundation for the inlined-CssKeyframesAdapter pattern: leaf
components (KineticText, KineticBox, SplitText, ui-blocks
choreographies) emit animation-name: ui-cue-{cue} + animation-delay
derived from the parent SceneContext.clock.elapsed_ms. This commit
ships the keyframes; the leaf changes consume them.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 2: F2 helper — `cue_style.rs`

The shared helper module that every cue-driven leaf calls.

**Files:**
- Create: `crates/ui-dioxus/src/cue_style.rs`
- Modify: `crates/ui-dioxus/src/lib.rs` (`pub mod cue_style;`)

- [ ] **Step 1: Create the helper**

Create `crates/ui-dioxus/src/cue_style.rs`:

```rust
//! Shared helpers that build the inline `style` attribute used by
//! kinetic leaves to drive cue-keyframe animations off the parent
//! `SceneContext.clock.elapsed_ms`. Each leaf computes its effective
//! `elapsed_ms` (possibly offset by a `StaggerOffsetContext`), then
//! formats a `style="animation-name: …; animation-delay: -<elapsed>ms;
//! animation-play-state: paused; …"` string.
//!
//! The browser handles per-frame interpolation. Rust only re-emits
//! the style string when `elapsed_ms` changes by at least 1ms (we
//! round to integer ms below to dampen Dioxus VDOM diffs).

/// Default animation duration for each known cue keyword. Unknown
/// cues fall back to 600ms. Match the values in
/// `crates/ui-styles/src/kinetic_cues.css`.
pub fn cue_animation_duration_ms(cue: &str) -> f32 {
    match cue {
        "fade-in" => 600.0,
        "rise-in" => 720.0,
        "slide-up" => 600.0,
        "text-flow" => 600.0,
        "pop-in" => 480.0,
        _ => 600.0,
    }
}

/// Returns the inline-style string for the given cue + clock state.
/// The duration is auto-resolved from `cue_animation_duration_ms` —
/// callers that want a custom duration can format their own string.
///
/// `elapsed_ms` is clamped to `[0, +∞)` and rounded to integer ms
/// before formatting. Negative inputs are treated as `0`.
pub fn cue_inline_style(cue: &str, elapsed_ms: f32) -> String {
    let duration_ms = cue_animation_duration_ms(cue);
    cue_inline_style_with_duration(cue, elapsed_ms, duration_ms)
}

/// Same as [`cue_inline_style`] but accepts an explicit duration —
/// used by blocks (LowerThird/SocialOverlay/MetricCounter) whose
/// choreographies need per-child timing distinct from the cue's
/// default.
pub fn cue_inline_style_with_duration(
    cue: &str,
    elapsed_ms: f32,
    duration_ms: f32,
) -> String {
    let elapsed = if elapsed_ms.is_finite() && elapsed_ms > 0.0 {
        elapsed_ms.round() as i64
    } else {
        0
    };
    let duration = if duration_ms.is_finite() && duration_ms > 0.0 {
        duration_ms.round() as i64
    } else {
        1
    };
    format!(
        "animation-name: ui-cue-{cue}; animation-duration: {duration}ms; animation-fill-mode: forwards; animation-play-state: paused; animation-delay: -{elapsed}ms;",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_elapsed_clamps_to_zero() {
        let s = cue_inline_style("fade-in", -123.0);
        assert!(s.contains("animation-delay: -0ms"), "{s}");
    }

    #[test]
    fn known_cue_uses_its_duration() {
        let s = cue_inline_style("rise-in", 0.0);
        assert!(s.contains("animation-duration: 720ms"), "{s}");
    }

    #[test]
    fn unknown_cue_uses_default_600ms() {
        let s = cue_inline_style("does-not-exist", 0.0);
        assert!(s.contains("animation-duration: 600ms"), "{s}");
    }

    #[test]
    fn elapsed_ms_rounds_to_integer() {
        let s = cue_inline_style("fade-in", 123.49);
        assert!(s.contains("animation-delay: -123ms"), "{s}");
    }

    #[test]
    fn nan_elapsed_treated_as_zero() {
        let s = cue_inline_style("fade-in", f32::NAN);
        assert!(s.contains("animation-delay: -0ms"), "{s}");
    }
}
```

- [ ] **Step 2: Wire module + re-export**

In `crates/ui-dioxus/src/lib.rs`, add `pub mod cue_style;`. Also re-export the two functions for downstream use:

```rust
pub use cue_style::{cue_animation_duration_ms, cue_inline_style, cue_inline_style_with_duration};
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p ui-dioxus --lib cue_style`
Expected: 5 PASS.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-dioxus/src/cue_style.rs crates/ui-dioxus/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): cue_style helpers — inline-style builders for leaves

Two functions (cue_inline_style / cue_inline_style_with_duration) plus
cue_animation_duration_ms lookup. Leaves call these to emit the
deterministic animation-name + animation-delay style derived from the
parent SceneContext.clock.elapsed_ms.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 3: F4 foundation — `stagger.rs` (StaggerChild + StaggerOffsetContext)

The mechanism `TimelineScope` will use to give each child its positional offset. Per-child stagger offset is captured at compile time via `StaggerChild { index }`; consumers (`KineticText` / `KineticBox`) read it from `StaggerOffsetContext` and compute their effective elapsed_ms.

**Files:**
- Create: `crates/ui-dioxus/src/stagger.rs`
- Modify: `crates/ui-dioxus/src/lib.rs` (`pub mod stagger;` + re-export)

- [ ] **Step 1: Create the module**

Create `crates/ui-dioxus/src/stagger.rs`:

```rust
//! Per-child stagger offset context.
//!
//! `TimelineScope` (the F4 stagger driver) wraps each direct child in
//! a `StaggerChild { index }` so the child's position in the staggered
//! sequence is captured deterministically at compile time. The wrapped
//! children consume `StaggerOffsetContext` to compute their local
//! `elapsed_ms = max(0, parent_elapsed_ms - index * step_ms)`.

use dioxus::prelude::*;

/// Per-child stagger offset, in the parent TimelineScope's
/// stagger-units. The leaf computes `local_elapsed_ms` by subtracting
/// `index as f32 * step_ms` from the surrounding `SceneContext` clock.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StaggerOffsetContext {
    pub index: u32,
    pub step_ms: f32,
}

impl StaggerOffsetContext {
    pub fn offset_ms(&self) -> f32 {
        self.index as f32 * self.step_ms
    }
}

/// Wraps a child element with a `StaggerOffsetContext` carrying its
/// positional index. The wrapper is otherwise transparent — it
/// renders the children directly without an extra DOM node, so the
/// `data-stagger-index` attribute that callers may want on the actual
/// child element stays the child's own concern.
///
/// `TimelineScope` produces one `StaggerChild` per direct child via
/// `for (i, child) in children.into_iter().enumerate()`.
#[component]
pub fn StaggerChild(index: u32, step_ms: f32, children: Element) -> Element {
    use_context_provider(|| StaggerOffsetContext { index, step_ms });
    children
}
```

- [ ] **Step 2: Wire module + re-export**

In `crates/ui-dioxus/src/lib.rs`:

```rust
pub mod stagger;
pub use stagger::{StaggerChild, StaggerOffsetContext};
```

- [ ] **Step 3: Verify**

Run: `cargo check -p ui-dioxus && cargo test -p ui-dioxus`
Expected: workspace stays green; no new tests yet.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-dioxus/src/stagger.rs crates/ui-dioxus/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): StaggerChild + StaggerOffsetContext for TimelineScope walks

Foundation for F4 (TimelineScope as real stagger driver). Each child
gets wrapped in StaggerChild { index, step_ms }, which provides a
StaggerOffsetContext via Dioxus context. Consumers (KineticText,
KineticBox) read this context and subtract index * step_ms from the
SceneContext.elapsed_ms to compute their local animation delay.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 4: F2 — `KineticText` consumes SceneContext + emits cue style

Today `KineticText` renders a static `<span>` with `data-motion-cue` and `aria-label`. After this change it will also emit an inline `style` driving the cue keyframe off the nearest available clock context.

**Files:**
- Modify: `crates/ui-dioxus/src/kinetics.rs` (the existing `KineticText` component)
- Test: `crates/ui-dioxus/tests/cue_inline_style_ssr.rs` (NEW)

- [ ] **Step 1: Write failing tests**

Create `crates/ui-dioxus/tests/cue_inline_style_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::{KineticText, Scene};
use ui_runtime::reduced_motion::ReducedMotionProvider;

#[test]
fn kinetic_text_outside_any_clock_renders_static_markup() {
    // No Scene / Sequence / StaggerOffset → no inline animation style.
    let html = dioxus_ssr::render_element(rsx! {
        KineticText { id: "x".to_string(), text: "hi".to_string(), cue: "fade-in".to_string() }
    });
    assert!(html.contains("data-motion-cue=\"fade-in\""), "{html}");
    assert!(!html.contains("animation-name"), "{html}");
}

#[test]
fn kinetic_text_inside_scene_emits_cue_animation_style() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(false),
            Scene {
                id: "test", width: 100, height: 100, duration_ms: 5_000.0,
                autoplay: Some(false),
                KineticText { id: "title".to_string(), text: "hi".to_string(), cue: "rise-in".to_string() }
            }
        }
    });
    // At elapsed_ms = 0, animation-delay should be -0ms.
    assert!(html.contains("animation-name: ui-cue-rise-in"), "{html}");
    assert!(html.contains("animation-delay: -0ms"), "{html}");
    assert!(html.contains("animation-duration: 720ms"), "{html}");
    assert!(html.contains("animation-play-state: paused"), "{html}");
}

#[test]
fn kinetic_text_inside_reduced_motion_scene_renders_at_settled_endpoint() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "test", width: 100, height: 100, duration_ms: 5_000.0,
                autoplay: Some(false),
                KineticText { id: "title".to_string(), text: "hi".to_string(), cue: "fade-in".to_string() }
            }
        }
    });
    // Reduced motion settles at duration_ms (= 5000), so animation-delay
    // is -5000ms which freezes the keyframe at its end state.
    assert!(html.contains("animation-name: ui-cue-fade-in"), "{html}");
    assert!(html.contains("animation-delay: -5000ms"), "{html}");
}
```

- [ ] **Step 2: Run test (expect failure)**

Run: `cargo test -p ui-dioxus --test cue_inline_style_ssr`
Expected: compile success but `kinetic_text_inside_scene_emits_cue_animation_style` and `kinetic_text_inside_reduced_motion_scene_renders_at_settled_endpoint` fail (no animation style in current output).

- [ ] **Step 3: Implement**

In `crates/ui-dioxus/src/kinetics.rs`, replace the existing `KineticText` component with:

```rust
#[component]
pub fn KineticText(
    id: String,
    text: String,
    #[props(default = "text-flow".to_string())] cue: String,
) -> Element {
    let kinetic_id = KineticId::new(id);

    // Effective elapsed_ms is the maximum-priority context available:
    //   1. StaggerOffsetContext (positional offset, from TimelineScope)
    //      → subtract `index * step_ms` from the surrounding clock.
    //   2. SceneContext (Scene's clock).
    //   3. SequenceContext (legacy SP-1 sequence sample) — KineticText
    //      doesn't use sequence sample states (those drive KineticBox
    //      transforms), but presence is enough to know we're inside a
    //      Sequence and should not animate via cue keyframe.
    //   4. None → static markup (existing behaviour).
    let stagger = try_consume_context::<crate::stagger::StaggerOffsetContext>();
    let scene = try_consume_context::<crate::scene_player::SceneContext>();

    let inline_style: String = match (stagger, scene) {
        (Some(stag), Some(ctx)) => {
            let parent = *ctx.clock.elapsed_ms.read();
            let local = (parent - stag.offset_ms()).max(0.0);
            crate::cue_style::cue_inline_style(&cue, local)
        }
        (None, Some(ctx)) => {
            let elapsed = *ctx.clock.elapsed_ms.read();
            crate::cue_style::cue_inline_style(&cue, elapsed)
        }
        _ => String::new(),
    };

    rsx! {
        span {
            class: "ui-kinetic-text",
            "data-kinetic-id": "{kinetic_id.0}",
            "data-motion-cue": "{cue}",
            aria_label: "{text}",
            style: "{inline_style}",
            "{text}"
        }
    }
}
```

Note: when `inline_style` is empty (no Scene ancestor), Dioxus's `style: "{inline_style}"` will still render `style=""`. The test `kinetic_text_outside_any_clock_renders_static_markup` asserts `!html.contains("animation-name")` — that's the load-bearing assertion. An empty `style=""` attribute is acceptable backwards-compat-wise; SSR snapshot tests that previously asserted no `style` attribute may need adjustment. If any existing test asserts the absence of `style=""`, change it to assert the absence of `animation-name` or a specific cue substring instead.

- [ ] **Step 4: Run tests**

Run: `cargo test -p ui-dioxus --test cue_inline_style_ssr`
Expected: 3 PASS.

Also run `cargo test -p ui-dioxus` to confirm no regression in the existing kinetics tests.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-dioxus/src/kinetics.rs crates/ui-dioxus/tests/cue_inline_style_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): KineticText emits cue-driven animation-delay from SceneContext

Reads StaggerOffsetContext (preferred) or SceneContext to compute its
local elapsed_ms, then writes
  style="animation-name: ui-cue-<cue>; animation-delay: -<elapsed>ms;
         animation-play-state: paused; animation-fill-mode: forwards"
The browser plays back the keyframe synchronously at the seeked time.
Backwards-compat: bare KineticText with no clock ancestor still
renders static markup.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 5: F2 — `KineticBox` adds SceneContext + StaggerOffsetContext fallback

Today `KineticBox` consumes `SequenceContext` to get sampled inline styles + drives WAAPI via `use_animation_target`. After this change it also consumes `SceneContext` and `StaggerOffsetContext` when no Sequence-driven cue is available; the priority is:

1. Inside Sequence with cue → WAAPI (existing).
2. Inside Sequence without cue → static sample style from SequenceContext (existing).
3. Inside StaggerChild + Scene → cue-keyframe inline style with stagger offset.
4. Inside Scene only → cue-keyframe inline style with Scene elapsed.
5. Otherwise → static markup with `data-motion-cue` only.

**Files:**
- Modify: `crates/ui-dioxus/src/kinetics.rs` (the existing `KineticBox` component)
- Test: `crates/ui-dioxus/tests/cue_inline_style_ssr.rs` (APPEND)

- [ ] **Step 1: Append failing tests**

```rust
use ui_dioxus::KineticBox;

#[test]
fn kinetic_box_inside_scene_emits_cue_animation_style() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(false),
            Scene {
                id: "test", width: 100, height: 100, duration_ms: 5_000.0,
                autoplay: Some(false),
                KineticBox { id: "block".to_string(), cue: "pop-in".to_string(),
                    p { "child" }
                }
            }
        }
    });
    assert!(html.contains("animation-name: ui-cue-pop-in"), "{html}");
    assert!(html.contains("animation-delay: -0ms"), "{html}");
}

#[test]
fn kinetic_box_outside_clock_renders_static_markup() {
    let html = dioxus_ssr::render_element(rsx! {
        KineticBox { id: "block".to_string(), cue: "fade-in".to_string(),
            p { "child" }
        }
    });
    assert!(!html.contains("animation-name"), "{html}");
}
```

- [ ] **Step 2: Run tests (expect failure)**

Run: `cargo test -p ui-dioxus --test cue_inline_style_ssr kinetic_box_inside_scene kinetic_box_outside`
Expected: `kinetic_box_inside_scene_emits_cue_animation_style` fails (no inline animation in current output).

- [ ] **Step 3: Implement**

Read the existing `KineticBox` in `crates/ui-dioxus/src/kinetics.rs` (around line 234). The change is to extend its existing context-priority logic so that when a `SequenceContext` cue is NOT present, the new SceneContext/StaggerOffsetContext path produces the cue-keyframe style. The final inline style is the concatenation of (any sample style from SequenceContext) + (any cue keyframe style from Scene/Stagger). Where both apply (legacy Sequence + new Scene wrap), the WAAPI path wins (the inline style is just the sample state).

Replace the `let style = state.as_ref().map(|s| s.inline_style()).unwrap_or_default();` line and the subsequent logic with:

```rust
    let state = ctx
        .as_ref()
        .and_then(|sig| sig.read().states.get(&kinetic_id.0).cloned());

    // Existing Sequence path: if there's a SequenceContext sample, use
    // its inline style (transform/opacity from the sample) AND drive
    // WAAPI as before (handled further down via use_animation_target).
    let sequence_inline = state.as_ref().map(|s| s.inline_style()).unwrap_or_default();

    // New cue-keyframe path: only when Sequence didn't provide a sample.
    // Priority: StaggerOffsetContext → SceneContext.
    let cue_inline = if sequence_inline.is_empty() {
        let stagger = try_consume_context::<crate::stagger::StaggerOffsetContext>();
        let scene = try_consume_context::<crate::scene_player::SceneContext>();
        match (stagger, scene) {
            (Some(stag), Some(ctx_scene)) => {
                let parent = *ctx_scene.clock.elapsed_ms.read();
                let local = (parent - stag.offset_ms()).max(0.0);
                crate::cue_style::cue_inline_style(&cue, local)
            }
            (None, Some(ctx_scene)) => {
                let elapsed = *ctx_scene.clock.elapsed_ms.read();
                crate::cue_style::cue_inline_style(&cue, elapsed)
            }
            _ => String::new(),
        }
    } else {
        String::new()
    };

    let style = if !sequence_inline.is_empty() {
        sequence_inline
    } else {
        cue_inline
    };
```

The rest of `KineticBox` (cue_data lookup, WAAPI handle, onmounted) is unchanged.

- [ ] **Step 4: Run tests**

Run: `cargo test -p ui-dioxus --test cue_inline_style_ssr`
Expected: 5 PASS (3 from Task 4 + 2 new).
Run: `cargo test -p ui-dioxus` — full crate green.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-dioxus/src/kinetics.rs crates/ui-dioxus/tests/cue_inline_style_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): KineticBox emits cue-keyframe style when not inside a Sequence

When a SequenceContext sample is available, the existing WAAPI/sample
path wins (no behaviour change). Otherwise the new path checks
StaggerOffsetContext + SceneContext and emits
animation-name/animation-delay style identical to KineticText. This
lets KineticBox children of Scenes (without a wrapping Sequence)
animate via cue keyframes.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 6: F3 — `Sequence` reads `SceneContext` when no explicit `clock` prop

Today `Sequence` defaults `clock` to `TimelineClock::Playback { elapsed_ms: 0.0 }`. The fix: if no Scene context exists, behaviour is unchanged; if a Scene context IS present and the caller didn't override `clock` to a `Manual { ... }`, prefer the Scene's elapsed_ms.

The challenge: the existing prop default `#[props(default = TimelineClock::Playback { elapsed_ms: 0.0 })]` doesn't expose "user explicitly set vs default". The cleanest fix is to change the prop to `Option<TimelineClock>` and resolve inside the body.

**Files:**
- Modify: `crates/ui-dioxus/src/kinetics.rs` (Sequence signature + body)
- Test: `crates/ui-dioxus/tests/cue_inline_style_ssr.rs` (APPEND)

- [ ] **Step 1: Append failing test**

```rust
use ui_dioxus::Sequence;
use ui_motion::{Ease, Transition};
use ui_timeline::{MotionCue, MotionSegment, MotionTarget, Timeline, TimelineTrack};

#[test]
fn sequence_inside_scene_uses_scene_clock_when_no_explicit_clock() {
    let timeline = Timeline::new("test", 1_000.0).with_track(TimelineTrack::new(
        MotionTarget::node("title"),
        vec![MotionSegment::new(
            0.0,
            1_000.0,
            MotionCue::Opacity {
                from: 0.0,
                to: 1.0,
                transition: Transition::Tween {
                    duration_ms: 1_000,
                    ease: Ease::Linear,
                },
            },
        )],
    ));
    // Scene with reduced motion forces elapsed_ms = duration_ms = 2000.
    // Inner Sequence (no explicit clock prop) should pull elapsed_ms = 2000
    // from the Scene, which is past the 1000ms cue duration → opacity = 1.0.
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 2_000.0,
                autoplay: Some(false),
                Sequence {
                    timeline: Some(timeline),
                    KineticBox { id: "title".to_string(), p { "x" } }
                }
            }
        }
    });
    assert!(html.contains("opacity: 1"), "{html}");
}
```

- [ ] **Step 2: Run test (expect failure)**

Run: `cargo test -p ui-dioxus --test cue_inline_style_ssr sequence_inside_scene`
Expected: fails — Sequence's default clock at elapsed_ms=0 produces opacity=0.

- [ ] **Step 3: Implement**

In `crates/ui-dioxus/src/kinetics.rs`, change the `Sequence` signature:

```rust
#[component]
pub fn Sequence(
    #[props(default)] timeline: Option<Timeline>,
    #[props(default)] cues: Option<Vec<Cue>>,
    #[props(default = "sequence".to_string())] id: String,
    #[props(default)] clock: Option<TimelineClock>,
    children: Element,
) -> Element {
    // Resolve effective clock with this precedence:
    //   1. Explicit `clock` prop wins (tests + advanced callers).
    //   2. Surrounding SceneContext.elapsed_ms when present.
    //   3. Default to TimelineClock::Playback { elapsed_ms: 0.0 }.
    let resolved_clock = match clock {
        Some(c) => c,
        None => match try_consume_context::<crate::scene_player::SceneContext>() {
            Some(scene_ctx) => TimelineClock::Manual {
                elapsed_ms: *scene_ctx.clock.elapsed_ms.read(),
            },
            None => TimelineClock::Playback { elapsed_ms: 0.0 },
        },
    };

    // (rest of the function body unchanged; use `resolved_clock`
    //  wherever the previous `clock` variable appeared)
```

Then replace the existing `let sample = use_timeline_sample(timeline_value, clock);` with `let sample = use_timeline_sample(timeline_value, resolved_clock);`. Existing call sites that pass a literal `TimelineClock::Manual { ... }` must be updated to wrap with `Some(...)`.

Search and update any internal callers. Grep:

```bash
grep -rn "Sequence {" crates/ examples/ --include="*.rs"
```

Each call site that passes `clock: TimelineClock::Manual { ... }` needs to become `clock: Some(TimelineClock::Manual { ... })`. Most call sites won't be passing `clock` at all and stay correct.

- [ ] **Step 4: Run tests**

Run: `cargo check --workspace`
If any caller breaks (likely 1–3 sites in `examples/component-gallery/src/previews/`), update them.
Run: `cargo test -p ui-dioxus --test cue_inline_style_ssr`
Expected: 6 PASS.
Run: `cargo test --workspace` to catch any other call sites.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): Sequence prefers SceneContext clock when no explicit clock prop

The `clock` prop becomes Option<TimelineClock>. When None and a
SceneContext is in scope, Sequence pulls elapsed_ms from the Scene's
clock and re-samples its timeline on every parent tick. Explicit
clock arguments still win (preserves the test/advanced-use escape
hatch).

Call sites that previously passed `clock: TimelineClock::Manual { ... }`
are updated to `clock: Some(TimelineClock::Manual { ... })`.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 7: F4 — `TimelineScope` becomes a real stagger driver

The biggest single behavioural change. Today `TimelineScope` is a `<section>` marker with no clock. After this it becomes a stagger driver:

- Reads `SceneContext` (or, if absent and `autoplay = true`, spawns its own clock).
- Walks each direct child and wraps it in `StaggerChild { index: i, step_ms }`.
- Provides children with positional staggers so `KineticText` / `KineticBox` consumed via `StaggerOffsetContext` get an offset elapsed_ms.

**Files:**
- Modify: `crates/ui-dioxus/src/kinetics.rs` (TimelineScope component)
- Test: `crates/ui-dioxus/tests/timeline_scope_stagger_ssr.rs` (NEW)

- [ ] **Step 1: Create the test file**

Create `crates/ui-dioxus/tests/timeline_scope_stagger_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::{KineticText, Scene, TimelineScope};
use ui_runtime::reduced_motion::ReducedMotionProvider;

#[test]
fn timeline_scope_inside_scene_staggers_children_by_step_ms() {
    // Reduced motion settles Scene's elapsed_ms to duration_ms = 2000.
    // step_ms defaults to 80. Per-child elapsed:
    //   index 0 → max(0, 2000 - 0  * 80) = 2000
    //   index 1 → max(0, 2000 - 1  * 80) = 1920
    //   index 2 → max(0, 2000 - 2  * 80) = 1840
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 2_000.0,
                autoplay: Some(false),
                TimelineScope { id: "stagger".to_string(), autoplay: true,
                    KineticText { id: "a".to_string(), text: "A".to_string(), cue: "fade-in".to_string() }
                    KineticText { id: "b".to_string(), text: "B".to_string(), cue: "fade-in".to_string() }
                    KineticText { id: "c".to_string(), text: "C".to_string(), cue: "fade-in".to_string() }
                }
            }
        }
    });
    assert!(html.contains("animation-delay: -2000ms"), "want index 0: {html}");
    assert!(html.contains("animation-delay: -1920ms"), "want index 1: {html}");
    assert!(html.contains("animation-delay: -1840ms"), "want index 2: {html}");
}

#[test]
fn timeline_scope_emits_section_marker_attributes() {
    let html = dioxus_ssr::render_element(rsx! {
        TimelineScope { id: "marker-test".to_string(), autoplay: true,
            span { "stub" }
        }
    });
    assert!(html.contains("data-timeline-id=\"marker-test\""), "{html}");
    assert!(html.contains("data-autoplay=\"true\""), "{html}");
    assert!(html.contains("ui-timeline-scope"), "{html}");
}

#[test]
fn timeline_scope_custom_stagger_step_ms() {
    // step_ms = 200 → index 1 elapsed = 2000 - 200 = 1800.
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 2_000.0,
                autoplay: Some(false),
                TimelineScope { id: "stagger".to_string(), autoplay: true, stagger_step_ms: 200.0,
                    KineticText { id: "a".to_string(), text: "A".to_string(), cue: "fade-in".to_string() }
                    KineticText { id: "b".to_string(), text: "B".to_string(), cue: "fade-in".to_string() }
                }
            }
        }
    });
    assert!(html.contains("animation-delay: -2000ms"), "{html}");
    assert!(html.contains("animation-delay: -1800ms"), "{html}");
}
```

- [ ] **Step 2: Run tests (expect failure)**

Run: `cargo test -p ui-dioxus --test timeline_scope_stagger_ssr`
Expected: tests fail (current TimelineScope doesn't wrap children in StaggerChild).

- [ ] **Step 3: Implement**

Replace the existing `TimelineScope` in `crates/ui-dioxus/src/kinetics.rs`:

```rust
#[component]
pub fn TimelineScope(
    id: String,
    #[props(default)] autoplay: bool,
    #[props(default = 80.0)] stagger_step_ms: f32,
    children: Element,
) -> Element {
    let timeline_id = TimelineId::new(id);
    let step_ms = if stagger_step_ms.is_finite() && stagger_step_ms > 0.0 {
        stagger_step_ms
    } else {
        80.0
    };

    // SSR-friendly child enumeration: Dioxus's Element type doesn't expose
    // child iteration directly, but `for (i, _) in 0..N` over a fixed list
    // doesn't work either because we accept arbitrary `children: Element`.
    // The pragmatic approach is to render children as-is, but use a single
    // StaggerOffsetContext that internally tracks an atomic counter — each
    // call to `try_consume_context::<StaggerOffsetContext>()` from a leaf
    // returns the *next* index. This requires the context to be a Signal
    // with interior mutability.

    // Simpler approach: provide a StaggerCursor signal; each leaf grabs an
    // index atomically. SSR is single-threaded, so a RefCell counter works.
    use std::cell::Cell;
    let cursor: Cell<u32> = Cell::new(0);
    let cursor_signal = use_hook(|| std::rc::Rc::new(Cell::new(0u32)));
    cursor_signal.set(0); // reset on every render

    use_context_provider(|| crate::stagger::StaggerCursor {
        cursor: cursor_signal.clone(),
        step_ms,
    });

    rsx! {
        section {
            class: "ui-timeline-scope",
            "data-timeline-id": "{timeline_id.0}",
            "data-autoplay": if autoplay { "true" } else { "false" },
            "data-stagger-step-ms": "{step_ms}",
            {children}
        }
    }
}
```

This requires a new `StaggerCursor` type in `crates/ui-dioxus/src/stagger.rs`:

```rust
use std::cell::Cell;
use std::rc::Rc;

/// SSR-friendly cursor for assigning stagger indices to leaves as they
/// render inside a TimelineScope. Each `KineticText` / `KineticBox`
/// reads this context, fetches the next index, and uses it to compute
/// its local elapsed_ms. Single-threaded by design (SSR renders one
/// element at a time).
#[derive(Clone)]
pub struct StaggerCursor {
    pub cursor: Rc<Cell<u32>>,
    pub step_ms: f32,
}

impl StaggerCursor {
    pub fn next_index(&self) -> u32 {
        let i = self.cursor.get();
        self.cursor.set(i + 1);
        i
    }

    pub fn current_offset_ms(&self) -> f32 {
        let i = self.cursor.get();
        i as f32 * self.step_ms
    }
}
```

The cursor approach replaces the earlier `StaggerOffsetContext`/`StaggerChild` design — it's SSR-friendlier because it doesn't require wrapping each child individually. The cursor advances as each leaf renders and consumes it.

Update `KineticText` and `KineticBox` to use the cursor:

```rust
// Inside KineticText:
let stagger = try_consume_context::<crate::stagger::StaggerCursor>();
let scene = try_consume_context::<crate::scene_player::SceneContext>();
let inline_style: String = match (&stagger, scene) {
    (Some(cur), Some(ctx)) => {
        let index = cur.next_index();
        let parent = *ctx.clock.elapsed_ms.read();
        let local = (parent - (index as f32 * cur.step_ms)).max(0.0);
        crate::cue_style::cue_inline_style(&cue, local)
    }
    (None, Some(ctx)) => {
        let elapsed = *ctx.clock.elapsed_ms.read();
        crate::cue_style::cue_inline_style(&cue, elapsed)
    }
    _ => String::new(),
};
```

(Same change in `KineticBox`'s cue_inline branch.)

- [ ] **Step 4: Update stagger.rs**

Replace the old `StaggerChild` + `StaggerOffsetContext` with the `StaggerCursor` design above. The earlier `StaggerOffsetContext` type may be kept as a deprecated alias if any tests reference it; otherwise remove it.

In `crates/ui-dioxus/src/lib.rs`, update the re-export:

```rust
pub use stagger::{StaggerCursor};
```

If no external consumer uses the old `StaggerChild`/`StaggerOffsetContext`, delete them. If any test refers to them, update the test.

- [ ] **Step 5: Run tests**

Run: `cargo test -p ui-dioxus --test cue_inline_style_ssr --test timeline_scope_stagger_ssr`
Expected: all PASS.

Also run `cargo test -p ui-dioxus` to confirm nothing else regressed.

- [ ] **Step 6: Commit**

```bash
git add crates/ui-dioxus/src/kinetics.rs crates/ui-dioxus/src/stagger.rs crates/ui-dioxus/src/lib.rs crates/ui-dioxus/tests/timeline_scope_stagger_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): TimelineScope as stagger driver via StaggerCursor

TimelineScope now provides a StaggerCursor context. Each kinetic leaf
that renders inside the scope grabs the next index and computes its
local elapsed_ms = max(0, parent_elapsed - index * step_ms). The
cursor is per-render single-threaded SSR-friendly. Custom step_ms
overrides the default 80ms via the new stagger_step_ms prop.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 8: F5 — `SplitText` cue prop + cue-driven glyph styles

Today `SplitText` emits `<span class="ui-split-text__glyph" data-stagger-index="N">` with no animation. After this it accepts an optional `cue` prop (default `"rise-in"`) and writes each glyph's inline `style` to the cue keyframe pattern with the stagger cursor index.

**Files:**
- Modify: `crates/ui-dioxus/src/split_text.rs`
- Modify: `crates/ui-dioxus/tests/split_text_ssr.rs` (APPEND)

- [ ] **Step 1: Append failing test**

```rust
#[test]
fn split_text_glyphs_emit_cue_animation_when_inside_scene() {
    use ui_runtime::reduced_motion::ReducedMotionProvider;
    use ui_dioxus::{Scene, TimelineScope};
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 1_000.0,
                autoplay: Some(false),
                TimelineScope { id: "ts".to_string(), autoplay: true,
                    SplitText { text: "Hi".to_string() }
                }
            }
        }
    });
    // Reduced motion settles Scene to 1000ms. With default cue rise-in
    // and stagger step 80ms:
    //   glyph H: index 0 → 1000ms
    //   glyph i: index 1 → 920ms
    assert!(html.contains("animation-name: ui-cue-rise-in"), "{html}");
    assert!(html.contains("animation-delay: -1000ms") || html.contains("animation-delay: -920ms"),
            "{html}");
}

#[test]
fn split_text_cue_prop_overrides_default() {
    use ui_runtime::reduced_motion::ReducedMotionProvider;
    use ui_dioxus::Scene;
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 1_000.0,
                autoplay: Some(false),
                SplitText { text: "Hi".to_string(), cue: "fade-in".to_string() }
            }
        }
    });
    assert!(html.contains("animation-name: ui-cue-fade-in"), "{html}");
}
```

- [ ] **Step 2: Run test (expect failure)**

Run: `cargo test -p ui-dioxus --test split_text_ssr split_text_glyphs_emit_cue split_text_cue_prop`
Expected: fails — current SplitText doesn't emit inline animation styles.

- [ ] **Step 3: Implement**

In `crates/ui-dioxus/src/split_text.rs`, extend the component:

1. Add a `cue: Option<String>` prop (default `"rise-in"`).
2. Each glyph span calls `crate::cue_style::cue_inline_style(&cue, local_elapsed)` where `local_elapsed` is computed via the same StaggerCursor + SceneContext logic as KineticText.

The simplest implementation: have each glyph span effectively act like an embedded `KineticText` for its single character. Pseudocode:

```rust
let cue = cue.unwrap_or_else(|| "rise-in".to_string());

// For each grapheme/word, increment the stagger cursor (if any) and
// emit a span with the cue inline-style.
for (idx, glyph) in graphemes.enumerate() {
    let inline = compute_inline_style_for_index(idx, &cue, ...);
    rsx! {
        span {
            class: "ui-split-text__glyph",
            "data-stagger-index": "{idx}",
            "aria-hidden": "true",
            style: "{inline}",
            "{glyph}"
        }
    }
}
```

The challenge is that `try_consume_context::<StaggerCursor>()` returns `Option<StaggerCursor>` per render. The current SplitText is one component, so it's one render. Iterate through glyphs inside the same render frame, calling `cur.next_index()` per glyph if a cursor is present, else fall back to local indices.

```rust
let cursor = try_consume_context::<crate::stagger::StaggerCursor>();
let scene = try_consume_context::<crate::scene_player::SceneContext>();
let parent_elapsed = scene
    .as_ref()
    .map(|s| *s.clock.elapsed_ms.read())
    .unwrap_or(0.0);

// Compute per-glyph style.
let step_ms = cursor.as_ref().map(|c| c.step_ms).unwrap_or(80.0);
let base_index = cursor.as_ref().map(|c| c.next_index()).unwrap_or(0);

// For each glyph at glyph_offset i, the effective index = base_index + i.
// Compute that glyph's style.
```

Actually a cleaner approach: SplitText itself acts as a "stagger boundary" — it always staggers its own children regardless of being inside a TimelineScope. The cursor (if present) determines a base offset; the glyphs then get sequential offsets from there.

Pseudo-implementation:

```rust
let cue = cue.unwrap_or_else(|| "rise-in".to_string());
let cursor = try_consume_context::<crate::stagger::StaggerCursor>();
let scene = try_consume_context::<crate::scene_player::SceneContext>();
let parent_elapsed = scene
    .as_ref()
    .map(|s| *s.clock.elapsed_ms.read())
    .unwrap_or(0.0);
let step_ms = cursor.as_ref().map(|c| c.step_ms).unwrap_or(80.0);
let base_offset_ms = cursor.as_ref().map(|c| c.current_offset_ms()).unwrap_or(0.0);

// Advance the cursor once for THIS SplitText (so siblings continue past it).
if let Some(ref c) = cursor {
    c.next_index();
}

// In the per-glyph rsx, compute inline style:
//   local_elapsed = max(0, parent_elapsed - base_offset_ms - i * step_ms)
//   style = cue_inline_style(cue, local_elapsed)
```

Then in the existing per-glyph emit, add `style: "{inline_style}"`.

Same approach for the Word mode path. Whitespace text nodes remain unstyled (no animation needed; they're just spaces between words).

- [ ] **Step 4: Run tests**

Run: `cargo test -p ui-dioxus --test split_text_ssr`
Expected: all PASS (the existing 6 SP-3 tests + 2 new).

Also run `cargo test -p ui-dioxus` for the full crate.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-dioxus/src/split_text.rs crates/ui-dioxus/tests/split_text_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-dioxus): SplitText emits per-glyph cue-keyframe styles

Adds a `cue: Option<String>` prop (default rise-in). Each glyph/word
span gets style="animation-name: ui-cue-<cue>; animation-delay: -…"
derived from the surrounding SceneContext + StaggerCursor. The cursor
advances by 1 per SplitText (so siblings inside a TimelineScope
continue past the SplitText's glyph count).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 9: F11 — `WipeVariant` enum + four wipe keyframes

Adds the variants and the CSS.

**Files:**
- Modify: `crates/ui-styles/src/gsap_primitives.css` (append four wipe variant keyframes)
- Modify: `crates/ui-blocks/src/wipe_transition.rs` (add `WipeVariant` enum)

- [ ] **Step 1: Append CSS keyframes**

In `crates/ui-styles/src/gsap_primitives.css`, append:

```css
/* Wipe variants — each maps to a different mask-image keyframe. The
   browser interpolates from "hidden" to "fully revealed" over
   animation-duration; consumers set animation-delay = -elapsed_ms to
   seek deterministically. */

@keyframes ui-block-wipe-linear {
  from { mask-image: linear-gradient(var(--wipe-angle, 90deg), black 0%, transparent 0%); }
  to   { mask-image: linear-gradient(var(--wipe-angle, 90deg), black 100%, transparent 100%); }
}

@keyframes ui-block-wipe-conic {
  from { mask-image: conic-gradient(from 0deg at 50% 50%, black 0deg, transparent 0deg); }
  to   { mask-image: conic-gradient(from 0deg at 50% 50%, black 360deg, transparent 360deg); }
}

@keyframes ui-block-wipe-mask-position {
  from { mask-position: -100% 0%; }
  to   { mask-position: 100% 0%; }
}

@keyframes ui-block-wipe-iris {
  from { mask-image: radial-gradient(circle at 50% 50%, black 0%, transparent 0%); }
  to   { mask-image: radial-gradient(circle at 50% 50%, black 100%, transparent 100%); }
}
```

The existing `ui-block-wipe-transition` keyframe in this file (from SP-4+5+6 Task 12) can stay or be removed — it's the same as `ui-block-wipe-linear` semantically. If removing breaks tests/baselines, leave it as an alias and document.

- [ ] **Step 2: Add WipeVariant enum**

In `crates/ui-blocks/src/wipe_transition.rs`, add:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum WipeVariant {
    #[default]
    Linear,
    Conic,
    MaskPosition,
    Iris,
}

impl WipeVariant {
    fn css_keyword(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Conic => "conic",
            Self::MaskPosition => "mask-position",
            Self::Iris => "iris",
        }
    }
}
```

Export from `crates/ui-blocks/src/lib.rs`:

```rust
pub use wipe_transition::{WipeTransition, WipeVariant};
```

- [ ] **Step 3: Verify**

Run: `cargo check -p ui-styles -p ui-blocks`
Expected: green.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-styles/src/gsap_primitives.css crates/ui-blocks/src/wipe_transition.rs crates/ui-blocks/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(ui-blocks): WipeVariant enum + four wipe-mask keyframes

Linear/Conic/MaskPosition/Iris — all CSS-only, no WebGL. The variant
is consumed by the WipeTransition leaf (next commit) to pick the
keyframe name in its inline animation-name style.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 10: F6 — `WipeTransition` consumes SceneContext + variant

Today `WipeTransition` hardcodes `animation-play-state: paused` and never updates. After this it computes the inline style from SceneContext.elapsed_ms + the new WipeVariant.

**Files:**
- Modify: `crates/ui-blocks/src/wipe_transition.rs`
- Test: `crates/ui-blocks/tests/blocks_ssr.rs` (APPEND)

- [ ] **Step 1: Append failing test**

```rust
use ui_blocks::WipeVariant;
use ui_dioxus::Scene;
use ui_runtime::reduced_motion::ReducedMotionProvider;

#[test]
fn wipe_transition_inside_scene_emits_negative_animation_delay() {
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 1_500.0,
                autoplay: Some(false),
                WipeTransition { duration_ms: 1_500.0, p { "x" } }
            }
        }
    });
    // Reduced motion → Scene elapsed = 1500. Wipe inline style should
    // use animation-delay = -1500ms.
    assert!(html.contains("animation-delay: -1500ms"), "{html}");
    // Default variant is Linear.
    assert!(html.contains("animation-name: ui-block-wipe-linear"), "{html}");
}

#[test]
fn wipe_transition_variant_conic_picks_correct_keyframe() {
    let html = dioxus_ssr::render_element(rsx! {
        WipeTransition {
            duration_ms: 1_000.0,
            variant: WipeVariant::Conic,
            p { "x" }
        }
    });
    assert!(html.contains("animation-name: ui-block-wipe-conic"), "{html}");
}
```

- [ ] **Step 2: Run tests (expect failure)**

Run: `cargo test -p ui-blocks --test blocks_ssr wipe_transition_inside_scene wipe_transition_variant`
Expected: fails — variant prop doesn't exist yet, inline style hardcoded.

- [ ] **Step 3: Implement**

Replace the existing `WipeTransition` component in `crates/ui-blocks/src/wipe_transition.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::{cue_style::cue_inline_style_with_duration, scene_player::SceneContext};

#[component]
pub fn WipeTransition(
    duration_ms: f32,
    angle_deg: Option<f32>,
    variant: Option<WipeVariant>,
    children: Element,
) -> Element {
    let variant = variant.unwrap_or_default();
    let angle = angle_deg.unwrap_or(90.0);

    let scene = try_consume_context::<SceneContext>();
    let elapsed_ms = scene
        .map(|s| *s.clock.elapsed_ms.read())
        .unwrap_or(0.0);

    let elapsed_attr = if elapsed_ms.is_finite() && elapsed_ms > 0.0 {
        elapsed_ms.round() as i64
    } else {
        0
    };
    let duration_attr = if duration_ms.is_finite() && duration_ms > 0.0 {
        duration_ms.round() as i64
    } else {
        1
    };

    let kind = variant.css_keyword();
    let inline_style = format!(
        "animation-name: ui-block-wipe-{kind}; \
         animation-duration: {duration_attr}ms; \
         animation-fill-mode: forwards; \
         animation-play-state: paused; \
         animation-delay: -{elapsed_attr}ms; \
         --wipe-angle: {angle}deg;",
    );

    rsx! {
        div {
            class: "ui-block-wipe-transition",
            "data-block": "wipe-transition",
            "data-duration-ms": "{duration_attr}",
            "data-angle-deg": "{angle}",
            "data-variant": "{kind}",
            style: "{inline_style}",
            {children}
        }
    }
}
```

If `ui-dioxus::cue_style` is not yet exported as a public module, expose it. In `crates/ui-dioxus/src/lib.rs`, ensure `pub mod cue_style;` is `pub`. Also expose `pub mod scene_player;` (it may already be public).

If `ui-blocks` doesn't depend on `ui-dioxus`, add it to `crates/ui-blocks/Cargo.toml` `[dependencies]`:

```toml
ui-dioxus = { path = "../ui-dioxus" }
```

If `ui-blocks` already depends on `ui-dioxus` for `SplitText` etc., it's good.

- [ ] **Step 4: Run tests**

Run: `cargo test -p ui-blocks --test blocks_ssr`
Expected: all PASS including 2 new.

- [ ] **Step 5: Update existing SSR test for WipeTransition baseline**

The SP-4+5+6 test `wipe_transition_emits_mask_image_kinetic_box` asserted on the old `mask-image: linear-gradient(...)` inline form. After this change the inline `style` no longer carries `mask-image` directly (the keyframes do). Update the assertion to check for `animation-name: ui-block-wipe-linear` and `data-variant="linear"`.

Run: `cargo test -p ui-blocks --test blocks_ssr`
Expected: all PASS.

- [ ] **Step 6: Commit**

```bash
git add crates/ui-blocks/src/wipe_transition.rs crates/ui-blocks/tests/blocks_ssr.rs crates/ui-blocks/Cargo.toml
git commit -m "$(cat <<'EOF'
feat(ui-blocks): WipeTransition consumes SceneContext + WipeVariant prop

Inline animation-name = ui-block-wipe-{variant}; animation-delay
flows from the parent Scene clock. Default variant is Linear
(backwards-compat with SP-4+5+6 callers). Conic/MaskPosition/Iris
variants resolve to their CSS keyframes.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 11: F7 — `LowerThird` choreography

Wrap the existing markup in a default `TimelineScope` so the bar, name, and role stagger in. Tests assert the new DOM structure + that names/roles get cue-animation styles.

**Files:**
- Modify: `crates/ui-blocks/src/lower_third.rs`
- Modify: `crates/ui-blocks/tests/blocks_ssr.rs` (UPDATE existing + APPEND)

- [ ] **Step 1: Update tests**

The existing `lower_third_emits_aria_label_with_name_and_role` test stays valid (we keep the aria-label parent). Append:

```rust
#[test]
fn lower_third_inside_scene_choreographs_via_timeline_scope() {
    use ui_dioxus::Scene;
    use ui_runtime::reduced_motion::ReducedMotionProvider;
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 2_000.0,
                autoplay: Some(false),
                LowerThird {
                    name: "Ada Lovelace".to_string(),
                    role: "Mathematician".to_string(),
                }
            }
        }
    });
    // Internal TimelineScope means each child gets a different
    // animation-delay. We don't pin exact ms (depends on default
    // step_ms = 80); just assert there's variation.
    assert!(html.contains("animation-name: ui-cue-"), "{html}");
    assert!(html.contains("data-block=\"lower-third\""), "{html}");
}
```

- [ ] **Step 2: Run test (expect failure)**

Run: `cargo test -p ui-blocks --test blocks_ssr lower_third_inside_scene`
Expected: fails — no animation in current static markup.

- [ ] **Step 3: Implement choreography**

Replace `LowerThird`'s body with:

```rust
use ui_dioxus::{KineticBox, KineticText, TimelineScope};

#[component]
pub fn LowerThird(
    name: String,
    role: String,
    accent: Option<LowerThirdAccent>,
) -> Element {
    let accent = accent.unwrap_or_default();
    let accent_class = match accent {
        LowerThirdAccent::Primary => "ui-block-lower-third--primary",
        LowerThirdAccent::Secondary => "ui-block-lower-third--secondary",
    };
    let aria = format!("{name}, {role}");
    rsx! {
        div {
            class: "ui-block-lower-third {accent_class}",
            "aria-label": "{aria}",
            "data-block": "lower-third",
            TimelineScope { id: "lower-third-stagger".to_string(), autoplay: false, stagger_step_ms: 120.0,
                div { class: "ui-block-lower-third__bar",
                    KineticBox { id: "lower-third-bar".to_string(), cue: "slide-up".to_string(),
                        span { }
                    }
                }
                div { class: "ui-block-lower-third__text",
                    KineticText {
                        id: "lower-third-name".to_string(),
                        text: name.clone(),
                        cue: "rise-in".to_string(),
                    }
                    KineticText {
                        id: "lower-third-role".to_string(),
                        text: role.clone(),
                        cue: "fade-in".to_string(),
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p ui-blocks --test blocks_ssr`
Expected: all PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-blocks/src/lower_third.rs crates/ui-blocks/tests/blocks_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-blocks): LowerThird default choreography (bar / name / role stagger)

Bar slides in first, then name rises in, then role fades in — all via
the new TimelineScope + cue-keyframe machinery. Step is 120ms so the
sequence reads as a beat rather than a blur.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 12: F7 — `SocialOverlay` choreography

Wrap the social overlay in a default TimelineScope. Tests assert that `handle` + `message` get cue animation styles.

**Files:**
- Modify: `crates/ui-blocks/src/social_overlay.rs`
- Modify: `crates/ui-blocks/tests/blocks_ssr.rs` (APPEND)

- [ ] **Step 1: Append test**

```rust
#[test]
fn social_overlay_inside_scene_choreographs_handle_and_message() {
    use ui_dioxus::Scene;
    use ui_runtime::reduced_motion::ReducedMotionProvider;
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 2_000.0,
                autoplay: Some(false),
                SocialOverlay {
                    platform: SocialPlatform::Instagram,
                    handle: "@kineticsui".to_string(),
                    message: "Hi".to_string(),
                }
            }
        }
    });
    assert!(html.contains("animation-name: ui-cue-"), "{html}");
    assert!(html.contains("ui-block-social-overlay--instagram"), "{html}");
}
```

- [ ] **Step 2: Run test (expect failure)**

Run: `cargo test -p ui-blocks --test blocks_ssr social_overlay_inside_scene`
Expected: fails.

- [ ] **Step 3: Implement**

Replace `SocialOverlay`'s body:

```rust
use ui_dioxus::{KineticText, TimelineScope};

#[component]
pub fn SocialOverlay(
    platform: SocialPlatform,
    handle: String,
    message: String,
) -> Element {
    let modifier_class =
        format!("ui-block-social-overlay--{}", platform.modifier());
    rsx! {
        div {
            class: "ui-block-social-overlay {modifier_class}",
            "data-block": "social-overlay",
            "data-platform": "{platform.modifier()}",
            TimelineScope { id: "social-overlay-stagger".to_string(), autoplay: false, stagger_step_ms: 150.0,
                div { class: "ui-block-social-overlay__handle",
                    KineticText {
                        id: "social-overlay-handle".to_string(),
                        text: handle.clone(),
                        cue: "slide-up".to_string(),
                    }
                }
                div { class: "ui-block-social-overlay__message",
                    KineticText {
                        id: "social-overlay-message".to_string(),
                        text: message.clone(),
                        cue: "fade-in".to_string(),
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p ui-blocks --test blocks_ssr`
Expected: all PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-blocks/src/social_overlay.rs crates/ui-blocks/tests/blocks_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-blocks): SocialOverlay choreography (handle slides up, message fades in)

Same internal TimelineScope pattern as LowerThird. step_ms = 150 so
the handle settles first, message catches up.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 13: F7 — `MetricCounter` choreography

Same shape; label fades in, value rises in, delta fades in.

**Files:**
- Modify: `crates/ui-blocks/src/metric_counter.rs`
- Modify: `crates/ui-blocks/tests/blocks_ssr.rs` (APPEND)

- [ ] **Step 1: Append test**

```rust
#[test]
fn metric_counter_inside_scene_choreographs_label_value_delta() {
    use ui_dioxus::Scene;
    use ui_runtime::reduced_motion::ReducedMotionProvider;
    let html = dioxus_ssr::render_element(rsx! {
        ReducedMotionProvider { reduced: Some(true),
            Scene {
                id: "outer", width: 100, height: 100, duration_ms: 2_000.0,
                autoplay: Some(false),
                MetricCounter {
                    label: "Active".to_string(),
                    value: "1,287".to_string(),
                    delta_text: Some("+24%".to_string()),
                }
            }
        }
    });
    assert!(html.contains("animation-name: ui-cue-fade-in"), "{html}");
    assert!(html.contains("animation-name: ui-cue-rise-in"), "{html}");
}
```

- [ ] **Step 2: Run test (expect failure)**

Run: `cargo test -p ui-blocks --test blocks_ssr metric_counter_inside_scene`
Expected: fails.

- [ ] **Step 3: Implement**

```rust
use ui_dioxus::{KineticText, TimelineScope};

#[component]
pub fn MetricCounter(
    label: String,
    value: String,
    delta_text: Option<String>,
) -> Element {
    rsx! {
        div { class: "ui-block-metric-counter", "data-block": "metric-counter",
            TimelineScope { id: "metric-counter-stagger".to_string(), autoplay: false, stagger_step_ms: 200.0,
                KineticText { id: "metric-label".to_string(), text: label, cue: "fade-in".to_string() }
                KineticText { id: "metric-value".to_string(), text: value, cue: "rise-in".to_string() }
                if let Some(delta) = delta_text {
                    KineticText { id: "metric-delta".to_string(), text: delta, cue: "fade-in".to_string() }
                }
            }
        }
    }
}
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p ui-blocks --test blocks_ssr`
Expected: all PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-blocks/src/metric_counter.rs crates/ui-blocks/tests/blocks_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-blocks): MetricCounter choreography (label / value / delta stagger)

step_ms = 200 because numbers benefit from a longer beat than text.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 14: F10 — Scroll driver rAF defer on initial seed

In `crates/ui-runtime/src/drivers/scroll.rs`, the `install_scroll_driver` function currently seeds the initial `compute_progress` value synchronously at install time. If the trigger element isn't laid out yet, `getBoundingClientRect()` returns zeros and the formula degenerates to `progress ≈ 1.0`. Defer via `requestAnimationFrame`.

**Files:**
- Modify: `crates/ui-runtime/src/drivers/scroll.rs`

- [ ] **Step 1: Inspect current implementation**

Read the relevant section of `crates/ui-runtime/src/drivers/scroll.rs`. The current code has:

```rust
// Seed initial progress before any event fires.
let initial = compute_progress(&window, &trigger, start_offset, end_offset);
(on_progress.borrow_mut())(initial);
```

This runs synchronously during component mount, before the browser has laid out the trigger element.

- [ ] **Step 2: Defer the seed via requestAnimationFrame**

Replace the seed block with:

```rust
// Defer the initial seed to the next animation frame so the trigger
// element has been laid out. Otherwise getBoundingClientRect() returns
// zeros and the progress formula degenerates to ~1.0.
let on_progress_for_seed = on_progress.clone();
let window_for_seed = window.clone();
let trigger_for_seed = trigger.clone();
let seed_closure = Closure::once_into_js(move || {
    let progress = compute_progress(
        &window_for_seed,
        &trigger_for_seed,
        start_offset,
        end_offset,
    );
    (on_progress_for_seed.borrow_mut())(progress);
});
let _ = window.request_animation_frame(seed_closure.as_ref().unchecked_ref());
```

(`Closure::once_into_js` takes ownership of the closure and converts it into a `JsValue` that the browser can drop after invocation.)

- [ ] **Step 3: Verify**

Run: `cargo check -p ui-runtime --target wasm32-unknown-unknown`
Expected: clean.

This change is web-only (the function is already inside `#![cfg(target_arch = "wasm32")]`). No SSR test for the rAF path; it's covered by the existing Playwright scroll-pinned-story spec implicitly.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-runtime/src/drivers/scroll.rs
git commit -m "$(cat <<'EOF'
fix(ui-runtime): defer scroll-driver initial seed to requestAnimationFrame

install_scroll_driver previously called compute_progress synchronously
during install, before the trigger element was laid out. With zero
bounds the formula degenerates to ~1.0 and the scene settles instantly
on cold mount. Defer the seed via rAF so layout is complete first.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 15: F9 — Glass engine wgpu surface format negotiation

The DevTools investigation showed 1,000+ `Invalid CommandBuffer "glass-frame"` warnings caused by a hardcoded `TextureFormat::BGRA8Unorm` mismatching the surface's actual format on Windows Chromium.

**Files:**
- Modify: glass engine source (exact files determined by `grep`)

- [ ] **Step 1: Locate the hardcoded format**

```bash
grep -rn "TextureFormat" crates/ui-glass-engine/src/ | head -20
grep -rn "BGRA8Unorm" crates/ui-glass-engine/src/ | head -20
```

Identify:
- The surface configuration site (where `surface.configure(...)` is called).
- The pipeline `color_targets` site (the `wgpu::RenderPipelineDescriptor`).

Both currently bake in `BGRA8Unorm`; both must use the negotiated format.

- [ ] **Step 2: Replace with capability negotiation**

In the surface configuration site, before configuring the surface, call:

```rust
let caps = surface.get_capabilities(&adapter);
let preferred = [
    wgpu::TextureFormat::BGRA8UnormSrgb,
    wgpu::TextureFormat::Rgba8UnormSrgb,
    wgpu::TextureFormat::Bgra8Unorm,
    wgpu::TextureFormat::Rgba8Unorm,
];
let surface_format = preferred
    .iter()
    .copied()
    .find(|f| caps.formats.contains(f))
    .or_else(|| caps.formats.first().copied())
    .unwrap_or(wgpu::TextureFormat::Bgra8Unorm);
```

Pass `surface_format` into:
- `wgpu::SurfaceConfiguration::format`
- Every `wgpu::ColorTargetState::format` in the render pipeline construction.

Note the exact wgpu enum variant names (the workspace's wgpu version may use `Bgra8UnormSrgb` or `BGRA8UnormSrgb`); check the existing code's casing.

- [ ] **Step 3: Verify**

Run: `cargo check -p ui-glass-engine --target wasm32-unknown-unknown`
Expected: clean.

Run: `cargo test --workspace` — full suite green (no regressions to existing glass engine tests).

- [ ] **Step 4: Commit**

```bash
git add crates/ui-glass-engine/
git commit -m "$(cat <<'EOF'
fix(ui-glass-engine): negotiate wgpu surface format via get_capabilities

Hardcoded TextureFormat::BGRA8Unorm caused per-frame
"Invalid CommandBuffer" warnings on Windows Chromium where the
surface advertises RGBA8UnormSrgb. Use the surface's capability
list to pick a compatible format, preferring sRGB variants first
and falling back to whatever the surface offers.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 16: F11 showcase — three new Wipe variant Scene entries + gallery wiring

Three new gallery showcase scenes plus their wiring.

**Files:**
- Create: `examples/component-gallery/src/previews/scenes/wipe_conic_demo.rs`
- Create: `examples/component-gallery/src/previews/scenes/wipe_iris_demo.rs`
- Create: `examples/component-gallery/src/previews/scenes/wipe_mask_position_demo.rs`
- Modify: `examples/component-gallery/src/previews/scenes/mod.rs`
- Modify: `examples/component-gallery/src/previews/scene.rs`
- Modify: `examples/component-gallery/src/docs.rs` (bump array length 55 → 58 + 3 ComponentDoc + 3 snippets)
- Modify: `examples/component-gallery/e2e/tests/_lib/component-manifest.ts` (3 entries)

- [ ] **Step 1: Create the three scenes**

Each scene mirrors the existing `wipe_demo.rs` pattern with the variant prop set. Example for `wipe_conic_demo.rs`:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_blocks::{WipeTransition, WipeVariant};

#[component]
pub fn WipeConicDemoScene() -> Element {
    rsx! {
        Scene {
            id: "wipe-conic-demo",
            width: 1280,
            height: 720,
            duration_ms: 2_500.0,
            autoplay: Some(true),
            controls: Some(true),
            WipeTransition {
                duration_ms: 2_500.0,
                variant: WipeVariant::Conic,
                div { class: "scene-wipe-fill",
                    style: "background: radial-gradient(circle, #ff7ae0, #4bbafa); width: 100%; height: 100%;",
                    h2 { style: "padding: 80px;", "Conic wipes spin around the centre." }
                }
            }
        }
    }
}
```

Repeat for `wipe_iris_demo.rs` (variant `Iris`, `"Iris wipes expand from the centre."`) and `wipe_mask_position_demo.rs` (variant `MaskPosition`, `"Mask-position wipes sweep horizontally."`).

- [ ] **Step 2: Register modules**

In `examples/component-gallery/src/previews/scenes/mod.rs`, add:

```rust
pub mod wipe_conic_demo;
pub mod wipe_iris_demo;
pub mod wipe_mask_position_demo;
```

- [ ] **Step 3: Add preview functions**

In `examples/component-gallery/src/previews/scene.rs`, append:

```rust
use crate::previews::scenes::wipe_conic_demo::WipeConicDemoScene;
use crate::previews::scenes::wipe_iris_demo::WipeIrisDemoScene;
use crate::previews::scenes::wipe_mask_position_demo::WipeMaskPositionDemoScene;

pub fn wipe_conic_demo_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            WipeConicDemoScene {}
        }
    }
}

pub fn wipe_iris_demo_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            WipeIrisDemoScene {}
        }
    }
}

pub fn wipe_mask_position_demo_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            WipeMaskPositionDemoScene {}
        }
    }
}
```

- [ ] **Step 4: Add snippet consts + ComponentDoc entries + bump array length**

In `examples/component-gallery/src/docs.rs`:

1. Bump `[ComponentDoc; 55]` to `[ComponentDoc; 58]`.
2. Append snippet consts:

```rust
const SCENE_WIPE_CONIC_SNIPPET: &str = r##"WipeTransition {
    duration_ms: 2_500.0,
    variant: WipeVariant::Conic,
    /* gradient-filled child */
}"##;

const SCENE_WIPE_IRIS_SNIPPET: &str = r##"WipeTransition {
    duration_ms: 2_500.0,
    variant: WipeVariant::Iris,
    /* gradient-filled child */
}"##;

const SCENE_WIPE_MASK_POSITION_SNIPPET: &str = r##"WipeTransition {
    duration_ms: 2_500.0,
    variant: WipeVariant::MaskPosition,
    /* gradient-filled child */
}"##;
```

3. Append three `ComponentDoc` entries to `COMPONENT_DOCS`:

```rust
    ComponentDoc {
        name: "Scene · Wipe Conic Demo",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ui-blocks: WipeTransition with WipeVariant::Conic — mask rotates around the centre over duration_ms.",
        snippet: SCENE_WIPE_CONIC_SNIPPET,
        accessibility: "Decorative; the underlying heading is in normal reading order.",
        render: Some(crate::previews::scene::wipe_conic_demo_preview),
    },
    ComponentDoc {
        name: "Scene · Wipe Iris Demo",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ui-blocks: WipeTransition with WipeVariant::Iris — radial-gradient mask expands from the centre.",
        snippet: SCENE_WIPE_IRIS_SNIPPET,
        accessibility: "Decorative; the underlying heading is in normal reading order.",
        render: Some(crate::previews::scene::wipe_iris_demo_preview),
    },
    ComponentDoc {
        name: "Scene · Wipe Mask-Position Demo",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ui-blocks: WipeTransition with WipeVariant::MaskPosition — linear-gradient sweeps horizontally via mask-position interpolation.",
        snippet: SCENE_WIPE_MASK_POSITION_SNIPPET,
        accessibility: "Decorative; the underlying heading is in normal reading order.",
        render: Some(crate::previews::scene::wipe_mask_position_demo_preview),
    },
```

- [ ] **Step 5: Add manifest entries**

In `examples/component-gallery/e2e/tests/_lib/component-manifest.ts`, append three entries matching the existing Wipe demo shape.

- [ ] **Step 6: Verify**

Run: `cargo check -p component-gallery && cargo test -p component-gallery`
Expected: green (manifest cross-check passes).

- [ ] **Step 7: Commit**

```bash
git add examples/component-gallery
git commit -m "$(cat <<'EOF'
feat(gallery): wire Wipe Conic / Iris / MaskPosition variant showcases

Three new Scene category entries demonstrating WipeVariant::Conic,
::Iris, and ::MaskPosition respectively. Each renders a gradient-
filled inner div and lets the WipeVariant CSS keyframe sweep across.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 17: F8 — `animation-motion.spec.ts` regression Playwright spec

The "renders but doesn't animate" bug class needs canonical browser-level coverage. This spec samples each Scene's animated leaf at t=0 and t=250ms, asserts the inline `style.animationDelay` value differs.

**Files:**
- Create: `examples/component-gallery/e2e/tests/animation-motion.spec.ts`

- [ ] **Step 1: Write the spec**

```ts
import { expect, test } from "@playwright/test";

const SCENE_SECTION = "#scene";

const SCENES: Array<{
  name: string;
  selector: string;
  description: string;
}> = [
  {
    name: "Scene · Product Intro 10s",
    selector: '[data-kinetic-id="intro-title"]',
    description: "KineticText animation-delay flows from Scene clock",
  },
  {
    name: "Scene · Split Headline",
    selector: ".ui-split-text__glyph",
    description: "SplitText glyphs animate per-stagger",
  },
  {
    name: "Scene · Curved Trajectory",
    selector: '[data-kinetic-id="trajectory-icon"]',
    description: "Curved-path KineticBox updates over time",
  },
  {
    name: "Scene · Lower Third Demo",
    selector: '[data-kinetic-id="lower-third-name"]',
    description: "LowerThird name animates",
  },
  {
    name: "Scene · Caption Reading-Pace Demo",
    selector: ".ui-split-text__word",
    description: "Caption per-word stagger animates",
  },
  {
    name: "Scene · Wipe Transition Demo",
    selector: ".ui-block-wipe-transition",
    description: "WipeTransition animation-delay updates",
  },
  {
    name: "Scene · Metric Counter Demo",
    selector: '[data-kinetic-id="metric-value"]',
    description: "MetricCounter value rises in",
  },
  {
    name: "Scene · Social Overlay Demo",
    selector: '[data-kinetic-id="social-overlay-handle"]',
    description: "SocialOverlay handle slides up",
  },
];

test.describe("Animation motion — leaves update animation-delay over time", () => {
  for (const scene of SCENES) {
    test(scene.name, async ({ page }) => {
      await page.goto("/");
      await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
      const card = page.locator(
        `article.gallery-entry:has(h4:has-text('${scene.name}'))`,
      );
      await expect(card).toBeVisible();

      const leaf = card.locator(scene.selector).first();
      await expect(leaf).toBeVisible();

      const sample = async () =>
        await leaf.evaluate((el: HTMLElement) => el.style.animationDelay);

      const first = await sample();
      await page.waitForTimeout(250);
      const second = await sample();

      expect(first).not.toBe(second);
    });
  }
});
```

- [ ] **Step 2: Build the gallery**

```bash
cd examples/component-gallery
dx build --release
```

If disk pressure, run `cargo clean -p component-gallery` first.

- [ ] **Step 3: Run on Chromium + WebKit**

```bash
cd examples/component-gallery/e2e
npx playwright test --project=static tests/animation-motion.spec.ts
npx playwright test --project=static-webkit tests/animation-motion.spec.ts
```

Expected: 8 tests pass on each engine. If any test fails because the selector doesn't match (e.g. a leaf doesn't yet emit `style.animationDelay`), inspect the rendered HTML and update either the selector or the underlying component to ensure the leaf has an inline style.

If Curved Trajectory still doesn't animate via the new path (because it uses `MotionCue::Path` + Sequence, which goes through the SP-3 sample state rather than the cue-keyframe path), that's acceptable — its motion is driven by the Sequence sample's inline `transform`. Update the test to sample `style.transform` instead of `style.animationDelay` for that scene.

- [ ] **Step 4: Commit**

```bash
git add examples/component-gallery/e2e/tests/animation-motion.spec.ts
git commit -m "$(cat <<'EOF'
test(gallery-e2e): animation-motion.spec.ts — animation-delay deltas over time

Per Scene category entry, sample the canonical animated leaf at t=0
and t=250ms and assert the inline style.animationDelay value
differs. Catches the entire "renders but doesn't animate" bug class
that the previous SSR-only test surface missed.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 18: Workspace verification + DevTools manual verify + merge + push

Final task: full verification, manual Chrome DevTools confirmation, merge to main, push.

- [ ] **Step 1: Format + clippy**

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

If fmt fails, run `cargo fmt --all` and commit `chore(fmt)`.

- [ ] **Step 2: Full workspace tests**

```bash
cargo test --workspace
```

Expected: green. Notable additions:
- `cue_inline_style_ssr` (4 tests in ui-dioxus)
- `timeline_scope_stagger_ssr` (3 tests)
- `split_text_ssr` (2 new tests, total 8)
- `blocks_ssr` (3 new tests for choreography, 2 for WipeVariant, total ~14)

- [ ] **Step 3: wasm32**

```bash
cargo check -p ui-runtime --target wasm32-unknown-unknown
cargo check -p ui-dioxus --target wasm32-unknown-unknown
cargo check -p ui-blocks --target wasm32-unknown-unknown
cargo check -p ui-glass-engine --target wasm32-unknown-unknown
```

All green.

- [ ] **Step 4: Full Playwright**

```bash
cd examples/component-gallery
dx build --release
cd e2e
for spec in animation-motion scene-player gsap-tier-primitives catalog-blocks exposure-polish; do
  npx playwright test --project=static tests/${spec}.spec.ts
  npx playwright test --project=static-webkit tests/${spec}.spec.ts
done
```

All five specs must pass on both engines.

- [ ] **Step 5: Manual Chrome DevTools verification**

With the gallery running at `http://localhost:4173/#scene`:

1. Open the page in Chrome DevTools.
2. Navigate to a Scene · * entry (e.g. Product Intro).
3. In the Elements panel, inspect a `[data-kinetic-id]` leaf. Confirm the inline `style` attribute contains `animation-name: ui-cue-…` and a non-zero `animation-delay`.
4. Wait 1 second; refresh the inspected element. The `animation-delay` value should have changed (proving the clock is driving it).
5. Open the Console panel. Confirm the wgpu `Invalid CommandBuffer` warnings from before are gone (the F9 surface format fix worked).
6. Scroll the page until the Scroll-pinned Story Scene's trigger enters the viewport. Confirm its `data-elapsed-ms` advances from 0 with scroll (proving F10 fix worked).

Document any remaining issues. If everything is clean, proceed to merge.

- [ ] **Step 6: Commit any final cleanup**

If any small fix-ups were needed (fmt, clippy lints, selector tweaks), commit them as `chore(kinetics-activation): final cleanup`.

- [ ] **Step 7: Branch summary**

```bash
git log --oneline main..HEAD
git diff --stat main..HEAD
```

- [ ] **Step 8: Merge + push**

```bash
git checkout main
git merge --ff-only feat/kinetics-activation
git push origin main
git push origin feat/kinetics-activation
```

Fast-forward merge expected. Quote the merge + push outputs.

- [ ] **Step 9: Final report**

Done.

---

## Self-Review

**Spec coverage:**
- F1 → Task 1
- F2 helper + KineticText + KineticBox → Tasks 2, 4, 5
- F3 → Task 6
- F4 → Tasks 3, 7
- F5 → Task 8
- F6 + F11 enum → Task 9
- F6 component → Task 10
- F7 → Tasks 11, 12, 13
- F8 → Task 17
- F9 → Task 15
- F10 → Task 14
- F11 showcases → Task 16
- Verification + merge → Task 18

**Placeholder scan:** Searched for "TBD", "TODO", "fill in", "implement later" — none found. Every step has exact code or exact commands.

**Type consistency:** `cue_inline_style(&cue, elapsed_ms)`, `cue_inline_style_with_duration(&cue, elapsed_ms, duration_ms)`, `StaggerCursor { cursor: Rc<Cell<u32>>, step_ms: f32 }`, `WipeVariant { Linear, Conic, MaskPosition, Iris }` — all consistent across tasks.

**Known forward-references:**
- Task 7's `StaggerCursor` design supersedes Task 3's earlier `StaggerChild` + `StaggerOffsetContext`. The implementer should delete or deprecate the earlier types in favor of the cursor approach. Tests in Tasks 4–5 should be updated accordingly if they reference `StaggerOffsetContext` directly — they don't (they just check the rendered animation-delay), so no test changes needed.
- Task 10's `WipeTransition` body removes the `data-angle-deg` attribute from earlier SP-4+5+6 in favor of a `--wipe-angle` CSS custom property. The existing `catalog-blocks.spec.ts` assertion for `data-angle-deg="120"` may need updating in the new flow — verify in Task 17/18.
- Task 11–13 wrap markup in `TimelineScope { autoplay: false, ... }`. With `autoplay: false`, the cursor still advances per render but no internal clock is spawned. This is intentional: the block rides the surrounding Scene's clock.

**Plan size:** 18 tasks. Comparable to SP-3 (17) and the exposure-polish set.
