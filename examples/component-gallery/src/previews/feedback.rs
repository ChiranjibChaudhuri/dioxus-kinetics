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
                actions: vec![
                    DialogAction::ghost("cancel", "Cancel"),
                    DialogAction::primary("archive", "Move it"),
                ],
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

pub fn alert_preview() -> Element {
    let tones = [
        (AlertTone::Neutral, "Neutral", "Cache warming up", "Some metrics may lag by a few minutes."),
        (AlertTone::Success, "Success", "Export complete", "The CSV is ready to download."),
        (AlertTone::Warning, "Warning", "Quota at 92%", "Plan auto-upgrades on Friday at midnight."),
        (AlertTone::Danger, "Danger", "Sync failed", "Two reports are out of date; retry to refresh."),
        (AlertTone::Info, "Info", "New feature available", "Try the redesigned billing dashboard."),
    ];
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            for (tone, label, title, description) in tones {
                div { class: "gallery-variant-tile",
                    span { class: "gallery-variant-label", "{label}" }
                    Alert { tone, title, description }
                }
            }
        }
    }
}

pub fn progress_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Determinate · 0%" }
                Progress { label: "Importing rows", value: 0.0, description: "0 / 12 400" }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Determinate · 65%" }
                Progress { label: "Importing rows", value: 0.65, description: "8 060 / 12 400" }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Determinate · 100%" }
                Progress { label: "Importing rows", value: 1.0, description: "12 400 / 12 400 complete" }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Indeterminate" }
                Progress { label: "Connecting…", description: "Waiting on the data broker." }
            }
        }
    }
}

pub fn popover_preview() -> Element {
    rsx! { PopoverPreviewBody {} }
}

#[component]
fn PopoverPreviewBody() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Popover {
            id: "filters-popover",
            open: *open.read(),
            side: PopoverSide::Bottom,
            on_open_change: move |next: bool| open.set(next),
            trigger: rsx! {
                Button { variant: ButtonVariant::Secondary, "Filters · 2 active" }
            },
            div { style: "display: grid; gap: 8px; min-width: 220px;",
                strong { "Filter by" }
                p { style: "margin: 0; color: var(--ui-muted-fg);",
                    "Replace this content with the consumer's filter form. The popover handles open/close + Escape; you supply the body."
                }
                Button { variant: ButtonVariant::Primary, "Apply" }
            }
        }
    }
}

pub fn skeleton_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Headline placeholder" }
                Skeleton { height: "20px".to_string(), width: "60%".to_string(), radius: "6px".to_string() }
            }
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Paragraph placeholder" }
                div { style: "display: grid; gap: 6px;",
                    Skeleton { height: "12px".to_string(), width: "100%".to_string(), radius: "4px".to_string() }
                    Skeleton { height: "12px".to_string(), width: "92%".to_string(), radius: "4px".to_string() }
                    Skeleton { height: "12px".to_string(), width: "78%".to_string(), radius: "4px".to_string() }
                }
            }
        }
    }
}
