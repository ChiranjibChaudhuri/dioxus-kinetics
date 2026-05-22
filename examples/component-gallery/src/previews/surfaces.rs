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
    rsx! {
        MetricCard {
            label: "Net revenue",
            value: "$128.4k",
            delta: "+12.5%",
            tone: MetricTone::Success,
        }
    }
}
