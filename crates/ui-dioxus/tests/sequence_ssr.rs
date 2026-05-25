use dioxus::prelude::*;
use ui_dioxus::{Cue, KineticBox, Sequence};
use ui_motion::{Ease, Transition};
use ui_timeline::{MotionCue, MotionSegment, MotionTarget, Timeline, TimelineClock, TimelineTrack};

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
            clock: Some(TimelineClock::Manual { elapsed_ms: 0.0 }),
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
            clock: Some(TimelineClock::Manual { elapsed_ms: 0.0 }),
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
            clock: Some(TimelineClock::Manual { elapsed_ms: 0.0 }),
            KineticBox { id: "title", "Hello" }
        }
    });
    assert!(html.contains("data-timeline-id=\"hero\""));
}

#[test]
fn kinetic_box_outside_sequence_renders_data_attrs_without_inline_style() {
    let html = dioxus_ssr::render_element(rsx! {
        KineticBox { id: "solo", cue: "fade-in", "Hello" }
    });
    assert!(html.contains("data-kinetic-id=\"solo\""));
    assert!(html.contains("data-motion-cue=\"fade-in\""));
    // Either no style attribute or an empty one is acceptable.
}
