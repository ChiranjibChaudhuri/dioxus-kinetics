use ui_tokens::elevation::{DARK_ELEVATION, LIGHT_ELEVATION};

#[test]
fn light_elevation_has_four_non_empty_tiers() {
    assert!(!LIGHT_ELEVATION.e0.is_empty());
    assert!(!LIGHT_ELEVATION.e1.is_empty());
    assert!(!LIGHT_ELEVATION.e2.is_empty());
    assert!(!LIGHT_ELEVATION.e3.is_empty());
}

#[test]
fn dark_elevation_has_four_non_empty_tiers() {
    assert!(!DARK_ELEVATION.e0.is_empty());
    assert!(!DARK_ELEVATION.e1.is_empty());
    assert!(!DARK_ELEVATION.e2.is_empty());
    assert!(!DARK_ELEVATION.e3.is_empty());
}

#[test]
fn elevation_tiers_progress_in_strength() {
    // A weak signal: deeper tiers contain larger blur radii than shallower ones.
    fn longest_blur_px(spec: &str) -> u32 {
        spec.split(',')
            .filter_map(|part| {
                let mut iter = part.split_whitespace();
                iter.next();
                iter.next();
                let blur = iter.next()?;
                blur.trim_end_matches("px").parse::<u32>().ok()
            })
            .max()
            .unwrap_or(0)
    }

    assert!(longest_blur_px(LIGHT_ELEVATION.e1) >= longest_blur_px(LIGHT_ELEVATION.e0));
    assert!(longest_blur_px(LIGHT_ELEVATION.e2) >= longest_blur_px(LIGHT_ELEVATION.e1));
    assert!(longest_blur_px(LIGHT_ELEVATION.e3) >= longest_blur_px(LIGHT_ELEVATION.e2));
}

#[test]
fn dark_elevation_includes_inner_highlight() {
    assert!(DARK_ELEVATION.e1.contains("inset"));
    assert!(DARK_ELEVATION.e2.contains("inset"));
    assert!(DARK_ELEVATION.e3.contains("inset"));
}
