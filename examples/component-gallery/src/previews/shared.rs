use crate::demo_frame::FlipFrame;
use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn shared_layout_preview() -> Element {
    rsx! {
        FlipFrame {
            label: "Cross-tree layout swap",
            layout_a: rsx! {
                SharedLayout {
                    div { class: "gallery-variant-grid gallery-variant-grid--2col",
                        div { class: "gallery-variant-tile",
                            SharedElement { id: "card-left".to_string(),
                                p { "Card A" }
                            }
                        }
                        div { class: "gallery-variant-tile",
                            SharedElement { id: "card-right".to_string(),
                                p { "Card B" }
                            }
                        }
                    }
                }
            },
            layout_b: rsx! {
                SharedLayout {
                    div { class: "gallery-variant-grid gallery-variant-grid--2col",
                        div { class: "gallery-variant-tile",
                            SharedElement { id: "card-right".to_string(),
                                p { "Card B" }
                            }
                        }
                        div { class: "gallery-variant-tile",
                            SharedElement { id: "card-left".to_string(),
                                p { "Card A" }
                            }
                        }
                    }
                }
            },
        }
    }
}

pub fn shared_element_preview() -> Element {
    rsx! {
        FlipFrame {
            label: "Shared element FLIP",
            layout_a: rsx! {
                SharedLayout {
                    div { class: "gallery-variant-grid gallery-variant-grid--2col",
                        div { class: "gallery-variant-tile",
                            SharedElement { id: "demo-hero".to_string(),
                                p { "Hero position A" }
                            }
                        }
                        div { class: "gallery-variant-tile",
                            span { class: "gallery-variant-label", "Other slot" }
                        }
                    }
                }
            },
            layout_b: rsx! {
                SharedLayout {
                    div { class: "gallery-variant-grid gallery-variant-grid--2col",
                        div { class: "gallery-variant-tile",
                            span { class: "gallery-variant-label", "Other slot" }
                        }
                        div { class: "gallery-variant-tile",
                            SharedElement { id: "demo-hero".to_string(),
                                p { "Hero position B" }
                            }
                        }
                    }
                }
            },
        }
    }
}
