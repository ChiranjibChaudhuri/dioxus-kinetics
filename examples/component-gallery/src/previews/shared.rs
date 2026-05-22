use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn shared_layout_preview() -> Element {
    rsx! {
        SharedLayout {
            div { class: "gallery-variant-grid gallery-variant-grid--2col",
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "Left" }
                    SharedElement { id: "card-left".to_string(),
                        p { "Same identity across renders." }
                    }
                }
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "Right" }
                    SharedElement { id: "card-right".to_string(),
                        p { "Independent identity." }
                    }
                }
            }
        }
    }
}

pub fn shared_element_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--2col",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Identity" }
                SharedElement { id: "demo-hero".to_string(),
                    p { "data-shared-id attribute carries the identity." }
                }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Within layout" }
                SharedLayout {
                    SharedElement { id: "scoped".to_string(),
                        p { "Scoped to its SharedLayout ancestor." }
                    }
                }
            }
        }
    }
}
