use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn surface_preview() -> Element {
    rsx! {
        Surface {
            h4 { "Pipeline health" }
            p { "12 workflows running" }
        }
    }
}

pub fn glass_surface_preview() -> Element {
    // A single instance on purpose — under the WgpuWebGl2 tier (webkit), each
    // GlassSurface consumes one WebGL context, and webkit warns past ~3 on a
    // single page. The full level/tone/density matrix is showcased by the
    // GlassLayer preview in foundations.rs (3×3 grid). Here we demonstrate
    // GlassSurface's semantic role: a high-level container with theme- and
    // tier-aware backdrop.
    rsx! {
        GlassSurface {
            level: GlassLevel::Floating,
            tone: GlassTone::Neutral,
            density: GlassDensity::Comfortable,
            h4 { "Revenue operations" }
            p { "Forecast updated 4 minutes ago" }
        }
    }
}

pub fn metric_card_preview() -> Element {
    let cards = [
        (MetricTone::Neutral, "Active users", "12 480", ""),
        (MetricTone::Success, "Net revenue", "$128.4k", "+12.5%"),
        (MetricTone::Warning, "Quota used", "92%", "+4.1%"),
        (MetricTone::Danger, "Failed jobs", "37", "+18"),
    ];
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--2col",
            for (tone, label, value, delta) in cards {
                MetricCard { tone, label, value, delta }
            }
        }
    }
}
