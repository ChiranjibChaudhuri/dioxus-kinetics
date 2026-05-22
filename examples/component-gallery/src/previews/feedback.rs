use dioxus::prelude::*;
use kinetics::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static TOAST_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, PartialEq)]
struct ToastInstance {
    id: u32,
    tone: ToastTone,
    title: &'static str,
    description: &'static str,
}

pub fn dialog_preview() -> Element {
    rsx! { DialogPreviewBody {} }
}

#[component]
fn DialogPreviewBody() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Dialog" }
                button {
                    class: "ui-button ui-button--primary",
                    r#type: "button",
                    onclick: move |_| open.set(true),
                    "Show dialog"
                }
            }
            Dialog {
                open: *open.read(),
                title: "Archive workspace",
                description: "Move this workspace out of active navigation.",
                body: "Team members can still request access later.",
                actions: vec!["Cancel".to_string(), "Move it".to_string()],
                on_dismiss: move |_| open.set(false),
                on_action: move |_action: String| open.set(false),
            }
        }
    }
}

pub fn toast_preview() -> Element {
    rsx! { ToastPreviewBody {} }
}

#[component]
fn ToastPreviewBody() -> Element {
    let mut toasts: Signal<Vec<ToastInstance>> = use_signal(Vec::new);

    let mut push = move |tone: ToastTone, title: &'static str, description: &'static str| {
        let id = TOAST_ID.fetch_add(1, Ordering::Relaxed);
        toasts.write().push(ToastInstance {
            id,
            tone,
            title,
            description,
        });
        let mut t = toasts;
        spawn(async move {
            #[cfg(target_arch = "wasm32")]
            {
                let promise = js_sys::Promise::new(&mut |resolve, _| {
                    let win = web_sys::window().unwrap();
                    let _ =
                        win.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 3000);
                });
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            }
            t.write().retain(|x| x.id != id);
        });
    };

    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Toast" }
                div { class: "gallery-demo-frame-transport",
                    button {
                        class: "ui-button ui-button--secondary",
                        r#type: "button",
                        onclick: move |_| push(ToastTone::Success, "Report exported", "The PDF is ready."),
                        "Trigger success"
                    }
                    button {
                        class: "ui-button ui-button--secondary",
                        r#type: "button",
                        onclick: move |_| push(ToastTone::Info, "Sync started", "Pulling the latest data."),
                        "Trigger info"
                    }
                    button {
                        class: "ui-button ui-button--secondary",
                        r#type: "button",
                        onclick: move |_| push(ToastTone::Warning, "Quota close", "You are at 92% of the plan."),
                        "Trigger warning"
                    }
                    button {
                        class: "ui-button ui-button--secondary",
                        r#type: "button",
                        onclick: move |_| push(ToastTone::Danger, "Export failed", "Retry or contact support."),
                        "Trigger error"
                    }
                }
            }
            div { class: "gallery-toast-stage",
                for t in toasts.read().iter() {
                    Toast {
                        key: "{t.id}",
                        tone: t.tone,
                        title: t.title,
                        description: t.description,
                        dismiss_label: "Dismiss",
                    }
                }
            }
        }
    }
}

pub fn tooltip_preview() -> Element {
    rsx! { TooltipPreviewBody {} }
}

#[component]
fn TooltipPreviewBody() -> Element {
    let mut visible = use_signal(|| false);
    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label", "Tooltip" }
                span { class: "gallery-demo-frame-elapsed", "Hover or focus the trigger" }
            }
            div {
                class: "gallery-demo-frame-body",
                onmouseenter: move |_| visible.set(true),
                onmouseleave: move |_| visible.set(false),
                onfocusin: move |_| visible.set(true),
                onfocusout: move |_| visible.set(false),
                Tooltip {
                    id: "net-revenue-tip",
                    visible: *visible.read(),
                    trigger_label: "Net revenue",
                    content: "Revenue after refunds and credits.",
                }
            }
        }
    }
}

pub fn empty_state_preview() -> Element {
    rsx! {
        EmptyState {
            title: "No reports yet",
            description: "Create a report to share performance with your team.",
            action_label: "Create report",
        }
    }
}
