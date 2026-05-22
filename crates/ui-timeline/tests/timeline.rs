use ui_motion::{Ease, Transition};
use ui_timeline::{
    Axis, FillMode, MotionCue, MotionSegment, MotionTarget, RepeatMode, StaggerFlow, Timeline,
    TimelineClock, TimelineLabel, TimelineTrack,
};

#[test]
fn timeline_resolves_labels_and_samples_track_values() {
    let timeline = Timeline::new("panel", 500.0)
        .with_label(TimelineLabel::new("enter", 100.0))
        .with_track(TimelineTrack::new(
            MotionTarget::node("panel-card"),
            vec![MotionSegment::new(
                100.0,
                300.0,
                MotionCue::opacity(0.0, 1.0, Transition::tween(300)),
            )],
        ));

    assert_eq!(timeline.label_offset("enter"), Some(100.0));

    let sample = timeline.sample(TimelineClock::Playback { elapsed_ms: 250.0 });
    let value = sample
        .states
        .iter()
        .find(|state| state.target == MotionTarget::node("panel-card"))
        .expect("target state exists")
        .opacity
        .unwrap();

    assert!(value > 0.0);
    assert!(value < 1.0);
}

#[test]
fn stagger_flow_produces_deterministic_offsets() {
    let offsets = StaggerFlow::ByIndex { step_ms: 24.0 }.offsets(4);

    assert_eq!(offsets, vec![0.0, 24.0, 48.0, 72.0]);
}

#[test]
fn reduced_motion_collapses_timeline_segments() {
    let timeline = Timeline::new("notice", 180.0).with_track(TimelineTrack::new(
        MotionTarget::self_node(),
        vec![MotionSegment::new(
            0.0,
            180.0,
            MotionCue::opacity(0.0, 1.0, Transition::tween(180)),
        )],
    ));

    let reduced = timeline.reduced_motion();
    let sample = reduced.sample(TimelineClock::Playback { elapsed_ms: 0.0 });

    assert_eq!(reduced.duration_ms, 0.0);
    assert_eq!(sample.states[0].opacity, Some(1.0));
}

#[test]
fn repeat_yoyo_maps_clock_into_reverse_progress() {
    let timeline = Timeline::new("pulse", 100.0)
        .with_repeat(RepeatMode::Count {
            count: 2,
            yoyo: true,
        })
        .with_fill(FillMode::Both)
        .with_track(TimelineTrack::new(
            MotionTarget::self_node(),
            vec![MotionSegment::new(
                0.0,
                100.0,
                MotionCue::opacity(
                    0.0,
                    1.0,
                    Transition::Tween {
                        duration_ms: 100,
                        ease: Ease::Linear,
                    },
                ),
            )],
        ));

    let sample = timeline.sample(TimelineClock::Playback { elapsed_ms: 150.0 });

    assert!(sample.states[0].opacity.unwrap() < 1.0);
    assert!(sample.states[0].opacity.unwrap() > 0.0);
}

#[test]
fn fill_none_drops_state_after_timeline_end() {
    let timeline = Timeline::new("fade", 100.0)
        .with_fill(FillMode::None)
        .with_track(TimelineTrack::new(
            MotionTarget::self_node(),
            vec![MotionSegment::new(
                0.0,
                100.0,
                MotionCue::opacity(
                    0.0,
                    1.0,
                    Transition::Tween {
                        duration_ms: 100,
                        ease: Ease::Linear,
                    },
                ),
            )],
        ));

    let sample = timeline.sample(TimelineClock::Playback { elapsed_ms: 101.0 });

    assert!(sample.states.is_empty());
}

#[test]
fn backwards_fill_does_not_fill_after_timeline_end() {
    let timeline = Timeline::new("fade", 100.0)
        .with_fill(FillMode::Backwards)
        .with_track(TimelineTrack::new(
            MotionTarget::self_node(),
            vec![MotionSegment::new(
                20.0,
                80.0,
                MotionCue::opacity(
                    0.0,
                    1.0,
                    Transition::Tween {
                        duration_ms: 80,
                        ease: Ease::Linear,
                    },
                ),
            )],
        ));

    let before = timeline.sample(TimelineClock::Playback { elapsed_ms: 0.0 });
    let after = timeline.sample(TimelineClock::Playback { elapsed_ms: 101.0 });

    assert_eq!(before.states[0].opacity, Some(0.0));
    assert!(after.states.is_empty());
}

#[test]
fn repeat_count_without_forwards_fill_drops_after_exhaustion() {
    let timeline = Timeline::new("pulse", 100.0)
        .with_repeat(RepeatMode::Count {
            count: 2,
            yoyo: false,
        })
        .with_fill(FillMode::None)
        .with_track(TimelineTrack::new(
            MotionTarget::self_node(),
            vec![MotionSegment::new(
                0.0,
                100.0,
                MotionCue::opacity(
                    0.0,
                    1.0,
                    Transition::Tween {
                        duration_ms: 100,
                        ease: Ease::Linear,
                    },
                ),
            )],
        ));

    let boundary = timeline.sample(TimelineClock::Playback { elapsed_ms: 100.0 });
    let exhausted = timeline.sample(TimelineClock::Playback { elapsed_ms: 201.0 });

    assert_eq!(boundary.states[0].opacity, Some(0.0));
    assert!(exhausted.states.is_empty());
}

// Task 1 tests – new MotionCue variants

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
    assert!((cue.sample(0.5).scale.unwrap() - 1.1).abs() < 0.001);
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
