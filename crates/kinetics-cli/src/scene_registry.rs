use std::path::PathBuf;

use dioxus::prelude::*;
use kinetics::prelude::*;
use kinetics_render::{RenderConfig, Renderer};
use ui_composition::ClipFill;
use ui_runtime::scene_clock::SceneClock;

pub struct SceneSpec {
    pub id: &'static str,
    pub width: u32,
    pub height: u32,
    // Declared per plan for downstream metadata exposure even though the
    // Renderer currently derives effective duration from `frames` / `fps`.
    #[allow(dead_code)]
    pub duration_ms: f32,
    pub scene_fn: fn(SceneClock) -> Element,
}

pub const SCENES: &[SceneSpec] = &[
    SceneSpec {
        id: "hello",
        width: 1280,
        height: 720,
        duration_ms: 5_000.0,
        scene_fn: hello_scene,
    },
    SceneSpec {
        id: "product-intro",
        width: 1920,
        height: 1080,
        duration_ms: 10_000.0,
        scene_fn: product_intro_scene,
    },
    SceneSpec {
        id: "report",
        width: 1920,
        height: 1080,
        duration_ms: 8_000.0,
        scene_fn: report_scene,
    },
    SceneSpec {
        id: "showreel",
        width: 1920,
        height: 1080,
        duration_ms: 5_000.0,
        scene_fn: showreel_scene,
    },
];

pub fn lookup(id: &str) -> Option<&'static SceneSpec> {
    SCENES.iter().find(|s| s.id == id)
}

pub fn run_render(
    spec: &SceneSpec,
    out: &std::path::Path,
    frames: u32,
    fps: u32,
    capture_png: bool,
    encode_mp4: bool,
    capture_pdf: bool,
) -> Result<(), String> {
    let config = RenderConfig {
        frames,
        fps,
        width: spec.width,
        height: spec.height,
        composition_id: spec.id.to_string(),
        output_dir: PathBuf::from(out),
        capture_png,
        encode_mp4,
        capture_pdf,
    };
    let renderer = Renderer::new(config);
    let report = renderer
        .render(spec.scene_fn)
        .map_err(|e| format!("render failed: {e}"))?;
    println!(
        "kinetics render: wrote {} frames to {:?}",
        report.frames_written, report.html_dir
    );
    for w in &report.warnings {
        eprintln!("warning: {w}");
    }
    if let Some(p) = report.png_dir {
        println!("PNG sequence: {}", p.display());
    }
    if let Some(p) = report.mp4_path {
        println!("MP4: {}", p.display());
    }
    if let Some(p) = report.pdf_path {
        println!("PDF: {}", p.display());
    }
    Ok(())
}

fn hello_scene(_clock: SceneClock) -> Element {
    rsx! {
        Scene {
            id: "hello",
            width: 1280,
            height: 720,
            duration_ms: 5_000.0,
            autoplay: Some(false),
            controls: Some(false),
            driver: Some(SceneDriver::Manual),
            Clip { start_ms: 0.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "hello-title",
                    text: "Hello, kinetics.".to_string(),
                    cue: "rise-in",
                }
            }
            Clip { start_ms: 1_500.0, duration_ms: 3_000.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "hello-body",
                    text: "Composable cinematic motion.".to_string(),
                    cue: "fade-in",
                }
            }
        }
    }
}

fn product_intro_scene(_clock: SceneClock) -> Element {
    rsx! {
        Scene {
            id: "product-intro",
            width: 1920,
            height: 1080,
            duration_ms: 10_000.0,
            autoplay: Some(false),
            controls: Some(false),
            driver: Some(SceneDriver::Manual),
            Clip { start_ms: 0.0, duration_ms: 2_400.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "intro-title",
                    text: "Kinetics moves like light.".to_string(),
                    cue: "rise-in",
                }
            }
            Clip { start_ms: 800.0, duration_ms: 2_400.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "intro-body",
                    text: "Composable motion for downstream SaaS.".to_string(),
                    cue: "fade-in",
                }
            }
            Clip { start_ms: 6_800.0, duration_ms: 3_200.0, fill: ClipFill::HoldEnd,
                Button { variant: ButtonVariant::Primary, "Start building" }
            }
        }
    }
}

