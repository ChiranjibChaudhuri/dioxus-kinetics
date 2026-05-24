use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn glass_layer_preview() -> Element {
    let levels = [
        (GlassLevel::Subtle, "Subtle"),
        (GlassLevel::Floating, "Floating"),
        (GlassLevel::Overlay, "Overlay"),
    ];
    let tones = [
        (GlassTone::Neutral, "Neutral"),
        (GlassTone::Info, "Info"),
        (GlassTone::Warning, "Warning"),
    ];

    // 9 tiles → 9 GlassSurface instances. On the WgpuWebGl2 tier each would
    // claim a WebGL context, exceeding webkit's per-page cap and forcing the
    // older ones to be dropped (they then render as dark blanks). The
    // design-token showcase here is the CSS path itself — the `data-glass-*`
    // attributes carrying the level + tone tokens — so force the CSS render
    // path regardless of tier detection.
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--3x3",
            for (level, level_label) in levels.iter() {
                for (tone, tone_label) in tones.iter() {
                    div { class: "gallery-variant-tile gallery-variant-tile--material",
                        span { class: "gallery-variant-label", "{level_label} · {tone_label}" }
                        GlassLayer {
                            level: *level,
                            tone: *tone,
                            density: GlassDensity::Comfortable,
                            force_css: true,
                            "Material preview"
                        }
                    }
                }
            }
        }
    }
}
