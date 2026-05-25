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
