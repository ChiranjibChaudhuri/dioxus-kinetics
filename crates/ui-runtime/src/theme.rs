//! Theme + density context with reactive DOM/media-query sourcing.
//!
//! This is the theme/density counterpart to [`ReducedMotionProvider`](crate::ReducedMotionProvider):
//! the library already ships a runtime-reactive provider for the motion
//! preference axis, and `ThemeProvider` does the same for the other two
//! first-class preference axes the design tokens define — [`ThemeMode`] and
//! [`Density`].
//!
//! - [`use_theme_mode`] / [`use_density`] consume a context if one is provided
//!   (e.g. by [`ThemeProvider`]); otherwise they fall back to a one-shot DOM
//!   probe of `prefers-color-scheme` and the nearest `data-ui-theme` /
//!   `data-ui-density` attribute.
//! - [`ThemeProvider`] provides reactive `Signal`-backed contexts, renders the
//!   resolved `data-ui-theme` / `data-ui-density` attributes onto a wrapper so
//!   the shared CSS applies to the subtree, and stays live to OS color-scheme
//!   and attribute changes via the same `MediaQueryList` + `MutationObserver`
//!   pattern proven in [`reduced_motion`](crate::reduced_motion).

use dioxus::prelude::*;
use ui_tokens::{Density, ThemeMode};

/// Read the effective [`ThemeMode`]. Reactive when a [`ThemeProvider`] is
/// installed above this component; otherwise a one-shot root probe.
pub fn use_theme_mode() -> ThemeMode {
    try_consume_context::<Signal<ThemeMode>>()
        .map(|s| *s.read())
        .unwrap_or_else(detect_theme_mode_at_root)
}

/// Read the effective [`Density`]. Reactive when a [`ThemeProvider`] is
/// installed above this component; otherwise a one-shot root probe.
pub fn use_density() -> Density {
    try_consume_context::<Signal<Density>>()
        .map(|s| *s.read())
        .unwrap_or_else(detect_density_at_root)
}

