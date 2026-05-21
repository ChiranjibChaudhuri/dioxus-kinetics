use ui_composition::{
    ClipFill, Composition, CompositionError, FrameClip, FrameClock, FrameCue, FrameEase, FrameLayer,
};

#[test]
fn composition_carries_render_metadata() {
    let composition = Composition::new("intro", 1920, 1080, 60, 240);

    assert_eq!(composition.id, "intro");
    assert_eq!(composition.width, 1920);
    assert_eq!(composition.height, 1080);
    assert_eq!(composition.fps, 60);
    assert_eq!(composition.frame_count, 240);
}

#[test]
fn composition_validation_rejects_zero_dimensions() {
    let composition = Composition::new("bad", 0, 1080, 30, 120);

    assert_eq!(
        composition.validate(),
        Err(CompositionError::InvalidDimensions)
    );
}

#[test]
fn frame_clock_reports_seconds_and_clamped_progress() {
    let clock = FrameClock { frame: 15, fps: 30 };

    assert_eq!(clock.seconds(), 0.5);
    assert_eq!(clock.progress(0, 30), 0.5);
    assert_eq!(clock.progress(20, 0), 1.0);
}

#[test]
fn frame_clip_activation_respects_fill_mode() {
    let clip = FrameClip::new(10, 20, ClipFill::None);

    assert!(!clip.active_at(9));
    assert!(clip.active_at(10));
    assert!(clip.active_at(29));
    assert!(!clip.active_at(30));
}

#[test]
fn frame_cue_samples_opacity_deterministically() {
    let cue = FrameCue::opacity(0, 30, 0.0, 1.0, FrameEase::Linear);

    assert_eq!(cue.sample_opacity(FrameClock { frame: 15, fps: 30 }), 0.5);
}

#[test]
fn frame_layers_sort_by_depth_then_id() {
    let mut layers = [
        FrameLayer::new("b", 10),
        FrameLayer::new("a", 10),
        FrameLayer::new("back", 0),
    ];

    layers.sort();

    assert_eq!(layers[0].id, "back");
    assert_eq!(layers[1].id, "a");
    assert_eq!(layers[2].id, "b");
}
