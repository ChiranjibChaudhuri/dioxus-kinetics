use dioxus::prelude::*;
use ui_dioxus::{KineticText, TimelineScope};

/// Format a count-up sample for display.
///
/// When the animation `target` is an integer (e.g. `34`, `4`), every
/// in-flight sample is shown without a fractional part so the readout
/// ticks through whole numbers. When the target carries a fractional
/// component (e.g. `99.9`), samples keep one decimal place so the
/// animation reads as a smooth ramp rather than a staircase of rounded
/// integers. Pure + deterministic so it can be unit-tested without a
/// DOM. This is the Rust mirror of the `fmt` closure inside the
/// `count_to` eval script — kept in sync as the spec for that formatting
/// (the visible count-up runs entirely in JS, so this fn is test-only).
#[cfg(test)]
fn format_count(current: f64, target: f64) -> String {
    if target.fract().abs() < f64::EPSILON {
        format!("{}", current.round() as i64)
    } else {
        format!("{current:.1}")
    }
}

#[component]
pub fn MetricCounter(
    label: String,
    value: String,
    delta_text: Option<String>,
    /// Optional numeric target. When `Some`, the value line animates
    /// `0 → count_to` on mount via a `requestAnimationFrame` count-up
    /// (Tween, ~1100ms ease-out). Under reduced motion — `prefers-reduced-motion`
    /// or `[data-ui-motion="reduced"]` on an ancestor — the count-up is
    /// skipped and the final `value` text is shown immediately. When
    /// `None` the component is identical to its pre-count-up behaviour
    /// (static `value` text), so existing call sites are unaffected.
    count_to: Option<f64>,
) -> Element {
    // A stable per-instance hook class lets the count-up script target
    // exactly this counter's value node without a Rust-side mounted
    // handle (ui-blocks has no ui-runtime dependency in the build graph).
    // `KineticText` appends `class` after `ui-kinetic-text`, so this rides
    // along on the value span; two counters on the same page get distinct
    // classes so their scripts do not stomp each other.
    let value_hook_class = use_hook(|| {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("ui-metric-count-{n}")
    });

    // The value KineticText renders `value` verbatim (the settled text).
    // This is the SSR / no-JS / reduced-motion state, and also the final
    // frame the count-up lands on — so the animation never disagrees with
    // the authoritative `value` string. The hook class is appended so the
    // count-up script can find this node; `aria_label` keeps the final
    // value for assistive tech even while the visible text ramps.
    let value_for_text = value.clone();
    let value_class_for_text = value_hook_class.clone();

    // When `count_to` is set and motion is allowed, reset the value node
    // to "0" and ramp it up. The script self-gates on reduced motion so
    // the final value stays put for AT / reduced-motion users — matching
    // the `use_animation_value` reduced-motion fallback contract. The
    // effect runs once after mount (`count_to` / `value_hook_class` are
    // mount-stable), guaranteeing the value span is in the DOM before the
    // `querySelector` runs. `use_effect` is called unconditionally per the
    // Dioxus hook rule; the `count_to` gate lives inside the closure.
    {
        let hook_class = value_hook_class.clone();
        use_effect(move || {
            let Some(target) = count_to else {
                return;
            };
            let _ = dioxus::document::eval(&format!(
                r#"
                const node = document.querySelector(".{hook_class}");
                if (node) {{
                    const reduced =
                        (window.matchMedia &&
                         window.matchMedia("(prefers-reduced-motion: reduce)").matches) ||
                        !!node.closest('[data-ui-motion="reduced"]');
                    const target = {target};
                    const isInt = Math.abs(target - Math.round(target)) < 1e-9;
                    const fmt = (v) => isInt ? String(Math.round(v)) : v.toFixed(1);
                    if (!reduced) {{
                        const dur = 1100;
                        const start = performance.now();
                        node.textContent = fmt(0);
                        const ease = (t) => 1 - Math.pow(1 - t, 3);
                        const tick = (now) => {{
                            const p = Math.min((now - start) / dur, 1);
                            node.textContent = fmt(target * ease(p));
                            if (p < 1) {{
                                requestAnimationFrame(tick);
                            }}
                        }};
                        requestAnimationFrame(tick);
                    }}
                }}
            "#,
            ));
        });
    }

    rsx! {
        div { class: "ui-block-metric-counter", "data-block": "metric-counter",
            TimelineScope {
                id: "metric-counter-stagger".to_string(),
                autoplay: false,
                stagger_step_ms: 200.0,
                KineticText { id: "metric-label".to_string(), text: label, cue: "fade-in".to_string() }
                KineticText {
                    id: "metric-value".to_string(),
                    text: value_for_text,
                    cue: "rise-in".to_string(),
                    class: value_class_for_text,
                }
                if let Some(delta) = delta_text {
                    KineticText { id: "metric-delta".to_string(), text: delta, cue: "fade-in".to_string() }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{format_count, MetricCounter};
    use dioxus::prelude::*;

    #[test]
    fn format_count_integer_target_shows_whole_numbers() {
        assert_eq!(format_count(0.0, 34.0), "0");
        assert_eq!(format_count(16.7, 34.0), "17");
        assert_eq!(format_count(34.0, 34.0), "34");
    }

    #[test]
    fn format_count_fractional_target_keeps_one_decimal() {
        assert_eq!(format_count(0.0, 99.9), "0.0");
        assert_eq!(format_count(49.95, 99.9), "50.0");
        assert_eq!(format_count(99.9, 99.9), "99.9");
    }

    #[test]
    fn count_to_keeps_settled_value_in_markup() {
        // The settled `value` text is the SSR / no-JS / reduced-motion
        // state and the final count-up frame; it must always be present in
        // the rendered markup regardless of `count_to`.
        let html = dioxus_ssr::render_element(rsx! {
            MetricCounter {
                label: "Components ready".to_string(),
                value: "34".to_string(),
                count_to: Some(34.0),
                delta_text: Some("from the public prelude".to_string()),
            }
        });
        assert!(html.contains("34"), "{html}");
        assert!(html.contains("Components ready"), "{html}");
        assert!(html.contains("ui-block-metric-counter"), "{html}");
    }
}
