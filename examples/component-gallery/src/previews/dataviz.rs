use dioxus::prelude::*;
use kinetics::prelude::*;

// ---------------------------------------------------------------------------
// Sparkline
// ---------------------------------------------------------------------------

pub fn sparkline_preview() -> Element {
    let trend = vec![4.0, 6.0, 5.0, 9.0, 7.0, 12.0, 11.0, 14.0];
    let dip = vec![14.0, 11.0, 12.0, 8.0, 9.0, 5.0, 6.0, 4.0];

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Labelled · success tone, filled" }
                Sparkline {
                    points: trend.clone(),
                    label: "Weekly active users trending up".to_string(),
                    tone: ChartTone::Success,
                    filled: true,
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Decorative · danger tone, line only" }
                Sparkline { points: dip, tone: ChartTone::Danger }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// LineChart
// ---------------------------------------------------------------------------

pub fn line_chart_preview() -> Element {
    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Two series · grid, legend, area" }
            }
            LineChart {
                label: "Monthly revenue versus forecast, January through June".to_string(),
                series: vec![
                    ChartSeries::new("Revenue", vec![42.0, 55.0, 49.0, 71.0, 68.0, 90.0]),
                    ChartSeries::new("Forecast", vec![40.0, 50.0, 56.0, 63.0, 72.0, 82.0]),
                ],
                x_labels: vec![
                    "Jan".to_string(),
                    "Feb".to_string(),
                    "Mar".to_string(),
                    "Apr".to_string(),
                    "May".to_string(),
                    "Jun".to_string(),
                ],
                show_area: true,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// BarChart
// ---------------------------------------------------------------------------

pub fn bar_chart_preview() -> Element {
    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Grouped bars · staggered rise-in" }
            }
            BarChart {
                label: "Quarterly seats by plan tier".to_string(),
                series: vec![
                    ChartSeries::new("Pro", vec![32.0, 41.0, 54.0, 61.0]),
                    ChartSeries::new("Enterprise", vec![18.0, 25.0, 33.0, 47.0]),
                ],
                x_labels: vec![
                    "Q1".to_string(),
                    "Q2".to_string(),
                    "Q3".to_string(),
                    "Q4".to_string(),
                ],
            }
        }
    }
}

// ---------------------------------------------------------------------------
// DonutGauge
// ---------------------------------------------------------------------------

pub fn donut_gauge_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Primary · default label" }
                DonutGauge {
                    label: "Storage used".to_string(),
                    value: 0.72,
                    description: "of 2 TB".to_string(),
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Success · custom display value" }
                DonutGauge {
                    label: "Uptime this quarter".to_string(),
                    value: 0.999,
                    display_value: "99.9%".to_string(),
                    description: "uptime".to_string(),
                    tone: ChartTone::Success,
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Warning · pinned at 40% sweep" }
                DonutGauge {
                    label: "Quota consumed".to_string(),
                    value: 0.4,
                    tone: ChartTone::Warning,
                    progress: 1.0,
                }
            }
        }
    }
}
