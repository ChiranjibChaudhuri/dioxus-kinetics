use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn text_field_preview() -> Element {
    rsx! {
        TextField {
            id: "workspace-name",
            label: "Workspace name",
            value: "Acme Ops",
            help_text: "Visible to teammates",
            leading_text: "Org",
        }
    }
}

pub fn checkbox_preview() -> Element {
    rsx! {
        Checkbox {
            id: "weekly-summary",
            label: "Send weekly summary",
            checked: true,
            description: "Every Monday morning",
        }
    }
}

pub fn switch_preview() -> Element {
    rsx! {
        Switch {
            id: "auto-renew",
            label: "Auto renew",
            checked: true,
            description: "Keep billing active",
        }
    }
}
