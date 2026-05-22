use dioxus::prelude::*;

#[component]
pub fn ReplayFrame(label: &'static str, children: Element) -> Element {
    let mut token = use_signal(|| 0u32);

    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "{label}" }
                button {
                    class: "ui-button ui-button--secondary gallery-demo-frame-replay",
                    r#type: "button",
                    "aria-label": "Replay",
                    onclick: move |_| token += 1,
                    "Replay"
                }
            }
            div { class: "gallery-demo-frame-body", key: "{token.read()}",
                {children}
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct ScrubElapsedMs(pub Signal<f32>);

#[derive(Clone, Copy)]
pub struct ScrubFps(pub u32);

#[component]
pub fn ScrubFrame(
    duration_ms: f32,
    fps: Option<u32>,
    label: &'static str,
    children: Element,
) -> Element {
    let mut elapsed = use_signal(|| 0.0_f32);
    let mut playing = use_signal(|| false);
    use_context_provider(|| ScrubElapsedMs(elapsed));
    use_context_provider(|| ScrubFps(fps.unwrap_or(30)));

    let max_str = format!("{:.0}", duration_ms);
    let value_str = format!("{:.0}", *elapsed.read());

    rsx! {
        div { class: "gallery-demo-frame gallery-demo-frame--scrub",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "{label}" }
                div { class: "gallery-demo-frame-transport",
                    button {
                        class: "ui-button ui-button--secondary gallery-demo-frame-play",
                        r#type: "button",
                        "aria-label": if *playing.read() { "Pause" } else { "Play" },
                        onclick: move |_| {
                            let now = *playing.read();
                            playing.set(!now);
                        },
                        if *playing.read() { "Pause" } else { "Play" }
                    }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "{max_str}",
                        step: "1",
                        value: "{value_str}",
                        oninput: move |evt| {
                            if let Ok(v) = evt.value().parse::<f32>() {
                                elapsed.set(v);
                            }
                        },
                    }
                    span { class: "gallery-demo-frame-elapsed", "{value_str} / {max_str} ms" }
                }
            }
            div { class: "gallery-demo-frame-body",
                {children}
            }
        }
    }
}
