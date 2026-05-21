use ui_composition::Composition;

#[test]
fn composition_carries_render_metadata() {
    let composition = Composition::new("intro", 1920, 1080, 60, 240);

    assert_eq!(composition.id, "intro");
    assert_eq!(composition.width, 1920);
    assert_eq!(composition.height, 1080);
    assert_eq!(composition.fps, 60);
    assert_eq!(composition.frame_count, 240);
}
