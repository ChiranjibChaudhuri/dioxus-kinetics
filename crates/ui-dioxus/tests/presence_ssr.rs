use dioxus::prelude::*;
use ui_dioxus::{Presence, PresenceCue};

#[test]
fn presence_true_renders_content_with_data_attrs() {
    let html = dioxus_ssr::render_element(rsx! {
        Presence { present: true, cue: PresenceCue::Fade,
            p { "hello" }
        }
    });

    assert!(html.contains("data-presence-cue=\"fade\""), "got {html}");
    assert!(
        html.contains("data-presence-state=\"visible\""),
        "got {html}",
    );
    assert!(html.contains("--ui-presence-t: 1"), "got {html}");
    assert!(html.contains("hello"));
}

#[test]
fn presence_false_renders_nothing() {
    let html = dioxus_ssr::render_element(rsx! {
        Presence { present: false,
            p { "hidden" }
        }
    });

    assert!(!html.contains("data-presence-cue"), "got {html}");
    assert!(!html.contains("hidden"));
}

#[test]
fn presence_cue_serializes_to_data_attribute() {
    for (cue, expected) in [
        (PresenceCue::Fade, "fade"),
        (PresenceCue::Rise, "rise"),
        (PresenceCue::Slide, "slide"),
        (PresenceCue::Scale, "scale"),
    ] {
        let html = dioxus_ssr::render_element(rsx! {
            Presence { present: true, cue: cue, "x" }
        });
        assert!(
            html.contains(&format!("data-presence-cue=\"{expected}\"")),
            "missing cue {expected}: {html}",
        );
    }
}
