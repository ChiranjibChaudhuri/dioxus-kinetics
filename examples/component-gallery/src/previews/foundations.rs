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

pub fn heading_preview() -> Element {
    // Walk the document outline 1..4 so the type ramp is visible top-to-bottom.
    // Each row keeps its semantic level (h1..h4) while the default variant
    // derives from that level, so the outline and the visual scale agree.
    let headings = [
        (1u8, "Level 1 · h1 → Title1", "Quarterly performance"),
        (2u8, "Level 2 · h2 → Title2", "Revenue by region"),
        (3u8, "Level 3 · h3 → Title3", "North America"),
        (4u8, "Level 4 · h4 → Body", "Enterprise accounts"),
    ];
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            for (level, label, copy) in headings {
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "{label}" }
                    Heading { level, "{copy}" }
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Level 2 · variant override → Display" }
                Heading { level: 2, variant: TextVariant::Display, "Display override" }
            }
        }
    }
}

pub fn text_preview() -> Element {
    // Surface several steps of the shared type scale, largest to smallest, so
    // the optical ramp reads clearly. `as_element` is set per tile to show the
    // tag allowlist (div/span/p) alongside the variant.
    let steps = [
        (
            TextVariant::Display,
            "Display",
            "div",
            "The optical top of the scale.",
        ),
        (
            TextVariant::Title1,
            "Title1",
            "div",
            "Primary section heading weight.",
        ),
        (
            TextVariant::Headline,
            "Headline",
            "span",
            "Emphasised inline lead-in.",
        ),
        (
            TextVariant::Body,
            "Body",
            "p",
            "Default reading size for paragraphs and prose.",
        ),
        (
            TextVariant::Footnote,
            "Footnote",
            "p",
            "Secondary supporting detail.",
        ),
        (
            TextVariant::Caption,
            "Caption",
            "span",
            "Smallest legible annotation.",
        ),
    ];
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            for (variant, label, tag, copy) in steps {
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "{label}" }
                    Text { variant, as_element: tag.to_string(), "{copy}" }
                }
            }
        }
    }
}
