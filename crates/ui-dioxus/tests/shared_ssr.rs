use dioxus::prelude::*;
use ui_dioxus::SharedLayout;

#[test]
fn shared_layout_renders_wrapper_with_class() {
    let html = dioxus_ssr::render_element(rsx! {
        SharedLayout {
            p { "inner" }
        }
    });
    assert!(html.contains("class=\"ui-shared-layout\""), "got {html}");
    assert!(html.contains("inner"));
}
