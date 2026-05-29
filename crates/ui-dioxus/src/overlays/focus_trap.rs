//! Shared focus-trap + opener-restoration helpers for stacked overlays.
//!
//! `install_trap(panel_id)` generalizes the per-component traps that
//! `dialog.rs` / `sheet.rs` used to query the single global panel
//! (`.ui-dialog-panel` / `.ui-sheet`). By keying off the panel's element
//! `id` instead, each surface in a stack (dialog over command menu over
//! popover) traps Tab focus inside *its own* panel without colliding with
//! the others.
//!
//! `capture_opener()` records `document.activeElement` on `window` under a
//! per-id key the moment an overlay opens; `restore_opener()` focuses it
//! back when the overlay dismisses, implementing
//! `FocusPolicy::RestoreOnClose`. The handlers are registered on the panel
//! element, so when the panel leaves the DOM the listener is
//! garbage-collected with it — no Rust-side teardown is needed.

/// Elements that should participate in the Tab cycle.
const FOCUSABLE_SELECTOR: &str =
    "button:not([disabled]),[href],input:not([disabled]),select:not([disabled]),textarea:not([disabled]),[tabindex]:not([tabindex=\"-1\"])";

/// Installs a Tab-cycling focus trap on the element whose `id` is
/// `panel_id`. Idempotent per element (guarded by a `__kineticsTrap`
/// marker), so re-mounts / re-renders never double-bind. The listener is
/// attached to the panel element itself, which means it is collected
/// automatically when the panel is removed from the DOM.
pub fn install_trap(panel_id: &str) {
    let script = build_trap_script(panel_id);
    let _ = dioxus::document::eval(&script);
}

/// Pure builder for the focus-trap script, extracted so the id
/// interpolation and selector wiring can be unit-tested without a DOM.
fn build_trap_script(panel_id: &str) -> String {
    format!(
        r#"
        (function() {{
            const panel = document.getElementById('{panel_id}');
            if (!panel || panel.__kineticsTrap) return;
            panel.__kineticsTrap = true;
            panel.addEventListener('keydown', (e) => {{
                if (e.key !== 'Tab') return;
                const f = panel.querySelectorAll('{selector}');
                if (f.length === 0) {{ e.preventDefault(); panel.focus(); return; }}
                const first = f[0];
                const last = f[f.length - 1];
                const active = document.activeElement;
                if (e.shiftKey && (active === first || active === panel)) {{
                    e.preventDefault();
                    last.focus();
                }} else if (!e.shiftKey && active === last) {{
                    e.preventDefault();
                    first.focus();
                }}
            }});
        }})();
        "#,
        panel_id = panel_id,
        selector = FOCUSABLE_SELECTOR,
    )
}

/// Stores the currently-focused element (the overlay's opener) on
/// `window` under a key derived from `panel_id`, so it can be restored on
/// dismiss. Call this when an overlay opens (e.g. on the panel's
/// `onmounted`).
pub fn capture_opener(panel_id: &str) {
    let script = build_capture_script(panel_id);
    let _ = dioxus::document::eval(&script);
}

fn build_capture_script(panel_id: &str) -> String {
    format!(
        r#"
        (function() {{
            window.__kineticsOpeners = window.__kineticsOpeners || {{}};
            window.__kineticsOpeners['{key}'] = document.activeElement;
        }})();
        "#,
        key = opener_key(panel_id),
    )
}

/// Focuses the element previously recorded by [`capture_opener`] for the
/// same `panel_id`, then clears the stored reference. Safe to call even if
/// nothing was captured (it no-ops). Call this when the overlay dismisses.
pub fn restore_opener(panel_id: &str) {
    let script = build_restore_script(panel_id);
    let _ = dioxus::document::eval(&script);
}

fn build_restore_script(panel_id: &str) -> String {
    format!(
        r#"
        (function() {{
            const map = window.__kineticsOpeners;
            if (!map) return;
            const el = map['{key}'];
            delete map['{key}'];
            if (el && typeof el.focus === 'function' && document.contains(el)) {{
                el.focus();
            }}
        }})();
        "#,
        key = opener_key(panel_id),
    )
}

/// The `window.__kineticsOpeners` map key for a given panel id.
fn opener_key(panel_id: &str) -> String {
    format!("opener:{panel_id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trap_script_targets_panel_by_id() {
        let script = build_trap_script("ui-dialog-panel");
        assert!(script.contains("document.getElementById('ui-dialog-panel')"));
        // The focusable selector must be interpolated verbatim.
        assert!(script.contains(FOCUSABLE_SELECTOR));
        // Guarded against double-binding.
        assert!(script.contains("panel.__kineticsTrap"));
    }

    #[test]
    fn trap_script_uses_distinct_ids_for_stacked_overlays() {
        let dialog = build_trap_script("ui-dialog-panel");
        let menu = build_trap_script("ui-command-menu-panel");
        assert!(dialog.contains("'ui-dialog-panel'"));
        assert!(menu.contains("'ui-command-menu-panel'"));
        assert_ne!(dialog, menu);
    }

    #[test]
    fn opener_key_is_namespaced_per_panel() {
        assert_eq!(opener_key("ui-dialog-panel"), "opener:ui-dialog-panel");
        assert_ne!(opener_key("a"), opener_key("b"));
    }

    #[test]
    fn capture_and_restore_share_the_same_key() {
        let cap = build_capture_script("ui-command-menu-panel");
        let res = build_restore_script("ui-command-menu-panel");
        let key = opener_key("ui-command-menu-panel");
        assert!(cap.contains(&key));
        assert!(res.contains(&key));
    }

    #[test]
    fn restore_script_guards_against_missing_opener() {
        let res = build_restore_script("ui-dialog-panel");
        assert!(res.contains("if (!map) return;"));
        assert!(res.contains("document.contains(el)"));
    }
}
