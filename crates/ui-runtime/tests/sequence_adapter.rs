use ui_motion::{Ease, Transition};
use ui_runtime::adapters::SequenceAdapter;
use ui_runtime::frame_adapter::FrameAdapter;
use ui_timeline::{MotionCue, MotionSegment, MotionTarget, Timeline, TimelineTrack};

fn linear_tween(ms: u32) -> Transition {
    Transition::Tween {
        duration_ms: ms,
        ease: Ease::Linear,
    }
}

fn opacity_timeline() -> Timeline {
    Timeline::new("intro", 200.0).with_track(TimelineTrack::new(
        MotionTarget::node("title"),
        vec![MotionSegment::new(
            0.0,
            200.0,
            MotionCue::Opacity {
                from: 0.0,
                to: 1.0,
                transition: linear_tween(200),
            },
        )],
    ))
}

#[test]
fn sequence_adapter_id_and_duration_track_timeline() {
    let adapter = SequenceAdapter::new(opacity_timeline());
    assert_eq!(adapter.id(), "intro");
    assert!((adapter.duration_ms() - 200.0).abs() < f32::EPSILON);
}

#[test]
fn seek_writes_resolved_state_into_slot() {
    let adapter = SequenceAdapter::new(opacity_timeline());
    adapter.seek(0.0, false);
    {
        let snap = adapter.snapshot();
        assert_eq!(snap.len(), 1);
        let opacity = snap[0].opacity.expect("opacity at t=0");
        assert!(opacity.abs() < 1e-3, "got {opacity}");
    }
    adapter.seek(100.0, false);
    {
        let snap = adapter.snapshot();
        let opacity = snap[0].opacity.expect("opacity at t=100");
        assert!((opacity - 0.5).abs() < 1e-2, "got {opacity}");
    }
    adapter.seek(200.0, false);
    {
        let snap = adapter.snapshot();
        let opacity = snap[0].opacity.expect("opacity at t=200");
        assert!((opacity - 1.0).abs() < 1e-3, "got {opacity}");
    }
}

#[test]
fn reduced_seek_uses_reduced_motion_timeline() {
    let adapter = SequenceAdapter::new(opacity_timeline());
    adapter.seek(0.0, true);
    let snap = adapter.snapshot();
    // Reduced motion collapses duration to 0, so any elapsed_ms emits the
    // final settled state (opacity = 1.0).
    assert!((snap[0].opacity.unwrap_or(0.0) - 1.0).abs() < 1e-3);
}
