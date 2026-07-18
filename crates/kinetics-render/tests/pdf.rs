use std::path::PathBuf;

use dioxus::prelude::*;
use kinetics_render::{RenderConfig, Renderer};

fn tmp_output() -> PathBuf {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().to_path_buf();
    std::mem::forget(d);
    path
}

#[test]
fn capture_pdf_with_missing_node_returns_warning_not_error() {
    // Drive the `node` lookup to fail by clearing PATH (and PATHEXT on
    // Windows) so Command::spawn returns a "program not found" error.
    let original_path = std::env::var_os("PATH");
    if cfg!(windows) {
        std::env::set_var("PATH", "");
    } else {
        std::env::set_var("PATH", "/nonexistent-12345");
    }
    let _guard = scopeguard::guard((), |_| {
        if let Some(p) = &original_path {
            std::env::set_var("PATH", p);
        } else {
            std::env::remove_var("PATH");
        }
    });

    let out = tmp_output();
    let cfg = RenderConfig {
        frames: 2,
        fps: 10,
        width: 100,
        height: 100,
        composition_id: "pdf-skip".into(),
        output_dir: out.clone(),
        capture_png: false,
        encode_mp4: false,
        capture_pdf: true,
    };
    let renderer = Renderer::new(cfg);
    let report = renderer
        .render(|_clock| rsx! { div { "x" } })
        .expect("render ok even when pdf capture is skipped");

    assert_eq!(report.frames_written, 2);
    assert!(
        report.pdf_path.is_none(),
        "expected pdf_path to be None when node is unavailable",
    );
    assert!(
        report
            .warnings
            .iter()
            .any(|w| w.to_lowercase().contains("node") || w.to_lowercase().contains("pdf")),
        "expected a warning mentioning node/pdf; got {:?}",
        report.warnings,
    );

    // The HTML frames remain the source of truth even when PDF export
    // is skipped — the orchestrator must not block frame emission.
    for frame in 0..2 {
        let path = out.join("frames").join(format!("{frame}.html"));
        assert!(path.exists(), "frame {frame} should still be written");
    }
}
