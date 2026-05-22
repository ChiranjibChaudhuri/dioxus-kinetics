use dioxus::prelude::*;

#[test]
fn replay_frame_renders_label_and_replay_button() {
    use component_gallery::demo_frame::ReplayFrame;
    let html = dioxus_ssr::render_element(rsx! {
        ReplayFrame {
            label: "Demo",
            children: rsx! { p { "child" } },
        }
    });
    assert!(html.contains("Demo"));
    assert!(html.contains(">Replay<") || html.contains("aria-label=\"Replay\""));
    assert!(html.contains("child"));
}

#[test]
fn scrub_frame_renders_slider_play_pause_and_label() {
    use component_gallery::demo_frame::ScrubFrame;
    let html = dioxus_ssr::render_element(rsx! {
        ScrubFrame {
            duration_ms: 1000.0,
            fps: None,
            label: "Demo scrub",
            children: rsx! { p { "scrubbed" } },
        }
    });
    assert!(html.contains("Demo scrub"));
    assert!(html.contains(r#"type="range""#));
    assert!(html.contains("scrubbed"));
    assert!(html.contains(r#"max="1000""#));
}

#[test]
fn flip_frame_renders_swap_button_and_one_layout_at_a_time() {
    use component_gallery::demo_frame::FlipFrame;
    let html = dioxus_ssr::render_element(rsx! {
        FlipFrame {
            label: "Demo flip",
            layout_a: rsx! { p { "Layout A" } },
            layout_b: rsx! { p { "Layout B" } },
        }
    });
    assert!(html.contains("Demo flip"));
    assert!(html.contains("Swap"));
    // Initial state renders layout_a only.
    assert!(html.contains("Layout A"));
    assert!(!html.contains("Layout B"));
}
