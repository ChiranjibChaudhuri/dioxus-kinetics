use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn Metrics() -> Element {
    // Scroll-reveal: an IntersectionObserver toggles data-revealed="true"
    // on each metric card as it scrolls into view, with a per-card stagger
    // (transition-delay set inline). The matching transition CSS in
    // styles.rs is gated under [data-ui-motion=reduced] so reduced-motion
    // users see the cards already settled (no transform/opacity ramp). The
    // observer is registered after mount so the grid is in the DOM. The
    // query is scoped to this section's [data-reveal-group="metrics"] so it
    // never collides with the CTA section's reveal observer.
    use_effect(|| {
        let _ = dioxus::document::eval(REVEAL_SCRIPT);
    });

    rsx! {
        section { class: "flagship-metrics", aria_labelledby: "flagship-metrics-heading",
            div { class: "flagship-metrics-inner",
                p { class: "flagship-eyebrow", "Honest numbers" }
                h2 { id: "flagship-metrics-heading", class: "flagship-display-2",
                    "Built to ship."
                }
                div { class: "flagship-metrics-grid", "data-reveal-group": "metrics",
                    // Purely numeric metrics count up 0 -> value on first
                    // view. "60 fps" carries a unit suffix and "WebGPU" is a
                    // word, so those stay static text (count_to: None) — the
                    // existing MetricCounter behaviour.
                    div { class: "flagship-reveal-card", "data-reveal-index": "0",
                        MetricCounter {
                            label: "Components ready".to_string(),
                            value: "34".to_string(),
                            count_to: Some(34.0),
                            delta_text: Some("from the public prelude".to_string()),
                        }
                    }
                    div { class: "flagship-reveal-card", "data-reveal-index": "1",
                        MetricCounter {
                            label: "Frame target".to_string(),
                            value: "60 fps".to_string(),
                            delta_text: Some("scene clock + frame scheduler".to_string()),
                        }
                    }
                    div { class: "flagship-reveal-card", "data-reveal-index": "2",
                        MetricCounter {
                            label: "Platform adapters".to_string(),
                            value: "4".to_string(),
                            count_to: Some(4.0),
                            delta_text: Some("Web · Desktop · Mobile · Native".to_string()),
                        }
                    }
                    div { class: "flagship-reveal-card", "data-reveal-index": "3",
                        MetricCounter {
                            label: "Glass engine".to_string(),
                            value: "WebGPU".to_string(),
                            delta_text: Some("SVG and solid fallbacks built in".to_string()),
                        }
                    }
                }
            }
        }
    }
}

/// IntersectionObserver that reveals this section's
/// `.flagship-reveal-card` elements as they enter the viewport. The query
/// is scoped to `[data-reveal-group="metrics"]` so it observes only the
/// metric cards (the CTA section registers its own observer over its own
/// group). Each card's `data-reveal-index` drives a staggered
/// `transition-delay`; once revealed it is unobserved so the reveal fires
/// once. Self-gates on reduced motion: when the document prefers reduced
/// motion the cards are revealed immediately with no stagger (the CSS
/// transition is also suppressed under `[data-ui-motion=reduced]`, so this
/// is belt-and-suspenders).
const REVEAL_SCRIPT: &str = r#"
    const reduced =
        (window.matchMedia &&
         window.matchMedia("(prefers-reduced-motion: reduce)").matches) ||
        !!document.querySelector('[data-ui-motion="reduced"]');
    const cards = Array.from(
        document.querySelectorAll('[data-reveal-group="metrics"] .flagship-reveal-card')
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
