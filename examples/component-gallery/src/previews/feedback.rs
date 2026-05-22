use dioxus::prelude::*;
use kinetics::prelude::*;

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
    rsx! {
        Toast {
            tone: ToastTone::Success,
            title: "Report exported",
            description: "The PDF is ready.",
            action_label: "Open",
            dismiss_label: "Dismiss",
        }
    }
}

pub fn tooltip_preview() -> Element {
    rsx! {
        Tooltip {
            id: "net-revenue-tip",
            visible: true,
            trigger_label: "Net revenue",
            content: "Revenue after refunds and credits.",
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
