use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use ui_layout::{compute_flip, FlipDelta};
use ui_motion::{apply_ease, Ease, Transition};
use ui_runtime::{
    spawn_frame_loop, use_element_computed_style, use_element_rect, use_shared_element_registry,
    ControlFlow, ElementSnapshot, FrameHandle, SharedElementRegistry, SharedTransition,
};

const TRACKED_PROPERTIES: &[&str] = &["border-radius", "background-color", "color", "opacity"];
const FLIP_TRANSLATE_EPS: f32 = 0.5;
const FLIP_SCALE_EPS: f32 = 0.001;

#[component]
pub fn SharedLayout(children: Element) -> Element {
    use_context_provider(|| Signal::new(Rc::new(SharedElementRegistry::default())));

    rsx! {
        div {
            class: "ui-shared-layout",
            {children}
        }
    }
}

#[component]
pub fn SharedElement(
    id: String,
    #[props(default)] transition: SharedTransition,
    children: Element,
) -> Element {
    let (rect_callback, rect) = use_element_rect();
    let (style_callback, computed) = use_element_computed_style(TRACKED_PROPERTIES);
    let registry = use_shared_element_registry();

    let mut style_signal = use_signal(String::new);
    let flip_runtime = use_hook(|| {
        Rc::new(RefCell::new(FlipRuntime {
            handle: None,
            elapsed_ms: 0.0,
            duration_ms: 0.0,
            delta: None,
        }))
    });

    let id_for_effect = id.clone();
    let transition_layout = transition.layout;

    use_effect(move || {
        let Some(current_rect) = rect() else {
            return;
        };
        let reg = registry.read().clone();
        let prev_snapshot = reg.snapshot(&id_for_effect);
        let computed_snapshot = computed.read().clone();

        // Always record the latest snapshot so future remounts can FLIP from
        // this position.
        reg.record(
            id_for_effect.clone(),
            ElementSnapshot {
                rect: current_rect,
                computed: computed_snapshot,
                timestamp_ms: 0.0,
            },
        );

        let Some(prev) = prev_snapshot else {
            return;
        };
        let delta = compute_flip(prev.rect, current_rect);
        if delta_is_identity(&delta) {
            style_signal.set(String::new());
            return;
        }

        let duration_ms = transition_layout.estimated_duration_ms().max(1.0);
        let ease = match transition_layout {
            Transition::Tween { ease, .. } => ease,
            Transition::Spring(_) => Ease::Standard,
        };

        // Apply the inverted transform synchronously so the element appears at
        // its previous position before the animation runs.
        style_signal.set(format_flip_style(&delta, 0.0));

        {
            let mut runtime = flip_runtime.borrow_mut();
            runtime.handle = None; // cancel any in-flight animation
            runtime.delta = Some(delta);
            runtime.elapsed_ms = 0.0;
            runtime.duration_ms = duration_ms;
        }

        let flip_runtime_loop = flip_runtime.clone();
        let mut style_signal_loop = style_signal;
        let new_handle = spawn_frame_loop(move |dt_ms| {
            let mut runtime = flip_runtime_loop.borrow_mut();
            let Some(delta_now) = runtime.delta else {
                return ControlFlow::Stop;
            };
            runtime.elapsed_ms += dt_ms as f32;
            let raw = (runtime.elapsed_ms / runtime.duration_ms).clamp(0.0, 1.0);
            let eased = apply_ease(raw, ease);

            if raw >= 1.0 {
                style_signal_loop.set(String::new());
                runtime.delta = None;
                runtime.handle = None;
                return ControlFlow::Stop;
            }

            style_signal_loop.set(format_flip_style(&delta_now, eased));
            ControlFlow::Continue
        });

        flip_runtime.borrow_mut().handle = Some(new_handle);
    });

    let style = style_signal();
    let id_attr = id.clone();
    rsx! {
        div {
            class: "ui-shared-element",
            "data-shared-id": "{id_attr}",
            style: "{style}",
            onmounted: move |evt| {
                rect_callback.0.call(evt.clone());
                style_callback.0.call(evt);
            },
            {children}
        }
    }
}

struct FlipRuntime {
    #[allow(dead_code)]
    handle: Option<FrameHandle>,
    elapsed_ms: f32,
    duration_ms: f32,
    delta: Option<FlipDelta>,
}

fn delta_is_identity(delta: &FlipDelta) -> bool {
    delta.translate_x.abs() < FLIP_TRANSLATE_EPS
        && delta.translate_y.abs() < FLIP_TRANSLATE_EPS
        && (delta.scale_x - 1.0).abs() < FLIP_SCALE_EPS
        && (delta.scale_y - 1.0).abs() < FLIP_SCALE_EPS
}

/// Returns the inline style for a FLIP animation at progress `t` (0.0 = fully
/// inverted to the previous position, 1.0 = identity).
fn format_flip_style(delta: &FlipDelta, t: f32) -> String {
    let inv_t = 1.0 - t;
    let tx = strip_neg_zero(delta.translate_x * inv_t);
    let ty = strip_neg_zero(delta.translate_y * inv_t);
    let sx = strip_neg_zero(delta.scale_x + (1.0 - delta.scale_x) * t);
    let sy = strip_neg_zero(delta.scale_y + (1.0 - delta.scale_y) * t);
    format!(
        "transform: translate({tx:.2}px, {ty:.2}px) scale({sx:.4}, {sy:.4}); transform-origin: top left; will-change: transform;"
    )
}

fn strip_neg_zero(value: f32) -> f32 {
    if value == 0.0 {
        0.0
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_layout::Rect;

    #[test]
    fn flip_style_at_zero_is_inverted_to_prev_rect() {
        let delta = compute_flip(
            Rect::new(0.0, 0.0, 100.0, 100.0),
            Rect::new(50.0, 80.0, 200.0, 200.0),
        );
        let style = format_flip_style(&delta, 0.0);
        assert!(style.contains("translate(-50.00px, -80.00px)"), "{style}");
        assert!(style.contains("scale(0.5000, 0.5000)"), "{style}");
    }

    #[test]
    fn flip_style_at_one_is_identity() {
        let delta = compute_flip(
            Rect::new(0.0, 0.0, 100.0, 100.0),
            Rect::new(50.0, 80.0, 200.0, 200.0),
        );
        let style = format_flip_style(&delta, 1.0);
        assert!(style.contains("translate(0.00px, 0.00px)"), "{style}");
        assert!(style.contains("scale(1.0000, 1.0000)"), "{style}");
    }

    #[test]
    fn identity_delta_is_detected() {
        let delta = compute_flip(
            Rect::new(10.0, 10.0, 100.0, 100.0),
            Rect::new(10.0, 10.0, 100.0, 100.0),
        );
        assert!(delta_is_identity(&delta));
    }

    #[test]
    fn non_identity_delta_is_not_collapsed() {
        let delta = compute_flip(
            Rect::new(10.0, 10.0, 100.0, 100.0),
            Rect::new(40.0, 10.0, 100.0, 100.0),
        );
        assert!(!delta_is_identity(&delta));
    }
}
