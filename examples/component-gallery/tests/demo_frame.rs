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