/// The LinkedIn showreel — a light, Apple-grade hero that progressive-
/// reveals the differentiators: SplitText-style kinetic title, animated
/// metric strip, and the new chart family with draw-in pinned to the
/// clock so `kinetics render --scene showreel --capture-png --encode-mp4`
/// produces a frame-accurate video. This is the render-optimized twin of
/// the live `examples/showreel` app.
fn showreel_scene(clock: SceneClock) -> Element {
    let elapsed = *clock.elapsed_ms.peek();
    let reveal = |start: f32, dur: f32| -> f32 { ((elapsed - start) / dur).clamp(0.0, 1.0) };
    let area_progress = reveal(1_500.0, 2_000.0);
    let funnel_progress = reveal(2_200.0, 2_000.0);

    rsx! {
        Scene {
            id: "showreel",
            width: 1920,
            height: 1080,
            duration_ms: 5_000.0,
            autoplay: Some(false),
            controls: Some(false),
            driver: Some(SceneDriver::Manual),
            Clip { start_ms: 0.0, duration_ms: 5_000.0, fill: ClipFill::HoldBoth,
                div {
                    style: "width:100%; min-height:100vh; box-sizing:border-box; padding:96px 112px; \
                            display:flex; flex-direction:column; gap:40px; \
                            background:linear-gradient(135deg, #f6f8fb 0%, #e8eef6 60%, #f2f5f9 100%); \
                            font-family:Inter, ui-sans-serif, system-ui, sans-serif; color:#111827;",
                    div { style: "display:flex; flex-direction:column; gap:14px;",
                        div { style: "font-size:18px; letter-spacing:0.18em; font-weight:600; color:#007aff; text-transform:uppercase;",
                            KineticText { id: "sr-eyebrow", text: "Dioxus Kinetics".to_string(), cue: "fade-in" }
                        }
                        div { style: "font-size:84px; line-height:1.04; letter-spacing:-0.022em; font-weight:700;",
                            KineticText { id: "sr-title", text: "Cinematic UI, rendered in Rust.".to_string(), cue: "rise-in" }
                        }
                        div { style: "font-size:30px; line-height:1.3; color:#5c6778; max-width:1100px;",
                            KineticText {
                                id: "sr-sub",
                                text: "Composable motion, liquid glass, and frame-accurate video export — from one Dioxus-first workspace.".to_string(),
                                cue: "fade-in",
                            }
                        }
                    }
                    div { style: "display:flex; gap:48px; margin-top:8px;",
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
                            label: "Zero JS runtime deps".to_string(),
                            value: "0".to_string(),
                            delta_text: Some("pure Rust + WGSL".to_string()),
                        }
                    }
                    div { style: "display:flex; gap:32px; margin-top:16px;",
                        div { style: "flex:1.4; background:#ffffff; border-radius:18px; padding:28px; box-shadow:0 18px 46px rgba(27,39,61,0.10);",
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
                                progress: Some(area_progress),
                            }
                        }
                        div { style: "flex:1; background:#ffffff; border-radius:18px; padding:28px; box-shadow:0 18px 46px rgba(27,39,61,0.10);",
                            FunnelChart {
                                label: "Signup conversion".to_string(),
                                stages: vec![
                                    FunnelStage::new("Visited", 1000.0),
                                    FunnelStage::new("Signed up", 420.0),
                                    FunnelStage::new("Activated", 210.0),
                                    FunnelStage::new("Paid", 90.0),
                                ],
                                progress: Some(funnel_progress),
                            }
                        }
                    }
                }
            }
        }
    }
}

/// The report-to-video scene: a data-driven composition (title, area chart,
/// and metric strip) whose chart draw-in is pinned to the clock so the
/// `kinetics render --scene report` pipeline produces a frame-accurate video.
fn report_scene(clock: SceneClock) -> Element {
    let elapsed = *clock.elapsed_ms.peek();
    let duration = clock.duration_ms.peek().max(1.0);
    let progress = (elapsed / duration).clamp(0.0, 1.0);

    rsx! {
        Scene {
            id: "report",
            width: 1920,
            height: 1080,
            duration_ms: 8_000.0,
            autoplay: Some(false),
            controls: Some(false),
            driver: Some(SceneDriver::Manual),
            Clip { start_ms: 0.0, duration_ms: 8_000.0, fill: ClipFill::HoldBoth,
                div { style: "padding:80px; display:grid; gap:32px; font-family:Inter, ui-sans-serif, system-ui, sans-serif; color:#111827;",
                    KineticText {
                        id: "report-title",
                        text: "Q3 Performance Report".to_string(),
                        cue: "rise-in",
                    }
                    AreaChart {
                        label: "Weekly revenue (thousands USD)".to_string(),
                        series: vec![ChartSeries::new(
                            "Revenue",
                            vec![12.0, 18.0, 15.0, 22.0, 28.0, 26.0, 34.0],
                        )],
                        x_labels: vec![
                            "W1".to_string(), "W2".to_string(), "W3".to_string(), "W4".to_string(),
                            "W5".to_string(), "W6".to_string(), "W7".to_string(),
                        ],
                        show_legend: false,
                        progress: Some(progress),
                    }
                    div { style: "display:flex; gap:48px;",
                        MetricCounter {
                            label: "Net MRR".to_string(),
                            value: "$128.4k".to_string(),
                            delta_text: Some("+12.5%".to_string()),
                        }
                        MetricCounter {
                            label: "Churn".to_string(),
                            value: "1.4%".to_string(),
                            delta_text: Some("-0.3pp".to_string()),
                        }
                    }
                }
            }
        }
    }
}
