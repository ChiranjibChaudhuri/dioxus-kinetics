use dioxus::prelude::*;
use ui_dioxus::{KineticText, TimelineScope};

#[component]
pub fn MetricCounter(label: String, value: String, delta_text: Option<String>) -> Element {
    rsx! {
        div { class: "ui-block-metric-counter", "data-block": "metric-counter",
            TimelineScope {
                id: "metric-counter-stagger".to_string(),
                autoplay: false,
                stagger_step_ms: 200.0,
                KineticText { id: "metric-label".to_string(), text: label, cue: "fade-in".to_string() }
                KineticText { id: "metric-value".to_string(), text: value, cue: "rise-in".to_string() }
                if let Some(delta) = delta_text {
                    KineticText { id: "metric-delta".to_string(), text: delta, cue: "fade-in".to_string() }
                }
            }
        }
    }
}
