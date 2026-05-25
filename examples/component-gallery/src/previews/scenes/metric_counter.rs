use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn MetricCounterScene() -> Element {
    rsx! {
        div { class: "scene-metric",
            KineticText { id: "metric-headline", text: "Active builds".to_string(), cue: "fade-in" }
            KineticText { id: "metric-value", text: "1,287".to_string(), cue: "rise-in" }
            KineticText {
                id: "metric-delta",
                text: "+24% week over week".to_string(),
                cue: "fade-in",
            }
        }
    }
}
