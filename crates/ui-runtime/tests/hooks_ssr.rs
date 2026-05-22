use dioxus::prelude::*;
use ui_motion::{Ease, Transition};
use ui_runtime::{use_animation_value, use_presence_state, use_timeline_sample, ReducedMotion};
use ui_timeline::{MotionCue, MotionSegment, MotionTarget, Timeline, TimelineClock, TimelineTrack};

#[component]
fn AnimationProbe(target: f32, transition: Transition) -> Element {
    let value = use_animation_value(target, transition);
    let rendered = value();
    rsx! {
        div { "data-value": "{rendered}" }
    }
}

#[component]
fn ContextProvider(value: ReducedMotion, children: Element) -> Element {
    use_context_provider(|| value);
    rsx! { {children} }
}

#[test]
fn animation_value_in_ssr_returns_target_synchronously() {
    let transition = Transition::Tween {
        duration_ms: 220,
        ease: Ease::Standard,
    };
    let html = dioxus_ssr::render_element(rsx! {
        AnimationProbe { target: 1.0, transition: transition }
    });
    assert!(html.contains("data-value=\"1\""), "got {html}");
}

#[test]
fn animation_value_with_reduced_motion_returns_target() {
    let transition = Transition::Tween {
        duration_ms: 220,
        ease: Ease::Standard,
    };
    let html = dioxus_ssr::render_element(rsx! {
        ContextProvider {
            value: ReducedMotion(true),
            AnimationProbe { target: 1.0, transition: transition }
        }
    });
    assert!(html.contains("data-value=\"1\""), "got {html}");
}

#[component]
fn PresenceProbe(present: bool) -> Element {
    let state = use_presence_state(
        present,
        Transition::Tween {
            duration_ms: 220,
            ease: Ease::Standard,
        },
        Transition::Tween {
            duration_ms: 180,
            ease: Ease::Standard,
        },
    );
    rsx! {
        div { "data-state": "{state().as_str()}" }
    }
}

#[test]
fn presence_state_initial_present_true_is_visible_in_ssr() {
    let html = dioxus_ssr::render_element(rsx! { PresenceProbe { present: true } });
    assert!(html.contains("data-state=\"visible\""), "got {html}",);
}

#[test]
fn presence_state_initial_present_false_is_unmounted_in_ssr() {
    let html = dioxus_ssr::render_element(rsx! { PresenceProbe { present: false } });
    assert!(html.contains("data-state=\"unmounted\""), "got {html}",);
}

#[component]
fn TimelineSampleProbe(timeline: Timeline, clock: TimelineClock) -> Element {
    let sample = use_timeline_sample(timeline, clock);
    let opacity = sample().states.first().and_then(|s| s.opacity).unwrap_or(-1.0);
    rsx! {
        div { "data-opacity": "{opacity}" }
    }
}

#[test]
fn use_timeline_sample_in_ssr_returns_initial_sample() {
    let cue = MotionCue::Opacity {
        from: 0.0,
        to: 1.0,
        transition: Transition::Tween {
            duration_ms: 220,
            ease: Ease::Linear,
        },
    };
    let timeline = Timeline::new("t", 220.0).with_track(TimelineTrack::new(
        MotionTarget::node("hero"),
        vec![MotionSegment::new(0.0, 220.0, cue)],
    ));
    let html = dioxus_ssr::render_element(rsx! {
        TimelineSampleProbe {
            timeline: timeline,
            clock: TimelineClock::Manual { elapsed_ms: 110.0 }
        }
    });
    assert!(html.contains("data-opacity=\"0.5\""), "got {html}");
}
