use ui_capture::{
    CaptureError, CaptureMark, CaptureStageDescriptor, ExportManifest, ViewportProfile,
};
use ui_composition::Composition;

#[test]
fn viewport_profile_rejects_zero_size() {
    let viewport = ViewportProfile::new("bad", 0, 844);

    assert_eq!(viewport.validate(), Err(CaptureError::InvalidViewport));
}

#[test]
fn capture_marks_resolve_by_name() {
    let manifest = ExportManifest::new("0.1.0")
        .with_mark(CaptureMark::new("modal-open", 24))
        .with_mark(CaptureMark::new("settled", 90));

    assert_eq!(manifest.mark_frame("settled"), Some(90));
    assert_eq!(manifest.mark_frame("missing"), None);
}

#[test]
fn manifest_validates_stage_composition_and_viewport() {
    let manifest = ExportManifest::new("0.1.0")
        .with_composition(Composition::new("demo", 1920, 1080, 30, 120))
        .with_stage(CaptureStageDescriptor::new("stage", "demo"))
        .with_viewport(ViewportProfile::desktop());

    assert_eq!(manifest.validate(), Ok(()));
}
