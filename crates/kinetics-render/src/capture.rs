//! PNG capture orchestrator.
//!
//! Spawns `node capture.cjs <output_dir>` as a child process. Returns
//! `Some(png_dir)` on success or `None` plus a warning string on any
//! failure (missing node, missing playwright, missing browsers, or
//! non-zero exit). This is a graceful-degradation stage — never errors.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::template::CAPTURE_CJS;

pub struct CaptureOutcome {
    pub png_dir: Option<PathBuf>,
    pub warnings: Vec<String>,
}

pub fn run_capture(output_dir: &Path) -> CaptureOutcome {
    let mut warnings = Vec::new();
    let script_path = output_dir.join("capture.cjs");
    if let Err(e) = std::fs::write(&script_path, CAPTURE_CJS) {
        warnings.push(format!(
            "failed to write capture.cjs: {e} (PNG capture skipped)",
        ));
        return CaptureOutcome {
            png_dir: None,
            warnings,
        };
    }

    let cmd = if cfg!(windows) { "node.exe" } else { "node" };
    let result = Command::new(cmd).arg(&script_path).arg(output_dir).output();

    let output = match result {
        Ok(o) => o,
        Err(e) => {
            warnings.push(format!(
                "PNG capture skipped: could not spawn `{cmd}` ({e}). \
                 Install Node.js + playwright to enable capture."
            ));
            return CaptureOutcome {
                png_dir: None,
                warnings,
            };
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warnings.push(format!(
            "PNG capture failed (exit {:?}): {}. \
             Run `npm install playwright && npx playwright install chromium` \
             in the output directory's parent to enable capture.",
            output.status.code(),
            stderr.trim()
        ));
        return CaptureOutcome {
            png_dir: None,
            warnings,
        };
    }

    CaptureOutcome {
        png_dir: Some(output_dir.join("png")),
        warnings,
    }
}
