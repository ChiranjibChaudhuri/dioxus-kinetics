use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn Metrics() -> Element {
    rsx! {
        section { class: "flagship-metrics", aria_labelledby: "flagship-metrics-heading",
            div { class: "flagship-metrics-inner",
                p { class: "flagship-eyebrow", "Honest numbers" }
                h2 { id: "flagship-metrics-heading", class: "flagship-display-2",
                    "Built to ship."
                }
                div { class: "flagship-metrics-grid",
                    MetricCounter {
                        label: "Components ready".to_string(),
                        value: "34".to_string(),
                        delta_text: Some("from the public prelude".to_string()),
                    }
                    MetricCounter {
                        label: "Frame target".to_string(),
                        value: "60 fps".to_string(),
                        delta_text: Some("scene clock + frame scheduler".to_string()),
                    }
                    MetricCounter {
                        label: "Platform adapters".to_string(),
                        value: "4".to_string(),
                        delta_text: Some("Web · Desktop · Mobile · Native".to_string()),
                    }
                    MetricCounter {
                        label: "Glass engine".to_string(),
                        value: "WebGPU".to_string(),
                        delta_text: Some("SVG and solid fallbacks built in".to_string()),
                    }
                }
            }
        }
    }
}
