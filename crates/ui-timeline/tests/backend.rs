use ui_timeline::{TimelineCapability, TimelineRuntime};

#[test]
fn timeline_runtime_declares_native_capabilities() {
    let runtime = TimelineRuntime::default();

    assert_eq!(
        runtime.capabilities(),
        &[
            TimelineCapability::Tracks,
            TimelineCapability::Labels,
            TimelineCapability::Stagger,
            TimelineCapability::SharedMove,
            TimelineCapability::ScrollProgress,
        ]
    );
    assert_eq!(runtime.target(), "native");
}
