use dioxus::prelude::*;
use ui_motion::{Ease, Transition};
use ui_runtime::{use_animation_value, ReducedMotion};

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
