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

/// The report-to-video scene: a data-driven composition (title + area chart
/// + metric strip) whose chart draw-in is pinned to the clock so the
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
