use dioxus::prelude::*;

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
/// (default 90.0 = left-to-right).
#[component]
pub fn WipeTransition(duration_ms: f32, angle_deg: Option<f32>, children: Element) -> Element {
    let angle = angle_deg.unwrap_or(90.0);
    let inline_style = format!(
        "mask-image: linear-gradient({angle}deg, black, transparent); \
         -webkit-mask-image: linear-gradient({angle}deg, black, transparent); \
         animation: ui-block-wipe-transition {duration_ms}ms forwards paused;"
    );
    rsx! {
        div {
            class: "ui-block-wipe-transition",
            "data-block": "wipe-transition",
            "data-duration-ms": "{duration_ms}",
            "data-angle-deg": "{angle}",
            style: "{inline_style}",
            {children}
        }
    }
}
