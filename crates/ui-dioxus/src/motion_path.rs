use dioxus::prelude::*;
use ui_timeline::PathPoint;

#[component]
pub fn MotionPath(
    id: String,
    path: Vec<PathPoint>,
    duration_ms: f32,
    rotate_along_path: Option<bool>,
    children: Element,
) -> Element {
    let rotate = rotate_along_path.unwrap_or(false);
    let path_json = serde_json::to_string(&path).unwrap_or_else(|_| "[]".to_string());
    let duration_attr = format!("{}", duration_ms as i64);
    rsx! {
        div {
            class: "ui-motion-path",
            "data-kinetic-id": "{id}",
            "data-motion-path": "{path_json}",
            "data-motion-path-duration-ms": "{duration_attr}",
            "data-motion-path-rotate": if rotate { "true" } else { "false" },
            {children}
        }
    }
}
