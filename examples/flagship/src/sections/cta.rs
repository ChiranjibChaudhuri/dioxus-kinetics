use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn CallToAction() -> Element {
    rsx! {
        section { class: "flagship-cta", aria_labelledby: "flagship-cta-heading",
            div { class: "flagship-cta-inner",
                p { class: "flagship-eyebrow", "Start moving" }
                h2 { id: "flagship-cta-heading", class: "flagship-display-2",
                    "Drop kinetics into your next Dioxus app."
                }
                p { class: "flagship-cta-caption",
                    "Built in Rust. MIT licensed. Web, desktop, mobile, and native."
                }
                div { class: "flagship-cta-actions",
                    Button { variant: ButtonVariant::Primary, "View on GitHub" }
                    Button { variant: ButtonVariant::Ghost, "Open the gallery" }
                }
            }
            footer { class: "flagship-footer",
                p { class: "flagship-footer-brand", "dioxus-kinetics" }
                p { class: "flagship-footer-meta",
                    "MIT · v"
                    "{env!(\"CARGO_PKG_VERSION\")}"
                }
            }
        }
    }
}
