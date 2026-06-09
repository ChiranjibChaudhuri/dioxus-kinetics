//! Roving-focus / keyboard-list primitives.
//!
//! Pure, dependency-free helpers shared by the components that implement a
//! WAI-ARIA roving-focus list (menu, tabs, select, combobox, calendar grid).
//! These were independently re-implemented in five components; centralising
//! them here keeps the wrapping/skip semantics identical everywhere.
//!
//! All functions are generic over a `is_focusable(index) -> bool` closure so
//! callers can drive them straight off their own items / visibility slices.

/// First index `< len` whose `is_focusable(index)` is true, or `None` when no
/// index is focusable.
pub fn first_focusable(len: usize, is_focusable: impl Fn(usize) -> bool) -> Option<usize> {
    (0..len).find(|&i| is_focusable(i))
}

/// Last index `< len` whose `is_focusable(index)` is true, or `None` when no
/// index is focusable.
pub fn last_focusable(len: usize, is_focusable: impl Fn(usize) -> bool) -> Option<usize> {
    (0..len).rev().find(|&i| is_focusable(i))
}

/// Step focus by `delta` from `from`, wrapping with `rem_euclid` and skipping
/// non-focusable indices. Returns `None` when nothing is focusable.
///
/// Loops up to `len` times stepping by `delta`; for `delta = ±1` this is
/// identical to the `((i + delta) % len + len) % len` idiom.
pub fn step_focusable(
    len: usize,
    from: usize,
    delta: i32,
    is_focusable: impl Fn(usize) -> bool,
) -> Option<usize> {
    if len == 0 || !(0..len).any(&is_focusable) {
        return None;
    }
    let len_i = len as i32;
    let mut idx = from as i32;
    for _ in 0..len {
        idx = (idx + delta).rem_euclid(len_i);
        if is_focusable(idx as usize) {
            return Some(idx as usize);
        }
    }
    None
}

/// Typeahead: first focusable index whose lowercased `label(index)` starts
/// with the lowercased `buffer`. An empty buffer yields `None` so callers can
/// keep the current active index.
pub fn typeahead_index(
    len: usize,
    buffer: &str,
    is_focusable: impl Fn(usize) -> bool,
    label: impl Fn(usize) -> String,
) -> Option<usize> {
    if buffer.is_empty() {
        return None;
    }
    let needle = buffer.to_lowercase();
    (0..len).find(|&i| is_focusable(i) && label(i).to_lowercase().starts_with(&needle))
}

/// Guard for interpolating an element id into a `getElementById` JS literal:
/// only ASCII alphanumerics and the id-safe punctuation `- _ : .` are allowed.
pub fn is_focus_id_safe(id: &str) -> bool {
    id.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ':' || c == '.')
}

#[cfg(test)]
mod tests {
    use super::*;

    // Focusable mask: index 1 and 3 are disabled.
    fn mask() -> Vec<bool> {
        vec![true, false, true, false, true]
    }

    #[test]
    fn first_and_last_skip_unfocusable_edges() {
        let m = [false, true, true, false];
        assert_eq!(first_focusable(m.len(), |i| m[i]), Some(1));
        assert_eq!(last_focusable(m.len(), |i| m[i]), Some(2));
    }

    #[test]
    fn first_and_last_none_when_empty_or_all_unfocusable() {
        assert_eq!(first_focusable(0, |_| true), None);
        let all = [false, false];
        assert_eq!(first_focusable(all.len(), |i| all[i]), None);
        assert_eq!(last_focusable(all.len(), |i| all[i]), None);
    }

    #[test]
    fn step_forward_skips_and_wraps() {
        let m = mask();
        // 0 → 2 (skip disabled 1).
        assert_eq!(step_focusable(m.len(), 0, 1, |i| m[i]), Some(2));
        // 2 → 4 (skip disabled 3).
        assert_eq!(step_focusable(m.len(), 2, 1, |i| m[i]), Some(4));
        // 4 → wraps to 0.
        assert_eq!(step_focusable(m.len(), 4, 1, |i| m[i]), Some(0));
    }

    #[test]
    fn step_backward_wraps() {
        let m = mask();
        // 0 → wraps backward to 4.
        assert_eq!(step_focusable(m.len(), 0, -1, |i| m[i]), Some(4));
    }

    #[test]
    fn step_matches_simple_modulo_idiom_for_unit_delta() {
        // With everything focusable, step_focusable must equal the
        // navigation `((i + delta) % len + len) % len` idiom for delta ±1.
        let len = 4usize;
        for from in 0..len {
            for &delta in &[1i32, -1] {
                let expected =
                    (((from as i32 + delta) % len as i32 + len as i32) % len as i32) as usize;
                assert_eq!(
                    step_focusable(len, from, delta, |_| true),
                    Some(expected),
                    "from={from} delta={delta}"
                );
            }
        }
    }

    #[test]
    fn step_none_when_all_unfocusable_or_empty() {
        let none = [false, false];
        assert_eq!(step_focusable(none.len(), 0, 1, |i| none[i]), None);
        assert_eq!(step_focusable(0, 0, 1, |_| true), None);
    }

    #[test]
    fn typeahead_matches_focusable_prefix_case_insensitive() {
        let labels = ["Rename", "", "Duplicate", "Delete", "Move"];
        let foc = mask();
        let f = |i: usize| foc[i];
        let l = |i: usize| labels[i].to_string();
        assert_eq!(typeahead_index(labels.len(), "du", f, l), Some(2));
        let l = |i: usize| labels[i].to_string();
        assert_eq!(typeahead_index(labels.len(), "MO", f, l), Some(4));
        // "Delete" is at a disabled index, so "de" matches nothing.
        let l = |i: usize| labels[i].to_string();
        assert_eq!(typeahead_index(labels.len(), "de", f, l), None);
        // Empty buffer never matches.
        let l = |i: usize| labels[i].to_string();
        assert_eq!(typeahead_index(labels.len(), "", f, l), None);
    }

    #[test]
    fn focus_id_safety_allowlist() {
        assert!(is_focus_id_safe("menu-1-item-rename"));
        assert!(is_focus_id_safe("grid_id:tab.0"));
        assert!(is_focus_id_safe("2026-05-23"));
        assert!(!is_focus_id_safe("bad id"));
        assert!(!is_focus_id_safe("inject');//"));
        assert!(!is_focus_id_safe("emoji😀"));
    }
}
