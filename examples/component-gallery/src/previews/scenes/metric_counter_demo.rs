use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_blocks::MetricCounter;
use ui_composition::ClipFill;

#[component]
pub fn MetricCounterDemoScene() -> Element {
    rsx! {
        Scene {
            id: "metric-counter-demo",
            width: 1280,
            height: 480,
            duration_ms: 4_000.0,
            autoplay: Some(true),
            controls: Some(true),
            TimelineScope { id: "metric-counter-timeline", autoplay: true,
                Clip { start_ms: 200.0, duration_ms: 3_500.0, fill: ClipFill::HoldEnd,
                    MetricCounter {
                        label: "Active users".to_string(),
                        value: "1,287".to_string(),
                        delta_text: Some("+24% week over week".to_string()),
                    }
                }
            }
        }
    }
}
