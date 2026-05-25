use dioxus::prelude::*;
use ui_dioxus::KineticText;

#[component]
pub fn MetricCounter(label: String, value: String, delta_text: Option<String>) -> Element {
    rsx! {
        div { class: "ui-block-metric-counter", "data-block": "metric-counter",
            KineticText { id: "metric-label", text: label, cue: "fade-in" }
            KineticText { id: "metric-value", text: value, cue: "rise-in" }
            if let Some(delta) = delta_text {
                KineticText { id: "metric-delta", text: delta, cue: "fade-in" }
            }
        }
    }
}
