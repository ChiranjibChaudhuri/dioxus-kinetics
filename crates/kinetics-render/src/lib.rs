#![forbid(unsafe_code)]

//! Frame-by-frame SSR exporter for kinetics Scene compositions.

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
