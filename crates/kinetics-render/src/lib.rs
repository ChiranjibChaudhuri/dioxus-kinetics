#![forbid(unsafe_code)]

//! Frame-by-frame SSR exporter for kinetics Scene compositions.

mod capture;
mod template;

use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub frames: u32,
    pub fps: u32,
    pub width: u32,
    pub height: u32,
    pub composition_id: String,
    pub output_dir: PathBuf,
    pub capture_png: bool,
    pub encode_mp4: bool,
}

impl RenderConfig {
    pub fn validate(&self) -> Result<(), RenderError> {
        if self.frames == 0 {
            return Err(RenderError::InvalidConfig(
                "frames must be > 0".into(),
            ));
        }
        if self.fps == 0 {
            return Err(RenderError::InvalidConfig("fps must be > 0".into()));
        }
        if self.width == 0 || self.height == 0 {
            return Err(RenderError::InvalidConfig(
                "width and height must be > 0".into(),
            ));
        }
        if self.encode_mp4 && !self.capture_png {
            return Err(RenderError::InvalidConfig(
                "encode_mp4 requires capture_png".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RenderError {
    InvalidConfig(String),
    Io(io::Error),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfig(msg) => write!(f, "invalid renderer config: {msg}"),
            Self::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for RenderError {}

impl From<io::Error> for RenderError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug, Clone)]
pub struct RenderReport {
    pub frames_written: u32,
    pub html_dir: PathBuf,
    pub png_dir: Option<PathBuf>,
    pub mp4_path: Option<PathBuf>,
    pub warnings: Vec<String>,
}

pub struct Renderer {
    config: RenderConfig,
}

impl Renderer {
    pub fn new(config: RenderConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &RenderConfig {
        &self.config
    }
}

use std::fs;
use std::rc::Rc;

use dioxus::prelude::*;
use ui_runtime::scene_clock::SceneClock;

/// Type-erased scene factory passed into `FrameProbe`. We type-erase here
/// because `#[component]` requires props to implement `PartialEq`, which
/// closures do not. We compare by `Rc` pointer identity instead — two
/// `SceneFn`s are equal iff they wrap the same underlying allocation.
#[derive(Clone)]
struct SceneFn(Rc<dyn Fn(SceneClock) -> Element>);

impl PartialEq for SceneFn {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Renderer {
    /// Walks the configured frame range, invoking `scene_fn(clock)`
    /// per frame, and serializing the result via `dioxus-ssr`.
    /// `scene_fn` MUST construct a deterministic tree given the
    /// supplied clock — the renderer does not memoize across frames.
    pub fn render<F>(&self, scene_fn: F) -> Result<RenderReport, RenderError>
    where
        F: Fn(SceneClock) -> Element + 'static,
    {
        self.config.validate()?;

        let html_dir = self.config.output_dir.join("frames");
        fs::create_dir_all(&html_dir)?;

        let scene_fn: SceneFn = SceneFn(Rc::new(scene_fn));

        // Each frame builds a fresh VirtualDom via dioxus_ssr's
        // render_element so SceneClock construction happens inside a
        // valid Dioxus runtime. We wrap the per-frame work in a tiny
        // probe component so the clock is constructed within scope.
        for frame in 0..self.config.frames {
            let elapsed_ms = (frame as f32 / self.config.fps as f32) * 1000.0;
            let scene_fn = scene_fn.clone();
            let duration_ms =
                (self.config.frames as f32 / self.config.fps as f32) * 1000.0;
            let fps = self.config.fps;

            let body = dioxus_ssr::render_element(rsx! {
                FrameProbe {
                    duration_ms: duration_ms,
                    fps: fps,
                    elapsed_ms: elapsed_ms,
                    scene_fn: scene_fn,
                }
            });

            let frame_path = html_dir.join(format!("{frame}.html"));
            fs::write(&frame_path, body)?;
        }

        // Write the ExportManifest. We don't pull ui_capture's full
        // manifest builder API in to avoid extra deps — write the
        // minimal JSON shape directly.
        let manifest_path = self.config.output_dir.join("manifest.json");
        let manifest_json = serde_json::json!({
            "schema_version": 1,
            "composition": {
                "id": self.config.composition_id,
                "width": self.config.width,
                "height": self.config.height,
                "fps": self.config.fps,
                "frame_count": self.config.frames,
            },
        });
        // Compact form: the assertion in tests/renderer.rs matches the
        // exact byte sequence `"frame_count":3` (no whitespace after `:`),
        // which `to_string_pretty` would break.
        fs::write(&manifest_path, serde_json::to_string(&manifest_json)?)?;

        let mut report_warnings: Vec<String> = Vec::new();
        let mut png_dir: Option<PathBuf> = None;
        if self.config.capture_png {
            let outcome = capture::run_capture(&self.config.output_dir);
            png_dir = outcome.png_dir;
            report_warnings.extend(outcome.warnings);
        }

        Ok(RenderReport {
            frames_written: self.config.frames,
            html_dir,
            png_dir,
            mp4_path: None,
            warnings: report_warnings,
        })
    }
}

#[component]
fn FrameProbe(
    duration_ms: f32,
    fps: u32,
    elapsed_ms: f32,
    scene_fn: SceneFn,
) -> Element {
    let clock = use_hook(|| SceneClock::new(duration_ms, fps, false));
    // Seek to the target frame before invoking scene_fn.
    clock.seek_ms(elapsed_ms);
    (scene_fn.0)(clock)
}

// serde_json::Error -> RenderError::Io is a stretch; map through an
// io::Error so the surface stays narrow.
impl From<serde_json::Error> for RenderError {
    fn from(e: serde_json::Error) -> Self {
        Self::Io(io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
    }
}

// Re-export so tests can construct a Composition struct if needed.
pub use ui_composition::Composition as CompositionInfo;
