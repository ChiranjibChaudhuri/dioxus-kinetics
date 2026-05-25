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
