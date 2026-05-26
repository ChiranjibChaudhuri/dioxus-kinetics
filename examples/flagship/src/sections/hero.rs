use component_gallery::previews::scenes::product_intro::ProductIntroScene;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "flagship-hero", aria_label: "Kinetics product introduction",
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
