use dioxus::prelude::*;
use ui_dioxus::{KineticBox, KineticText, TimelineScope};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LowerThirdAccent {
    #[default]
    Primary,
    Secondary,
}

/// Broadcast-style chyron with name + role. Sits at the bottom-left
/// of the parent container; consumer is responsible for positioning
/// (we apply no absolute positioning).
#[component]
pub fn LowerThird(name: String, role: String, accent: Option<LowerThirdAccent>) -> Element {
    let accent = accent.unwrap_or_default();
    let accent_class = match accent {
        LowerThirdAccent::Primary => "ui-block-lower-third--primary",
        LowerThirdAccent::Secondary => "ui-block-lower-third--secondary",
    };
    let aria = format!("{name}, {role}");
    rsx! {
        div {
            class: "ui-block-lower-third {accent_class}",
            "aria-label": "{aria}",
            "data-block": "lower-third",
            TimelineScope {
                id: "lower-third-stagger".to_string(),
                autoplay: false,
                stagger_step_ms: 120.0,
                div { class: "ui-block-lower-third__bar",
                    KineticBox {
                        id: "lower-third-bar".to_string(),
                        cue: "slide-up".to_string(),
                        span { }
                    }
                }
                div { class: "ui-block-lower-third__text",
                    KineticText {
                        id: "lower-third-name".to_string(),
                        text: name.clone(),
                        cue: "rise-in".to_string(),
                    }
                    KineticText {
                        id: "lower-third-role".to_string(),
                        text: role.clone(),
                        cue: "fade-in".to_string(),
                    }
                }
            }
        }
    }
}
