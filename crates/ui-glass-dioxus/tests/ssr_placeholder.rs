//! SSR renders the LiquidSurface as a div + canvas pair without running any
//! wgpu init. The output HTML is what the browser hydrates against.

use dioxus::prelude::*;
use ui_glass::LiquidMaterial;
use ui_glass_dioxus::LiquidSurface;

#[test]
fn ssr_renders_div_and_canvas() {
    let html = dioxus_ssr::render_element(rsx! {
        LiquidSurface {
            material: LiquidMaterial::floating(),
            width: 320,
            height: 200,
        }
    });
    assert!(html.contains("class=\"ui-liquid-surface\""), "missing wrapper class, got: {html}");
    assert!(html.contains("<canvas"), "missing canvas element, got: {html}");
    assert!(html.contains("width=\"320\""), "canvas width should match prop, got: {html}");
    assert!(html.contains("height=\"200\""), "canvas height should match prop, got: {html}");
}

#[test]
fn ssr_renders_foreground_children() {
    let html = dioxus_ssr::render_element(rsx! {
        LiquidSurface {
            material: LiquidMaterial::floating(),
            width: 200,
            height: 100,
            "hello from inside"
        }
    });
    assert!(html.contains("hello from inside"), "child text should appear, got: {html}");
}
