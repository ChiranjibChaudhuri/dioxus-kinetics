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
fn capture_png_with_missing_npx_returns_warning_not_error() {
    // Simulate npx being unavailable by setting PATH to an empty value.
    let original_path = std::env::var_os("PATH");
    let isolated_path = if cfg!(windows) {
        // Windows needs SYSTEM32 to spawn processes at all; clear PATH
        // and rely on a deliberately-empty PATHEXT to fail the lookup.
        std::env::set_var("PATH", "");
        Some("")
    } else {
        std::env::set_var("PATH", "/nonexistent-12345");
        Some("/nonexistent-12345")
    };
    let _path_guard = scopeguard::guard((), |_| {
        if let Some(p) = &original_path {
            std::env::set_var("PATH", p);
        } else {
            std::env::remove_var("PATH");
        }
    });
    let _ = isolated_path;

    let out = tmp_output();
    let cfg = RenderConfig {
        frames: 2,
        fps: 10,
        width: 100,
        height: 100,
        composition_id: "capture-skip".into(),
        output_dir: out.clone(),
        capture_png: true,
        encode_mp4: false,
    };
    let renderer = Renderer::new(cfg);
    let report = renderer
        .render(|_clock| rsx! { div { "x" } })
        .expect("render ok even when capture is skipped");

    assert_eq!(report.frames_written, 2);
    assert!(
        report.png_dir.is_none(),
        "expected png_dir to be None when capture is skipped",
    );
    assert!(
        report
            .warnings
            .iter()
            .any(|w| w.to_lowercase().contains("playwright") || w.to_lowercase().contains("npx")),
        "expected a warning about missing playwright/npx; got {:?}",
        report.warnings,
    );
}
