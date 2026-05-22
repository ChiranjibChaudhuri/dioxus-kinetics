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
