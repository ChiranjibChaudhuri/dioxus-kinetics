use component_gallery::previews::scenes::product_intro::ProductIntroScene;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "flagship-hero", aria_labelledby: "flagship-hero-heading",
            // The Hero's visible title is rendered inside ProductIntroScene
            // via KineticText, which emits a span rather than a semantic
            // heading. Screen-reader users still need a real h1 to anchor
            // the page outline (and to satisfy the spec's heading-hierarchy
            // requirement). The sr-only utility hides it visually while
            // keeping it in the accessibility tree.
            h1 {
                id: "flagship-hero-heading",
                class: "flagship-sr-only",
                "Kinetics — composable motion for Rust apps"
            }
            div { class: "flagship-hero-stage",
                // Freeze the scene at the cinematic peak (t=2200ms) so the
                // hero presents a curated still — title and body together,
                // pre-flip-cards — rather than racing autoplay against
                // first paint. The title clip's window is [0, 2400) and
                // the body clip's is [800, 3200); 2200ms sits inside both
                // with the body fully faded in. The transport stays
                // hidden.
                ProductIntroScene {
                    controls: false,
                    autoplay: false,
                    initial_elapsed_ms: 2_200.0,
                }
            }
        }
    }
}
