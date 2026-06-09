//! Shared DOM-focus tail for the roving-focus components.
//!
//! The menu, tabs, select, and calendar grid each move DOM focus by id via a
//! `document::eval`-driven `element.focus()`. They build different id shapes
//! (`{menu_id}-item-{item_id}`, `tab-{value}`, `{grid_id}-day-{iso}`, …) but
//! share the same validate-then-eval tail, which lives here so the
//! char-allowlist guard ([`ui_core::roving::is_focus_id_safe`]) is applied in
//! exactly one place.

/// Move DOM focus to the element whose id is `dom_id`, but only when the id
/// passes the [`ui_core::roving::is_focus_id_safe`] allowlist (so it is safe to
/// interpolate into the `getElementById` JS literal). A non-safe id is a no-op.
pub(crate) fn focus_element_by_id(dom_id: &str) {
    if !ui_core::roving::is_focus_id_safe(dom_id) {
        return;
    }
    let _ = dioxus::document::eval(&format!(
        "const el = document.getElementById('{dom_id}'); if (el) el.focus();"
    ));
}
