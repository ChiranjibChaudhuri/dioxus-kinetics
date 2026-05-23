//! With `liquid-glass` feature enabled, GlassSurface SSRs to a LiquidSurface
//! wrapper (which itself contains a div+canvas). Without the feature, it
//! renders the CSS section. SSR runs on native, so we get the LiquidSurface
//! placeholder shape (div + canvas with no actual wgpu init).

#![cfg(feature = "liquid-glass")]

use dioxus::prelude::*;
use ui_dioxus::GlassSurface;
use ui_glass::{GlassDensity, GlassLevel, GlassTone};

#[test]
fn glass_surface_renders_liquid_surface_when_feature_on() {
    let html = dioxus_ssr::render_element(rsx! {
        GlassSurface {
            level: GlassLevel::Floating,
            tone: GlassTone::Neutral,
            density: GlassDensity::Comfortable,
            "child content"
        }
    });
    // With the feature on AND native SSR (which is non-wasm32, so detect()
    // returns has_webgpu=true), the upgrade routes to LiquidSurface, which
    // renders the ui-liquid-surface wrapper.
    assert!(
        html.contains("class=\"ui-liquid-surface\""),
        "expected ui-liquid-surface wrapper, got: {html}",
    );
    assert!(html.contains("child content"));
}
