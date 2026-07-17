//! LinkedIn showreel — a live, autoplaying showcase of the workspace's most
//! advanced renderable feature: the cinematic Scene system (kinetic text +
//! animated charts + liquid-glass metric card).
//!
//! Run it:
//!   dx serve --package showreel --port 9175
//!
//! Render the same composition to an MP4 (no browser needed):
//!   cargo run -p kinetics-cli -- render --scene showreel \
//!     --out ./out --frames 150 --fps 30 --capture-png --encode-mp4

use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_styles::library_css;

#[component]
pub fn App() -> Element {
    let css = library_css();
    rsx! {
        style { "{css}" }
        div {
            "data-ui-theme": "light",
            "data-ui-density": "comfortable",
            Showreel {}
        }
    }
}

#[component]
fn Showreel() -> Element {
    rsx! {
        div {
            style: "min-height:100vh; padding:64px 48px; box-sizing:border-box; \
                    background:linear-gradient(135deg, #f6f8fb 0%, #e8eef6 60%, #f2f5f9 100%); \
                    font-family:Inter, ui-sans-serif, system-ui, sans-serif; color:#111827;",
            Scene {
                id: "showreel",
                width: 1920,
                height: 1080,
                duration_ms: 5_000.0,
                autoplay: Some(true),
                controls: Some(true),
                Clip { start_ms: 0.0, duration_ms: 5_000.0, fill: ClipFill::HoldBoth,
                    div {
                        style: "display:flex; flex-direction:column; gap:36px;",
                        div { style: "display:flex; flex-direction:column; gap:12px;",
                            div { style: "font-size:16px; letter-spacing:0.18em; font-weight:600; color:#007aff; text-transform:uppercase;",
                                KineticText { id: "sr-eyebrow", text: "Dioxus Kinetics".to_string(), cue: "fade-in" }
                            }
                            div { style: "font-size:64px; line-height:1.04; letter-spacing:-0.022em; font-weight:700;",
                                KineticText { id: "sr-title", text: "Cinematic UI, rendered in Rust.".to_string(), cue: "rise-in" }
                            }
                            div { style: "font-size:24px; line-height:1.3; color:#5c6778; max-width:920px;",
                                KineticText {
                                    id: "sr-sub",
                                    text: "Composable motion, liquid glass, and frame-accurate video export \u{2014} from one Dioxus-first workspace.".to_string(),
                                    cue: "fade-in",
                                }
                            }
                        }
                        GlassSurface {
                            level: GlassLevel::Floating,
                            tone: GlassTone::Neutral,
                            div { style: "display:flex; gap:48px; padding:28px 32px; border-radius:18px;",
                                MetricCounter {
                                    label: "Public components".to_string(),
                                    value: "140+".to_string(),
                                    delta_text: Some("all WCAG 2.2 AA".to_string()),
                                }
                                MetricCounter {
                                    label: "Glass render tiers".to_string(),
                                    value: "5".to_string(),
                                    delta_text: Some("WebGPU \u{2192} solid".to_string()),
                                }
                                MetricCounter {
                                    label: "JS runtime deps".to_string(),
                                    value: "0".to_string(),
                                    delta_text: Some("pure Rust + WGSL".to_string()),
                                }
                            }
                        }
                        div { style: "display:flex; gap:24px;",
                            div { style: "flex:1.4; background:#ffffff; border-radius:18px; padding:24px; box-shadow:0 18px 46px rgba(27,39,61,0.10);",
                                AreaChart {
                                    label: "Weekly revenue (thousands USD)".to_string(),
                                    series: vec![ChartSeries::new(
                                        "Revenue",
                                        vec![12.0, 18.0, 15.0, 22.0, 28.0, 26.0, 34.0, 41.0],
                                    )],
                                    x_labels: vec![
                                        "W1".to_string(), "W2".to_string(), "W3".to_string(), "W4".to_string(),
                                        "W5".to_string(), "W6".to_string(), "W7".to_string(), "W8".to_string(),
                                    ],
                                    show_legend: false,
                                }
                            }
                            div { style: "flex:1; background:#ffffff; border-radius:18px; padding:24px; box-shadow:0 18px 46px rgba(27,39,61,0.10);",
                                FunnelChart {
                                    label: "Signup conversion".to_string(),
                                    stages: vec![
                                        FunnelStage::new("Visited", 1000.0),
                                        FunnelStage::new("Signed up", 420.0),
                                        FunnelStage::new("Activated", 210.0),
                                        FunnelStage::new("Paid", 90.0),
                                    ],
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
