use dioxus::prelude::*;
use ui_dioxus::{KineticBox, KineticText, PresenceGate, TimelineScope};

#[test]
fn timeline_scope_and_kinetic_box_render_stable_attributes() {
    let html = dioxus_ssr::render_element(rsx! {
        TimelineScope { id: "dashboard-enter", autoplay: true,
            KineticBox { id: "metric-card", cue: "rise-in",
                "Revenue"
            }
        }
    });

    assert!(html.contains("ui-timeline-scope"));
    assert!(html.contains("data-timeline-id=\"dashboard-enter\""));
    assert!(html.contains("ui-kinetic-box"));
    assert!(html.contains("data-kinetic-id=\"metric-card\""));
    assert!(html.contains("data-motion-cue=\"rise-in\""));
}

#[test]
fn presence_gate_does_not_render_removed_children() {
    let html = dioxus_ssr::render_element(rsx! {
        PresenceGate { present: false,
            KineticText { id: "toast-copy", text: "Saved" }
        }
    });

    assert!(!html.contains("Saved"));
}
