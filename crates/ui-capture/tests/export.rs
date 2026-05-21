use ui_capture::CaptureStageDescriptor;

#[test]
fn capture_stage_descriptor_carries_stage_id() {
    let stage = CaptureStageDescriptor::new("launch-demo");

    assert_eq!(stage.id, "launch-demo");
}
