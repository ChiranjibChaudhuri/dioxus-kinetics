use dioxus::prelude::*;
use ui_dioxus::SceneContext;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum WipeVariant {
    #[default]
    Linear,
    Conic,
    MaskPosition,
    Iris,
}

impl WipeVariant {
    fn css_keyword(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Conic => "conic",
            Self::MaskPosition => "mask-position",
            Self::Iris => "iris",
        }
    }
}

/// Full-coverage wipe transition. The mask sweeps across the child
/// region over `duration_ms`. Direction controlled by `angle_deg`
/// (default 90.0 = left-to-right). When mounted inside a `Scene`, the
/// surrounding `SceneContext.clock.elapsed_ms` drives the
/// `animation-delay = -elapsed_ms` deterministic seek; otherwise the
/// keyframe sits at its initial state with `animation-delay: -0ms`.
#[component]
pub fn WipeTransition(
    duration_ms: f32,
    angle_deg: Option<f32>,
    variant: Option<WipeVariant>,
    children: Element,
) -> Element {
    let variant = variant.unwrap_or_default();
    let angle = angle_deg.unwrap_or(90.0);

    let scene = try_consume_context::<SceneContext>();
    let elapsed_ms = scene.map(|s| *s.clock.elapsed_ms.read()).unwrap_or(0.0);

    let elapsed_attr = if elapsed_ms.is_finite() && elapsed_ms > 0.0 {
        elapsed_ms.round() as i64
    } else {
        0
    };
    let duration_attr = if duration_ms.is_finite() && duration_ms > 0.0 {
        duration_ms.round() as i64
    } else {
        1
    };

    let kind = variant.css_keyword();
    let inline_style = format!(
        "animation-name: ui-block-wipe-{kind}; \
         animation-duration: {duration_attr}ms; \
         animation-fill-mode: forwards; \
         animation-play-state: paused; \
         animation-delay: -{elapsed_attr}ms; \
         --wipe-angle: {angle}deg;",
    );

    rsx! {
        div {
            class: "ui-block-wipe-transition",
            "data-block": "wipe-transition",
            "data-duration-ms": "{duration_attr}",
            "data-angle-deg": "{angle}",
            "data-variant": "{kind}",
            style: "{inline_style}",
            {children}
        }
    }
}
