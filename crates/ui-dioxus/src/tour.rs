use dioxus::prelude::*;
use ui_core::roving::is_focus_id_safe;

use crate::overlays::focus_trap::{capture_opener, install_trap, restore_opener};
use crate::roving::focus_element_by_id;

// ---------------------------------------------------------------------------
// Step vocabulary
// ---------------------------------------------------------------------------

/// Where the tour callout sits relative to the spotlighted target.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TourPlacement {
    /// Below the target (default).
    #[default]
    Bottom,
    /// Above the target.
    Top,
    /// Centered in the viewport; used automatically when a step has no
    /// resolvable target.
    Center,
}

impl TourPlacement {
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Bottom => "bottom",
            Self::Top => "top",
            Self::Center => "center",
        }
    }
}

/// One step of a [`Tour`]: which element to spotlight (`target_id` is a DOM
/// id; empty means "no target, center the callout") and what to say about it.
#[derive(Clone, Debug, PartialEq)]
pub struct TourStep {
    pub id: String,
    pub target_id: String,
    pub title: String,
    pub body: String,
    pub placement: TourPlacement,
}

impl TourStep {
    pub fn new(id: impl Into<String>, title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            target_id: String::new(),
            title: title.into(),
            body: body.into(),
            placement: TourPlacement::default(),
        }
    }

    /// Spotlights the element with this DOM id while the step is active.
    pub fn with_target(mut self, target_id: impl Into<String>) -> Self {
        self.target_id = target_id.into();
        self
    }

    pub fn with_placement(mut self, placement: TourPlacement) -> Self {
        self.placement = placement;
        self
    }
}

// ---------------------------------------------------------------------------
// Pure helpers (unit-tested)
// ---------------------------------------------------------------------------

fn step_counter(active: usize, count: usize) -> String {
    format!(
        "Step {} of {}",
        active.min(count.saturating_sub(1)) + 1,
        count
    )
}

/// Effective placement for a step: steps without a target are always
/// centered, regardless of the requested placement.
fn effective_placement(step: &TourStep) -> TourPlacement {
    if step.target_id.is_empty() {
        TourPlacement::Center
    } else {
        step.placement
    }
}

/// Script that measures the spotlight target and writes its viewport rect
/// into CSS custom properties on the overlay element, so the cutout and the
/// callout track the target without any wasm-side geometry. Re-measures on
/// resize/scroll while the overlay is mounted (listeners are bound to the
/// overlay element and collected with it). Returns `None` when either id
/// fails the focus-id allowlist, refusing to interpolate unsafe strings
/// into JS.
fn spotlight_measure_script(overlay_id: &str, target_id: &str, padding: f32) -> Option<String> {
    if !is_focus_id_safe(overlay_id) || !is_focus_id_safe(target_id) {
        return None;
    }
    let padding = if padding.is_finite() {
        padding.clamp(0.0, 64.0)
    } else {
        0.0
    };
    Some(format!(
        r#"
        (function() {{
            const overlay = document.getElementById('{overlay_id}');
            if (!overlay) return;
            const measure = () => {{
                const target = document.getElementById('{target_id}');
                if (!target) {{
                    overlay.removeAttribute('data-anchored');
                    return;
                }}
                const r = target.getBoundingClientRect();
                overlay.style.setProperty('--ui-tour-x', (r.left - {padding}) + 'px');
                overlay.style.setProperty('--ui-tour-y', (r.top - {padding}) + 'px');
                overlay.style.setProperty('--ui-tour-w', (r.width + 2 * {padding}) + 'px');
                overlay.style.setProperty('--ui-tour-h', (r.height + 2 * {padding}) + 'px');
                overlay.setAttribute('data-anchored', 'true');
            }};
            measure();
            if (!overlay.__kineticsTourMeasure) {{
                overlay.__kineticsTourMeasure = measure;
                window.addEventListener('resize', measure, {{ passive: true }});
                window.addEventListener('scroll', measure, {{ passive: true, capture: true }});
            }} else {{
                overlay.__kineticsTourMeasure = measure;
            }}
        }})();
        "#
    ))
}

