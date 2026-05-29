use dioxus::prelude::*;
use ui_runtime::reduced_motion::use_reduced_motion;
use ui_timeline::{sample_path, PathPoint};

#[component]
pub fn MotionPath(
    id: String,
    path: Vec<PathPoint>,
    duration_ms: f32,
    rotate_along_path: Option<bool>,
    children: Element,
) -> Element {
    let rotate = rotate_along_path.unwrap_or(false);
    let reduced = use_reduced_motion();
    let path_json = serde_json::to_string(&path).unwrap_or_else(|_| "[]".to_string());
    let duration_attr = format!("{}", duration_ms as i64);

    // When reduced motion is requested, snap children to the path's
    // terminal point and emit `data-motion-path-reduced="true"` so the
    // Wave-1 `[data-ui-motion=reduced] .ui-motion-path` neutralizer (and
    // the JS adapter) skip the tween. The data-motion-path payload is
    // still emitted so the adapter can resolve the settled transform.
    let terminal_style = if reduced {
        terminal_transform_style(&path)
    } else {
        String::new()
    };

    rsx! {
        div {
            class: "ui-motion-path",
            "data-kinetic-id": "{id}",
            "data-motion-path": "{path_json}",
            "data-motion-path-duration-ms": "{duration_attr}",
            "data-motion-path-rotate": if rotate { "true" } else { "false" },
            "data-motion-path-reduced": if reduced { "true" } else { "false" },
            style: "{terminal_style}",
            {children}
        }
    }
}

/// Builds an inline `transform` placing the element at the path's terminal
/// point (sampled at `t = 1.0`, so Bezier segments resolve correctly),
/// used under reduced motion so children render at the final position with
/// no animation. Returns an empty string when the path has no points
/// (nothing to position).
fn terminal_transform_style(path: &[PathPoint]) -> String {
    if path.is_empty() {
        return String::new();
    }
    let (x, y) = sample_path(path, 1.0);
    format!("transform: translate({x}px, {y}px); transition: none; animation: none;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_transform_uses_last_point() {
        let path = vec![
            PathPoint::Line { end: (0.0, 0.0) },
            PathPoint::Line { end: (10.0, 20.0) },
            PathPoint::Line { end: (30.0, 40.0) },
        ];
        let style = terminal_transform_style(&path);
        assert!(style.contains("translate(30px, 40px)"));
        assert!(style.contains("transition: none"));
        assert!(style.contains("animation: none"));
    }

    #[test]
    fn terminal_transform_empty_path_is_blank() {
        assert_eq!(terminal_transform_style(&[]), "");
    }
}
