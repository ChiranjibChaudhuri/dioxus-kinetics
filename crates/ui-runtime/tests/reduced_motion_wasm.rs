#![cfg(target_arch = "wasm32")]

use ui_runtime::detect_reduced_motion_at_root;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn detects_data_ui_motion_reduced_on_body() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    body.set_attribute("data-ui-motion", "reduced").unwrap();

    assert!(detect_reduced_motion_at_root());

    body.remove_attribute("data-ui-motion").unwrap();
}

#[wasm_bindgen_test]
fn returns_false_when_no_signal_present() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    body.remove_attribute("data-ui-motion").ok();

    // prefers-reduced-motion is environment-dependent. Test only the
    // data-attr path here.
    assert!(!detect_reduced_motion_at_root() || media_query_says_reduce());
}

fn media_query_says_reduce() -> bool {
    web_sys::window()
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .map(|m| m.matches())
        .unwrap_or(false)
}
