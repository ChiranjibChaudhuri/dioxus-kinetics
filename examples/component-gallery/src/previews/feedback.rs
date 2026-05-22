use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn dialog_preview() -> Element {
    rsx! {
        Dialog {
            open: true,
            title: "Archive workspace",
            description: "Move this workspace out of active navigation.",
            body: "Team members can still request access later.",
            actions: vec!["Cancel".to_string(), "Move it".to_string()],
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
