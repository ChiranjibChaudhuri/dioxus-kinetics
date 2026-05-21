use ui_motion::{Ease, Transition};
use ui_timeline::{
    FillMode, MotionCue, MotionSegment, MotionTarget, RepeatMode, StaggerFlow, Timeline,
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
        .opacity;

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
    assert_eq!(sample.states[0].opacity, 1.0);
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

    assert!(sample.states[0].opacity < 1.0);
    assert!(sample.states[0].opacity > 0.0);
}
