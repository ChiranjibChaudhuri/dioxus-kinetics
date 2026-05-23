//! Pins the contract that Timeline.sample at elapsed_ms=0 with a single
//! cue starting at start_ms=0 returns the cue's `from` value (not `to`).
//! This was the Sequence test 1 audit signal: opacity at t=0 was NOT
//! <= 0.1 although the cue declared `from: 0.0`.

use ui_motion::{Ease, Transition};
use ui_timeline::{
    FillMode, MotionCue, MotionSegment, MotionTarget, Timeline, TimelineClock, TimelineTrack,
};

fn opacity_cue() -> MotionCue {
    MotionCue::Opacity {
        from: 0.0,
        to: 1.0,
        transition: Transition::Tween {
            duration_ms: 220,
            ease: Ease::Standard,
        },
    }
}

#[test]
fn sample_at_t_zero_returns_from_value_for_first_cue() {
    let track = TimelineTrack::new(
        MotionTarget::node("title"),
        vec![MotionSegment::new(0.0, 220.0, opacity_cue())],
    );
    // Timeline duration is set to cover the cue; fill=Both so before/after phases
    // also emit values (mirrors the gallery's FillMode::Both usage).
    let timeline = Timeline::new("t", 220.0)
        .with_track(track)
        .with_fill(FillMode::Both);

    let sample = timeline.sample(TimelineClock::Manual { elapsed_ms: 0.0 });
    let state = sample
        .states
        .iter()
        .find(|s| matches!(&s.target, MotionTarget::Node(id) if id.0 == "title"))
        .expect("title state must be present in sample");

    let style = state.inline_style();
    // At t=0 with from=0.0, the interpolated opacity must be 0.0, not 1.0.
    // The style format is "opacity: {value}" (space after colon).
    assert!(
        !style.contains("opacity: 1") && !style.contains("opacity:1"),
        "expected from value (opacity: 0...) at t=0; got {style}"
    );
    // Also assert positively that opacity is close to 0.
    assert!(
        state.opacity.map(|v| v <= 0.1).unwrap_or(false),
        "expected opacity <= 0.1 at t=0; got {:?}",
        state.opacity
    );
}