// ---------------------------------------------------------------------------
// Spotlight
// ---------------------------------------------------------------------------

/// A viewport scrim with a tracked cutout over the element whose DOM id is
/// `target_id`. The cutout is drawn with an oversized box-shadow, so the
/// dimmed surround needs no extra layers and the highlighted element keeps
/// its live interactivity. With an empty / unresolvable target the scrim
/// renders without a cutout. Children render above the scrim and inherit
/// the measured `--ui-tour-*` variables, so anchored callouts can position
/// themselves with pure CSS.
///
/// `Tour` composes this; it is exported for custom guidance surfaces.
#[component]
pub fn Spotlight(
    id: String,
    #[props(default)] target_id: String,
    #[props(default = true)] active: bool,
    #[props(default = 8.0)] padding: f32,
    on_dismiss: Option<EventHandler<()>>,
    #[props(default)] children: Element,
) -> Element {
    let overlay_id = format!("{id}-overlay");

    {
        let overlay_id = overlay_id.clone();
        let target_id = target_id.clone();
        use_effect(use_reactive!(|(overlay_id, target_id, active)| {
            if active && !target_id.is_empty() {
                if let Some(script) = spotlight_measure_script(&overlay_id, &target_id, padding) {
                    let _ = dioxus::document::eval(&script);
                }
            }
        }));
    }

    if !active {
        return rsx! {};
    }

    rsx! {
        div {
            id: "{overlay_id}",
            class: "ui-spotlight-overlay",
            onclick: move |_| {
                if let Some(handler) = &on_dismiss {
                    handler.call(());
                }
            },
            div { class: "ui-spotlight-cutout", "aria-hidden": "true" }
            {children}
        }
    }
}

// ---------------------------------------------------------------------------
// Tour
// ---------------------------------------------------------------------------

