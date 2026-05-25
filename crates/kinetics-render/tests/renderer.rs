use std::path::PathBuf;
use kinetics_render::{RenderConfig, RenderError, Renderer};

fn base_config() -> RenderConfig {
    RenderConfig {
        frames: 10,
        fps: 30,
        width: 320,
        height: 240,
        composition_id: "test".to_string(),
        output_dir: PathBuf::from("/tmp/kinetics-render-test"),
        capture_png: false,
        encode_mp4: false,
    }
}

#[test]
fn renderer_constructs_from_valid_config() {
    let r = Renderer::new(base_config());
    let _ = r;
}

#[test]
fn invalid_fps_zero_rejected_via_validate() {
    let mut cfg = base_config();
    cfg.fps = 0;
    let err = RenderConfig::validate(&cfg).unwrap_err();
    assert!(matches!(err, RenderError::InvalidConfig(_)));
}

#[test]
fn invalid_frames_zero_rejected_via_validate() {
    let mut cfg = base_config();
    cfg.frames = 0;
    let err = RenderConfig::validate(&cfg).unwrap_err();
    assert!(matches!(err, RenderError::InvalidConfig(_)));
}

#[test]
fn invalid_width_zero_rejected_via_validate() {
    let mut cfg = base_config();
    cfg.width = 0;
    let err = RenderConfig::validate(&cfg).unwrap_err();
    assert!(matches!(err, RenderError::InvalidConfig(_)));
}

#[test]
fn encode_mp4_without_capture_png_rejected() {
    let mut cfg = base_config();
    cfg.encode_mp4 = true;
    let err = RenderConfig::validate(&cfg).unwrap_err();
    assert!(
        matches!(&err, RenderError::InvalidConfig(msg) if msg.contains("capture_png")),
        "got {err:?}",
    );
}

use dioxus::prelude::*;

fn tmp_output() -> PathBuf {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().to_path_buf();
    std::mem::forget(d); // leak handle so dir survives the test
    path
}

#[test]
fn renders_n_html_files_with_distinct_elapsed_ms() {
    let out = tmp_output();
    let cfg = RenderConfig {
        frames: 5,
        fps: 10,
        width: 100,
        height: 100,
        composition_id: "test-distinct-elapsed".into(),
        output_dir: out.clone(),
        capture_png: false,
        encode_mp4: false,
    };
    let renderer = Renderer::new(cfg);
    let report = renderer
        .render(|clock| {
            let elapsed = clock.peek_elapsed_ms() as i64;
            rsx! { div { "data-test-elapsed-ms": "{elapsed}", "frame" } }
        })
        .expect("render ok");

    assert_eq!(report.frames_written, 5);
    assert!(report.png_dir.is_none());
    assert!(report.mp4_path.is_none());

    for frame in 0..5 {
        let path = out.join("frames").join(format!("{frame}.html"));
        let body = std::fs::read_to_string(&path).expect("frame exists");
        let expected_ms = (frame as f32 / 10.0 * 1000.0) as i64;
        assert!(
            body.contains(&format!("data-test-elapsed-ms=\"{expected_ms}\"")),
            "frame {frame}: expected elapsed_ms={expected_ms}, body={body}",
        );
    }
}

#[test]
fn writes_export_manifest_alongside_frames() {
    let out = tmp_output();
    let cfg = RenderConfig {
        frames: 3,
        fps: 30,
        width: 640,
        height: 480,
        composition_id: "manifest-test".into(),
        output_dir: out.clone(),
        capture_png: false,
        encode_mp4: false,
    };
    let renderer = Renderer::new(cfg);
    renderer
        .render(|_clock| rsx! { div { "x" } })
        .expect("render ok");

    let manifest_path = out.join("manifest.json");
    let manifest_body =
        std::fs::read_to_string(&manifest_path).expect("manifest exists");
    assert!(
        manifest_body.contains("\"manifest-test\""),
        "manifest body: {manifest_body}",
    );
    assert!(manifest_body.contains("\"frame_count\":3"));
}
