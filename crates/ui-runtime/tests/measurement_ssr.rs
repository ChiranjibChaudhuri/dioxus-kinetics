use dioxus::prelude::*;
use ui_runtime::use_element_rect;

#[component]
fn RectProbe() -> Element {
    let (callback, rect) = use_element_rect();
    let rect_str = match rect() {
        Some(r) => format!("{}x{}", r.width, r.height),
        None => "none".to_string(),
    };
    rsx! {
        div {
            onmounted: move |evt| callback.0.call(evt),
            "data-rect": "{rect_str}",
        }
    }
}

#[test]
fn element_rect_in_ssr_returns_none() {
    let html = dioxus_ssr::render_element(rsx! { RectProbe {} });
    assert!(html.contains("data-rect=\"none\""), "got {html}");
}