#[cfg(target_arch = "wasm32")]
pub fn detect_theme_mode_at_root() -> ThemeMode {
    if let Some(mode) = root_attr_theme() {
        return mode;
    }
    if media_query_dark() {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn detect_theme_mode_at_root() -> ThemeMode {
    ThemeMode::Light
}

#[cfg(target_arch = "wasm32")]
pub fn detect_density_at_root() -> Density {
    root_attr_density().unwrap_or(Density::Comfortable)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn detect_density_at_root() -> Density {
    Density::Comfortable
}

#[cfg(target_arch = "wasm32")]
fn media_query_dark() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|m| m.matches())
        .unwrap_or(false)
}

#[cfg(target_arch = "wasm32")]
fn root_attr(name: &str) -> Option<String> {
    let window = web_sys::window()?;
    let document = window.document()?;
    // The document element and body are where apps conventionally set the
    // `data-ui-*` attributes; an explicit nearer override beats them via the
    // querySelector fallback.
    if let Some(el) = document.document_element() {
        if let Some(v) = el.get_attribute(name) {
            return Some(v);
        }
    }
    if let Some(body) = document.body() {
        if let Some(v) = body.get_attribute(name) {
            return Some(v);
        }
        if let Ok(Some(found)) = body.query_selector(&format!("[{name}]")) {
            if let Some(v) = found.get_attribute(name) {
                return Some(v);
            }
        }
    }
    None
}

#[cfg(target_arch = "wasm32")]
fn root_attr_theme() -> Option<ThemeMode> {
    root_attr("data-ui-theme").and_then(|v| ThemeMode::from_attr(&v))
}

#[cfg(target_arch = "wasm32")]
fn root_attr_density() -> Option<Density> {
    root_attr("data-ui-density").and_then(|v| Density::from_attr(&v))
}

/// Provides reactive [`ThemeMode`] and [`Density`] contexts to children.
///
/// When `mode` / `density` is `Some`, that value is used verbatim (a forced
/// override) and DOM/media-query changes for that axis are ignored. When it is
/// `None`, the value is sourced from `prefers-color-scheme` (theme only) and
/// the nearest `data-ui-theme` / `data-ui-density` attribute, and stays
/// reactive. The resolved values are rendered as `data-ui-theme` /
/// `data-ui-density` on a wrapper `div` so the shared CSS themes the subtree.
#[component]
pub fn ThemeProvider(
    mode: Option<ThemeMode>,
    density: Option<Density>,
    children: Element,
) -> Element {
    #[allow(unused_mut)]
    let mut detected_mode = use_signal(detect_theme_mode_at_root);
    #[allow(unused_mut)]
    let mut detected_density = use_signal(detect_density_at_root);

    let effective_mode = mode.unwrap_or_else(|| *detected_mode.read());
    let effective_density = density.unwrap_or_else(|| *detected_density.read());

    // Reactive contexts: provide signals once, then keep them in sync with the
    // effective values on every render so consumers re-render on change.
    let mut mode_ctx = use_context_provider(|| Signal::new(effective_mode));
    let mut density_ctx = use_context_provider(|| Signal::new(effective_density));
    use_effect(move || {
        if *mode_ctx.peek() != effective_mode {
            mode_ctx.set(effective_mode);
        }
        if *density_ctx.peek() != effective_density {
            density_ctx.set(effective_density);
        }
    });

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        use wasm_bindgen::prelude::Closure;
        use wasm_bindgen::JsCast;

        let Some(window) = web_sys::window() else {
            return;
        };

        // 1. MediaQueryList listener for prefers-color-scheme (theme only, and
        //    only when the theme is not a forced override).
        if mode.is_none() {
            if let Some(mql) = window
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
            {
                let mut signal = detected_mode;
                let closure = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
                    signal.set(detect_theme_mode_at_root());
                }) as Box<dyn FnMut(_)>);
                let _ = mql
                    .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
                closure.forget();
            }
        }

        // 2. MutationObserver on the document element for data-ui-theme /
        //    data-ui-density attribute changes (per non-overridden axis).
        if mode.is_some() && density.is_some() {
            return;
        }
        if let Some(document) = window.document() {
            if let Some(root) = document.document_element() {
                let mut mode_signal = detected_mode;
                let mut density_signal = detected_density;
                let track_mode = mode.is_none();
                let track_density = density.is_none();
                let cb = Closure::wrap(Box::new(
                    move |_records: js_sys::Array, _obs: web_sys::MutationObserver| {
                        if track_mode {
                            mode_signal.set(detect_theme_mode_at_root());
                        }
                        if track_density {
                            density_signal.set(detect_density_at_root());
                        }
                    },
                ) as Box<dyn FnMut(_, _)>);
                if let Ok(observer) = web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()) {
                    let init = js_sys::Object::new();
                    js_sys::Reflect::set(&init, &"attributes".into(), &true.into()).ok();
                    js_sys::Reflect::set(&init, &"subtree".into(), &true.into()).ok();
                    let filter = js_sys::Array::new();
                    filter.push(&"data-ui-theme".into());
                    filter.push(&"data-ui-density".into());
                    js_sys::Reflect::set(&init, &"attributeFilter".into(), &filter).ok();
                    let _ = observer.observe_with_options(&root, init.unchecked_ref());
                }
                cb.forget();
            }
        }
    });

    rsx! {
        div {
            "data-ui-theme": effective_mode.data_attr(),
            "data-ui-density": effective_density.data_attr(),
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_mode_attr_round_trips() {
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            assert_eq!(ThemeMode::from_attr(mode.data_attr()), Some(mode));
        }
        assert_eq!(ThemeMode::from_attr("nope"), None);
    }

    #[test]
    fn density_attr_round_trips() {
        for density in [Density::Compact, Density::Comfortable, Density::Spacious] {
            assert_eq!(Density::from_attr(density.data_attr()), Some(density));
        }
        assert_eq!(Density::from_attr("nope"), None);
    }

    #[test]
    fn native_detection_defaults_are_sane() {
        // Off the web there is no DOM to probe; the providers fall back to the
        // light/comfortable defaults.
        assert_eq!(detect_theme_mode_at_root(), ThemeMode::Light);
        assert_eq!(detect_density_at_root(), Density::Comfortable);
    }
}
