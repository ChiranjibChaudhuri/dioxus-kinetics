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

    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--3x3",
            for (level, level_label) in levels.iter() {
                for (tone, tone_label) in tones.iter() {
                    div { class: "gallery-variant-tile",
                        span { class: "gallery-variant-label", "{level_label} · {tone_label}" }
                        GlassLayer {
                            level: *level,
                            tone: *tone,
                            density: GlassDensity::Comfortable,
                            "Material preview"
                        }
                    }
                }
            }
        }
    }
}
