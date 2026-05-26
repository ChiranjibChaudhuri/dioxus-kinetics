use dioxus::prelude::*;
use component_gallery::previews::scenes::product_intro::ProductIntroScene;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "flagship-hero", aria_label: "Kinetics product introduction",
            div { class: "flagship-hero-stage",
                ProductIntroScene {}
            }
        }
    }
}
