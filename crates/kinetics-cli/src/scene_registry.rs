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
    println!("kinetics render: wrote {} frames to {:?}", report.frames_written, report.html_dir);
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
