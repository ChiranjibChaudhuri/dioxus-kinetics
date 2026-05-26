use dioxus::prelude::*;

const REPO_URL: &str = "https://github.com/ChiranjibChaudhuri/dioxus-kinetics";
const GALLERY_HASH: &str = "#flagship-features";

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
                // Anchors styled as buttons — real navigation, keyboard-
                // activatable, no JS handler needed. The library's
                // .ui-button rules style any element that carries the
                // class, so anchors inherit the same hover/focus/press
                // visual contract that <button> would.
                div { class: "flagship-cta-actions",
                    a {
                        class: "ui-button ui-button--primary",
                        href: REPO_URL,
                        rel: "noopener",
                        "View on GitHub"
                    }
                    a {
                        class: "ui-button ui-button--ghost",
                        href: GALLERY_HASH,
                        "Browse the components"
                    }
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
