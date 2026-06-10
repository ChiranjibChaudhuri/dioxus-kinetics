use dioxus::prelude::*;
use kinetics::prelude::*;

// ---------------------------------------------------------------------------
// Tour (+ Spotlight)
// ---------------------------------------------------------------------------

pub fn tour_preview() -> Element {
    rsx! { TourPreviewBody {} }
}

#[component]
fn TourPreviewBody() -> Element {
    let mut open = use_signal(|| false);
    let mut active = use_signal(|| 0usize);

    let steps = vec![
        TourStep::new(
            "tour-step-compose",
            "Compose anywhere",
            "This button starts a new report. The spotlight tracks it through resizes and scrolls.",
        )
        .with_target("tour-demo-compose"),
        TourStep::new(
            "tour-step-filters",
            "Refine the view",
            "Saved filters live here. Steps can place the callout above or below the target.",
        )
        .with_target("tour-demo-filters")
        .with_placement(TourPlacement::Top),
        TourStep::new(
            "tour-step-finish",
            "You're all set",
            "Steps without a target render as a centered dialog — handy for a closing summary.",
        ),
    ];

    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Three steps · anchored, flipped, centered" }
                Button {
                    onclick: move |_| {
                        active.set(0);
                        open.set(true);
                    },
                    "Start tour"
                }
            }
            div {
                style: "display:flex; gap: 12px; align-items: center;",
                button {
                    id: "tour-demo-compose",
                    class: "ui-button ui-button--primary",
                    r#type: "button",
                    "New report"
                }
                button {
                    id: "tour-demo-filters",
                    class: "ui-button ui-button--secondary",
                    r#type: "button",
                    "Saved filters"
                }
            }
            Tour {
                id: "gallery-tour".to_string(),
                open: *open.read(),
                steps,
                active: *active.read(),
                on_change: move |next: usize| active.set(next),
                on_dismiss: move |_| open.set(false),
            }
        }
    }
}
