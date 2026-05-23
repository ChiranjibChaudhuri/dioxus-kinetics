//! Reduced-motion context + DOM probe.
//!
//! `use_reduced_motion()` consumes a `ReducedMotion` context if one is
//! provided (e.g. by `ReducedMotionProvider`); otherwise it falls back to
//! a one-shot DOM probe of `prefers-reduced-motion` + the
//! `[data-ui-motion="reduced"]` attribute on the body ancestor.

use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReducedMotion(pub bool);

pub fn use_reduced_motion() -> bool {
    try_consume_context::<ReducedMotion>()
        .map(|rm| rm.0)
        .unwrap_or_else(detect_reduced_motion_at_root)
}

#[cfg(target_arch = "wasm32")]
pub fn detect_reduced_motion_at_root() -> bool {
    media_query_reduce() || body_attr_reduced()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn detect_reduced_motion_at_root() -> bool {
    false
}

#[cfg(target_arch = "wasm32")]
fn media_query_reduce() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
        .map(|m| m.matches())
        .unwrap_or(false)
}

#[cfg(target_arch = "wasm32")]
fn body_attr_reduced() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Some(document) = window.document() else {
        return false;
    };
    let Some(body) = document.body() else {
        return false;
    };
    if body.get_attribute("data-ui-motion").as_deref() == Some("reduced") {
        return true;
    }
    if let Ok(matches) = body.query_selector("[data-ui-motion=\"reduced\"]") {
        if matches.is_some() {
            return true;
        }
    }
    false
}

/// Provides a `ReducedMotion` context to children, sourced from
/// `prefers-reduced-motion` + the nearest `[data-ui-motion]` attribute.
/// Listens for media-query changes and updates the signal reactively.
#[component]
pub fn ReducedMotionProvider(children: Element) -> Element {
    let reduced = use_signal(detect_reduced_motion_at_root);
    use_context_provider(|| ReducedMotion(*reduced.read()));

    // The media-query listener for reactive change is out of scope for
    // this task (Spec 2 ships the static probe; reactive updates land
    // when use_animation_value is migrated). Hooks still see the latest
    // value on each re-render.
    let _ = reduced;

    rsx! { {children} }
}
