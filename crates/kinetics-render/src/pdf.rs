//! PDF report orchestrator.
//!
//! Spawns `node pdf.cjs <output_dir> <width> <height>`. The script loads the
//! highest-numbered (settled) HTML frame via Playwright Chromium and writes
//! `report.pdf` through `page.pdf()`. Like the PNG/MP4 stages this is a
//! graceful-degradation step — any missing tool or non-zero exit becomes a
//! warning, never an error, so HTML-frame rendering stays the source of truth.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::template::PDF_CJS;

pub struct PdfOutcome {
    pub pdf_path: Option<PathBuf>,
    pub warnings: Vec<String>,
}

pub fn run_pdf(output_dir: &Path, width: u32, height: u32) -> PdfOutcome {
    let mut warnings = Vec::new();
    let script_path = output_dir.join("pdf.cjs");
    if let Err(e) = std::fs::write(&script_path, PDF_CJS) {
        warnings.push(format!("failed to write pdf.cjs: {e} (PDF capture skipped)"));
        return PdfOutcome {
            pdf_path: None,
            warnings,
        };
    }

    let cmd = if cfg!(windows) { "node.exe" } else { "node" };
    let result = Command::new(cmd)
        .arg(&script_path)
        .arg(output_dir)
        .arg(width.to_string())
        .arg(height.to_string())
        .output();

    let output = match result {
        Ok(o) => o,
        Err(e) => {
            warnings.push(format!(
                "PDF capture skipped: could not spawn `{cmd}` ({e}). \
                 Install Node.js + playwright to enable PDF export."
            ));
            return PdfOutcome {
                pdf_path: None,
                warnings,
            };
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warnings.push(format!(
            "PDF capture failed (exit {:?}): {}. \
             Run `npm install playwright && npx playwright install chromium` \
             in the output directory's parent to enable PDF export.",
            output.status.code(),
            stderr.trim()
        ));
        return PdfOutcome {
            pdf_path: None,
            warnings,
        };
    }

    let pdf_path = output_dir.join("report.pdf");
    if pdf_path.exists() {
        PdfOutcome {
            pdf_path: Some(pdf_path),
            warnings,
        }
    } else {
        warnings.push("PDF capture reported success but report.pdf is missing".into());
        PdfOutcome {
            pdf_path: None,
            warnings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_output_dir_pushes_warning_not_error() {
        let outcome = run_pdf(Path::new("C:/no/such/dir/here/__at_all__"), 800, 600);
        assert!(outcome.pdf_path.is_none());
        assert!(!outcome.warnings.is_empty());
    }
}
