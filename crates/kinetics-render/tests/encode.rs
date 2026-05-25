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
fn encode_mp4_with_missing_ffmpeg_returns_warning() {
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
        composition_id: "encode-skip".into(),
        output_dir: out.clone(),
        capture_png: true, // requires capture_png to also be true
        encode_mp4: true,
    };
    let renderer = Renderer::new(cfg);
    let report = renderer
        .render(|_clock| rsx! { div { "x" } })
        .expect("render ok even when encode + capture skip");

    assert_eq!(report.frames_written, 2);
    assert!(
        report.mp4_path.is_none(),
        "expected mp4_path to be None when ffmpeg/png missing",
    );
    let lower: Vec<String> = report.warnings.iter().map(|w| w.to_lowercase()).collect();
    assert!(
        lower
            .iter()
            .any(|w| w.contains("ffmpeg") || w.contains("png")),
        "expected a warning mentioning ffmpeg or png; got {:?}",
        report.warnings,
    );
}
