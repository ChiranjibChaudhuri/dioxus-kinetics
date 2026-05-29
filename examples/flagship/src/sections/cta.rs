use dioxus::prelude::*;

const REPO_URL: &str = "https://github.com/ChiranjibChaudhuri/dioxus-kinetics";
const GALLERY_HASH: &str = "#flagship-features";

#[component]
pub fn CallToAction() -> Element {
    // Scroll-reveal for the CTA band: an IntersectionObserver toggles
    // data-revealed="true" on each .flagship-reveal-card as the band
    // enters the viewport, with a per-element stagger (transition-delay
    // set inline). Scoped to [data-reveal-group="cta"] so it never
    // collides with the Metrics section's observer. Matching transition
    // CSS lives in styles.rs, gated under [data-ui-motion=reduced]; the
    // script also self-gates on reduced motion (reveals immediately, no
    // stagger). Registered after mount so the band is in the DOM.
    use_effect(|| {
        let _ = dioxus::document::eval(REVEAL_SCRIPT);
    });

    rsx! {
        section { class: "flagship-cta", aria_labelledby: "flagship-cta-heading",
            div { class: "flagship-cta-inner", "data-reveal-group": "cta",
                p { class: "flagship-eyebrow flagship-reveal-card", "data-reveal-index": "0",
                    "Start moving"
                }
                h2 {
                    id: "flagship-cta-heading",
                    class: "flagship-display-2 flagship-reveal-card",
                    "data-reveal-index": "1",
                    "Drop kinetics into your next Dioxus app."
                }
                p { class: "flagship-cta-caption flagship-reveal-card", "data-reveal-index": "2",
                    "Built in Rust. MIT licensed. Web, desktop, mobile, and native."
                }
                // Anchors styled as buttons — real navigation, keyboard-
                // activatable, no JS handler needed. The library's
                // .ui-button rules style any element that carries the
                // class, so anchors inherit the same hover/focus/press
                // visual contract that <button> would.
                div { class: "flagship-cta-actions flagship-reveal-card", "data-reveal-index": "3",
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

/// IntersectionObserver that reveals the CTA band's `.flagship-reveal-card`
/// elements as they enter the viewport. Scoped to
/// `[data-reveal-group="cta"]` so it observes only this section. Each
/// element's `data-reveal-index` drives a staggered `transition-delay`;
/// once revealed it is unobserved so the reveal fires once. Self-gates on
/// reduced motion: when the document prefers reduced motion the elements
/// are revealed immediately with no stagger (the CSS transition is also
/// suppressed under `[data-ui-motion=reduced]`, belt-and-suspenders).
const REVEAL_SCRIPT: &str = r#"
    const reduced =
        (window.matchMedia &&
         window.matchMedia("(prefers-reduced-motion: reduce)").matches) ||
        !!document.querySelector('[data-ui-motion="reduced"]');
    const cards = Array.from(
        document.querySelectorAll('[data-reveal-group="cta"] .flagship-reveal-card')
    );
    if (reduced || !("IntersectionObserver" in window)) {
        cards.forEach((c) => c.setAttribute("data-revealed", "true"));
    } else {
        const obs = new IntersectionObserver((entries) => {
            entries.forEach((entry) => {
                if (entry.isIntersecting) {
                    const el = entry.target;
                    const idx = parseInt(el.getAttribute("data-reveal-index") || "0", 10);
                    el.style.transitionDelay = (idx * 90) + "ms";
                    el.setAttribute("data-revealed", "true");
                    obs.unobserve(el);
                }
            });
        }, { threshold: 0.2 });
        cards.forEach((c) => obs.observe(c));
    }
"#;
