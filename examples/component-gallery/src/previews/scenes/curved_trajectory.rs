use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_motion::{Ease, Transition};
use ui_timeline::{MotionSegment, MotionTarget, Timeline, TimelineTrack};

#[component]
pub fn CurvedTrajectoryScene() -> Element {
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
        rotate_along_path: true,
        transition: Transition::Tween {
            duration_ms: 4_000,
            ease: Ease::Standard,
        },
    };
    let timeline =
        Timeline::new("curved-trajectory-timeline", 4_000.0).with_track(TimelineTrack::new(
            MotionTarget::node("trajectory-icon"),
            vec![MotionSegment::new(0.0, 4_000.0, cue)],
        ));

    rsx! {
        Scene {
            id: "curved-trajectory",
            width: 720,
            height: 480,
            duration_ms: 4_000.0,
            autoplay: Some(true),
            controls: Some(true),
            Sequence {
                timeline: Some(timeline),
                clock: TimelineClock::Manual { elapsed_ms: 0.0 },
                MotionPath {
                    id: "trajectory-icon".to_string(),
                    path: pts,
                    duration_ms: 4_000.0,
                    KineticBox { id: "trajectory-icon", "•" }
                }
            }
        }
    }
}
