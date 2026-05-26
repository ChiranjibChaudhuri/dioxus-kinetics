use component_gallery::previews::scenes::product_intro::ProductIntroScene;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "flagship-hero", aria_label: "Kinetics product introduction",
            div { class: "flagship-hero-stage",
                ProductIntroScene { controls: false }
            }
        }
    }
}
