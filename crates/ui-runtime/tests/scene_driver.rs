use ui_runtime::scene_driver::{SceneDriver, ScrollObserverConfig};

#[test]
fn autoplay_is_default() {
    let d = SceneDriver::default();
    assert!(matches!(d, SceneDriver::Autoplay));
}

#[test]
fn manual_is_distinct_from_autoplay() {
    let m = SceneDriver::Manual;
    let a = SceneDriver::Autoplay;
    assert_ne!(m, a);
}

#[test]
fn scroll_carries_observer_config() {
    let config = ScrollObserverConfig {
        trigger_selector: "#hero".to_string(),
        start_offset_px: Some(100.0),
        end_offset_px: Some(0.0),
    };
    let d = SceneDriver::Scroll(config.clone());
    match d {
        SceneDriver::Scroll(c) => {
            assert_eq!(c.trigger_selector, "#hero");
            assert_eq!(c.start_offset_px, Some(100.0));
        }
        _ => panic!("expected Scroll"),
    }
}
