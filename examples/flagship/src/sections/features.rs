use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn Features() -> Element {
    rsx! {
        section { class: "flagship-features", aria_labelledby: "flagship-features-heading",
            div { class: "flagship-features-inner",
                p { class: "flagship-eyebrow", "Three pillars" }
                h2 { id: "flagship-features-heading", class: "flagship-display-2",
                    "Glass. Scenes. Render."
                }
                div { class: "flagship-features-grid",
                    GlassSurface {
                        level: GlassLevel::Floating,
                        tone: GlassTone::Info,
                        density: GlassDensity::Comfortable,
                        h3 { class: "flagship-card-title", "Liquid glass. Honestly rendered." }
                        p { class: "flagship-card-body",
                            "WebGPU when it's available. SVG filter fallback. Solid fallback when accessibility says so."
                        }
                    }
                    GlassSurface {
                        level: GlassLevel::Floating,
                        tone: GlassTone::Primary,
                        density: GlassDensity::Comfortable,
                        h3 { class: "flagship-card-title", "One clock. Every runtime." }
                        p { class: "flagship-card-body",
                            "Scene owns the time. Clip, SplitText, MotionPath, presence, and shared-element layout all read from it."
                        }
                    }
                    GlassSurface {
                        level: GlassLevel::Floating,
                        tone: GlassTone::Success,
                        density: GlassDensity::Comfortable,
                        h3 { class: "flagship-card-title", "Frame-perfect render." }
                        p { class: "flagship-card-body",
                            "kinetics render walks any scene with SceneDriver::Manual, writes per-frame HTML, ships a manifest, and optionally encodes PNG or MP4."
                        }
                    }
                }
            }
        }
    }
}
