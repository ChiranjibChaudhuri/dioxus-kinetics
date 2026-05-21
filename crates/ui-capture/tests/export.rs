use ui_capture::CaptureStageDescriptor;

#[test]
fn capture_stage_descriptor_carries_stage_id() {
    let stage = CaptureStageDescriptor::new("launch-demo", "intro-composition");

    assert_eq!(stage.id, "launch-demo");
    assert_eq!(stage.composition_id, "intro-composition");
}
