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
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
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

/// Provides a `ReducedMotion` context to children. If `reduced` is set,
/// the context uses that value verbatim and ignores DOM/media-query
/// changes; otherwise the value is sourced from `prefers-reduced-motion`
/// + the nearest `[data-ui-motion="reduced"]` attribute and stays
/// reactive to media-query and attribute changes.
#[component]
pub fn ReducedMotionProvider(reduced: Option<bool>, children: Element) -> Element {
    #[allow(unused_mut)]
    let mut detected = use_signal(detect_reduced_motion_at_root);
    let effective = reduced.unwrap_or_else(|| *detected.read());
    use_context_provider(|| ReducedMotion(effective));

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        if reduced.is_some() {
            // Forced override; ignore DOM / media-query changes.
            return;
        }

        use wasm_bindgen::prelude::Closure;
        use wasm_bindgen::JsCast;

        let Some(window) = web_sys::window() else {
            return;
        };

        // 1. MediaQueryList listener for prefers-reduced-motion changes.
        let mql = window
            .match_media("(prefers-reduced-motion: reduce)")
            .ok()
            .flatten();
        if let Some(mql) = mql {
            let mut signal = detected;
            let closure = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
                signal.set(detect_reduced_motion_at_root());
            }) as Box<dyn FnMut(_)>);
            let _ =
                mql.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
            closure.forget();
        }

        // 2. MutationObserver on body for data-ui-motion attribute changes.
        if let Some(document) = window.document() {
            if let Some(body) = document.body() {
                let mut signal = detected;
                let cb = Closure::wrap(Box::new(
                    move |_records: js_sys::Array, _obs: web_sys::MutationObserver| {
                        signal.set(detect_reduced_motion_at_root());
                    },
                ) as Box<dyn FnMut(_, _)>);
                if let Ok(observer) = web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()) {
                    let init = js_sys::Object::new();
                    js_sys::Reflect::set(&init, &"attributes".into(), &true.into()).ok();
                    js_sys::Reflect::set(&init, &"subtree".into(), &true.into()).ok();
                    let filter = js_sys::Array::new();
                    filter.push(&"data-ui-motion".into());
                    js_sys::Reflect::set(&init, &"attributeFilter".into(), &filter).ok();
                    let _ = observer.observe_with_options(&body, init.unchecked_ref());
                }
                cb.forget();
            }
        }
    });

    rsx! { {children} }
}
