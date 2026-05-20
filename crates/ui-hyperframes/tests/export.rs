use ui_hyperframes::{Composition, RenderTrack};

#[test]
fn composition_exports_deterministic_metadata() {
    let composition = Composition::new("launch-demo", 1920, 1080, 240);
    let track = RenderTrack::from_composition(&composition);

    assert_eq!(track.composition_id, "launch-demo");
    assert_eq!(track.frame_count, 240);
    assert_eq!(track.aspect_ratio(), "16:9");
}
