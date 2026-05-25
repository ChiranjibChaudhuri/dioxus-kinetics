//! MP4 encode orchestrator. Wraps FFmpeg as a child process and
//! gracefully skips on missing binary or non-zero exit.

use std::path::{Path, PathBuf};
use std::process::Command;

pub struct EncodeOutcome {
    pub mp4_path: Option<PathBuf>,
    pub warnings: Vec<String>,
}

pub fn run_encode(output_dir: &Path, fps: u32) -> EncodeOutcome {
    let mut warnings = Vec::new();
    let png_dir = output_dir.join("png");
    if !png_dir.exists() {
        warnings.push(
            "MP4 encode skipped: PNG directory does not exist (PNG capture must have failed)."
                .to_string(),
        );
        return EncodeOutcome {
            mp4_path: None,
            warnings,
        };
    }

    let mp4_path = output_dir.join("render.mp4");
    let pattern = png_dir.join("%d.png");
    let cmd = if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" };

    let result = Command::new(cmd)
        .arg("-y")
        .arg("-framerate")
        .arg(fps.to_string())
        .arg("-i")
        .arg(&pattern)
        .arg("-c:v")
        .arg("libx264")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg(&mp4_path)
        .output();

    let output = match result {
        Ok(o) => o,
        Err(e) => {
            warnings.push(format!(
                "MP4 encode skipped: could not spawn `{cmd}` ({e}). \
                 Install FFmpeg and add it to PATH to enable encoding."
            ));
            return EncodeOutcome {
                mp4_path: None,
                warnings,
            };
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warnings.push(format!(
            "MP4 encode failed (exit {:?}): {}",
            output.status.code(),
            stderr.trim()
        ));
        return EncodeOutcome {
            mp4_path: None,
            warnings,
        };
    }

    EncodeOutcome {
        mp4_path: Some(mp4_path),
        warnings,
    }
}
