use dioxus::prelude::*;
use ui_dioxus::MotionPath;
use ui_timeline::PathPoint;

#[test]
fn motion_path_wraps_children_in_kinetic_box() {
    let path = vec![
        PathPoint::Line { end: (0.0, 0.0) },
        PathPoint::Line { end: (100.0, 50.0) },
    ];
    let html = dioxus_ssr::render_element(rsx! {
        MotionPath {
            id: "hero-arc".to_string(),
            path: path,
            duration_ms: 1200.0,
            "Trail"
        }
    });

    assert!(html.contains("ui-motion-path"));
    assert!(html.contains("data-motion-path"));
    assert!(html.contains("data-kinetic-id=\"hero-arc\""));
    assert!(html.contains("Trail"));
}

#[test]
fn motion_path_data_attribute_is_json_serialized() {
    let path = vec![PathPoint::Line { end: (0.0, 0.0) }];
    let html = dioxus_ssr::render_element(rsx! {
        MotionPath {
            id: "arc".to_string(),
            path: path,
            duration_ms: 500.0,
            ""
        }
    });

    // Find the data-motion-path attribute value.
    let needle = "data-motion-path=\"";
    let start = html.find(needle).expect("data-motion-path attribute present");
    let value_start = start + needle.len();
    let value = &html[value_start..];
    // The serialized JSON starts with `[`. Note: SSR may escape `"` as `&quot;`,
    // so we check the raw attribute begins with `[`.
    assert!(
        value.starts_with('[') || value.starts_with("&#91;"),
        "expected data-motion-path to start with [, got: {value}"
    );
    assert!(
        html.contains("Line") || html.contains("&quot;Line&quot;"),
        "expected serialized JSON to contain Line, got: {html}"
    );
}
