use std::collections::HashMap;
use ui_layout::Rect;
use ui_runtime::shared::{ElementSnapshot, SharedElementRegistry};

fn snapshot(x: f32, y: f32) -> ElementSnapshot {
    ElementSnapshot {
        rect: Rect::new(x, y, 100.0, 50.0),
        computed: HashMap::new(),
        timestamp_ms: 0.0,
    }
}

#[test]
fn record_and_snapshot_round_trip() {
    let r = SharedElementRegistry::default();
    let s = snapshot(0.0, 0.0);
    r.record("a".to_string(), s.clone());
    assert_eq!(r.snapshot("a"), Some(s));
}

#[test]
fn forget_removes_snapshot() {
    let r = SharedElementRegistry::default();
    r.record("a".to_string(), snapshot(0.0, 0.0));
    r.forget("a");
    assert_eq!(r.snapshot("a"), None);
}

#[test]
fn record_overwrites_existing_id() {
    let r = SharedElementRegistry::default();
    r.record("a".to_string(), snapshot(0.0, 0.0));
    r.record("a".to_string(), snapshot(10.0, 10.0));
    assert_eq!(r.snapshot("a").unwrap().rect.x, 10.0);
}
