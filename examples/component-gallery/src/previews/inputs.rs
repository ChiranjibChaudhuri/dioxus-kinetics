use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn text_field_preview() -> Element {
    rsx! { TextFieldPreviewBody {} }
}

#[component]
fn TextFieldPreviewBody() -> Element {
    let mut value = use_signal(|| "Acme Ops".to_string());
    rsx! {
        TextField {
            id: "workspace-name",
            label: "Workspace name",
            value: value.read().clone(),
            help_text: "Visible to teammates",
            leading_text: "Org",
            oninput: move |evt: FormEvent| value.set(evt.value()),
        }
    }
}

pub fn checkbox_preview() -> Element {
    rsx! { CheckboxPreviewBody {} }
}

#[component]
fn CheckboxPreviewBody() -> Element {
    let mut checked = use_signal(|| true);
    rsx! {
        Checkbox {
            id: "weekly-summary",
            label: "Send weekly summary",
            checked: *checked.read(),
            description: "Every Monday morning",
            onchange: move |evt: FormEvent| {
                // FormEvent.value() returns "true"/"false" for checkboxes
                checked.set(evt.value() == "true");
            },
        }
    }
}

pub fn switch_preview() -> Element {
    rsx! { SwitchPreviewBody {} }
}

#[component]
fn SwitchPreviewBody() -> Element {
    let mut checked = use_signal(|| true);
    rsx! {
        Switch {
            id: "auto-renew",
            label: "Auto renew",
            checked: *checked.read(),
            description: "Keep billing active",
            onchange: move |next: bool| checked.set(next),
        }
    }
}
