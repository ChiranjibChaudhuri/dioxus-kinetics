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

pub fn badge_preview() -> Element {
    let badges = [
        (BadgeTone::Neutral, "Neutral", "Draft"),
        (BadgeTone::Primary, "Primary", "New"),
        (BadgeTone::Success, "Success", "Active"),
        (BadgeTone::Warning, "Warning", "Degraded"),
        (BadgeTone::Danger, "Danger", "Down"),
        (BadgeTone::Info, "Info", "Beta"),
    ];
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            for (tone, label, text) in badges {
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "{label}" }
                    Badge { tone, "{text}" }
                }
            }
        }
    }
}

// A stable, self-contained avatar image so the SSR audit snapshot never
// reaches the network: a tiny inline SVG portrait encoded as a data URI.
const AVATAR_DATA_URI: &str = "data:image/svg+xml;utf8,\
<svg xmlns='http://www.w3.org/2000/svg' width='96' height='96' viewBox='0 0 96 96'>\
<rect width='96' height='96' fill='%234f46e5'/>\
<circle cx='48' cy='38' r='18' fill='%23e0e7ff'/>\
<path d='M16 88c0-19 14-30 32-30s32 11 32 30z' fill='%23e0e7ff'/>\
</svg>";

pub fn avatar_preview() -> Element {
    let sizes = [
        (AvatarSize::Sm, "Small · sm"),
        (AvatarSize::Md, "Medium · md"),
        (AvatarSize::Lg, "Large · lg"),
    ];
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            for (size, label) in sizes {
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "{label}" }
                    div { style: "display: flex; align-items: center; gap: 16px;",
                        Avatar { name: "Ada Lovelace", size }
                        Avatar { name: "Ada Lovelace", src: AVATAR_DATA_URI, size }
                    }
                }
            }
        }
    }
}
