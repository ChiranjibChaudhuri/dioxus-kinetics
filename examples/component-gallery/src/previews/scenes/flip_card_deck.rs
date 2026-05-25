use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn FlipCardDeckScene() -> Element {
    rsx! {
        SharedLayout {
            div { class: "scene-card-deck",
                for (i, label) in ["Concept", "Build", "Ship"].iter().enumerate() {
                    SharedElement { id: format!("card-{i}"),
                        div {
                            class: "scene-card",
                            "data-card-index": "{i}",
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}
