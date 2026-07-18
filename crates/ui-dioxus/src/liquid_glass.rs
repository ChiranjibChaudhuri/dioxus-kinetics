//! `LiquidGlass` — an Apple "Liquid Glass" surface rendered with a
//! tier-consistent CSS recipe so the look holds in the browser (where the
//! WebGPU engine can also engage via `GlassSurface`), in SSR, and in
//! frame-by-frame video capture. The matching WGSL recipe lives in
//! [`ui_glass::LiquidMaterial::apple_liquid`].

use dioxus::prelude::*;
use ui_glass::GlassTone;

/// A thick, edge-lit glass surface. Renders a layered backdrop-filter +
/// specular + inner-shadow treatment that mimics Apple's Liquid Glass on
/// any tier (the `apple_liquid` WGSL recipe is the engine-side twin).
/// Children sit above the lens overlay.
#[component]
pub fn LiquidGlass(#[props(default)] tone: GlassTone, children: Element) -> Element {
    rsx! {
        div {
            class: "ui-liquid-glass ui-liquid-glass--{tone_name(tone)}",
            "data-glass-tone": "{tone_name(tone)}",
            {children}
        }
    }
}

fn tone_name(tone: GlassTone) -> &'static str {
    match tone {
        GlassTone::Neutral => "neutral",
        GlassTone::Primary => "primary",
        GlassTone::Success => "success",
        GlassTone::Warning => "warning",
        GlassTone::Danger => "danger",
        GlassTone::Info => "info",
    }
}