/// A step-by-step product tour: a [`Spotlight`] scrim plus an anchored
/// `role="dialog"` callout with Back / Next / Skip controls.
///
/// The component is controlled: `active` selects the current step,
/// `on_change` requests a different one, and `on_dismiss` fires for Skip,
/// Escape, scrim clicks, and Done on the final step. Focus moves into the
/// callout when it opens or the step changes, Tab cycles inside it, and the
/// opener regains focus on dismiss.
#[component]
pub fn Tour(
    id: String,
    open: bool,
    steps: Vec<TourStep>,
    #[props(default)] active: usize,
    on_change: EventHandler<usize>,
    on_dismiss: EventHandler<()>,
    #[props(default = "Next".to_string())] next_label: String,
    #[props(default = "Back".to_string())] back_label: String,
    #[props(default = "Done".to_string())] done_label: String,
    #[props(default = "Skip tour".to_string())] skip_label: String,
) -> Element {
    let count = steps.len();
    let panel_id = format!("{id}-panel");

    {
        let panel_id = panel_id.clone();
        use_effect(use_reactive!(|(panel_id, open, active)| {
            // `active` participates so focus returns to the panel after the
            // body re-renders for every step change, not just on open.
            let _ = active;
            if open {
                capture_opener(&panel_id);
                install_trap(&panel_id);
                focus_element_by_id(&panel_id);
            }
        }));
    }

    if !open || count == 0 {
        return rsx! {};
    }

    let active = active.min(count - 1);
    let step = &steps[active];
    let placement = effective_placement(step);
    let is_first = active == 0;
    let is_last = active + 1 == count;
    let title_id = format!("{id}-title");
    let body_id = format!("{id}-body");
    let panel_class = format!("ui-tour-panel ui-tour-panel--{}", placement.class_suffix());

    let dismiss_panel_id = panel_id.clone();
    let dismiss = move |_| {
        restore_opener(&dismiss_panel_id);
        on_dismiss.call(());
    };
    let dismiss_for_key = dismiss.clone();
    let dismiss_for_skip = dismiss.clone();
    let dismiss_for_scrim = dismiss.clone();

    rsx! {
        div { class: "ui-tour",
            Spotlight {
                id: id.clone(),
                target_id: step.target_id.clone(),
                active: true,
                on_dismiss: move |()| dismiss_for_scrim(()),
                div {
                    id: "{panel_id}",
                    class: "{panel_class}",
                    role: "dialog",
                    "aria-modal": "true",
                    "aria-labelledby": "{title_id}",
                    "aria-describedby": "{body_id}",
                    tabindex: "-1",
                    onclick: move |evt| evt.stop_propagation(),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Escape {
                            evt.prevent_default();
                            dismiss_for_key(());
                        }
                    },
                p { class: "ui-tour-counter", "{step_counter(active, count)}" }
                h3 { id: "{title_id}", class: "ui-tour-title", "{step.title}" }
                p { id: "{body_id}", class: "ui-tour-body", "{step.body}" }
                div { class: "ui-tour-actions",
                    button {
                        class: "ui-button ui-button--ghost ui-tour-skip",
                        r#type: "button",
                        onclick: move |_| dismiss_for_skip(()),
                        "{skip_label}"
                    }
                    div { class: "ui-tour-steps-nav",
                        if !is_first {
                            button {
                                class: "ui-button ui-button--secondary",
                                r#type: "button",
                                onclick: move |_| on_change.call(active.saturating_sub(1)),
                                "{back_label}"
                            }
                        }
                        button {
                            class: "ui-button ui-button--primary",
                            r#type: "button",
                            onclick: move |_| {
                                if is_last {
                                    dismiss(());
                                } else {
                                    on_change.call(active + 1);
                                }
                            },
                            if is_last { "{done_label}" } else { "{next_label}" }
                        }
                    }
                }
                div { class: "ui-tour-progress", "aria-hidden": "true",
                    for index in 0..count {
                        span {
                            class: if index == active {
                                "ui-tour-dot ui-tour-dot--active"
                            } else {
                                "ui-tour-dot"
                            },
                        }
                    }
                }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_counter_is_one_based_and_clamped() {
        assert_eq!(step_counter(0, 4), "Step 1 of 4");
        assert_eq!(step_counter(3, 4), "Step 4 of 4");
        assert_eq!(step_counter(99, 4), "Step 4 of 4");
    }

    #[test]
    fn placement_falls_back_to_center_without_target() {
        let anchored = TourStep::new("s1", "Title", "Body")
            .with_target("hero")
            .with_placement(TourPlacement::Top);
        assert_eq!(effective_placement(&anchored), TourPlacement::Top);

        let floating = TourStep::new("s2", "Title", "Body").with_placement(TourPlacement::Top);
        assert_eq!(effective_placement(&floating), TourPlacement::Center);
    }

    #[test]
    fn placement_class_suffixes() {
        assert_eq!(TourPlacement::Bottom.class_suffix(), "bottom");
        assert_eq!(TourPlacement::Top.class_suffix(), "top");
        assert_eq!(TourPlacement::Center.class_suffix(), "center");
    }

    #[test]
    fn measure_script_interpolates_both_ids() {
        let script = spotlight_measure_script("tour-overlay", "hero-cta", 8.0).unwrap();
        assert!(script.contains("getElementById('tour-overlay')"));
        assert!(script.contains("getElementById('hero-cta')"));
        assert!(script.contains("--ui-tour-x"));
        assert!(script.contains("data-anchored"));
    }

    #[test]
    fn measure_script_refuses_unsafe_ids() {
        assert!(spotlight_measure_script("a'); alert(1); ('", "hero", 8.0).is_none());
        assert!(spotlight_measure_script("tour", "x'); alert(1); ('", 8.0).is_none());
    }

    #[test]
    fn measure_script_clamps_padding() {
        let script = spotlight_measure_script("tour", "hero", f32::NAN).unwrap();
        assert!(script.contains("- 0"));
        let script = spotlight_measure_script("tour", "hero", 1000.0).unwrap();
        assert!(script.contains("64"));
    }
}
