use dioxus::prelude::*;
use ui_icons::*;

fn render(component: Element) -> String {
    dioxus_ssr::render_element(component)
}

#[test]
fn each_icon_renders_an_svg_with_viewbox_and_aria_hidden() {
    for html in [
        render(rsx! { Close { size: 24 } }),
        render(rsx! { Check { size: 24 } }),
        render(rsx! { ChevronDown { size: 24 } }),
        render(rsx! { ChevronRight { size: 24 } }),
        render(rsx! { Plus { size: 24 } }),
        render(rsx! { Minus { size: 24 } }),
        render(rsx! { Trash { size: 24 } }),
        render(rsx! { Search { size: 24 } }),
    ] {
        assert!(html.contains("<svg"), "expected svg element in {html}");
        assert!(
            html.contains("viewBox=\"0 0 24 24\""),
            "viewBox missing in {html}"
        );
        assert!(
            html.contains("aria-hidden=\"true\""),
            "aria-hidden missing in {html}"
        );
    }
}

#[test]
fn size_prop_controls_width_and_height() {
    let html = render(rsx! { Plus { size: 12 } });
    assert!(html.contains("width=\"12\""), "width missing: {html}");
    assert!(html.contains("height=\"12\""), "height missing: {html}");
}

#[test]
fn all_icons_export_non_empty_path_constants() {
    for path in [
        CLOSE_PATH_D,
        CHECK_PATH_D,
        CHEVRON_DOWN_PATH_D,
        CHEVRON_RIGHT_PATH_D,
        PLUS_PATH_D,
        MINUS_PATH_D,
        TRASH_PATH_D,
        SEARCH_PATH_D,
    ] {
        assert!(!path.is_empty());
    }
}
