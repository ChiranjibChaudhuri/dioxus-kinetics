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
    let prefs = try_consume_context::<crate::controls::GalleryPrefs>();

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        if !*playing.read() {
            return;
        }
        let mut eval = dioxus::document::eval(&format!(
            r#"
                const start = performance.now();
                const dur = {duration_ms};
                let raf;
                const tick = (now) => {{
                    const t = now - start;
                    if (t >= dur) {{
                        dioxus.send(dur);
                        return;
                    }}
                    dioxus.send(t);
                    raf = requestAnimationFrame(tick);
                }};
                raf = requestAnimationFrame(tick);
            "#,
        ));
        spawn(async move {
            loop {
                match eval.recv::<f64>().await {
                    Ok(t) => {
                        let t_f32 = t as f32;
                        elapsed.set(t_f32);
                        if t_f32 >= duration_ms {
                            playing.set(false);
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    });

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
                            if let Some(prefs) = prefs {
                                if *prefs.motion.read() == crate::controls::MotionPref::Reduced {
                                    elapsed.set(duration_ms);
                                    playing.set(false);
                                    return;
                                }
                            }
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

#[component]
pub fn FlipFrame(label: &'static str, layout_a: Element, layout_b: Element) -> Element {
    let mut at_b = use_signal(|| false);

    rsx! {
        div { class: "gallery-demo-frame gallery-demo-frame--flip",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "{label}" }
                button {
                    class: "ui-button ui-button--secondary gallery-demo-frame-swap",
                    r#type: "button",
                    onclick: move |_| {
                        let now = *at_b.read();
                        at_b.set(!now);
                    },
                    "Swap layout"
                }
            }
            div { class: "gallery-demo-frame-body",
                if *at_b.read() {
                    {layout_b}
                } else {
                    {layout_a}
                }
            }
        }
    }
}
